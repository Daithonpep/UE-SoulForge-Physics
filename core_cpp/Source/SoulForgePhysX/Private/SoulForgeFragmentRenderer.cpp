#include "SoulForgeFragmentRenderer.h"
#include "SoulForgeBridge.h"

USoulForgeFragmentRenderer::USoulForgeFragmentRenderer()
{
    PrimaryComponentTick.bCanEverTick = false; 
}

void USoulForgeFragmentRenderer::ExplodeIntoFragments(FString JsonData)
{
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
    if (!SlabMesh || !ChunkMesh || !GravelMesh || !DustMesh)
    {
        if (AActor* Owner = GetOwner())
        {
            TArray<UStaticMeshComponent*> MeshComponents;
            Owner->GetComponents<UStaticMeshComponent>(MeshComponents);
            
            for (UStaticMeshComponent* SMC : MeshComponents)
            {
                // Ignorar HISMs internos del propio sistema de fragmentos
                if (SMC == this || SMC->IsA<UInstancedStaticMeshComponent>()) continue;
                
                if (SMC->GetStaticMesh())
                {
                    FallbackMesh = SMC->GetStaticMesh();
                    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] Auto-detectado mesh base: %s"), *FallbackMesh->GetName());
                    break;
                }
            }
        }
    }

    if (!SlabMesh)   SlabMesh   = FallbackMesh;
    if (!ChunkMesh)  ChunkMesh  = FallbackMesh;
    if (!GravelMesh) GravelMesh = FallbackMesh;
    if (!DustMesh)   DustMesh   = FallbackMesh;

    UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] Inicializando HISMs (Slab, Chunk, Gravel, Dust)..."));

	HISMSlab   = InitHISM(SlabMesh,   TEXT("HISMSlab"));
    HISMChunk  = InitHISM(ChunkMesh,  TEXT("HISMChunk"));
    HISMGravel = InitHISM(GravelMesh, TEXT("HISMGravel"));
    HISMDust   = InitHISM(DustMesh,   TEXT("HISMDust"));

    if (HISMSlab && HISMChunk && HISMGravel && HISMDust) {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] ✅ Todos los HISMs (4 categorías) creados con éxito."));
    } else {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge-Renderer] ❌ Error creando uno o más componentes HISM!"));
    }
    
    // FORZAR INICIALIZACIÓN TEMPRANA (Evita tirones en explosión)
    EnsureHISMsCreated();
}

UInstancedStaticMeshComponent* USoulForgeFragmentRenderer::InitHISM(UStaticMesh* Mesh, const FName& Name)
{
    UInstancedStaticMeshComponent* HISM = NewObject<UInstancedStaticMeshComponent>(
        GetOwner(), UInstancedStaticMeshComponent::StaticClass(), Name
    );

    if (!HISM) { return nullptr; }

    HISM->RegisterComponent();
    HISM->AttachToComponent(GetOwner()->GetRootComponent(), FAttachmentTransformRules::KeepWorldTransform);

    if (Mesh) { HISM->SetStaticMesh(Mesh); }
    if (FragmentMaterial) { HISM->SetMaterial(0, FragmentMaterial); }

    HISM->SetCastShadow(true);
    HISM->SetMobility(EComponentMobility::Movable);
    HISM->bSelectable = false;
    HISM->SetCollisionEnabled(ECollisionEnabled::NoCollision);
    HISM->SetCanEverAffectNavigation(false);
    HISM->bFillCollisionUnderneathForNavmesh = false;
    HISM->bUseAsOccluder = false;
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
}

void USoulForgeFragmentRenderer::Cleanup()
{
    auto ClearHISM = [](UInstancedStaticMeshComponent* H)
    {
        if (H) { H->ClearInstances(); }
    };

    ClearHISM(HISMSlab);
    ClearHISM(HISMChunk);
    ClearHISM(HISMGravel);
    ClearHISM(HISMDust);

    Fragments.Empty();
    ActiveCount = 0;
}

int32 USoulForgeFragmentRenderer::GetActiveFragmentCount() const
{
    return ActiveCount;
}

UInstancedStaticMeshComponent* USoulForgeFragmentRenderer::HISMForCategory(int32 Category)
{
    EnsureHISMsCreated();
    
    switch (Category)
    {
        case 0:  return HISMSlab;
        case 1:  return HISMChunk;
        case 2:  return HISMGravel;
        case 3:  return HISMDust;
        default: return nullptr;
    }
}

void USoulForgeFragmentRenderer::EnsureHISMsCreated()
{
    if (AActor* Owner = GetOwner())
    {
        TArray<UInstancedStaticMeshComponent*> FoundHISMs;
        Owner->GetComponents<UInstancedStaticMeshComponent>(FoundHISMs);

        for (auto* H : FoundHISMs) {
            FString HName = H->GetName();
            if (HName.Contains(TEXT("HISMSlab")))   HISMSlab = H;
            else if (HName.Contains(TEXT("HISMChunk")))  HISMChunk = H;
            else if (HName.Contains(TEXT("HISMGravel"))) HISMGravel = H;
            else if (HName.Contains(TEXT("HISMDust")))   HISMDust = H;
        }
    }

    // Inicializar o actualizar mallas si es necesario
    auto SetupHISM = [&](TObjectPtr<UInstancedStaticMeshComponent>& Target, UStaticMesh* Mesh, const FName& Name) {
        if (!Target) {
            Target = InitHISM(Mesh, Name);
        } else if (Mesh && !Target->GetStaticMesh()) {
            Target->SetStaticMesh(Mesh);
            if (FragmentMaterial) Target->SetMaterial(0, FragmentMaterial);
        }
    };

    SetupHISM(HISMSlab,   SlabMesh,   TEXT("SF_HISMSlab"));
    SetupHISM(HISMChunk,  ChunkMesh,  TEXT("SF_HISMChunk"));
    SetupHISM(HISMGravel, GravelMesh, TEXT("SF_HISMGravel"));
    SetupHISM(HISMDust,   DustMesh,   TEXT("SF_HISMDust"));
}

void USoulForgeFragmentRenderer::UpdateHISMTransforms(int32 Category, const TArray<FTransform>& NewTransforms)
{
    UInstancedStaticMeshComponent* TargetHISM = HISMForCategory(Category);
    if (!TargetHISM) return;

    int32 CurrentInstances = TargetHISM->GetInstanceCount();
    int32 RequiredInstances = NewTransforms.Num();

    // 1. Ampliar el pool si es necesario (Add es barato)
    if (CurrentInstances < RequiredInstances)
    {
        TArray<FTransform> PaddingTransforms;
        PaddingTransforms.Init(FTransform(FQuat::Identity, FVector::ZeroVector, FVector::ZeroVector), RequiredInstances - CurrentInstances);
        TargetHISM->AddInstances(PaddingTransforms, false);
    }
    
    // 2. Actualización total o parcial
    if (RequiredInstances > 0)
    {
        TargetHISM->BatchUpdateInstancesTransforms(0, NewTransforms, true, true, true);
    }

    // 3. Ocultar los sobrantes (Pool Management)
    // En lugar de RemoveInstance (muy lento), escalamos a 0 los que no necesitemos hoy.
    if (CurrentInstances > RequiredInstances)
    {
        TArray<int32> IndicesToHide;
        TArray<FTransform> HideTransforms;
        for (int32 i = RequiredInstances; i < CurrentInstances; ++i)
        {
            IndicesToHide.Add(i);
            HideTransforms.Add(FTransform(FQuat::Identity, FVector::ZeroVector, FVector::ZeroVector));
        }
        TargetHISM->BatchUpdateInstancesTransforms(RequiredInstances, HideTransforms, true, true, true);
    }
}
