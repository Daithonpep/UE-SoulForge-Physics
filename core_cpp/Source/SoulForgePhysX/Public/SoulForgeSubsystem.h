// ─── Source/SoulForgePhysX/Public/SoulForgeSubsystem.h ───────────────────
#pragma once

#include "CoreMinimal.h"
#include "Subsystems/WorldSubsystem.h"
#include "SoulForgeBridge.h"
#include "SoulForgeSubsystem.generated.h"

// Modos de rendimiento para el usuario
UENUM(BlueprintType)
enum class ESoulForgePerformanceMode : uint8 {
    Eco      UMETA(DisplayName = "Ahorro (1-2 Núcleos)"),
    Balanced UMETA(DisplayName = "Equilibrado (Automático)"),
    Insight  UMETA(DisplayName = "Extreme Insight (GPU + Todos los Núcleos)")
};

/**
 * Cerebro Global de SoulForge.
 * Gestiona el Modo Dios, el rendimiento y la limpieza del motor.
 */
UCLASS()
class SOULFORGEPHYSX_API USoulForgeSubsystem : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    /** Escala global de energía para todas las explosiones. */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Global")
    float GlobalEnergyScale = 1.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Performance")
    ESoulForgePerformanceMode PerformanceMode = ESoulForgePerformanceMode::Balanced;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Performance")
    bool bEnableGPUAcceleration = true;

    /** Si true, se dibujará telemetría visual de IDs en el mundo (próximamente). */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Debug")
    bool bDrawDebugIDs = true;

    /** 
     * Botón de Pánico: Limpia Rust y Unreal a la vez.
     */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
    void EmergencyReset();

    /** Sincroniza la escala de energía con el núcleo de Rust. */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Global")
    void SyncGlobalPower();

    /** Envía los ajustes de rendimiento a la DLL de Rust. */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
    void ApplyPerformanceSettings();

    virtual void Initialize(FSubsystemCollectionBase& Collection) override;
    virtual void Deinitialize() override;

protected:
    virtual void OnWorldBeginPlay(UWorld& InWorld) override;
};
