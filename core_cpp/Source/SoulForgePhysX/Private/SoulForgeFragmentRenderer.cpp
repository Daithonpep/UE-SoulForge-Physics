// ─── SoulForgeFragmentRenderer.cpp ───────────────────────────────────────────

#include "SoulForgeFragmentRenderer.h"
#include "SoulForgeBridge.h"

USoulForgeFragmentRenderer::USoulForgeFragmentRenderer()
{
    PrimaryComponentTick.bCanEverTick = false; 
}

void USoulForgeFragmentRenderer::ExplodeIntoFragments(FString JsonData)
{
	// Esta función es legacy. Ahora usamos DetonarNativo que devuelve un array de estructuras.
}

void USoulForgeFragmentRenderer::ClearAllFragments()
{
	Cleanup();
}

void USoulForgeFragmentRenderer::BeginPlay()
{
    Super::BeginPlay();

    // Auto-detectar mesh del actor si no se asignaron meshes de fragmento
    UStaticMesh* FallbackMesh = nullptr;
    if (!SlabMesh || !ChunkMesh || !GravelMesh)
    {
        if (AActor* Owner = GetOwner())
        {
            if (UStaticMeshComponent* SMC = Owner->FindComponentByClass<UStaticMeshComponent>())
            {
                FallbackMesh = SMC->GetStaticMesh();
                UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] Auto-detectado mesh: %s"), FallbackMesh ? *FallbackMesh->GetName() : TEXT("null"));
            }
        }
    }

    if (!SlabMesh)   SlabMesh   = FallbackMesh;
    if (!ChunkMesh)  ChunkMesh  = FallbackMesh;
    if (!GravelMesh) GravelMesh = FallbackMesh;

    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] Inicializando HISMs en BeginPlay..."));

	HISMSlab   = InitHISM(SlabMesh,   TEXT("HISMSlab"));
    HISMChunk  = InitHISM(ChunkMesh,  TEXT("HISMChunk"));
    HISMGravel = InitHISM(GravelMesh, TEXT("HISMGravel"));

    if (HISMSlab && HISMChunk && HISMGravel) {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] ✅ Todos los HISMs creados y registrados."));
    } else {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge-Renderer] ❌ Error creando los componentes HISM!"));
    }
}

UHierarchicalInstancedStaticMeshComponent* USoulForgeFragmentRenderer::InitHISM(
    UStaticMesh* Mesh,
    const FName& Name
)
{
    UHierarchicalInstancedStaticMeshComponent* HISM = NewObject<UHierarchicalInstancedStaticMeshComponent>(
        GetOwner(), UHierarchicalInstancedStaticMeshComponent::StaticClass(), Name
    );

    if (!HISM) { return nullptr; }

    HISM->RegisterComponent();
    HISM->AttachToComponent(
        GetOwner()->GetRootComponent(),
        FAttachmentTransformRules::KeepWorldTransform
    );

    if (Mesh) { HISM->SetStaticMesh(Mesh); }
    if (FragmentMaterial) { HISM->SetMaterial(0, FragmentMaterial); }

    HISM->SetCastShadow(true);
    HISM->bSelectable = false;
    
    // --- OPTIMIZACIÓN CRÍTICA (Anticrash) ---
    // Los fragmentos son miles; no pueden afectar la navegación ni tener colisión compleja
    HISM->SetCollisionEnabled(ECollisionEnabled::NoCollision);
    HISM->SetCanEverAffectNavigation(false);
    HISM->bFillCollisionUnderneathForNavmesh = false;
    HISM->bUseAsOccluder = false; // Ahorra GPU
    HISM->SetGenerateOverlapEvents(false);

    return HISM;
}

void USoulForgeFragmentRenderer::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
    Cleanup();
    Super::EndPlay(EndPlayReason);
}

void USoulForgeFragmentRenderer::TickUpdate(float DeltaTime)
{
    // La actualización ahora se hace vía Shared Memory Pool en el USoulForgeDestructible
}

void USoulForgeFragmentRenderer::Cleanup()
{
    auto ClearHISM = [](UHierarchicalInstancedStaticMeshComponent* H)
    {
        if (H) { H->ClearInstances(); }
    };

    ClearHISM(HISMSlab);
    ClearHISM(HISMChunk);
    ClearHISM(HISMGravel);

    Fragments.Empty();
    ActiveCount = 0;
}

int32 USoulForgeFragmentRenderer::GetActiveFragmentCount() const
{
    return ActiveCount;
}

UHierarchicalInstancedStaticMeshComponent* USoulForgeFragmentRenderer::HISMForCategory(int32 Category)
{
    EnsureHISMsCreated();
    
    switch (Category)
    {
        case 0:  return HISMSlab;
        case 1:  return HISMChunk;
        case 2:  return HISMGravel;
        default: return nullptr;
    }
}

void USoulForgeFragmentRenderer::EnsureHISMsCreated()
{
    if (HISMSlab != nullptr) return;

    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] 🚀 Inicialización ON-DEMAND de HISMs ante posible fallo de BeginPlay."));

    HISMSlab   = InitHISM(SlabMesh,   TEXT("HISMSlab"));
    HISMChunk  = InitHISM(ChunkMesh,  TEXT("HISMChunk"));
    HISMGravel = InitHISM(GravelMesh, TEXT("HISMGravel"));

    if (HISMSlab && HISMChunk && HISMGravel) {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] ✅ Pools creados bajo demanda con éxito."));
    }
}

void USoulForgeFragmentRenderer::UpdateHISMTransforms(int32 Category, const TArray<FTransform>& NewTransforms)
{
    static bool bLogCallOnce = false;
    if (!bLogCallOnce) {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer-Entry] Llamada a UpdateHISMTransforms: Categoría %d, Transformaciones %d"), Category, NewTransforms.Num());
        bLogCallOnce = true;
    }

    // 1. Buscamos qué tipo de pedazo vamos a actualizar (Slab, Chunk o Gravel)
    UHierarchicalInstancedStaticMeshComponent* TargetHISM = HISMForCategory(Category);
    
    if (!TargetHISM || NewTransforms.Num() == 0) {
        static bool bLogFailOnce = false;
        if (!bLogFailOnce) {
            UE_LOG(LogTemp, Error, TEXT("[SoulForge-Renderer-Entry] ❌ Error: HISM para categoría %d es NULO o no hay transformaciones."), Category);
            bLogFailOnce = true;
        }
        return;
    }

    // 2. Si no hay suficientes instancias creadas, las creamos de golpe
    int32 CurrentInstances = TargetHISM->GetInstanceCount();
    int32 RequiredInstances = NewTransforms.Num();

    if (CurrentInstances < RequiredInstances)
    {
        // Añadimos las que faltan en la posición cero temporalmente
        TArray<FTransform> PaddingTransforms;
        PaddingTransforms.Init(FTransform::Identity, RequiredInstances - CurrentInstances);
        TargetHISM->AddInstances(PaddingTransforms, false);
    }
    else if (CurrentInstances > RequiredInstances)
    {
        // Si sobran (porque algunos se destruyeron), podríamos borrarlos, 
        // pero por rendimiento es mejor solo escalarlos a 0 o simplemente ignorarlos
        // ya que BatchUpdate solo actualizará los primeros RequiredInstances.
    }

    // 3. LA INYECCIÓN DIRECTA: Actualizamos todas las posiciones en 1 solo frame
    // Ponemos bMarkRenderStateDirty a FALSE para evitar que el sistema se congele
    // tratando de reconstruir el Octree cada vez que un fragmento se mueve 1cm.
    TargetHISM->BatchUpdateInstancesTransforms(0, NewTransforms, true, false, true);
    TargetHISM->MarkRenderTransformDirty();
}
