#include "SoulForgeVFXLibrary.h"
#include "Engine/World.h"
#include "DrawDebugHelpers.h"
#include "SoulForgeBridge.h"
#include "Kismet/GameplayStatics.h"

void USoulForgeVFXLibrary::GenerarOndaReactiva(UObject* WorldContextObject, FVector Epicentro, float Energia)
{
    if (!WorldContextObject || !WorldContextObject->GetWorld()) return;
    UWorld* World = WorldContextObject->GetWorld();

    // 1. EL LÁSER ESCÁNER: Disparamos un rayo 5 metros hacia abajo para detectar el suelo
    FHitResult Hit;
    FVector HaciaAbajo = Epicentro - FVector(0, 0, 500.0f);
    
    FCollisionQueryParams Params;
    Params.bReturnPhysicalMaterial = true;
    Params.AddIgnoredActor(Cast<AActor>(WorldContextObject));
    Params.AddIgnoredActor(WorldContextObject->GetTypedOuter<AActor>());

    if (World->LineTraceSingleByChannel(Hit, Epicentro, HaciaAbajo, ECC_Visibility, Params))
    {
        // 2. IDENTIFICAR EL TERRENO
        EPhysicalSurface TipoSuelo = SurfaceType_Default;
        if (Hit.PhysMaterial.IsValid()) {
            TipoSuelo = Hit.PhysMaterial->SurfaceType;
        }

        // 3. CONSULTAR A RUST LA FÍSICA DE LA ONDA
        FShockwaveData DatosOnda = USoulForgeBridge::CalcularOnda(Energia);

        // 4. LA REACCIÓN MODULAR
        FString NombreMaterial = TEXT("Suelo Genérico");
        FColor ColorOnda = FColor::White;

        // Mapeo de superficies (Asignar en Project Settings -> Physics)
        switch (TipoSuelo)
        {
            case SurfaceType1: // Concreto
                NombreMaterial = TEXT("Concreto (Humo Gris)");
                ColorOnda = FColor::Silver;
                break;
            case SurfaceType2: // Arena / Tierra
                NombreMaterial = TEXT("Tierra (Polvo Marrón)");
                ColorOnda = FColor::Orange;
                break;
            case SurfaceType3: // Agua
                NombreMaterial = TEXT("Agua (Salpicadura)");
                ColorOnda = FColor::Cyan;
                break;
            case SurfaceType4: // Metal
                NombreMaterial = TEXT("Metal (Chispas)");
                ColorOnda = FColor::Yellow;
                break;
            default:
                break;
        }

        UE_LOG(LogTemp, Display, TEXT("[SoulForge VFX] Onda sobre %s | R: %.1f cm | V: %.1f cm/s"), 
            *NombreMaterial, DatosOnda.RadiusCM, DatosOnda.SpeedCMS);
        
        // 5. DEBUG VISUAL (Anillo expansivo)
        DrawDebugCircle(World, Hit.ImpactPoint + FVector(0,0,5), DatosOnda.RadiusCM, 32, ColorOnda, false, 3.0f, 0, 5.0f, FVector(0,0,1), FVector(1,0,0), false);
        DrawDebugSphere(World, Hit.ImpactPoint, 15.0f, 12, ColorOnda, false, 2.0f);
    }
}
