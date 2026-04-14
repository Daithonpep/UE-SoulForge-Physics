#include "RustPhysicsComponent.h"
#include "SoulForgeBridge.h"
#include "Components/InstancedStaticMeshComponent.h"
#include "Engine/StaticMesh.h"
#include "UObject/ConstructorHelpers.h"
#include "Engine/World.h"
#include "Logging/LogMacros.h"

static EngineHandle* GRustHandle = nullptr;

URustPhysicsComponent::URustPhysicsComponent()
{
	PrimaryComponentTick.bCanEverTick = true;
    PrimaryComponentTick.TickGroup = TG_PrePhysics;
}

void URustPhysicsComponent::BeginPlay()
{
	Super::BeginPlay();

    if (!GRustHandle)
    {
        GRustHandle = sf_physics_engine_create(MaxNodes);
    }

    // Inicializar ISMC
    InstancedMesh = NewObject<UInstancedStaticMeshComponent>(GetOwner());
    if (InstancedMesh)
    {
        InstancedMesh->SetupAttachment(GetOwner()->GetRootComponent());
        InstancedMesh->RegisterComponent();
        
        if (NodeMesh)
        {
            InstancedMesh->SetStaticMesh(NodeMesh);
        }
        
        InstancedMesh->SetMobility(EComponentMobility::Movable);
        InstancedMesh->SetCastShadow(false);
        InstancedMesh->SetCollisionEnabled(ECollisionEnabled::NoCollision);
        InstancedMesh->NumCustomDataFloats = 4; // Para colores custom
    }

    if (GRustHandle)
    {
        UE_LOG(LogTemp, Display, TEXT("[SoulForge-Rust] 🚀 Núcleo de Física Rust CONECTADO (Handle: %p)"), GRustHandle);
        sf_physics_engine_set_gravity(GRustHandle, Gravity.X, Gravity.Y, Gravity.Z);
    }
}

void URustPhysicsComponent::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	Super::EndPlay(EndPlayReason);
}

void URustPhysicsComponent::TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction)
{
	Super::TickComponent(DeltaTime, TickType, ThisTickFunction);

    if (!GRustHandle) return;

    // Tick físico
    sf_physics_engine_tick(GRustHandle, DeltaTime);

    // Actualizar visuales
    if (bUseInstancedRendering && InstancedMesh)
    {
        UpdateISMCInstances();
    }
}

void URustPhysicsComponent::UpdateISMCInstances()
{
    FRenderData Data;
    if (sf_physics_engine_get_render_data(GRustHandle, &Data))
    {
        if (Data.Count == 0)
        {
            InstancedMesh->ClearInstances();
            return;
        }

        if (Data.Count != InstancedMesh->GetInstanceCount())
        {
            InstancedMesh->ClearInstances();
            InstancedMesh->PreAllocateInstancesMemory(Data.Count);

            for (int32 i = 0; i < Data.Count; ++i)
            {
                FTransform T;
                T.SetLocation(FVector(Data.PositionsPtr[i*3], Data.PositionsPtr[i*3+1], Data.PositionsPtr[i*3+2]));
                T.SetScale3D(FVector(Data.ScalesPtr[i]));
                
                int32 Idx = InstancedMesh->AddInstance(T);
                
                uint32 C = Data.ColorsPtr[i];
                FLinearColor LinC(
                    (float)(C & 0xFF) / 255.f,
                    (float)((C >> 8) & 0xFF) / 255.f,
                    (float)((C >> 16) & 0xFF) / 255.f,
                    (float)((C >> 24) & 0xFF) / 255.f
                );
                
                InstancedMesh->SetCustomDataValue(Idx, 0, LinC.R);
                InstancedMesh->SetCustomDataValue(Idx, 1, LinC.G);
                InstancedMesh->SetCustomDataValue(Idx, 2, LinC.B);
                InstancedMesh->SetCustomDataValue(Idx, 3, LinC.A);
            }
        }
        else
        {
            for (int32 i = 0; i < Data.Count; ++i)
            {
                FTransform T;
                T.SetLocation(FVector(Data.PositionsPtr[i*3], Data.PositionsPtr[i*3+1], Data.PositionsPtr[i*3+2]));
                T.SetScale3D(FVector(Data.ScalesPtr[i]));
                
                InstancedMesh->UpdateInstanceTransform(i, T, false, false, false);
                
                uint32 C = Data.ColorsPtr[i];
                InstancedMesh->SetCustomDataValue(i, 0, (float)(C & 0xFF) / 255.f, false);
                InstancedMesh->SetCustomDataValue(i, 1, (float)((C >> 8) & 0xFF) / 255.f, false);
                InstancedMesh->SetCustomDataValue(i, 2, (float)((C >> 16) & 0xFF) / 255.f, false);
                InstancedMesh->SetCustomDataValue(i, 3, (float)((C >> 24) & 0xFF) / 255.f, false);
            }
            InstancedMesh->MarkRenderStateDirty();
        }
    }
}

void URustPhysicsComponent::ActivateKamehameha(FVector Origin, FVector Direction, float Intensity)
{
    if (GRustHandle)
    {
        sf_physics_engine_activate_kamehameha(GRustHandle, Origin.X, Origin.Y, Origin.Z, Direction.X, Direction.Y, Direction.Z, Intensity);
    }
}

void URustPhysicsComponent::SpawnNodeGrid(FVector Center, int32 Width, int32 Height, int32 Depth, float Spacing)
{
    if (!GRustHandle) return;

    for (int32 x = 0; x < Width; ++x)
    {
        for (int32 y = 0; y < Height; ++y)
        {
            for (int32 z = 0; z < Depth; ++z)
            {
                FVector Pos = Center + FVector(
                    (x - Width / 2) * Spacing,
                    (y - Height / 2) * Spacing,
                    (z - Depth / 2) * Spacing
                );
                sf_physics_engine_add_node(GRustHandle, Pos.X, Pos.Y, Pos.Z, 1.0f, 0);
            }
        }
    }
}
