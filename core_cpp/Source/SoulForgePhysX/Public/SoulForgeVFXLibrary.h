#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "PhysicalMaterials/PhysicalMaterial.h"
#include "SoulForgeVFXLibrary.generated.h"

UCLASS()
class SOULFORGEPHYSX_API USoulForgeVFXLibrary : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    /**
     * Genera una onda reactiva basada en el terreno.
     * @param WorldContextObject Objeto de contexto (ej. el componente o actor detonante)
     * @param Epicentro Posición de la detonación
     * @param Energia Fuerza de la explosión para escalar efectos
     */
    UFUNCTION(BlueprintCallable, Category = "SoulForge | Efectos Visuales")
    static void GenerarOndaReactiva(UObject* WorldContextObject, FVector Epicentro, float Energia);
};
