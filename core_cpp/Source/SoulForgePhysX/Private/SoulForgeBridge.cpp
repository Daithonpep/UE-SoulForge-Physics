// ─── core_cpp/Source/SoulForgePhysX/Private/SoulForgeBridge.cpp ───────────────
#include "SoulForgeBridge.h"
#include "SoulForgePhysX.h"
#include "Misc/Paths.h"
#include "HAL/PlatformProcess.h"
#include "Logging/LogMacros.h"

DEFINE_LOG_CATEGORY_STATIC(LogSoulForgeBridge, Log, All);

// Definición real de los punteros (El "Láser")
DetonarNativoFunc CppDetonarNativo = nullptr;
GetEngineStateFunc CppGetEngineState = nullptr;
GetAllFragmentDataFunc CppGetAllFragmentData = nullptr;

namespace
{
    void* GMasterHandle = nullptr;

    // Símbolos para FFI
    using FnVoid = void(*)();
    using FnTick = int32_t(*)(float);
    typedef bool (*FnRegProxy)(const char*, float, float, float, float, float, float, float);
    using FnFree = void(*)(const char*);
    using FnSetPower = void(*)(float);

    struct CExplosionParams {
        double x, y, z;
        double energy;
        double radius;
        int64_t explosive_type;
        int64_t fragment_count;
    };

    using FnApplySett = void(*)(int32_t, bool);
    using FnPurge = int32_t(*)();
    using FnFloat = void(*)(float);

    FnVoid GInitPhysics = nullptr;
    FnTick GTickPhysics = nullptr;
    FnVoid GShutdownPhysics = nullptr;
    FnVoid GShutdownEngine = nullptr;
    FnRegProxy GRegisterProxy = nullptr;
    FnFree GFreeString = nullptr;
    FnSetPower GSetGlobalPower = nullptr;
    FnSetPower GSetGlobalChaos = nullptr;
    FnApplySett GApplySettings = nullptr;
    FnPurge GPurgeSleeping = nullptr;
    FnFloat GSetGroundZ = nullptr;
    typedef void(*_sf_limpiar_nucleo)();
    _sf_limpiar_nucleo GLimpiarNucleo = nullptr;
    typedef void (*FnSetMilitar)(const char*, int32_t, int32_t);
    FnSetMilitar GSetProxyMilitar = nullptr;
    typedef void (*FnDetMilitar)(const char*, CExplosionParams);
    FnDetMilitar GDetonarMilitar = nullptr;
    typedef void (*FnAddBulk)(const float*, int32_t, float, uint16_t);
    FnAddBulk GAddNodesBulk = nullptr;
    typedef bool (*FnRemoveProxy)(const char*);
    FnRemoveProxy GRemoveProxy = nullptr;
    typedef const FSoulForgeFragmentData* (*FnSimPreview)(const char*, float, int32_t, int32_t, int32_t*);
    FnSimPreview GSimularPreview = nullptr;

    struct ShockwaveDataFFI { float R; float S; };
    typedef ShockwaveDataFFI (*FnCalcOnda)(float);
    FnCalcOnda GCalcularOnda = nullptr;
    
    typedef void(*_sf_registrar_logger)(void(*)(const char*));
    _sf_registrar_logger GRegistrarLogger = nullptr;

    typedef void (*FnActivarPoder)(int32, float, float, float, float, float, float, float, float);
    FnActivarPoder GActivarPoder = nullptr;

    typedef void (*FnUpdatePowers)(const FActivePowerPayload*, int32, float);
    FnUpdatePowers GUpdateAllPowers = nullptr;

    typedef const float* (*FnGetNodePositions)(int32*);
    FnGetNodePositions GGetNodePositions = nullptr;

    typedef const float* (*FnGetNodeTemps)(int32*);
    FnGetNodeTemps GGetNodeTemperatures = nullptr;
    typedef EngineHandle* (*FnPECreate)(uint32_t);
    typedef void (*FnPEDestroy)(EngineHandle*);
    typedef void (*FnPETick)(EngineHandle*, float);
    typedef bool (*FnPEGetData)(EngineHandle*, struct FRenderData*);
    typedef int32_t (*FnPEAddNode)(EngineHandle*, float, float, float, float, uint16_t);
    typedef bool (*FnPEKame)(EngineHandle*, float, float, float, float, float, float, float);
    typedef void (*FnPESetGrav)(EngineHandle*, float, float, float);
    
    // El TRONO - ZERO COPY
    typedef void (*FnSetMasterPointer)(FVector*, int32_t);
    FnSetMasterPointer GSetMasterPointer = nullptr;

    FnPECreate GPE_Create = nullptr;
    FnPEDestroy GPE_Destroy = nullptr;
    FnPETick GPE_Tick = nullptr;
    FnPEGetData GPE_GetData = nullptr;
    FnPEAddNode GPE_AddNode = nullptr;
    FnPEKame GPE_ActivateKame = nullptr;
    FnPESetGrav GPE_SetGravity = nullptr;

    static void HandleRustLog(const char* MensajeRust)
    {
        FString Mensaje(UTF8_TO_TCHAR(MensajeRust));
        UE_LOG(LogTemp, Warning, TEXT("[RUST-CORE] %s"), *Mensaje);
    }

    bool LoadMasterLibrary()
    {
        if (GMasterHandle) return true;

        FString DllPath = FPaths::Combine(FPaths::ProjectPluginsDir(), TEXT("SoulForgePhysX/core_cpp/Binaries/Win64/soulforge_physx.dll"));
        GMasterHandle = FPlatformProcess::GetDllHandle(*DllPath);
        
        if (!GMasterHandle) {
            UE_LOG(LogSoulForgeBridge, Error, TEXT("FATAL: No se pudo cargar el Nucleo Maestro en %s"), *DllPath);
            return false;
        }

        GInitPhysics = (FnVoid)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_init_physics"));
        GTickPhysics = (FnTick)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_tick_physics"));
        GShutdownPhysics = (FnVoid)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_shutdown_physics"));
        GShutdownEngine = (FnVoid)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_shutdown_engine"));
        GRegisterProxy = (FnRegProxy)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_register_proxy"));
        if (!GRegisterProxy) {
            UE_LOG(LogSoulForgeBridge, Error, TEXT("[SF-Bridge] ¡FATAL! La DLL cargó, pero no se encontró la función 'sf_register_proxy'. Revisa el #[no_mangle] en Rust."));
        }
        
        GFreeString = (FnFree)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_free_string"));
        GSetGlobalPower = (FnSetPower)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_set_global_power"));
        GApplySettings = (FnApplySett)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_apply_settings"));
        GPurgeSleeping = (FnPurge)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_purge_sleeping_fragments"));
        GSetGroundZ = (FnFloat)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_set_ground_z"));
        GLimpiarNucleo = (_sf_limpiar_nucleo)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_limpiar_nucleo"));
        GSetProxyMilitar = (FnSetMilitar)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_set_proxy_militar"));
        GDetonarMilitar = (FnDetMilitar)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_detonar_militar"));
        GAddNodesBulk = (FnAddBulk)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_add_nodes_bulk"));
        GRemoveProxy = (FnRemoveProxy)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_remove_proxy"));
        GSimularPreview = (FnSimPreview)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_simular_preview"));
        GCalcularOnda = (FnCalcOnda)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_calcular_onda"));

        GRegistrarLogger = (_sf_registrar_logger)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_registrar_logger"));
        GActivarPoder = (FnActivarPoder)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_activar_poder"));
        GUpdateAllPowers = (FnUpdatePowers)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_update_all_powers"));
        GSetGlobalChaos = (FnSetPower)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_set_global_chaos"));
        
        // Zero-Copy Memory Pointers
        GSetMasterPointer = (FnSetMasterPointer)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_set_master_pointer"));
        GGetNodeTemperatures = (FnGetNodeTemps)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_get_node_temperatures"));
        
        GPE_Create = (FnPECreate)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_create"));
        GPE_Destroy = (FnPEDestroy)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_destroy"));
        GPE_Tick = (FnPETick)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_tick"));
        GPE_GetData = (FnPEGetData)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_get_render_data"));
        GPE_AddNode = (FnPEAddNode)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_add_node"));
        GPE_ActivateKame = (FnPEKame)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_activate_kamehameha"));
        GPE_SetGravity = (FnPESetGrav)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("physics_engine_set_gravity"));

        // Cargar Símbolos Láser
        CppDetonarNativo = (DetonarNativoFunc)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_detonar_nativo"));
        CppGetEngineState = (GetEngineStateFunc)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_get_engine_state"));
        CppGetAllFragmentData = (GetAllFragmentDataFunc)FPlatformProcess::GetDllExport(GMasterHandle, TEXT("sf_get_all_fragment_data"));

        if (!GInitPhysics) {
            UE_LOG(LogSoulForgeBridge, Error, TEXT("[SF-Bridge] ¡FATAL! No se encontró la función vital 'sf_init_physics'."));
        }

        return GInitPhysics != nullptr;
    }
}

// Global Static TArray for Zero-Copy
TArray<FVector> GMasterPositionsArray;
int32 GActiveNodeCount = 0;
static float LastTickFrameTimeGlobal = -1.0f;

void USoulForgeBridge::Initialize()
{
    static UWorld* LastInitializedWorld = nullptr;
    UWorld* CurrentWorld = nullptr;

    // Detectar el mundo de juego activo (PIE o Game)
    if (GEngine)
    {
        for (const FWorldContext& Context : GEngine->GetWorldContexts())
        {
            if (Context.WorldType == EWorldType::PIE || Context.WorldType == EWorldType::Game)
            {
                CurrentWorld = Context.World();
                break;
            }
        }
    }

    if (LoadMasterLibrary()) {
        // Solo inicializamos Rust si cambiamos de Mundo (Inicio de PIE)
        if (GInitPhysics && CurrentWorld != LastInitializedWorld) {
            GInitPhysics();
            LastInitializedWorld = CurrentWorld;
            
            // --- RESET DE TIEMPO (CRÍTICO PARA PIE) ---
            LastTickFrameTimeGlobal = -1.0f;

            // Setup Zero-Copy Memory for 100k nodes max by default
            const int32 MAX_MASTER_NODES = 100000;
            GMasterPositionsArray.SetNumUninitialized(MAX_MASTER_NODES);
            if (GSetMasterPointer) {
                GSetMasterPointer(GMasterPositionsArray.GetData(), MAX_MASTER_NODES);
                UE_LOG(LogSoulForgeBridge, Display, TEXT("[SF-Bridge] Zero-Copy Master Pointer Established (%d nodes)"), MAX_MASTER_NODES);
            }
            
            if (GRegistrarLogger)
            {
                GRegistrarLogger(HandleRustLog);
                UE_LOG(LogTemp, Display, TEXT("[SoulForge] Canal de telemetría con Rust ESTABLECIDO."));
            }
            
            UE_LOG(LogSoulForgeBridge, Display, TEXT("Nucleo Maestro SoulForge fusionado y activo (Nueva Sesión)."));
        }
    } else {
        UE_LOG(LogSoulForgeBridge, Error, TEXT("[SF-Bridge] ¡FATAL! Fallo crítico al cargar el Nucleo Maestro."));
    }
}

FEngineState USoulForgeBridge::GetPerformanceState()
{
    FEngineState State;
    FMemory::Memzero(&State, sizeof(FEngineState));
    if (CppGetEngineState) {
        CppGetEngineState(&State);
    }
    return State;
}

TArray<int32> USoulForgeBridge::FiltrarInstancias(const TArray<FVector>& Posiciones, FVector CamaraPos, float DistanciaVision)
{
    // La CppInsightFilter y FiltrarInstancias han sido deprecados
    // porque SoulForgeInsightBridge ahora usa el Núcleo Unificado Maestro
    // con sf_insight_filter_instances de manera independiente.
    return TArray<int32>();
}

int32 USoulForgeBridge::DetonarNativo(FString ProxyId, float Energy, int32 Preset, float FragLevel, TArray<FSoulForgeFragmentData>& OutData, int32 CantidadNodos)
{
    if (CppDetonarNativo != nullptr)
    {
        FTCHARToUTF8 ConvertId(*ProxyId);
        // Usamos dist=FragLevel y radius=150.0 (o el radio configurado)
        return (int32)CppDetonarNativo(ConvertId.Get(), Energy, (float)Preset, FragLevel, 150.0f, CantidadNodos);
    }
    
    return 0;
}

bool USoulForgeBridge::RegisterProxy(FString ProxyId, FVector Posicion, FVector HalfExtent, float Densidad)
{
    if (!GRegisterProxy) return false;

    FTCHARToUTF8 ConvertId(*ProxyId);
    return GRegisterProxy(ConvertId.Get(), Posicion.X, Posicion.Y, Posicion.Z, HalfExtent.X, HalfExtent.Y, HalfExtent.Z, Densidad);
}

bool USoulForgeBridge::AnclarObjeto(FString ProxyId, FVector Posicion)
{
    if (!GRegisterProxy) return false;
    
    FTCHARToUTF8 ConvertId(*ProxyId);
    // Usamos extents genéricos (50cm) y densidad estándar para actualización rápida de posición
    return GRegisterProxy(ConvertId.Get(), Posicion.X, Posicion.Y, Posicion.Z, 50.0f, 50.0f, 50.0f, 0.0024f);
}

bool USoulForgeBridge::BorrarProxyEnRust(FString ProxyId)
{
    if (!GRemoveProxy) return false;
    FTCHARToUTF8 ConvertId(*ProxyId);
    return GRemoveProxy(ConvertId.Get());
}

void USoulForgeBridge::Shutdown()
{
    if (GShutdownPhysics) GShutdownPhysics();
    if (GMasterHandle) {
        FPlatformProcess::FreeDllHandle(GMasterHandle);
        GMasterHandle = nullptr;
    }
}

void USoulForgeBridge::SetGlobalPower(float Multiplier)
{
    if (GSetGlobalPower) GSetGlobalPower(Multiplier);
}

void USoulForgeBridge::SetGlobalChaos(float ChaosFactor)
{
    if (GSetGlobalChaos) GSetGlobalChaos(ChaosFactor);
}

void USoulForgeBridge::ApplySettings(int32 NumThreads, bool bUseGPU)
{
    if (GApplySettings) GApplySettings(NumThreads, bUseGPU);
}

void USoulForgeBridge::SetGroundZ(float Z)
{
    if (GSetGroundZ) GSetGroundZ(Z);
}

const TArray<FVector>& USoulForgeBridge::GetMasterPositions(int32& OutCount)
{
    OutCount = GActiveNodeCount;
    return GMasterPositionsArray;
}

const float* USoulForgeBridge::GetNodePositions(int32* OutCount)
{
    if (GGetNodePositions) return GGetNodePositions(OutCount);
    if (OutCount) *OutCount = 0;
    return nullptr;
}

const float* USoulForgeBridge::GetNodeTemperatures(int32* OutCount)
{
    if (GGetNodeTemperatures) return GGetNodeTemperatures(OutCount);
    if (OutCount) *OutCount = 0;
    return nullptr;
}

void USoulForgeBridge::ActivarPoder(EPowerType Tipo, FVector Origen, FVector Direccion, float Intensidad, float Duracion)
{
    if (GActivarPoder)
    {
        GActivarPoder(
            (int32)Tipo,
            Origen.X, Origen.Y, Origen.Z,
            Direccion.X, Direccion.Y, Direccion.Z,
            Intensidad,
            Duracion
        );
    }
}

void USoulForgeBridge::UpdateAllPowers(const TArray<FActivePowerPayload>& Payloads, float DeltaTime)
{
    if (GUpdateAllPowers && Payloads.Num() > 0)
    {
        GUpdateAllPowers(Payloads.GetData(), Payloads.Num(), DeltaTime);
    }
}
void USoulForgeBridge::LimpiarNucleo()
{
    if (GLimpiarNucleo != nullptr)
    {
        GLimpiarNucleo();
        UE_LOG(LogTemp, Warning, TEXT("[SF-Bridge] Orden de limpieza enviada a Rust."));
    }
}

void USoulForgeBridge::SetProxyMilitar(FString ProxyId, EMaterialType Material, EPhysicsLOD LOD)
{
    if (GSetProxyMilitar)
    {
        FTCHARToUTF8 ConvertId(*ProxyId);
        GSetProxyMilitar(ConvertId.Get(), (int32)Material, (int32)LOD);
    }
}
void USoulForgeBridge::DetonarMilitar(FString ProxyId, FVector PosicionActual, float Energia, EExplosiveType Explosivo, int32 CantidadNodos, float Radio)
{
    if (GDetonarMilitar)
    {
        FTCHARToUTF8 ConvertId(*ProxyId);
        
        CExplosionParams Params;
        Params.x = (double)PosicionActual.X;
        Params.y = (double)PosicionActual.Y;
        Params.z = (double)PosicionActual.Z;
        Params.energy = (double)Energia;
        Params.radius = (double)Radio;
        Params.explosive_type = (int64_t)Explosivo;
        Params.fragment_count = (int64_t)CantidadNodos;

        GDetonarMilitar(ConvertId.Get(), Params);

        FString TipoTexto = TEXT("Desconocida");
        switch(Explosivo) {
            case EExplosiveType::Matrix: TipoTexto = TEXT("Matrix (Zero-G)"); break;
            case EExplosiveType::AgujeroNegro: TipoTexto = TEXT("Agujero Negro (Implosión)"); break;
            case EExplosiveType::TNT: TipoTexto = TEXT("TNT (Clásica)"); break;
            default: TipoTexto = TEXT("Especial"); break;
        }

        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Physics] Detonación [%s] enviada a Rust | Tipo: %s | Nodos: %d | Radio: %.1f"), *ProxyId, *TipoTexto, CantidadNodos, Radio);
    }
}

void USoulForgeBridge::SpawnNodesBulk(const TArray<FVector>& Posiciones, float Masa, int32 Material)
{
    if (GAddNodesBulk && Posiciones.Num() > 0)
    {
        // Pasamos el array directamente. FVector es {float X, Y, Z} que coincide con Rust f32 array [X,Y,Z]
        GAddNodesBulk((const float*)Posiciones.GetData(), Posiciones.Num(), Masa, (uint16_t)Material);
    }
}

TArray<FVector> USoulForgeBridge::GetExplosionPreview(FString ProxyId, float Energia, int32 ExplosiveType, int32 CantidadNodos)
{
    TArray<FVector> Trayectorias;
    if (GSimularPreview)
    {
        FTCHARToUTF8 ConvertId(*ProxyId);
        int32 Count = 0;
        const FSoulForgeFragmentData* RawData = GSimularPreview(ConvertId.Get(), Energia, ExplosiveType, CantidadNodos, &Count);
        
        if (RawData && Count > 0)
        {
            for (int32 i = 0; i < Count; ++i)
            {
                // En el ffi.rs de Rust, guardamos la velocidad en ScaleX, ScaleY, ScaleZ para el preview
                Trayectorias.Add(FVector(RawData[i].ScaleX, RawData[i].ScaleY, RawData[i].ScaleZ));
            }
        }
    }
    return Trayectorias;
}

FSoulForgeFragmentData* USoulForgeBridge::GetAllFragmentData(uint32 FilterHash, int32& OutCount)
{
    if (CppGetAllFragmentData)
    {
        return (FSoulForgeFragmentData*)CppGetAllFragmentData(FilterHash, &OutCount);
    }
    OutCount = 0;
    return nullptr;
}
FShockwaveData USoulForgeBridge::CalcularOnda(float Energia)
{
    FShockwaveData Data = {0.0f, 0.0f};
    if (GCalcularOnda)
    {
        ShockwaveDataFFI Result = GCalcularOnda(Energia);
        Data.RadiusCM = Result.R;
        Data.SpeedCMS = Result.S;
    }
    return Data;
}
EngineHandle* sf_physics_engine_create(uint32_t max_nodes) {
    if (GPE_Create) return GPE_Create(max_nodes);
    return nullptr;
}

void sf_physics_engine_destroy(EngineHandle* handle) {
    if (GPE_Destroy) GPE_Destroy(handle);
}

void sf_physics_engine_tick(EngineHandle* handle, float dt) {
    if (GPE_Tick) GPE_Tick(handle, dt);
}

void USoulForgeBridge::TickPhysics(float DeltaSeconds)
{
    if (!GTickPhysics) return;

    // --- PROTECCIÓN ANTI-CONGELAMIENTO (TICK ÚNICO) ---
    // Si tenemos varios actores, todos intentarán tickear el motor.
    // Esto asegura que la simulación solo avance UNA VEZ por frame de Unreal.
    static float LastTickFrameTime = -1.0f;
    UWorld* World = nullptr;
    
    // Obtenemos el mundo de forma segura
    for (const FWorldContext& Context : GEngine->GetWorldContexts())
    {
        if (Context.WorldType == EWorldType::PIE || Context.WorldType == EWorldType::Game)
        {
            World = Context.World();
            break;
        }
    }

    if (World)
    {
        float CurrentTime = World->GetTimeSeconds();
        if (CurrentTime <= LastTickFrameTimeGlobal) 
        {
            return; // Ya hemos avanzado la física en este frame
        }
        LastTickFrameTimeGlobal = CurrentTime;

        GActiveNodeCount = GTickPhysics(DeltaSeconds);
    }
}

bool sf_physics_engine_get_render_data(EngineHandle* handle, struct FRenderData* out_data) {
    if (GPE_GetData) return GPE_GetData(handle, out_data);
    return false;
}

int32_t sf_physics_engine_add_node(EngineHandle* handle, float x, float y, float z, float mass, uint16_t material) {
    if (GPE_AddNode) return GPE_AddNode(handle, x, y, z, mass, material);
    return -1;
}

bool sf_physics_engine_activate_kamehameha(EngineHandle* handle, float ox, float oy, float oz, float dx, float dy, float dz, float intensity) {
    if (GPE_ActivateKame) return GPE_ActivateKame(handle, ox, oy, oz, dx, dy, dz, intensity);
    return false;
}

void sf_physics_engine_set_gravity(EngineHandle* handle, float x, float y, float z) {
    if (GPE_SetGravity) GPE_SetGravity(handle, x, y, z);
}

void USoulForgeBridge::SpawnNodeOptimized(FVector Pos, float Mass, int32 Material)
{
    // Usamos el handle global o el singleton de Rust
    sf_physics_engine_add_node(nullptr, Pos.X, Pos.Y, Pos.Z, Mass, (uint16_t)Material);
}
