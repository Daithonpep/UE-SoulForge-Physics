// ─── SoulForgeDestructible.cpp ────────────────────────────────────────────────

#include "SoulForgeDestructible.h"
#include "SoulForgeBridge.h"
#include "GameFramework/Actor.h"
#include "Components/StaticMeshComponent.h"
#include "Engine/World.h"
#include "DrawDebugHelpers.h"
#include "SoulForgeFragmentRenderer.h"
#include "Components/InstancedStaticMeshComponent.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "SoulForgeVFXLibrary.h"
#include "NiagaraComponent.h"
#include "NiagaraDataInterfaceArrayFunctionLibrary.h"
#include "Kismet/GameplayStatics.h"

// ─── Constructor ──────────────────────────────────────────────────────────────

USoulForgeDestructible::USoulForgeDestructible()
{
    PrimaryComponentTick.bCanEverTick = true;
    PrimaryComponentTick.bStartWithTickEnabled = true;

    // Esto hace que la sección "Tick" aparezca en el panel de detalles de Unreal
    bAutoActivate = true;
}

void USoulForgeDestructible::DeactivateOriginal()
{
    if (!GetOwner()) return;

    TArray<UStaticMeshComponent*> MeshComponents;
    GetOwner()->GetComponents<UStaticMeshComponent>(MeshComponents);

    for (UStaticMeshComponent* SMC : MeshComponents)
    {
        // NO ocultamos el renderer de fragmentos ni sus hijos HISM
        if (SMC == FragmentRenderer || SMC->IsA<UInstancedStaticMeshComponent>()) 
        {
            continue;
        }

        SMC->SetSimulatePhysics(false); 
        SMC->SetCollisionEnabled(ECollisionEnabled::NoCollision);

        // REGLA DE ORO DE LA FORJA: 
        // Si el component es la RAÍZ, no lo ocultamos (eso ocultaría a sus hijos de forma jerárquica).
        // En su lugar, simplemente removemos la mallas estática de la vista.
        if (SMC == GetOwner()->GetRootComponent())
        {
            SMC->SetStaticMesh(nullptr);
            UE_LOG(LogTemp, Warning, TEXT("[SoulForge] Root Mesh removido para preservar hijos."));
        }
        else
        {
            SMC->SetVisibility(false, false); 
            SMC->SetHiddenInGame(true);
            UE_LOG(LogTemp, Log, TEXT("[SoulForge-Cleanup] Mesh secundario oculto: %s"), *SMC->GetName());
        }
    }
}

// ─── OnRegister: El "Cerebro Total" basado en Nombre ────────────────────────────
void USoulForgeDestructible::OnRegister()
{
    Super::OnRegister();

    if (GetOwner())
    {
        // 1. Obtenemos el nombre del "Outliner" (ej: "Muro_01")
        FString MiNombre = GetOwner()->GetActorLabel();
        if (MiNombre.IsEmpty()) MiNombre = GetOwner()->GetName();

        // 2. Guardamos este nombre como nuestra identidad persistente
        this->ProxyName = MiNombre;

        UE_LOG(LogTemp, Display, TEXT("[SoulForge] Identidad 'Smarter' establecida: %s"), *ProxyName);
    }
}

// ─── PostInitProperties ──────────────────────────────────────────────────────
void USoulForgeDestructible::PostInitProperties()
{
    Super::PostInitProperties();
}

// ─── BeginPlay: Registro en Rust ─────────────────────────────────────────────

void USoulForgeDestructible::BeginPlay()
{
    Super::BeginPlay();

    // --- RESET TOTAL DE ESTADO DE SESIÓN (CRÍTICO) ---
    this->ProxyId = 0;
    this->bIsEngineReady = false;
    this->bDestroyed = false;
    this->Health = 1.0f;

    // --- EL PUENTE MANUAL DE ENCENDIDO ---
    USoulForgeBridge::Initialize();
    
    ProxyName = GetOwner()->GetName();

    // --- AUTO-CONFIGURACIÓN DE GEOMETRÍA REAL ---
    if (UStaticMeshComponent* SMC = GetOwner()->FindComponentByClass<UStaticMeshComponent>())
    {
        HalfExtent = SMC->Bounds.BoxExtent;
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge] Auto-detectado HalfExtent: %s"), *HalfExtent.ToString());
    }

    // Hash para el filtrado Zero-Copy
    auto GetSoulForgeStableHash = [](const FString& s) -> uint32 {
        uint32 h = 2166136261u;
        FTCHARToUTF8 Convert(*s);
        const char* ptr = Convert.Get();
        while (ptr && *ptr) {
            h = h ^ (uint32)((uint8)*ptr);
            h = h * 16777619u;
            ptr++;
        }
        return h;
    };
    ObjectFilterHash = GetSoulForgeStableHash(ProxyName);

    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Link] Identidad: %s | Hash de Filtrado: %u"), *ProxyName, ObjectFilterHash);

    // 1. Buscamos el Renderer
    if (!FragmentRenderer)
    {
        FragmentRenderer = GetOwner()->FindComponentByClass<USoulForgeFragmentRenderer>();
    }

    // 2. RESERVAMOS LA RAM (Soporte per-actor)
    NodePool.SetNumUninitialized(MaxNodos);
    for (int32 i = 0; i < MaxNodos; i++) {
        NodePool[i].bEstaActivo = 0;
        NodePool[i].Escala3D = FVector(1.0f);
    }

    // -------------------------------------

    // Sincronizamos las propiedades militares antes de la simulación
    USoulForgeBridge::SetProxyMilitar(ProxyName, MaterialType, PhysicsLOD);
    // -------------------------------------

    // Ahora sí, Rust está despierto y debería darnos un ProxyId válido (0, 1, 2...)
    AutoLinkToRust();

    // 4. PREPARAMOS EL MATERIAL (Copiamos textura del original)
    PreparaRenderizadoFragmentos();
}

// ─── Tick ──────────────────────────────────────────────────────────────

void USoulForgeDestructible::TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction)
{
    Super::TickComponent(DeltaTime, TickType, ThisTickFunction);

    // 1. Reloj de la física – guard para que solo corra 1 vez por frame
    {
        static uint64 LastFrameTicked = 0;
        uint64 CurrentFrame = GFrameCounter;
        if (CurrentFrame != LastFrameTicked)
        {
            LastFrameTicked = CurrentFrame;
            USoulForgeBridge::TickPhysics(DeltaTime);
        }
    }

    // 2. Niagara
    UpdateNiagaraVisuals();

    // 2.5 TELEMETRÍA LÁSER (HUD) - Visualización en el Laboratorio
    if (bActivarHUD && GEngine)
    {
        FEngineState PerfState = USoulForgeBridge::GetPerformanceState();
        
        FString HUDMsg = FString::Printf(TEXT("[SOULFORGE] Nodos: %.0f | Core FPS: %.0f | Sim: %.1fs | Capacidad: %.0f%%"), 
                                        PerfState.ActiveNodes, PerfState.FPS, PerfState.SimTime, PerfState.Efficiency * 100.0f);
        GEngine->AddOnScreenDebugMessage(PersistentDNI, 0.0f, FColor::Cyan, HUDMsg);
    }

    // 3. Renderizado Rústico (HISMs)
    if (bUsarRenderizadoRustico && FragmentRenderer)
    {
        int32 RustCount = 0;
        // UNI-CORE: Pasamos ObjectFilterHash para ver SOLO los fragmentos de este actor
        FSoulForgeFragmentData* RustData = USoulForgeBridge::GetAllFragmentData(ObjectFilterHash, RustCount);

        if (RustCount > 0) 
        {
            // --- DIAGNÓSTICO EXTREMO (V8.8) ---
            static int32 DebugSkip = 0;
            bool bShouldLog = (DebugSkip++ % 120 == 0); // Cada ~2 segundos

            if (bShouldLog) {
                UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Insight] Actor: %s | Nodos en memoria Rust: %d"), *GetOwner()->GetName(), RustCount);
            }

            // Agrupamos por categoría (0, 1, 2, 3)
            TArray<FTransform> Grouped[4];
            
            for (int32 i = 0; i < RustCount; ++i)
            {
                int32 Cat = FMath::Clamp(RustData[i].Category, 0, 3);
                
                FVector Pos(RustData[i].X, RustData[i].Y, RustData[i].Z);
                FVector Scale(RustData[i].ScaleX, RustData[i].ScaleY, RustData[i].ScaleZ);
                FRotator Rot(RustData[i].Pitch, RustData[i].Yaw, RustData[i].Roll);

                if (bShouldLog && i < 3) {
                    UE_LOG(LogTemp, Display, TEXT("  > Fragm %d: Cat=%d | Pos=%s | Scale=%s"), i, Cat, *Pos.ToString(), *Scale.ToString());
                }

                // VALIDACIÓN DE SEGURIDAD: Si la escala es casi 0, forzamos visibilidad
                if (Scale.SizeSquared() < 0.0001f) Scale = FVector(1.0f);

                Grouped[Cat].Add(FTransform(Rot.Quaternion(), Pos, Scale));
            }

            // Enviamos los batches al renderer
            for (int32 c = 0; c < 4; ++c)
            {
                FragmentRenderer->UpdateHISMTransforms(c, Grouped[c]);
            }

            // AJUSTE DINÁMICO DE SUELO (Sigue-Montañas v1.0)
            // Usamos el primer fragmento (el más pesado) para detectar si el terreno ha cambiado
            if (RustCount > 0 && DebugSkip % 30 == 0) // Cada 0.5s
            {
                FHitResult TerrainHit;
                FVector MyPos(RustData[0].X, RustData[0].Y, RustData[0].Z);
                if (GetWorld()->LineTraceSingleByChannel(TerrainHit, MyPos + FVector(0,0,500), MyPos - FVector(0,0,1000), ECC_Visibility))
                {
                    USoulForgeBridge::SetGroundZ(TerrainHit.ImpactPoint.Z);
                }
            }
        }
        else if (bDestroyed)
        {
            // Si el objeto explotó pero Rust devuelve 0, hay un problema de filtrado
            static int32 LogCounterErr = 0;
            if (LogCounterErr++ % 120 == 0)
            {
                UE_LOG(LogTemp, Error, TEXT("[SoulForge-Error] El actor %s está destruido pero Rust no devuelve sus fragmentos (Hash %u)."), 
                       *ProxyName, ObjectFilterHash);
            }
        }
    }
}

// ─── EndPlay ─────────────────────────────────────────────────────────────────

void USoulForgeDestructible::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
    // Si el objeto se destruye o salimos del Play, avisamos a Rust que borre ese ID
    if (ProxyId > 0) {
        USoulForgeBridge::BorrarProxyEnRust(ProxyName);
        ProxyId = 0; // Lo ponemos a 0 para que en el próximo Play se registre de nuevo
    }

    if (FragmentRenderer) { FragmentRenderer->Cleanup(); }
    Super::EndPlay(EndPlayReason);
}

// ─── ApplyDamage ─────────────────────────────────────────────────────────────

void USoulForgeDestructible::ApplyDamage(float DamageAmount)
{
    if (bDestroyed) { return; }

    Health = FMath::Clamp(Health - DamageAmount, 0.0f, 1.0f);
    if (Health <= 0.0f)
    {
        TriggerDestruction();
    }
}

// ─── TriggerDestruction ───────────────────────────────────────────────────────

void USoulForgeDestructible::TriggerDestruction()
{
    // EL LOG DEBE IR ANTES DE CUALQUIER RETURN
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge] DETONANDO IDENTIDAD: %s"), *ProxyName);

    if (bDestroyed || ProxyName.IsEmpty()) { 
        UE_LOG(LogTemp, Error, TEXT("[SoulForge] DETONACIÓN CANCELADA. bDestroyed es true o ProxyName está vacío."));
        return; 
    }
    
    bDestroyed = true;

    // Forzamos al componente a despertar por si acaso el tick estaba apagado
    SetComponentTickEnabled(true);

    DeactivateOriginal();
    
    // Obtenemos el epicentro real (Centro del Objeto por defecto)
    FVector CentroEfecto = GetOwner()->GetActorLocation();

    // Sincronizar Material y LOD justo antes de detonar para que los cambios en el editor surtan efecto
    USoulForgeBridge::SetProxyMilitar(ProxyName, MaterialType, PhysicsLOD);

    // 1. Llamada unificada nativa militar (Clásica) - Convierte el objeto en Nodos físicos
    // --- DETECCIÓN DINÁMICA DE SUELO (EVITAR ATRAVESAR EL PISO) ---
    FHitResult GroundHit;
    FVector TraceStart = GetOwner()->GetActorLocation();
    FVector TraceEnd = TraceStart - FVector(0, 0, 50000.0f); // Trazamos 500 metros hacia abajo
    FCollisionQueryParams Params;
    Params.AddIgnoredActor(GetOwner());
    // También ignoramos el renderer para que no nos bloquee el rayo
    if (FragmentRenderer) Params.AddIgnoredComponent(FragmentRenderer.Get());

    if (GetWorld()->LineTraceSingleByChannel(GroundHit, TraceStart, TraceEnd, ECC_Visibility, Params))
    {
        float RealGroundZ = GroundHit.ImpactPoint.Z;
        USoulForgeBridge::SetGroundZ(RealGroundZ);
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Physics] Suelo detectado por trazado de rayos en Z: %f"), RealGroundZ);
    }
    else
    {
        // Si no hay suelo cerca, usamos el Z del actor - un margen pequeño
        USoulForgeBridge::SetGroundZ(TraceStart.Z - 500.0f);
    }

    // Convertimos nuestro EExplosiveType a preset de Rust:
    // TNT=0(Explosion), AgujeroNegro=1(Implosion), RDX=2(Impact), PETN=3(ShapedCharge), 
    // Matrix=4(Cinematic), Collapse=5(Collapse), RealityShatter=6
    int32 RustPreset = 0;
    switch (ExplosiveType) {
        case EExplosiveType::TNT:            RustPreset = 0; break;
        case EExplosiveType::AgujeroNegro:   RustPreset = 1; break;
        case EExplosiveType::RDX:            RustPreset = 2; break;
        case EExplosiveType::PETN:           RustPreset = 3; break;
        case EExplosiveType::Matrix:         RustPreset = 4; break;
        case EExplosiveType::Collapse:       RustPreset = 5; break;
        case EExplosiveType::RealityShatter: RustPreset=6; break;
        case EExplosiveType::Vortex:         RustPreset = 7; break;
        default: RustPreset = 0; break;
    }

    TArray<FSoulForgeFragmentData> OutFragments;
    USoulForgeBridge::DetonarNativo(ProxyName, FuerzaDeImpacto, RustPreset, FragmentacionJupiter, OutFragments, CantidadDeNodos);
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Physics] Detonación Física [%s] | Preset: %d | Nodos: %d | Radio: %.1f"), *CentroEfecto.ToString(), RustPreset, CantidadDeNodos, RadioDeImpacto);

    // --- GENERAR ONDA VFG REACTIVA ---
    USoulForgeVFXLibrary::GenerarOndaReactiva(this, CentroEfecto, FuerzaDeImpacto);
    
    if (DustVFX)
    {
        UNiagaraFunctionLibrary::SpawnSystemAtLocation(
            GetWorld(),
            DustVFX,
            GetOwner()->GetActorLocation(),
            GetOwner()->GetActorRotation()
        );
    }

    if (FragmentRenderer) { FragmentRenderer->NotifyExplosion(); }
}

void USoulForgeDestructible::PreviewExplosion()
{
    AActor* Owner = GetOwner();
    if (!Owner) return;

    // Limpiamos dibujos anteriores
    FlushPersistentDebugLines(GetWorld());
    
    // Sincronizamos el nombre por si cambió en el editor
    if (ProxyName.IsEmpty()) ProxyName = Owner->GetActorLabel();

    FVector Origen = Owner->GetActorLocation();
    
    // 1. Pedimos a Rust los vectores de fuerza (usamos la nueva función del puente)
    // Pasamos el tipo como int32 y la cantidad solicitada para que el preview sea preciso
    TArray<FVector> Trayectorias = USoulForgeBridge::GetExplosionPreview(ProxyName, FuerzaDeImpacto, (int32)ExplosiveType, CantidadDeNodos);

    // 2. Dibujamos en el visor
    for (const FVector& Vel : Trayectorias) {
        // En preview, Vel es el vector de velocidad [cm/s]. 
        // Multiplicamos por 0.1s para tener una trayectoria visible de ~1 metro si v=10m/s
        FVector Destino = Origen + (Vel * 0.1f); 
        
        FColor ColorPreview = (ExplosiveType == EExplosiveType::AgujeroNegro) ? FColor::Blue : FColor::Red;
        
        DrawDebugLine(GetWorld(), Origen, Destino, ColorPreview, false, PreviewDuration, 0, 1.5f);
        DrawDebugPoint(GetWorld(), Destino, 5.0f, ColorPreview, false, PreviewDuration);
    }

    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Lab] Preview Generado [%s] | %d Trayectorias calculadas."), *ProxyName, Trayectorias.Num());
}

void USoulForgeDestructible::RecibirImpacto(FVector PuntoDeGolpe, FVector Direccion)
{
    if (!InstancedRenderer || ProxyId <= 0) return;

    // Llamada al Núcleo Maestro Unificado    // Detonamos en Rust pasando el PRESET del menú (ExplosiveType) y el nivel de fragmentación de Júpiter
    TArray<FSoulForgeFragmentData> OutFragments;
    int32 RealCount = USoulForgeBridge::DetonarNativo(ProxyName, FuerzaDeImpacto, (int32)ExplosiveType, FragmentacionJupiter, OutFragments, CantidadDeNodos);
    
    bDestroyed = true;
    
    if (FragmentRenderer) { FragmentRenderer->NotifyExplosion(); }

    if (RealCount > 0 && OutFragments.Num() > 0) {
        for (int32 i = 0; i < FMath::Min(OutFragments.Num(), MaxNodos); i++) {
            NodePool[i].bEstaActivo = 1;
            NodePool[i].Posicion = FVector(OutFragments[i].X, OutFragments[i].Y, OutFragments[i].Z);
            NodePool[i].Rotacion = FQuat::Identity; // La rotación base
            NodePool[i].Escala3D = FVector(OutFragments[i].ScaleX, OutFragments[i].ScaleY, OutFragments[i].ScaleZ);
        }
    }

    // NO ocultamos el actor completo o matamos el renderizado de los HISMs
    // GetOwner()->SetActorHiddenInGame(true); 
    GetOwner()->SetActorEnableCollision(false);

    if (InstancedRenderer) {
        InstancedRenderer->ClearInstances();
        for (int32 i = 0; i < MaxNodos; i++) {
            if (NodePool[i].bEstaActivo == 1) {
                FTransform Transform(NodePool[i].Rotacion, NodePool[i].Posicion, NodePool[i].Escala3D);
                InstancedRenderer->AddInstance(Transform);
            }
        }
    }
}

void USoulForgeDestructible::AutoLinkToRust()
{
    if (this->ProxyId == 0)
    {
        // Aseguramos que el nombre exista
        if (ProxyName.IsEmpty() && GetOwner()) {
            ProxyName = GetOwner()->GetActorLabel();
            if (ProxyName.IsEmpty()) ProxyName = GetOwner()->GetName();
        }

        const FVector WorldPos = GetOwner()->GetActorLocation();
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Link] Intentando re-anclaje inteligente de [%s]..."), *ProxyName);
        
        // 2. REGISTRO: Le decimos a Rust que use este Nombre
        if (USoulForgeBridge::RegisterProxy(ProxyName, WorldPos, HalfExtent, GetDensity()))
        {
            this->ProxyId = 1; // Marcamos como registrado (el ID real es el nombre)
        }
        
        if (this->ProxyId != 0) {
            UE_LOG(LogTemp, Display, TEXT("[SoulForge-Link] ✅ Objeto anclado al ADN correctamente."));
            bIsEngineReady = true;
        } else {
            UE_LOG(LogTemp, Error, TEXT("[SoulForge-Link] ❌ Fallo en el anclaje del DNI %u."), PersistentDNI);
            bIsEngineReady = false;
        }
    }
}

void USoulForgeDestructible::ResetInternalState()
{
    this->ProxyId = 0;
    this->PersistentDNI = 0; // Se recalculará por Ruta
    this->bDestroyed = false;
    this->Health = 1.0f;
    this->bIsEngineReady = false;
    AutoLinkToRust();
}

float USoulForgeDestructible::GetDensity() const
{
    switch (MaterialType)
    {
        case EMaterialType::Steel4340:    return 0.0079f;
        case EMaterialType::Aluminum6061: return 0.0027f;
        case EMaterialType::Concrete35MPa: return 0.0024f;
        case EMaterialType::WoodOak:      return 0.0007f;
        case EMaterialType::GlassTempered: return 0.0025f;
        default:                          return 0.0024f;
    }
}


void USoulForgeDestructible::PreparaRenderizadoFragmentos()
{
    AActor* Owner = GetOwner();
    if (!Owner || !InstancedRenderer || !MasterShardMaterial) return;

    // 1. Obtenemos el mesh original del cubo
    UStaticMeshComponent* OriginalMesh = Owner->FindComponentByClass<UStaticMeshComponent>();
    // Evitamos capturar el InstancedRenderer que acabamos de crear
    TArray<UStaticMeshComponent*> Meshes;
    Owner->GetComponents<UStaticMeshComponent>(Meshes);
    
    for (UStaticMeshComponent* Mesh : Meshes)
    {
        if (Mesh != InstancedRenderer)
        {
            OriginalMesh = Mesh;
            break;
        }
    }

    if (!OriginalMesh) return;

    // 2. Extraemos el material y su textura
    UMaterialInterface* CubeMat = OriginalMesh->GetMaterial(0);
    if (!CubeMat) return;

    UTexture* CapturedTexture = nullptr;
    // Intentamos obtener la textura (buscamos parámetros comunes si 'BaseColorTexture' falla)
    bool bFound = CubeMat->GetTextureParameterValue(FMaterialParameterInfo(TEXT("BaseColorTexture")), CapturedTexture);
    if (!bFound || !CapturedTexture)
    {
        CubeMat->GetTextureParameterValue(FMaterialParameterInfo(TEXT("Texture")), CapturedTexture);
    }

    // 3. Inyectamos en el material de los fragmentos
    UMaterialInstanceDynamic* ShardMID = UMaterialInstanceDynamic::Create(MasterShardMaterial, this);
    if (ShardMID)
    {
        if (CapturedTexture)
        {
            ShardMID->SetTextureParameterValue(TEXT("TextureToCopy"), CapturedTexture);
            UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Visual] Textura capturada [%s] y aplicada a fragmentos."), *CapturedTexture->GetName());
        }
        else
        {
            UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Visual] No se encontró textura base, usando material limpio."));
        }

        if (InstancedRenderer) {
            InstancedRenderer->SetMaterial(0, ShardMID);
        }

        if (FragmentRenderer) {
            FragmentRenderer->FragmentMaterial = ShardMID;
            // Forzar actualización en HISMs existentes
            FragmentRenderer->EnsureHISMsCreated();
            if (auto* S = FragmentRenderer->HISMForCategory(0)) S->SetMaterial(0, ShardMID);
            if (auto* C = FragmentRenderer->HISMForCategory(1)) C->SetMaterial(0, ShardMID);
            if (auto* G = FragmentRenderer->HISMForCategory(2)) G->SetMaterial(0, ShardMID);
            if (auto* D = FragmentRenderer->HISMForCategory(3)) D->SetMaterial(0, ShardMID);
        }
    }
}

void USoulForgeDestructible::ForzarEncendidoRust()
{
    UE_LOG(LogTemp, Warning, TEXT("==========================================="));
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Debug] INICIANDO SECUENCIA MANUAL"));
    
    // 1. Encendemos el motor maestro
    USoulForgeBridge::Initialize();
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Debug] Comando Initialize() enviado a la DLL."));

    // 2. Reseteamos el ProxyId por si acaso estaba "trabado"
    ProxyId = -1;

    // 3. Intentamos el registro
    AutoLinkToRust();

    if (ProxyId >= 0) {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Debug] ¡ÉXITO TOTAL! Rust está vivo y aceptó el Proxy."));
    } else {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge-Debug] FALLO: Rust sigue devolviendo -1."));
    }
    UE_LOG(LogTemp, Warning, TEXT("==========================================="));
}

void USoulForgeDestructible::ActivarInsight()
{
    UE_LOG(LogTemp, Warning, TEXT("==========================================="));
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Insight] CONFIGURANDO RENDIMIENTO"));
    
    // Enviamos los parámetros al núcleo de Rust
    // --- DIAGNÓSTICO: Un solo hilo para evitar conflictos de Rayon ---
    USoulForgeBridge::ApplySettings(1, false);
    
    ProxyId = GetTypeHash(GetOwner()->GetName());
    ObjectFilterHash = (uint32)ProxyId;
    ProxyName = GetOwner()->GetName();
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Insight] Núcleos CPU: %d"), 1);
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Insight] GPU Aceleración: %s"), false ? TEXT("ACTIVADA") : TEXT("DESACTIVADA"));
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Insight] Configuración aplicada correctamente."));
    UE_LOG(LogTemp, Warning, TEXT("==========================================="));
}

void USoulForgeDestructible::UpdateNiagaraVisuals()
{
    if (!SharedNiagaraVFX) return;

    int32 NodeCount = 0;
    const TArray<FVector>& MasterPositions = USoulForgeBridge::GetMasterPositions(NodeCount);

    if (NodeCount > 0)
    {
        // TRUCO ZERO-COPY:
        // Hacemos una copia ultra-rápida (shallow copy / slice assignment si fuera posible, 
        // pero a nivel de UE TArray usa Copy Constructor o SetNum).
        // En Unreal 5.x, pasar el TArray ya construido sin loop iterativo es 10x más rápido.
        TArray<FVector> ActivePositions;
        
        // Copiamos solo los nodos activos con memcopy nivel C++ usando Append
        // (Esto evita el loop for pieza por pieza)
        ActivePositions.Append(MasterPositions.GetData(), NodeCount);

        // Enviar a Niagara vía Data Interface Array
        UNiagaraDataInterfaceArrayFunctionLibrary::SetNiagaraArrayVector(
            SharedNiagaraVFX,
            FName("PosicionesDesdeRust"),
            ActivePositions
        );

        SharedNiagaraVFX->SetIntParameter(FName("CantidadNodos"), NodeCount);
    }
}

void USoulForgeDestructible::SpawnNodeGrid(FVector Center, int32 Width, int32 Height, int32 Depth, float Spacing)
{
    TArray<FVector> Positions;
    Positions.Reserve(Width * Height * Depth);

    FVector Start = Center - FVector(Width * Spacing * 0.5f, Height * Spacing * 0.5f, Depth * Spacing * 0.5f);

    for (int32 z = 0; z < Depth; ++z)
    {
        for (int32 y = 0; y < Height; ++y)
        {
            for (int32 x = 0; x < Width; ++x)
            {
                Positions.Add(Start + FVector(x * Spacing, y * Spacing, z * Spacing));
            }
        }
    }

    USoulForgeBridge::SpawnNodesBulk(Positions, 1.0f, 100); // Material 100 = Efecto Cian Bouncy
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Lab] Generada rejilla de %d nodos."), Positions.Num());
}

void USoulForgeDestructible::ActivateKamehameha(FVector Origin, FVector Direction, float Intensity)
{
    // El Kamehameha en SoulForge es un EnergyBeam (Tipo 0) que dura 8 segundos
    USoulForgeBridge::ActivarPoder(EPowerType::EnergyBeam, Origin, Direction, Intensity, 8.0f);
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Power] ¡KAMEHAMEHA ACTIVADO! Intensidad: %.1f"), Intensity);
}


