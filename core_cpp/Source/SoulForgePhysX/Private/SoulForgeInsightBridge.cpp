#include "SoulForgeInsightBridge.h"
#include "Misc/Paths.h"
#include "HAL/PlatformProcess.h"
#include "Interfaces/IPluginManager.h"

// Variable global para guardar la DLL en la memoria
void* InsightDllHandle = nullptr;

// Definimos la "firma" de la función de Rust para que C++ sepa cómo llamarla
typedef int32 (*FilterInstancesFunc)(const FVector* Posiciones, int32 Count, double CamX, double CamY, double CamZ, double VisionDist, int32* OutIndices);

bool USoulForgeInsightBridge::IniciarSoulForgeInsight()
{
    // 1. Buscamos tu DLL en la carpeta Binaries del Plugin
    TSharedPtr<IPlugin> Plugin = IPluginManager::Get().FindPlugin(TEXT("SoulForgePhysX"));
    if (!Plugin.IsValid())
    {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge] ERROR: No se encontró el Plugin SoulForgePhysX activo."));
        return false;
    }

    FString PluginDir = Plugin->GetBaseDir();
    FString DllPath = FPaths::Combine(PluginDir, TEXT("Binaries"), TEXT("Win64"), TEXT("soulforge_physx.dll"));
    
    // 2. Intentamos cargarla en la memoria de Unreal si no está ya cargada
    if (InsightDllHandle == nullptr)
    {
        InsightDllHandle = FPlatformProcess::GetDllHandle(*DllPath);
    }
    
    if (InsightDllHandle != nullptr)
    {
        UE_LOG(LogTemp, Warning, TEXT("[SoulForge] NÚCLEO INSIGHT UNIFICADO (MAESTRO) CONECTADO. ¡Concurrencia Activa!"));
        return true;
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge] ERROR: No se encontró la DLL Maestra en: %s"), *DllPath);
        return false;
    }
}

TArray<int32> USoulForgeInsightBridge::FiltrarInstanciasSoulForge(const TArray<FVector>& Posiciones, FVector CamaraPos, float DistanciaVision)
{
    TArray<int32> Visibles;
    
    // Si la DLL no está cargada o no hay posiciones, devolvemos vacío
    if (!InsightDllHandle || Posiciones.Num() == 0) 
    {
        return Visibles;
    }

    // 1. Buscamos la función exacta dentro de tu archivo .dll
    FString FuncName = "sf_insight_filter_instances";
    FilterInstancesFunc RustFilter = (FilterInstancesFunc)FPlatformProcess::GetDllExport(InsightDllHandle, *FuncName);

    if (!RustFilter) 
    {
        UE_LOG(LogTemp, Error, TEXT("[SoulForge] No se encontró la función %s en la DLL."), *FuncName);
        return Visibles;
    }

    // 2. EL TRUCO DE MEMORIA: Preparamos un arreglo vacío del mismo tamaño que la entrada
    // SetNumUninitialized crea el espacio en RAM sin llenarlo, es ultra rápido.
    Visibles.SetNumUninitialized(Posiciones.Num());

    // 3. ¡Llamada al núcleo de Rust! (Cero copias, le pasamos los punteros directos)
    int32 CountVisibles = RustFilter(
        Posiciones.GetData(), 
        Posiciones.Num(), 
        CamaraPos.X, CamaraPos.Y, CamaraPos.Z, 
        (double)DistanciaVision, 
        Visibles.GetData()
    );

    // 4. Cortamos la caja vacía para que solo tenga el tamaño de los que realmente pasaron el filtro
    Visibles.SetNum(CountVisibles);

    return Visibles;
}
