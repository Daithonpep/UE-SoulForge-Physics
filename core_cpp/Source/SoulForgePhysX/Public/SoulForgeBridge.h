// ─── core_cpp/Source/SoulForgePhysX/Public/SoulForgeBridge.h ──────────────────
#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "SoulForgeBridge.generated.h"

// ==================================================
// 1. LOS ENUMS VAN AQUÍ ARRIBA (Global Scope)
// ==================================================

/** Presets de daño disponibles para DestroyProxy. */
UENUM(BlueprintType)
enum class ESoulForgeDamagePreset : uint8
{
    Explosion    UMETA(DisplayName = "Explosion (Radial)"),
    Collapse     UMETA(DisplayName = "Collapse (Gravity)"),
    Impact       UMETA(DisplayName = "Impact (Cone)"),
    ShapedCharge UMETA(DisplayName = "Shaped Charge (Penetrator)"),
    RealityShatter UMETA(DisplayName = "Reality Shatter (Epic)"),
};

UENUM(BlueprintType)
enum class EPhysicsLOD : uint8
{
    Primary     UMETA(DisplayName = "Primario (Grady-Kipp)"),
    Secondary   UMETA(DisplayName = "Secundario (Gurney)"),
    Tertiary    UMETA(DisplayName = "Terciario (Cinematográfico)")
};

UENUM(BlueprintType)
enum class EMaterialType : uint8
{
    Steel4340    UMETA(DisplayName = "Acero 4340 (Militar)"),
    Aluminum6061 UMETA(DisplayName = "Aluminio 6061"),
    Concrete35MPa UMETA(DisplayName = "Concreto 35MPa"),
    WoodOak      UMETA(DisplayName = "Madera de Roble"),
    GlassTempered UMETA(DisplayName = "Vidrio Templado")
};

UENUM(BlueprintType)
enum class EExplosiveType : uint8
{
    TNT         UMETA(DisplayName = "TNT (Explosión Convencional)"),
    AgujeroNegro UMETA(DisplayName = "Agujero Negro (Implosión Épica)"),
    RDX         UMETA(DisplayName = "RDX (Cañón de Riel)"),
    PETN        UMETA(DisplayName = "PETN (Carga Hueca Táctica)"),
    Matrix      UMETA(DisplayName = "Matrix (Gravedad Cero)"),
    Collapse    UMETA(DisplayName = "Colapso (Gravedad Real)"),
    RealityShatter UMETA(DisplayName = "Realidad Fracturada (Épico)")
};

UENUM(BlueprintType)
enum class EPowerType : uint8
{
    EnergyBeam       UMETA(DisplayName = "Kamehameha / Rayo de Energía"),
    WaterBending     UMETA(DisplayName = "Control de Agua"),
    FireBending      UMETA(DisplayName = "Piroquinesis"),
    EarthBending     UMETA(DisplayName = "Control de Tierra"),
    AirBending       UMETA(DisplayName = "Control de Aire"),
    Lightning        UMETA(DisplayName = "Rayo / Electricidad"),
    GravityZero      UMETA(DisplayName = "Gravedad Cero"),
    GravityBlackHole UMETA(DisplayName = "Agujero Negro"),
    IceControl       UMETA(DisplayName = "Control de Hielo"),
    // Combos
    FireStorm        UMETA(DisplayName = "Tormenta de Fuego"),
    Electrocution    UMETA(DisplayName = "Electrocución Masiva"),
    LavaBurst        UMETA(DisplayName = "Explosión de Lava"),
    Blizzard         UMETA(DisplayName = "Ventisca Ártica"),
    Supernova        UMETA(DisplayName = "Supernova Gravitacional")
};

// ==================================================
// 2. ESTRUCTURAS DE MEMORIA COMPARTIDA
// ==================================================

USTRUCT(BlueprintType)
struct FSoulForgeFragmentData {
    GENERATED_BODY()
    UPROPERTY(BlueprintReadOnly) float X;
    UPROPERTY(BlueprintReadOnly) float Y;
    UPROPERTY(BlueprintReadOnly) float Z;
    UPROPERTY(BlueprintReadOnly) float Pitch;
    UPROPERTY(BlueprintReadOnly) float Yaw;
    UPROPERTY(BlueprintReadOnly) float Roll;
    UPROPERTY(BlueprintReadOnly) float ScaleX;
    UPROPERTY(BlueprintReadOnly) float ScaleY;
    UPROPERTY(BlueprintReadOnly) float ScaleZ;
    UPROPERTY(BlueprintReadOnly) int32 Category;
};

USTRUCT(BlueprintType)
struct FEngineState {
    GENERATED_BODY()
    UPROPERTY(BlueprintReadOnly, Category = "Stats") float ActiveNodes;
    UPROPERTY(BlueprintReadOnly, Category = "Stats") float FPS;
    UPROPERTY(BlueprintReadOnly, Category = "Stats") float Efficiency;
    UPROPERTY(BlueprintReadOnly, Category = "Stats") float SimTime;
};

USTRUCT(BlueprintType)
struct FShockwaveData {
    GENERATED_BODY()
    UPROPERTY(BlueprintReadOnly) float RadiusCM;
    UPROPERTY(BlueprintReadOnly) float SpeedCMS;
};

/** Payload plano para envío masivo de poderes a Rust */
USTRUCT(BlueprintType)
struct FActivePowerPayload {
    GENERATED_BODY()

    UPROPERTY(BlueprintReadWrite) int32 PowerType;
    UPROPERTY(BlueprintReadWrite) FVector Origin;
    UPROPERTY(BlueprintReadWrite) FVector Direction;
    UPROPERTY(BlueprintReadWrite) float Intensity;
    UPROPERTY(BlueprintReadWrite) float Phase;
};

/** Datos compactos para el renderizado masivo */
USTRUCT(BlueprintType)
struct FRenderData {
    GENERATED_BODY()
    UPROPERTY(BlueprintReadOnly) int32 Count;
    const float* PositionsPtr;
    const uint32* ColorsPtr;
    const float* ScalesPtr;
};

USTRUCT(BlueprintType)
struct FActivePowerInstance {
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite) EPowerType Type;
    UPROPERTY(EditAnywhere, BlueprintReadWrite) FVector Origin;
    UPROPERTY(EditAnywhere, BlueprintReadWrite) FVector Direction;
    UPROPERTY(EditAnywhere, BlueprintReadWrite) float Intensity;
    UPROPERTY(EditAnywhere, BlueprintReadWrite) float TimeRemaining;
    UPROPERTY(EditAnywhere, BlueprintReadWrite) float TotalDuration;
};

DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam(FOnPowerVisualEffect, float, BloomIntensity);

// ==================================================
// 3. CONTRATO FFI
// ==================================================

typedef void (*DetonarNativoFunc)(const char*, float, float, float, float, int32_t);
typedef void (*GetEngineStateFunc)(float*);
typedef void (*InsightFilterFunc)(int32*, int32);
typedef struct FSoulForgeFragmentData* (*GetAllFragmentDataFunc)(uint32, int32*);

extern "C"
{
    // Motor Base
    void sf_init_physics();
    void sf_tick_physics(float delta_seconds);
    void sf_shutdown_physics();
    void sf_shutdown_engine();

    // Proxies
    bool sf_register_proxy(const char* id, float pos_x, float pos_y, float pos_z, float ext_x, float ext_y, float ext_z, float density);
    const char* sf_destroy_proxy(const char* proxy_id, float energy, float dir_x, float dir_y, float dir_z);

    // Fast-Path (Láser)
    void sf_detonar_nativo(const char* proxy_id, float energy, float dir_x, float dir_y, float dir_z, int32_t fragment_count);
    void sf_get_engine_state(float* estado_ptr);
    void sf_insight_filter(int32* array_ptr, int32 count);
    struct FSoulForgeFragmentData* sf_get_all_fragment_data(uint32_t filter_hash, int32* out_count);

    // Fragmentos
    bool sf_get_fragment_pos(uint32_t id, float* out_x, float* out_y, float* out_z);
    void sf_set_ground_z(float z);
    int32_t sf_purge_sleeping_fragments();
    void sf_set_global_power(float multiplier);
    void sf_apply_settings(int32_t num_threads, bool use_gpu);
    void sf_set_proxy_militar(const char* proxy_id, int32_t material_type, int32_t lod_level);
    void sf_detonar_militar(const char* proxy_id, float x, float y, float z, float energy, int32_t explosive_type, int32_t fragment_count);
    void sf_add_nodes_bulk(const float* positions, int32_t count, float mass, uint16_t material);
    const FSoulForgeFragmentData* sf_simular_preview(const char* proxy_id, float energy, int32_t explosive_type, int32_t fragment_count, int32_t* out_count);
    void sf_limpiar_nucleo();
    void sf_registrar_logger(void(*callback)(const char*));
    void sf_free_string(const char* ptr);
    bool sf_remove_proxy(const char* proxy_id);
    // --- API OPTIMIZADO (ISMC) ---
    struct EngineHandle;
    struct RenderData;
    
    EngineHandle* sf_physics_engine_create(uint32_t max_nodes);
    void sf_physics_engine_destroy(EngineHandle* handle);
    void sf_physics_engine_tick(EngineHandle* handle, float dt);
    bool sf_physics_engine_get_render_data(EngineHandle* handle, struct FRenderData* out_data);
    int32_t sf_physics_engine_add_node(EngineHandle* handle, float x, float y, float z, float mass, uint16_t material);
    bool sf_physics_engine_activate_kamehameha(EngineHandle* handle, float ox, float oy, float oz, float dx, float dy, float dz, float intensity);
    void sf_physics_engine_set_gravity(EngineHandle* handle, float x, float y, float z);
}

// ==================================================
// 4. CLASE PUENTE
// ==================================================

UCLASS(Blueprintable, BlueprintType)
class SOULFORGEPHYSX_API USoulForgeBridge : public UObject
{
    GENERATED_BODY()

public:

    /** 🔌 INICIALIZACIÓN UNIFICADA */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Core")
    static void Initialize();

    /** 📊 TELEMETRÍA DE ALTA VELOCIDAD */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Stats")
    static FEngineState GetPerformanceState();

    /** 👁️ INSIGHT FILTER (Culling Nativo) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Insight")
    static TArray<int32> FiltrarInstancias(const TArray<FVector>& Posiciones, FVector CamaraPos, float DistanciaVision);

    /** 💥 DESTRUCCIÓN NATIVA */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Proxy")
    static int32 DetonarNativo(FString ProxyId, float Energy, int32 Preset, float FragLevel, TArray<FSoulForgeFragmentData>& OutData, int32 CantidadNodos = 0);

    // Métodos Legacy / Auxiliares
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Proxy")
    static bool RegisterProxy(FString ProxyId, FVector Center, FVector HalfExtent, float Density = 0.0024f);

    /** Actualiza la posición del objeto en Rust antes de la detonación (GPS Update) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Proxy")
    static bool AnclarObjeto(FString ProxyId, FVector Posicion);

    UFUNCTION(BlueprintCallable, Category = "SoulForge|Proxy")
    static bool BorrarProxyEnRust(FString ProxyId);

    UFUNCTION(BlueprintCallable, Category = "SoulForge|PhysX")
    static void TickPhysics(float DeltaSeconds);

    UFUNCTION(BlueprintCallable, Category = "SoulForge|Core")
    static void Shutdown();

    UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
    static void SetGlobalPower(float Multiplier);

    /** Ajusta qué tan deformados/caóticos son los fragmentos (0.0 = Cubos perfectos, 1.0 = Caos total) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
    static void SetGlobalChaos(float ChaosFactor);

    /** Limpia la memoria del núcleo de Rust */
    UFUNCTION(BlueprintCallable, Category = "Soul Forge|Core")
    static void LimpiarNucleo();

    /** Configura el rendimiento (Hilos y GPU) del núcleo */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Rendimiento")
    static void ApplySettings(int32 NumThreads, bool bUseGPU);

    /** Configura las propiedades militares de un proxy */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Core")
    static void SetProxyMilitar(FString ProxyId, EMaterialType Material, EPhysicsLOD LOD);

    /** Detonación con actualización de GPS en tiempo real y número de fragmentos configurable */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Proxy")
    static void DetonarMilitar(FString ProxyId, FVector PosicionActual, float Energia, EExplosiveType Explosivo, int32 CantidadNodos = 0, float Radio = 250.0f);

    /** Inyección masiva de nodos para evitar lag en el FFI (1000+ nodos en una sola llamada) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|PhysX")
    static void SpawnNodesBulk(const TArray<FVector>& Posiciones, float Masa = 1.0f, int32 Material = 0);

    /** 🧪 LABORATORIO DE PREVISUALIZACIÓN */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Lab")
    static TArray<FVector> GetExplosionPreview(FString ProxyId, float Energia, int32 ExplosiveType, int32 CantidadNodos = 0);

    /** Calcula datos físicos de la onda expansiva usando Rust */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Physics")
    static FShockwaveData CalcularOnda(float Energia);

    /** Configura la altura del suelo en el motor de Rust */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|PhysX")
    static void SetGroundZ(float Z);

    /** [Zero-Copy] Obtiene el puntero directo al Master Buffer de posiciones para Niagara */
    static const TArray<FVector>& GetMasterPositions(int32& OutCount);

    /** [Deprecated] Obtiene el puntero directo a las posiciones (float interleavado) */
    static const float* GetNodePositions(int32* OutCount);

    /** Obtiene el puntero directo a las temperaturas de los nodos (1 float por nodo) */
    static const float* GetNodeTemperatures(int32* OutCount);

    /** Activa un poder elemental en el motor físico */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Powers")
    static void ActivarPoder(EPowerType Tipo, FVector Origen, FVector Direccion, float Intensidad, float Duracion);

    /** Actualización masiva de todos los poderes activos en un solo viaje FFI */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Powers")
    static void UpdateAllPowers(const TArray<FActivePowerPayload>& Payloads, float DeltaTime);

    /** Acceso directo a la memoria compartida de Rust */
    static FSoulForgeFragmentData* GetAllFragmentData(uint32 FilterHash, int32& OutCount);

    /** 🚀 NUEVA API DE NODOS */
    UFUNCTION(BlueprintCallable, Category = "SoulForge")
    static void SpawnNodeOptimized(FVector Pos, float Mass = 1.0f, int32 Material = 0);
};
