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

    HISMSlab   = InitHISM(SlabMesh,   TEXT("HISMSlab"));
    HISMChunk  = InitHISM(ChunkMesh,  TEXT("HISMChunk"));
    HISMGravel = InitHISM(GravelMesh, TEXT("HISMGravel"));
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

    HISM->SetCastShadow(false);
    HISM->bSelectable = false;

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

UHierarchicalInstancedStaticMeshComponent*
USoulForgeFragmentRenderer::HISMForCategory(int32 Category) const
{
    switch (Category)
    {
        case 0:  return HISMSlab;
        case 1:  return HISMChunk;
        case 2:  return HISMGravel;
        default: return nullptr;
    }
}

void USoulForgeFragmentRenderer::UpdateHISMTransforms(int32 Category, const TArray<FTransform>& NewTransforms)
{
    // 1. Buscamos qué tipo de pedazo vamos a actualizar (Slab, Chunk o Gravel)
    UHierarchicalInstancedStaticMeshComponent* TargetHISM = HISMForCategory(Category);
    
    if (!TargetHISM || NewTransforms.Num() == 0) return;

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
    TargetHISM->BatchUpdateInstancesTransforms(0, NewTransforms, true, true, true);
}
