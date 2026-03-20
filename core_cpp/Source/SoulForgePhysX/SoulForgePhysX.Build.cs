// ─── core_cpp/Source/SoulForgePhysX/SoulForgePhysX.Build.cs (v2) ─────────────
//
//  Script de compilación de Unreal (UnrealBuildTool) — versión completa con
//  todas las dependencias necesarias para el plugin.

using UnrealBuildTool;
using System.IO;

public class SoulForgePhysX : ModuleRules
{
    public SoulForgePhysX(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;

        // ── Dependencias públicas ─────────────────────────────────────────────
        PublicDependencyModuleNames.AddRange(new string[]
        {
            "Core",
            "CoreUObject",
            "Engine",
            "Json",              // Para parsear el JSON de DestructionResult
            "JsonUtilities",
            "Niagara",           // Para el VFX de polvo (DustVFX)
        });

        // ── Dependencias privadas ────────────────────────────────────────────
        PrivateDependencyModuleNames.AddRange(new string[]
        {
            "Projects",
            "RenderCore",        // Para MarkRenderStateDirty en HISM
        });

        // ── Ruta de la DLL de Rust ────────────────────────────────────────────
        // Localizada en core_cpp/Binaries/Win64
        string RustBinariesDir = Path.Combine(PluginDirectory, "Binaries", "Win64");

        string ImportLibPath = Path.Combine(RustBinariesDir, "soulforge_physx.lib");
        if (File.Exists(ImportLibPath))
        {
            PublicAdditionalLibraries.Add(ImportLibPath);
        }
        else
        {
            // Si no hay .lib, cargamos en runtime (sin enlace estático)
            System.Console.WriteLine(
                "SoulForge: .lib no encontrado en " + RustBinariesDir +
                " — la DLL se cargará dinámicamente en runtime."
            );
        }

        // ── Auto-Instalador y Carga Diferida ──────────────────────────────────
        // 1. Calculamos la ruta dinámica hacia la carpeta Binaries de tu plugin
        string PluginBinariesDir = Path.Combine(PluginDirectory, "Binaries", "Win64");

        // 2. Definimos el nombre exacto de tu DLL maestra
        string MasterDllName = "soulforge_physx.dll"; 
        string FullDllPath = Path.Combine(PluginBinariesDir, MasterDllName);

        // 3. Instrucciones críticas para Unreal
        // Obligamos a Unreal a incluir este archivo si empaquetas el proyecto
        RuntimeDependencies.Add(FullDllPath);

        // Le decimos al motor que no entre en pánico si la DLL tarda unos milisegundos en responder
        PublicDelayLoadDLLs.Add(MasterDllName);
    }
}
