// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

// IWYU pragma: private, include "SoulForgeDestructible.h"

#ifdef SOULFORGEPHYSX_SoulForgeDestructible_generated_h
#error "SoulForgeDestructible.generated.h already included, missing '#pragma once' in SoulForgeDestructible.h"
#endif
#define SOULFORGEPHYSX_SoulForgeDestructible_generated_h

#include "UObject/ObjectMacros.h"
#include "UObject/ScriptMacros.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS

// ********** Begin ScriptStruct FSoulForgeNode ****************************************************
struct Z_Construct_UScriptStruct_FSoulForgeNode_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_26_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FSoulForgeNode_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FSoulForgeNode;
// ********** End ScriptStruct FSoulForgeNode ******************************************************

// ********** Begin Class USoulForgeDestructible ***************************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_RPC_WRAPPERS_NO_PURE_DECLS \
	DECLARE_FUNCTION(execPreparaRenderizadoFragmentos); \
	DECLARE_FUNCTION(execGetHealth); \
	DECLARE_FUNCTION(execRecibirImpacto); \
	DECLARE_FUNCTION(execApplyDamage); \
	DECLARE_FUNCTION(execActivarInsight); \
	DECLARE_FUNCTION(execForzarEncendidoRust); \
	DECLARE_FUNCTION(execPreviewExplosion); \
	DECLARE_FUNCTION(execResetInternalState); \
	DECLARE_FUNCTION(execAutoLinkToRust); \
	DECLARE_FUNCTION(execActivateKamehameha); \
	DECLARE_FUNCTION(execSpawnNodeGrid); \
	DECLARE_FUNCTION(execTriggerDestruction); \
	DECLARE_FUNCTION(execDeactivateOriginal);


struct Z_Construct_UClass_USoulForgeDestructible_Statics;
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeDestructible_NoRegister();

#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_INCLASS_NO_PURE_DECLS \
private: \
	static void StaticRegisterNativesUSoulForgeDestructible(); \
	friend struct ::Z_Construct_UClass_USoulForgeDestructible_Statics; \
	static UClass* GetPrivateStaticClass(); \
	friend SOULFORGEPHYSX_API UClass* ::Z_Construct_UClass_USoulForgeDestructible_NoRegister(); \
public: \
	DECLARE_CLASS2(USoulForgeDestructible, UActorComponent, COMPILED_IN_FLAGS(0 | CLASS_Config), CASTCLASS_None, TEXT("/Script/SoulForgePhysX"), Z_Construct_UClass_USoulForgeDestructible_NoRegister) \
	DECLARE_SERIALIZER(USoulForgeDestructible)


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_ENHANCED_CONSTRUCTORS \
	/** Deleted move- and copy-constructors, should never be used */ \
	USoulForgeDestructible(USoulForgeDestructible&&) = delete; \
	USoulForgeDestructible(const USoulForgeDestructible&) = delete; \
	DECLARE_VTABLE_PTR_HELPER_CTOR(NO_API, USoulForgeDestructible); \
	DEFINE_VTABLE_PTR_HELPER_CTOR_CALLER(USoulForgeDestructible); \
	DEFINE_DEFAULT_CONSTRUCTOR_CALL(USoulForgeDestructible) \
	NO_API virtual ~USoulForgeDestructible();


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_37_PROLOG
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_GENERATED_BODY \
PRAGMA_DISABLE_DEPRECATION_WARNINGS \
public: \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_RPC_WRAPPERS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_INCLASS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h_40_ENHANCED_CONSTRUCTORS \
private: \
PRAGMA_ENABLE_DEPRECATION_WARNINGS


class USoulForgeDestructible;

// ********** End Class USoulForgeDestructible *****************************************************

#undef CURRENT_FILE_ID
#define CURRENT_FILE_ID FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h

// ********** Begin Enum ESoulForgePattern *********************************************************
#define FOREACH_ENUM_ESOULFORGEPATTERN(op) \
	op(ESoulForgePattern::Radial) \
	op(ESoulForgePattern::Directional) \
	op(ESoulForgePattern::Implosion) 

enum class ESoulForgePattern : uint8;
template<> struct TIsUEnumClass<ESoulForgePattern> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgePattern>();
// ********** End Enum ESoulForgePattern ***********************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
