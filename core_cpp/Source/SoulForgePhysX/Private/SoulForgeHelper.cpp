// ─── SoulForgeHelper.cpp ─────────────────────────────────────────────────────

#include "SoulForgeHelper.h"

TArray<FTransform> USoulForgeHelper::ParseRustJson(FString JsonData)
{
    TArray<FTransform> Transforms;
    TSharedPtr<FJsonObject> JsonObject;
    TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(JsonData);

    if (FJsonSerializer::Deserialize(Reader, JsonObject))
    {
        // El núcleo de Rust devuelve una lista llamada "fragments"
        const TArray<TSharedPtr<FJsonValue>>* Fragments = nullptr;
        if (JsonObject->TryGetArrayField(TEXT("fragments"), Fragments))
        {
            Transforms.Reserve(Fragments->Num());

            for (const auto& Value : *Fragments)
            {
                TSharedPtr<FJsonObject> Frag = Value->AsObject();
                if (!Frag.IsValid()) { continue; }
                
                // Extraemos posición (x, y, z)
                // Rust usa coordenadas de Unreal (cm)
                double X = Frag->GetNumberField(TEXT("x"));
                double Y = Frag->GetNumberField(TEXT("y"));
                double Z = Frag->GetNumberField(TEXT("z"));

                // Extraemos rotación (pitch, yaw, roll)
                double Pitch = 0.0, Yaw = 0.0, Roll = 0.0;
                Frag->TryGetNumberField(TEXT("pitch"), Pitch);
                Frag->TryGetNumberField(TEXT("yaw"),   Yaw);
                Frag->TryGetNumberField(TEXT("roll"),  Roll);
                
                FVector Pos(X, Y, Z);
                FRotator Rot(Pitch, Yaw, Roll);
                
                Transforms.Add(FTransform(Rot, Pos, FVector(1.0f))); // Escala inicial 1:1
            }
        }
    }

    return Transforms;
}
