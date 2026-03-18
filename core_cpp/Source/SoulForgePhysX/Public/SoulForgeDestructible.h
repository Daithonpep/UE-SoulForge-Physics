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

// Las formas en las que Rust romperá el objeto
UENUM(BlueprintType)
enum class ESoulForgePattern : uint8 {
    Radial      UMETA(DisplayName = "Explosión Radial (Esfera)"),
    Directional UMETA(DisplayName = "Perforación (Bala)"),
    Implosion   UMETA(DisplayName = "Implosión (Colapso Interno)")
};

// Esta es la estructura súper optimizada (Solo números puros)
USTRUCT(BlueprintType)
struct FSoulForgeNode {
    GENERATED_BODY()
    
    FVector Posicion;
    FQuat Rotacion;
    FVector Velocidad;
    FVector Escala3D;
    uint8 bEstaActivo; // 0 = Dormido, 1 = Explotando
};

// ─── Componente Destruible ────────────────────────────────────────────────────

UCLASS(ClassGroup=(SoulForge), meta=(BlueprintSpawnableComponent))
class SOULFORGEPHYSX_API USoulForgeDestructible : public UActorComponent
{
	GENERATED_BODY()

public:
	USoulForgeDestructible();

	// ── Configuración (Mejorada para Panel de Detalles) ──────────────────────────

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats")
	FString ProxyName;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats")
	int32 ProxyId = 0;

	/** DNI persistente calculado por el componente */
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats")
	int32 PersistentDNI = 0;

	/** ADN único del objeto (Persistente entre sesiones) */
	UPROPERTY(VisibleAnywhere, Category = "SoulForge|Stats")
	FGuid PersistentGUID;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
	EMaterialType MaterialType = EMaterialType::Concrete35MPa;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
    EPhysicsLOD PhysicsLOD = EPhysicsLOD::Primary;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
    EExplosiveType ExplosiveType = EExplosiveType::TNT;

	/**
	 * Semi-extensiones de la caja AABB del objeto (cm).
	 * X=ancho/2, Y=profundidad/2, Z=altura/2.
	 */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Geometry")
	FVector HalfExtent = FVector(50.f, 50.f, 100.f);

	/** Preset de daño aplicado cuando se destruya este objeto. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Destruction")
	ESoulForgeDamagePreset DamagePreset = ESoulForgeDamagePreset::Explosion;

	/** Sistema Niagara para el polvo y partículas finas (Dust category). */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|VFX")
	TObjectPtr<UNiagaraSystem> DustVFX;

	/**
	 * Referencia al renderer de fragmentos del mismo Actor.
	 * Se busca automáticamente en BeginPlay.
	 */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Renderer")
	TObjectPtr<USoulForgeFragmentRenderer> FragmentRenderer;

	/** Sensibilidad de la simulación de fragmentos (0.0 = caótico, 1.0 = rígido). */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Config")
	float StabilityThreshold = 0.5f;

    // --- CONFIGURACIÓN DE IMPACTO (Panel Maestro) ---
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
    float FuerzaDeImpacto = 500000000.0f; // 500 Millones por defecto

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
    float RadioDeImpacto = 150.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio")
    ESoulForgePattern FormaDeExplosion = ESoulForgePattern::Directional;

    // --- CONFIGURACIÓN DE FRAGMENTOS (NODOS) ---
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio", meta = (ToolTip = "Cantidad de fragmentos. Poner en 0 para que Rust calcule el número real según la física de Grady-Kipp."))
    int32 CantidadDeNodos = 0; // 0 = Auto (Grady-Kipp)

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Laboratorio", meta = (ToolTip = "Nodos que genera el Kamehameha para el estrés test."))
    int32 CantidadDeNodosHecatombe = 10000;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge | Visuals")
    bool bUsarRenderizadoRustico = true;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge | Visuals")
    class UNiagaraComponent* SharedNiagaraVFX;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Asistencia")
    bool bActivarMiraLaser = true; // Dibuja la trayectoria y el radio de impacto

	// ── API Blueprint ─────────────────────────────────────────────────────────

	/** Función para que el objeto se oculte y desactive colisiones solo */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Actions")
	void DeactivateOriginal();

	/**
	 * Destruye el objeto: oculta el mesh original y activa la simulación.
	 */
	UFUNCTION(BlueprintCallable, Category = "SoulForge")
	void TriggerDestruction();

    /** 🚀 NUEVA API OPTIMIZADA (Llamado masivo de nodos) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge")
    void SpawnNodeGrid(FVector Center, int32 Width, int32 Height, int32 Depth, float Spacing);

    UFUNCTION(BlueprintCallable, Category = "SoulForge")
    void ActivateKamehameha(FVector Origin, FVector Direction, float Intensity);

	/** Indica si el componente tiene conexión activa con el núcleo Rust. */
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "SoulForge|Stats")
	bool bIsEngineReady = false;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Stats")
	bool bDestroyed = false;

	/**
	 * Re-vincula el objeto con el núcleo de Rust si el ProxyId se ha perdido o es inválido.
	 */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Admin")
	void AutoLinkToRust();

	/** Botón en el Editor para resetear el estado interno. */
	UFUNCTION(CallInEditor, Category = "SoulForge|Debug")
	void ResetInternalState();

    /** 🧪 LABORATORIO: Pre-visualizar patrón de explosión sin simular nada */
    UFUNCTION(CallInEditor, Category = "SoulForge|Laboratorio")
    void PreviewExplosion();

    UPROPERTY(EditAnywhere, Category = "SoulForge|Laboratorio")
    float PreviewDuration = 2.0f;

	/** Botón mágico que aparecerá en el editor de Unreal */
	UFUNCTION(CallInEditor, Category = "SoulForge|Debug")
	void ForzarEncendidoRust();

    // --- PANEL DE RENDIMIENTO INSIGHT ---

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rendimiento", meta = (ClampMin = "1", ClampMax = "32"))
    int32 NucleosCPU = 8; // Tú decides cuántos núcleos dedicarle

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "SoulForge|Rendimiento")
    bool bUsarAceleracionGPU = true; // Activar o desactivar la GPU

    // El botón maestro para encender el monitor de Insight
    UFUNCTION(CallInEditor, Category = "SoulForge|Rendimiento")
    void ActivarInsight();


	/** Aplica daño al objeto. */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Destruction")
	void ApplyDamage(float DamageAmount);

    /** LA ÚNICA FUNCIÓN QUE USARÁS EN BLUEPRINTS (Panel Maestro) */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Acción")
    void RecibirImpacto(FVector PuntoDeGolpe, FVector Direccion);

	/** Salud actual del objeto [0.0 = destruido, 1.0 = intacto]. */
	UFUNCTION(BlueprintCallable, BlueprintPure, Category = "SoulForge|State")
	float GetHealth() const { return Health; }

    // La piscina de memoria que no crasheará tu PC
    TArray<FSoulForgeNode> NodePool;

    // El componente que dibujará los pedazos a la velocidad de la luz
    UPROPERTY()
    class UInstancedStaticMeshComponent* InstancedRenderer;

    UPROPERTY(EditAnywhere, Category = "SoulForge|Memoria")
    int32 MaxNodos = 1000; // Reserva memoria para 1000 pedazos desde el inicio

    // Tu malla genérica (El cubito o roca que usaremos como "Nodo")
    UPROPERTY(EditAnywhere, Category = "SoulForge|Visual")
    UStaticMesh* MallaDelNodo;

    // El material maestro del fragmento (donde inyectaremos la textura)
    UPROPERTY(EditAnywhere, Category = "SoulForge|Visual")
    UMaterialInterface* MasterShardMaterial;

    /** Captura la textura del objeto original y la inyecta en los fragmentos */
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Visual")
    void PreparaRenderizadoFragmentos();

protected:
	virtual void OnRegister() override;
	virtual void PostInitProperties() override;
	virtual void BeginPlay()  override;
	virtual void TickComponent(float DeltaTime, ELevelTick TickType,
							   FActorComponentTickFunction* ThisTickFunction) override;
	virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;

private:
    float Health         = 1.0f;
    float TimeSincePurge = 0.0f;

    float GetDensity() const;

};
