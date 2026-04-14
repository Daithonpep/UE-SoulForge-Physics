// ─── SoulForgeHelper.h ───────────────────────────────────────────────────────
#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "Dom/JsonObject.h"
#include "Serialization/JsonReader.h"
#include "Serialization/JsonSerializer.h"
#include "SoulForgeHelper.generated.h"

/**
 * Biblioteca de funciones auxiliares para el ecosistema SoulForge.
 * El "Eslabón Perdido" que facilita la traducción de datos entre Rust y Unreal.
 */
UCLASS()
class SOULFORGEPHYSX_API USoulForgeHelper : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

public:
	/**
	 * Toma el JSON crudo de Rust y lo convierte en una lista de transformadas.
	 * Útil para visualizar o pre-procesar fragmentos en Blueprints.
	 * 
	 * @param JsonData Texto en formato JSON devuelto por el núcleo de Rust.
	 * @return Array de transformadas listas para usar en Unreal.
	 */
	UFUNCTION(BlueprintCallable, Category = "SoulForge|Helpers")
	static TArray<FTransform> ParseRustJson(FString JsonData);
};
