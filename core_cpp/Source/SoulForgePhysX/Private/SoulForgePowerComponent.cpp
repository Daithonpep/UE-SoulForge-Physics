#include "SoulForgePowerComponent.h"
#include "SoulForgeBridge.h"
#include "Kismet/GameplayStatics.h"
#include "NiagaraComponent.h"
#include "NiagaraDataInterfaceArrayFunctionLibrary.h"

USoulForgePowerComponent::USoulForgePowerComponent()
{
	PrimaryComponentTick.bCanEverTick = true;
}

void USoulForgePowerComponent::BeginPlay()
{
	Super::BeginPlay();
}

void USoulForgePowerComponent::TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction)
{
	Super::TickComponent(DeltaTime, TickType, ThisTickFunction);

    // --- ¡CAMBIO CRÍTICO! ---
    // No salimos si no hay poderes locales, porque Rust podría estar generando nodos de efecto
    // que necesitamos pintar en Niagara.
    
    // Actualizar duraciones de poderes locales (si existen)
    TArray<FActivePowerPayload> Payloads;
    for (int32 i = ActivePowers.Num() - 1; i >= 0; --i)
    {
        FActivePowerInstance& Power = ActivePowers[i];
        Power.TimeRemaining -= DeltaTime;

        if (Power.TimeRemaining <= 0.0f)
        {
            ActivePowers.RemoveAt(i);
            continue;
        }

        FActivePowerPayload Payload;
        Payload.PowerType = (int32)Power.Type;
        Payload.Origin = Power.Origin;
        Payload.Direction = Power.Direction;
        Payload.Intensity = Power.Intensity;
        // Phase = Normalized progress (0 to 1)
        Payload.Phase = 1.0f - (Power.TimeRemaining / Power.TotalDuration);
        
        Payloads.Add(Payload);
    }

    // Un solo viaje por frame hacia Rust
    if (Payloads.Num() > 0)
    {
        USoulForgeBridge::UpdateAllPowers(Payloads, DeltaTime);
    }

    // Actualizar los visuales de Niagara
    UpdateNiagaraVisuals();

    // --- RENDERIZADO RÚSTICO (Mismo estilo que las explosiones) ---
    if (bUsarRenderizadoRustico)
    {
        int32 RustCount = 0;
        const float* RawPositions = USoulForgeBridge::GetNodePositions(&RustCount);
        if (RawPositions != nullptr && RustCount > 0)
        {
            for (int32 i = 0; i < RustCount; ++i)
            {
                FVector NodePos = FVector(RawPositions[i*3], RawPositions[i*3+1], RawPositions[i*3+2]);
                DrawDebugSolidBox(GetWorld(), NodePos, FVector(4.0f), FColor::Cyan, false, 0.05f);
            }
        }
    }
}

void USoulForgePowerComponent::UpdateNiagaraVisuals()
{
    if (!NiagaraVFX) return;

    int32 NodeCount = 0;
    const float* RawPositions = USoulForgeBridge::GetNodePositions(&NodeCount);

    if (NodeCount > 0 && RawPositions != nullptr)
    {
        TArray<FVector> PositionsArray;
        PositionsArray.SetNum(NodeCount); // Safe allocation

        for (int32 i = 0; i < NodeCount; ++i)
        {
            PositionsArray[i] = FVector(
                RawPositions[i * 3 + 0],
                RawPositions[i * 3 + 1],
                RawPositions[i * 3 + 2]
            );
        }

        // Enviar a Niagara vía Data Interface Array
        UNiagaraDataInterfaceArrayFunctionLibrary::SetNiagaraArrayVector(
            NiagaraVFX,
            FName("PosicionesDesdeRust"),
            PositionsArray
        );

        NiagaraVFX->SetIntParameter(FName("CantidadNodos"), NodeCount);
    }

    // --- NUEVO: Autopista Térmica ---
    int32 TempCount = 0;
    const float* RawTemperatures = USoulForgeBridge::GetNodeTemperatures(&TempCount);

    if (TempCount > 0 && RawTemperatures != nullptr)
    {
        TArray<float> TemperaturasArray;
        TemperaturasArray.SetNum(TempCount);
        
        for (int32 i = 0; i < TempCount; ++i)
        {
            TemperaturasArray[i] = RawTemperatures[i];
        }

        UNiagaraDataInterfaceArrayFunctionLibrary::SetNiagaraArrayFloat(
            NiagaraVFX,
            FName("TemperaturasDesdeRust"),
            TemperaturasArray
        );
    }
}

void USoulForgePowerComponent::ActivatePower(EPowerType PowerType, FVector Origin, FVector Direction, float Intensity, float Duration)
{
    // En lugar de llamar a Rust directamente, añadimos a la lista para el Tick
    FActivePowerInstance Power;
    Power.Type = PowerType;
    Power.Origin = Origin;
    Power.Direction = Direction;
    Power.Intensity = Intensity;
    Power.TotalDuration = Duration;
    Power.TimeRemaining = Duration;

    ActivePowers.Add(Power);

    // Feedback visual y sonoro (solo al inicio)
    PlayPowerEffects(PowerType, Origin, Direction, Intensity);
}

void USoulForgePowerComponent::ActivateCombo(TArray<EPowerType> Elements, FVector Origin, FVector Direction, float Intensity, float Duration)
{
    // Lógica simplificada: tomamos el primer elemento como base y aumentamos intensidad
    if (Elements.Num() > 0)
    {
        ActivatePower(Elements[0], Origin, Direction, Intensity * Elements.Num(), Duration);
    }
}

void USoulForgePowerComponent::PlayPowerEffects(EPowerType PowerType, FVector Origin, FVector Direction, float Intensity)
{
    // 1. Feedback en el Log
    UE_LOG(LogTemp, Log, TEXT("✨ SoulForge Power Played: %d at %s"), (int32)PowerType, *Origin.ToString());

    // 2. Camera Shake (Impacto Cinematográfico)
    if (PowerCameraShake)
    {
        APlayerController* PC = UGameplayStatics::GetPlayerController(GetWorld(), 0);
        if (PC && PC->IsLocalController())
        {
            // Solo si el impacto es lo suficientemente cerca o es un rayo masivo
            PC->ClientStartCameraShake(PowerCameraShake, Intensity / 1000.0f);
        }
    }

    // 3. Post-Process Bloom Pulse (Vía Delegado para Blueprints)
    // Enviamos una señal de intensidad de brillo para que el sol o el post-process reaccione
    float BloomIntensity = (PowerType == EPowerType::EnergyBeam) ? 2.5f : 1.2f;
    OnPowerVisualPulse.Broadcast(BloomIntensity);

    // 4. Inestabilidad del Rayo (Lógica de Rust ya configurada para vibrar)
}

void USoulForgePowerComponent::TestKamehameha()
{
    ActivatePower(EPowerType::EnergyBeam, GetOwner()->GetActorLocation(), GetOwner()->GetActorForwardVector(), 5000.0f, 3.0f);
}

void USoulForgePowerComponent::TestFireTornado()
{
    ActivatePower(EPowerType::FireBending, GetOwner()->GetActorLocation(), FVector(0,0,1), 2000.0f, 5.0f);
}

void USoulForgePowerComponent::TestGravityWell()
{
    ActivatePower(EPowerType::GravityBlackHole, GetOwner()->GetActorLocation(), FVector::ZeroVector, 1000.0f, 5.0f);
}
