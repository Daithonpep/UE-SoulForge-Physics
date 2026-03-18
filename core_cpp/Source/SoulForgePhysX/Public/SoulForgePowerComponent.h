#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "SoulForgeBridge.h"
#include "SoulForgePowerComponent.generated.h"



UCLASS(ClassGroup=(SoulForge), meta=(BlueprintSpawnableComponent))
class SOULFORGEPHYSX_API USoulForgePowerComponent : public UActorComponent
{
	GENERATED_BODY()

public:	
	USoulForgePowerComponent();

protected:
	virtual void BeginPlay() override;

public:	
	virtual void TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction) override;

    /** Activa un poder elemental en el motor físico */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Powers")
    void ActivatePower(EPowerType PowerType, FVector Origin, FVector Direction, float Intensity, float Duration);

    /** Activa una combinación de elementos */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Powers")
    void ActivateCombo(TArray<EPowerType> Elements, FVector Origin, FVector Direction, float Intensity, float Duration);

    /** Actualiza los visuales de Niagara con las posiciones de los nodos de Rust */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Visuals")
    void UpdateNiagaraVisuals();

    // --- DEBUG BUTTONS (Para el Panel de Detalles) ---
    UFUNCTION(BlueprintCallable, CallInEditor, Category = "SoulForge | Debug")
    void TestKamehameha();

    UFUNCTION(BlueprintCallable, CallInEditor, Category = "SoulForge | Debug")
    void TestFireTornado();

    UFUNCTION(BlueprintCallable, CallInEditor, Category = "SoulForge | Debug")
    void TestGravityWell();

    UPROPERTY(BlueprintAssignable, Category = "SoulForge | Events")
    FOnPowerVisualEffect OnPowerVisualPulse;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge | Cinematics")
    TSubclassOf<class UCameraShakeBase> PowerCameraShake;

    UPROPERTY(EditAnywhere, Category = "SoulForge | Visuals")
    bool bUsarRenderizadoRustico = true;

private:
    void PlayPowerEffects(EPowerType PowerType, FVector Origin, FVector Direction, float Intensity);

    UPROPERTY()
    TArray<FActivePowerInstance> ActivePowers;

    UPROPERTY(EditAnywhere, Category = "SoulForge | Visuals")
    class UNiagaraComponent* NiagaraVFX;
};
