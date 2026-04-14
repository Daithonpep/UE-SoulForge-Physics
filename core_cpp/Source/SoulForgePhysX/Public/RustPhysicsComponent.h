#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "Components/InstancedStaticMeshComponent.h"
#include "RustPhysicsComponent.generated.h"

// Datos de renderizado optimizados (Debe coincidir con ffi.rs)
USTRUCT(BlueprintType)
struct FRenderDataOptimized {
    GENERATED_BODY()
    
    UPROPERTY(BlueprintReadOnly) int32 Count;
    const float* PositionsPtr;
    const uint32* ColorsPtr;
    const float* ScalesPtr;
};

UCLASS(ClassGroup=(SoulForge), meta=(BlueprintSpawnableComponent))
class SOULFORGEPHYSX_API URustPhysicsComponent : public UActorComponent
{
    GENERATED_BODY()

public:
    URustPhysicsComponent();

    virtual void TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction) override;
    virtual void BeginPlay() override;
    virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;

    // === CONFIGURACIÓN ===
    
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rust Physics")
    int32 MaxNodes = 200000;
    
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rust Physics")
    UStaticMesh* NodeMesh;
    
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rust Physics")
    FVector Gravity = FVector(0, 0, -981.0f);
    
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rust Physics")
    bool bUseInstancedRendering = true;

    // === BLUEPRINT API ===
    
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Powers")
    void ActivateKamehameha(FVector Origin, FVector Direction, float Intensity);
    
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Debug")
    void SpawnNodeGrid(FVector Center, int32 Width, int32 Height, int32 Depth, float Spacing);

private:
    UPROPERTY()
    UInstancedStaticMeshComponent* InstancedMesh;

    void UpdateISMCInstances();
};
