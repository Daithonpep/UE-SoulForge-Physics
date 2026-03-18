#include "SoulForgePhysX.h"
#include "SoulForgeBridge.h"
#include "Misc/Paths.h"
#include "HAL/PlatformProcess.h"
#include "Interfaces/IPluginManager.h"

#define LOCTEXT_NAMESPACE "FSoulForgePhysXModule"

FDestroyProxyFunc FSoulForgePhysXModule::CppDestroyProxy = nullptr;
FDestroyProxyAvanzadoFunc FSoulForgePhysXModule::CppDestroyProxyAvanzado = nullptr;
extern DetonarNativoFunc CppDetonarNativo;
extern GetEngineStateFunc CppGetEngineState;
extern InsightFilterFunc CppInsightFilter;

void FSoulForgePhysXModule::StartupModule()
{
    FString PluginBaseDir = IPluginManager::Get().FindPlugin(TEXT("SoulForgePhysX"))->GetBaseDir();
    FString DllPath = FPaths::Combine(*PluginBaseDir, TEXT("Binaries/Win64/soulforge_core.dll"));

    PhysXHandle = FPlatformProcess::GetDllHandle(*DllPath);

    if (PhysXHandle)
    {
        UE_LOG(LogTemp, Display, TEXT("[SoulForge] NÚCLEO MAESTRO UNIFICADO CARGADO: %s"), *DllPath);
        
        CppDestroyProxy = (FDestroyProxyFunc)FPlatformProcess::GetDllExport(PhysXHandle, TEXT("sf_destroy_proxy"));
        CppDestroyProxyAvanzado = (FDestroyProxyAvanzadoFunc)FPlatformProcess::GetDllExport(PhysXHandle, TEXT("sf_destroy_proxy_advanced"));
        CppDetonarNativo = (DetonarNativoFunc)FPlatformProcess::GetDllExport(PhysXHandle, TEXT("sf_detonar_nativo"));
        CppGetEngineState = (GetEngineStateFunc)FPlatformProcess::GetDllExport(PhysXHandle, TEXT("sf_get_engine_state"));
        CppInsightFilter = (InsightFilterFunc)FPlatformProcess::GetDllExport(PhysXHandle, TEXT("sf_insight_filter"));

        if (!CppDetonarNativo || !CppGetEngineState || !CppInsightFilter) {
            UE_LOG(LogTemp, Warning, TEXT("[SoulForge] No se pudieron enlazar todos los simbolos Laser."));
        }
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge] ERROR: No se pudo cargar soulforge_core.dll en %s"), *DllPath);
    }
}

void FSoulForgePhysXModule::ShutdownModule()
{
    if (PhysXHandle)
    {
        FPlatformProcess::FreeDllHandle(PhysXHandle);
        PhysXHandle = nullptr;
    }
}

#undef LOCTEXT_NAMESPACE
IMPLEMENT_MODULE(FSoulForgePhysXModule, SoulForgePhysX)
