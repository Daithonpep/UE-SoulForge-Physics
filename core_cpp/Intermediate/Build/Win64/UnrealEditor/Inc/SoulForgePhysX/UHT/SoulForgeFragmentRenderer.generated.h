// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

// IWYU pragma: private, include "SoulForgeFragmentRenderer.h"

#ifdef SOULFORGEPHYSX_SoulForgeFragmentRenderer_generated_h
#error "SoulForgeFragmentRenderer.generated.h already included, missing '#pragma once' in SoulForgeFragmentRenderer.h"
#endif
#define SOULFORGEPHYSX_SoulForgeFragmentRenderer_generated_h

#include "UObject/ObjectMacros.h"
#include "UObject/ScriptMacros.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS

// ********** Begin ScriptStruct FSoulForgeFragmentInstance ****************************************
struct Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_32_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FSoulForgeFragmentInstance;
// ********** End ScriptStruct FSoulForgeFragmentInstance ******************************************

// ********** Begin Class USoulForgeFragmentRenderer ***********************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_RPC_WRAPPERS_NO_PURE_DECLS \
	DECLARE_FUNCTION(execGetActiveFragmentCount); \
	DECLARE_FUNCTION(execCleanup); \
	DECLARE_FUNCTION(execTickUpdate); \
	DECLARE_FUNCTION(execNotifyExplosion); \
	DECLARE_FUNCTION(execUpdateHISMTransforms); \
	DECLARE_FUNCTION(execClearAllFragments); \
	DECLARE_FUNCTION(execExplodeIntoFragments);


struct Z_Construct_UClass_USoulForgeFragmentRenderer_Statics;
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister();

#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_INCLASS_NO_PURE_DECLS \
private: \
	static void StaticRegisterNativesUSoulForgeFragmentRenderer(); \
	friend struct ::Z_Construct_UClass_USoulForgeFragmentRenderer_Statics; \
	static UClass* GetPrivateStaticClass(); \
	friend SOULFORGEPHYSX_API UClass* ::Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister(); \
public: \
	DECLARE_CLASS2(USoulForgeFragmentRenderer, UInstancedStaticMeshComponent, COMPILED_IN_FLAGS(0 | CLASS_Config), CASTCLASS_None, TEXT("/Script/SoulForgePhysX"), Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister) \
	DECLARE_SERIALIZER(USoulForgeFragmentRenderer)


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_ENHANCED_CONSTRUCTORS \
	/** Deleted move- and copy-constructors, should never be used */ \
	USoulForgeFragmentRenderer(USoulForgeFragmentRenderer&&) = delete; \
	USoulForgeFragmentRenderer(const USoulForgeFragmentRenderer&) = delete; \
	DECLARE_VTABLE_PTR_HELPER_CTOR(NO_API, USoulForgeFragmentRenderer); \
	DEFINE_VTABLE_PTR_HELPER_CTOR_CALLER(USoulForgeFragmentRenderer); \
	DEFINE_DEFAULT_CONSTRUCTOR_CALL(USoulForgeFragmentRenderer) \
	NO_API virtual ~USoulForgeFragmentRenderer();


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_53_PROLOG
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_GENERATED_BODY \
PRAGMA_DISABLE_DEPRECATION_WARNINGS \
public: \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_RPC_WRAPPERS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_INCLASS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h_56_ENHANCED_CONSTRUCTORS \
private: \
PRAGMA_ENABLE_DEPRECATION_WARNINGS


class USoulForgeFragmentRenderer;

// ********** End Class USoulForgeFragmentRenderer *************************************************

#undef CURRENT_FILE_ID
#define CURRENT_FILE_ID FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h

PRAGMA_ENABLE_DEPRECATION_WARNINGS
