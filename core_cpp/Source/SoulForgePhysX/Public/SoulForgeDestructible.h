// ─── SoulForgeDestructible.h ──────────────────────────────────────────────────
#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "SoulForgeBridge.h"
#include "SoulForgeFragmentRenderer.h"
#include "Dom/JsonObject.h"
#include "Serialization/JsonReader.h"
#include "Serialization/JsonSerializer.h"
#include "NiagaraFunctionLibrary.h"
#include "NiagaraSystem.h"
#include "SoulForgeDestructible.generated.h"

UENUM(BlueprintType)
enum class ESoulForgePattern : uint8 {
    Radial      UMETA(DisplayName = "Explosión Radial (Esfera)"),
    Directional UMETA(DisplayName = "Perforación (Bala)"),
    Implosion   UMETA(DisplayName = "Implosión (Colapso Interno)")
};

USTRUCT(BlueprintType)
struct FSoulForgeNode {
    GENERATED_BODY()
    
    UPROPERTY(BlueprintReadOnly) FVector Posicion;
    UPROPERTY(BlueprintReadOnly) FQuat Rotacion;
    UPROPERTY(BlueprintReadOnly) FVector Velocidad;
    UPROPERTY(BlueprintReadOnly) FVector Escala3D;
    UPROPERTY(BlueprintReadOnly) uint8 bEstaActivo;
};

UCLASS(ClassGroup=(SoulForge), meta=(BlueprintSpawnableComponent))
class SOULFORGEPHYSX_API USoulForgeDestructible : public UActorComponent
{
	GENERATED_BODY()

public:
	USoulForgeDestructible();

	// ── Configuración Maestra (V8.4.2) ───────────────────────────────────────────

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats") FString ProxyName;
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats") int32 ProxyId = 0;
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats") int32 PersistentDNI = 0;
	UPROPERTY(VisibleAnywhere, Category = "SoulForge|Stats") FGuid PersistentGUID;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") EMaterialType MaterialType = EMaterialType::Concrete35MPa;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") EPhysicsLOD PhysicsLOD = EPhysicsLOD::Primary;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") EExplosiveType ExplosiveType = EExplosiveType::TNT;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") float FuerzaDeImpacto = 500000000.0f;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") float RadioDeImpacto = 150.0f;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") ESoulForgePattern FormaDeExplosion = ESoulForgePattern::Radial;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") int32 CantidadDeNodos = 1000;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") int32 CantidadDeNodosHecatombe = 10000;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") bool bUsarRenderizadoRustico = true;
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") float PreviewDuration = 2.0f;
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio") bool bActivarHUD = true;
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio", meta=(ClampMin="0.1", ClampMax="10.0")) float FragmentacionJupiter = 1.0f;

	// ── Geometría y Render ──

    UPROPERTY(EditAnywhere, Category = "SoulForge|Config") UStaticMesh* MallaDelNodo;
    UPROPERTY(EditAnywhere, Category = "SoulForge|Config") int32 MaxNodos = 2000;
    UPROPERTY(EditAnywhere, Category = "SoulForge|Config") UMaterialInterface* MasterShardMaterial;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Geometry") FVector HalfExtent = FVector(50.f, 50.f, 100.f);
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Destruction") ESoulForgeDamagePreset DamagePreset = ESoulForgeDamagePreset::Explosion;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|VFX") TObjectPtr<UNiagaraSystem> DustVFX;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Renderer") TObjectPtr<USoulForgeFragmentRenderer> FragmentRenderer;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Config") float StabilityThreshold = 0.5f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Visuals") class UNiagaraComponent* SharedNiagaraVFX;
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Asistencia") bool bActivarMiraLaser = true;

	// ── API Blueprint ──

	UFUNCTION(BlueprintCallable, Category = "SoulForge|Destruction") void TriggerDestruction();
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Destruction") void ApplyDamage(float DamageAmount);
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Destruction") void RecibirImpacto(FVector PuntoDeGolpe, FVector Direccion);
    UFUNCTION(BlueprintCallable, CallInEditor, Category = "SoulForge|Laboratorio") void PreviewExplosion();
    UFUNCTION(BlueprintCallable, CallInEditor, Category = "SoulForge|Laboratorio") void ResetInternalState();
    
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Debug") void ForzarEncendidoRust();
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Debug") void ActivarInsight();
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Debug") void SpawnNodeGrid(FVector Center, int32 Width, int32 Height, int32 Depth, float Spacing);
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Debug") void ActivateKamehameha(FVector Origin, FVector Direction, float Intensity);

	virtual void BeginPlay() override;
    virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;
	virtual void TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction) override;
    virtual void OnRegister() override;
    virtual void PostInitProperties() override;

    float GetDensity() const;

protected:
	void SyncWithProxy();
	void ProcessFragmentData(const TArray<FSoulForgeFragmentData>& Data);
    void DeactivateOriginal();
    void AutoLinkToRust();
    void PreparaRenderizadoFragmentos();
    void UpdateNiagaraVisuals();

    // --- Estado Interno ---
    UPROPERTY() bool bDestroyed = false;
    UPROPERTY() float Health = 1.0f;
    UPROPERTY() uint32 ObjectFilterHash = 0;
    UPROPERTY() bool bIsEngineReady = false;
    UPROPERTY() TArray<FSoulForgeNode> NodePool;
    UPROPERTY() UInstancedStaticMeshComponent* InstancedRenderer;
};
