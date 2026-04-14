// ─── Source/SoulForgePhysX/Private/SoulForgeSubsystem.cpp ─────────────────
#include "SoulForgeSubsystem.h"
#include "Logging/LogMacros.h"
#include "SoulForgeBridge.h"

void USoulForgeSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);
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
    USoulForgeBridge::LimpiarNucleo();
}

void USoulForgeSubsystem::ApplyPerformanceSettings()
{
    // Las configuraciones de hilos y GPU ahora se auto-gestionan en el inicio del DLL
}

void USoulForgeSubsystem::Deinitialize()
{
    UE_LOG(LogTemp, Warning, TEXT("[SoulForge] Apagando Subsystem... Limpiando memoria."));
    USoulForgeBridge::LimpiarNucleo();
    Super::Deinitialize();
}
