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
	if (UStaticMeshComponent* SMC = GetOwner()->FindComponentByClass<UStaticMeshComponent>())
	{
        // --- ¡NUEVO! Apagamos la física para que Unreal no se queje ---
        SMC->SetSimulatePhysics(false); 
        // --------------------------------------------------------------

		SMC->SetVisibility(false);
		SMC->SetCollisionEnabled(ECollisionEnabled::NoCollision);
		UE_LOG(LogTemp, Log, TEXT("[SoulForge] Mesh original de '%s' desactivado."), *GetOwner()->GetName());
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

    // El nombre ya viene de OnRegister
    if (ProxyName.IsEmpty() && GetOwner()) {
        ProxyName = GetOwner()->GetActorLabel();
        if (ProxyName.IsEmpty()) ProxyName = GetOwner()->GetName();
    }

    UE_LOG(LogTemp, Display, TEXT("[SoulForge] Iniciando con Identidad Persistente: %s"), *ProxyName);
    if (ProxyId > 0) {
        return; 
    }

    // ESTO DEBE SALIR EN CUANTO LE DES A PLAY
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-C++] COMPONENTE INICIALIZADO EN: %s"), *GetOwner()->GetName());

    // --- ESTO ES LO QUE FALTA: ARRANQUE FORZADO ---
    PrimaryComponentTick.SetTickFunctionEnable(true); 
    // ----------------------------------------------

    // --- EL GRITO DEL NÚCLEO DE RENDIMIENTO ---
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Rendimiento] Núcleo enlazado. Límite de fragmentos (Pool): %d"), MaxNodos);
    // ------------------------------------------

    // 1. Buscamos el Renderer si no viene de serie
    if (!FragmentRenderer)
    {
        FragmentRenderer = GetOwner()->FindComponentByClass<USoulForgeFragmentRenderer>();
    }

    if (FragmentRenderer) {
        UE_LOG(LogTemp, Log, TEXT("[SoulForge-C++] Renderer encontrado y vinculado."));
    } else {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge-C++] ERROR: No tienes el Renderer asignado en el Blueprint!"));
    }

    // 2. RESERVAMOS LA RAM (Lógica interna esencial)
    NodePool.SetNumUninitialized(MaxNodos);
    for (int32 i = 0; i < MaxNodos; i++) {
        NodePool[i].bEstaActivo = 0;
        NodePool[i].Escala3D = FVector(1.0f);
    }

    // 3. CREAMOS EL RENDERIZADOR SÚPER RÁPIDO (ISM)
    if (GetOwner()) {
        InstancedRenderer = NewObject<UInstancedStaticMeshComponent>(GetOwner());
        if (InstancedRenderer) {
            InstancedRenderer->RegisterComponent();
            InstancedRenderer->SetStaticMesh(MallaDelNodo);
            InstancedRenderer->SetCollisionEnabled(ECollisionEnabled::NoCollision);
        }
    }

    // --- EL PUENTE MANUAL DE ENCENDIDO ---
    // ¡AQUÍ DESPERTAMOS A RUST ANTES DE PEDIRLE NADA!
    USoulForgeBridge::Initialize();
    
    // Y le decimos dónde está el suelo (por defecto Z=0)
    USoulForgeBridge::SetGroundZ(0.0f);

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

    // --- ¡EL RELOJ DE LA FÍSICA! ---
    // Sincronizamos el tiempo de Unreal con la integración de Rust.
    USoulForgeBridge::TickPhysics(DeltaTime);
    // -------------------------------


    // Actualizar Visuales (Niagara y Rústico)
    UpdateNiagaraVisuals();

    if (bUsarRenderizadoRustico && FragmentRenderer)
    {
        // 1. OBTENER DATOS MASIVOS (Shared Memory)
        int32 RustCount = 0;
        FSoulForgeFragmentData* RustData = USoulForgeBridge::GetAllFragmentData(RustCount);

        if (RustData && RustCount > 0)
        {
            // 2. AGRUPAR POR CATEGORÍA PARA RENDIMIENTO MÁXIMO (HISM Batch)
            TMap<int32, TArray<FTransform>> GroupedTransforms;
            
            // Pre-asignamos memoria para evitar miles de reasignaciones (Esto causaba el lag inicial)
            GroupedTransforms.FindOrAdd(0).Reserve(RustCount);
            GroupedTransforms.FindOrAdd(1).Reserve(RustCount);
            GroupedTransforms.FindOrAdd(2).Reserve(RustCount);
            
            for (int32 i = 0; i < RustCount; ++i)
            {
                FVector Pos = FVector(RustData[i].X, RustData[i].Y, RustData[i].Z);
                FVector Scale = FVector(RustData[i].ScaleX, RustData[i].ScaleY, RustData[i].ScaleZ);
                
                // Rotación basada en la posición para que no se vean como cubos perfectos alienados
                FQuat Rot = FQuat::MakeFromEuler(FVector(Pos.X * 13.0f, Pos.Y * 17.0f, Pos.Z * 23.0f));
                
                int32 SafeCategory = FMath::Clamp(RustData[i].Category, 0, 2);
                GroupedTransforms[SafeCategory].Add(FTransform(Rot, Pos, Scale));
            }

            // 3. ENVIAR AL RENDERIZADOR (Pase de dibujo único por categoría)
            for (auto& Elem : GroupedTransforms)
            {
                if (Elem.Value.Num() > 0)
                {
                    FragmentRenderer->UpdateHISMTransforms(Elem.Key, Elem.Value);
                }
            }
        }
    }

    // El renderizado se maneja ahora en el bloque optimizado de arriba via FragmentRenderer
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

    // 1. Llamada unificada nativa militar (Clásica) - Convierte el objeto en Nodos físicos
    USoulForgeBridge::DetonarMilitar(ProxyName, CentroEfecto, FuerzaDeImpacto, ExplosiveType, CantidadDeNodos);
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Physics] Detonación Física [%s] | Nodos Solicitados: %d"), *CentroEfecto.ToString(), CantidadDeNodos);

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

    // Llamada al Núcleo Maestro Unificado - Ahora respetando la cantidad de nodos
    TArray<FSoulForgeFragmentData> FragmentData;
    int32 RealCount = USoulForgeBridge::DetonarNativo(ProxyName, FuerzaDeImpacto, FragmentData, CantidadDeNodos);

    if (RealCount > 0) {
        for (int32 i = 0; i < FMath::Min(RealCount, MaxNodos); i++) {
            NodePool[i].bEstaActivo = 1;
            NodePool[i].Posicion = FVector(FragmentData[i].X, FragmentData[i].Y, FragmentData[i].Z);
            NodePool[i].Rotacion = FQuat::Identity; // La rotación base
            NodePool[i].Escala3D = FVector(FragmentData[i].ScaleX, FragmentData[i].ScaleY, FragmentData[i].ScaleZ);
        }
    }

    GetOwner()->SetActorHiddenInGame(true);
    GetOwner()->SetActorEnableCollision(false);

    InstancedRenderer->ClearInstances();
    for (int32 i = 0; i < MaxNodos; i++) {
        if (NodePool[i].bEstaActivo == 1) {
            FTransform Transform(NodePool[i].Rotacion, NodePool[i].Posicion, NodePool[i].Escala3D);
            InstancedRenderer->AddInstance(Transform);
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

        InstancedRenderer->SetMaterial(0, ShardMID);
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
    USoulForgeBridge::ApplySettings(NucleosCPU, bUsarAceleracionGPU);
    
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Insight] Núcleos CPU: %d"), NucleosCPU);
    UE_LOG(LogTemp, Display, TEXT("[SoulForge-Insight] GPU Aceleración: %s"), bUsarAceleracionGPU ? TEXT("ACTIVADA") : TEXT("DESACTIVADA"));
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Insight] Configuración aplicada correctamente."));
    UE_LOG(LogTemp, Warning, TEXT("==========================================="));
}


