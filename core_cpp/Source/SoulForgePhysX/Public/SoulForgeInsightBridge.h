#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "SoulForgeInsightBridge.generated.h"

UCLASS()
class SOULFORGEPHYSX_API USoulForgeInsightBridge : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    // Este nodo de Blueprint cargará el núcleo y probará la conexión
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Insight")
    static bool IniciarSoulForgeInsight();

    // Nodo para pasar el arreglo de Unreal a Rust y recibir los visibles
    UFUNCTION(BlueprintCallable, Category = "SoulForge|Insight")
    static TArray<int32> FiltrarInstanciasSoulForge(const TArray<FVector>& Posiciones, FVector CamaraPos, float DistanciaVision);
};
