// ─── Source/SoulForgePhysX/Private/SoulForgeSubsystem.cpp ─────────────────
#include "SoulForgeSubsystem.h"
#include "Logging/LogMacros.h"
#include "SoulForgeBridge.h"

void USoulForgeSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);
    // No limpiamos el núcleo aquí para evitar borrar actores que se registran temprano
}

void USoulForgeSubsystem::OnWorldBeginPlay(UWorld& InWorld)
{
    Super::OnWorldBeginPlay(InWorld);
    UE_LOG(LogTemp, Display, TEXT("[SoulForge] Subsystem Inicializado: Modo Maestro Activo."));
}

void USoulForgeSubsystem::SyncGlobalPower()
{
    USoulForgeBridge::SetGlobalPower(GlobalEnergyScale);
}

void USoulForgeSubsystem::EmergencyReset()
{
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge] !!! EMERGENCY RESET ACTIVATED !!!"));
    // El núcleo maestro ahora gestiona su propia limpieza interna
    USoulForgeBridge::LimpiarNucleo();
}

void USoulForgeSubsystem::ApplyPerformanceSettings()
{
    // Las configuraciones de hilos y GPU ahora se auto-gestionan en el inicio del DLL
}

void USoulForgeSubsystem::Deinitialize()
{
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge] Apagando Subsystem... Limpiando memoria."));
    
    // Disparamos la limpieza global en Rust al detener la simulación
    USoulForgeBridge::LimpiarNucleo();

    Super::Deinitialize();
}
