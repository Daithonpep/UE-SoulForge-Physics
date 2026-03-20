// ─── SoulForgeFragmentRenderer.h ─────────────────────────────────────────────
//
//  Renderiza todos los fragmentos físicos usando
//  UHierarchicalInstancedStaticMeshComponent (HISM).
//
//  POR QUÉ HISM:
//    Un edificio de tamaño mediano genera ~40 fragmentos.
//    Con Actors individuales eso son 40 draw calls por frame.
//    Con HISM son 4 draw calls (uno por categoría de fragmento),
//    independientemente de cuántos fragmentos haya en escena.
//
//  FLUJO:
//    1. Detonación Nativa → envía ProxyId a Rust → rellena NodePool.
//    2. Tick → el componente gestiona el renderizado desde la Shared Memory.
//    3. FinalizeFragment()   → cuando state==Sleeping, deja de consultar Rust.
//    4. Cleanup()            → elimina todas las instancias del HISM.

#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "Components/InstancedStaticMeshComponent.h"
#include "Engine/StaticMesh.h"
#include "SoulForgeBridge.h"
#include "SoulForgeFragmentRenderer.generated.h"

// ─── Info de un fragmento individual ─────────────────────────────────────────

USTRUCT(BlueprintType)
struct FSoulForgeFragmentInstance
{
    GENERATED_BODY()

    /** ID del fragmento en la simulación de Rust (fragment_sim). */
    UPROPERTY(BlueprintReadOnly)
    int32 FragmentSimId = -1;

    /** Índice de la instancia en su HISM. */
    UPROPERTY(BlueprintReadOnly)
    int32 InstanceIndex = -1;

    /** Categoría: 0=Slab, 1=Chunk, 2=Gravel, 3=Dust */
    UPROPERTY(BlueprintReadOnly)
    int32 Category = 0;

    /** Si es true, el fragmento está en reposo y no se consulta Rust. */
    UPROPERTY(BlueprintReadOnly)
    bool bSleeping = false;
};

// ─── Renderer Component ───────────────────────────────────────────────────────

UCLASS(ClassGroup=(SoulForge), meta=(BlueprintSpawnableComponent))
class SOULFORGEPHYSX_API USoulForgeFragmentRenderer : public UInstancedStaticMeshComponent
{
	GENERATED_BODY()

public:
	USoulForgeFragmentRenderer();

	/** 
	 * ESTA ES LA FUNCIÓN QUE BUSCABAS: Aparecerá al arrastrar el cable del componente.
	 * Procesa los datos de explosión en formato JSON y registra los fragmentos.
	 */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Destruction")
	void ExplodeIntoFragments(FString JsonData);

	UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
	void ClearAllFragments();

    UFUNCTION(BlueprintCallable, Category = "SoulForge|Renderer")
    void UpdateHISMTransforms(int32 Category, const TArray<FTransform>& NewTransforms);

    /** Señaliza que ha ocurrido una explosión para activar efectos internos */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Renderer")
    void NotifyExplosion() { UE_LOG(LogTemp, Warning, TEXT("[SoulForge-Renderer] Explosión notificada.")); }

	UInstancedStaticMeshComponent* HISMForCategory(int32 Category);
    void EnsureHISMsCreated();

	// ── Configuración de Meshes por categoría ─────────────────────────────────

	/** Mesh para fragmentos tipo Losa (grandes, pesados). */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Meshes")
	TObjectPtr<UStaticMesh> SlabMesh;

	/** Mesh para cascotes medianos. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Meshes")
	TObjectPtr<UStaticMesh> ChunkMesh;

	/** Mesh para grava fina. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Meshes")
	TObjectPtr<UStaticMesh> GravelMesh;

	/** Mesh para polvo/partículas muy pequeñas. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Meshes")
	TObjectPtr<UStaticMesh> DustMesh;

	/** Material base compartido por todos los fragmentos. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Meshes")
	TObjectPtr<UMaterialInterface> FragmentMaterial;

	// ── API Original (Preservada para compatibilidad interna) ───────────────────

	/**
	 * Registra los fragmentos de una destrucción para que sean renderizados.
	 */

	/** Actualiza las transformadas de todos los fragmentos activos. Llamar en Tick. */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Renderer")
	void TickUpdate(float DeltaTime);

	/** Elimina todas las instancias HISM y limpia el estado. */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Renderer")
	void Cleanup();

	/** Número de fragmentos aún en vuelo (no dormidos). */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Renderer")
	int32 GetActiveFragmentCount() const;

protected:
	virtual void BeginPlay() override;
	virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;

private:
	// Cuatro ISM: uno por categoría (Slab, Chunk, Gravel, Dust).
	UPROPERTY()
	TObjectPtr<UInstancedStaticMeshComponent> HISMSlab;

	UPROPERTY()
	TObjectPtr<UInstancedStaticMeshComponent> HISMChunk;

	UPROPERTY()
	TObjectPtr<UInstancedStaticMeshComponent> HISMGravel;

	UPROPERTY()
	TObjectPtr<UInstancedStaticMeshComponent> HISMDust;

	/** Lista de todos los fragmentos registrados. */
	TArray<FSoulForgeFragmentInstance> Fragments;

	/** Cuántos fragmentos siguen activos (no dormidos). */
	int32 ActiveCount = 0;

	UInstancedStaticMeshComponent* InitHISM(
		UStaticMesh* Mesh,
		const FName& Name
	);

	// Helper para parsear JSON (movido desde Destructible para autosuficiencia)
	bool ParseDestructionJSON(
		const FString& Json,
		TArray<int32>& OutFragSimIds,
		TArray<int32>& OutCategories,
		TArray<float>& OutRadii
	);
};
