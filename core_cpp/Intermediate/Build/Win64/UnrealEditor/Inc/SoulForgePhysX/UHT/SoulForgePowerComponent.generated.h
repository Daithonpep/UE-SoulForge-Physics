// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

// IWYU pragma: private, include "SoulForgePowerComponent.h"

#ifdef SOULFORGEPHYSX_SoulForgePowerComponent_generated_h
#error "SoulForgePowerComponent.generated.h already included, missing '#pragma once' in SoulForgePowerComponent.h"
#endif
#define SOULFORGEPHYSX_SoulForgePowerComponent_generated_h

#include "UObject/ObjectMacros.h"
#include "UObject/ScriptMacros.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
enum class EPowerType : uint8;

// ********** Begin Class USoulForgePowerComponent *************************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_RPC_WRAPPERS_NO_PURE_DECLS \
	DECLARE_FUNCTION(execTestGravityWell); \
	DECLARE_FUNCTION(execTestFireTornado); \
	DECLARE_FUNCTION(execTestKamehameha); \
	DECLARE_FUNCTION(execUpdateNiagaraVisuals); \
	DECLARE_FUNCTION(execActivateCombo); \
	DECLARE_FUNCTION(execActivatePower);


struct Z_Construct_UClass_USoulForgePowerComponent_Statics;
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgePowerComponent_NoRegister();

#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_INCLASS_NO_PURE_DECLS \
private: \
	static void StaticRegisterNativesUSoulForgePowerComponent(); \
	friend struct ::Z_Construct_UClass_USoulForgePowerComponent_Statics; \
	static UClass* GetPrivateStaticClass(); \
	friend SOULFORGEPHYSX_API UClass* ::Z_Construct_UClass_USoulForgePowerComponent_NoRegister(); \
public: \
	DECLARE_CLASS2(USoulForgePowerComponent, UActorComponent, COMPILED_IN_FLAGS(0 | CLASS_Config), CASTCLASS_None, TEXT("/Script/SoulForgePhysX"), Z_Construct_UClass_USoulForgePowerComponent_NoRegister) \
	DECLARE_SERIALIZER(USoulForgePowerComponent)


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_ENHANCED_CONSTRUCTORS \
	/** Deleted move- and copy-constructors, should never be used */ \
	USoulForgePowerComponent(USoulForgePowerComponent&&) = delete; \
	USoulForgePowerComponent(const USoulForgePowerComponent&) = delete; \
	DECLARE_VTABLE_PTR_HELPER_CTOR(NO_API, USoulForgePowerComponent); \
	DEFINE_VTABLE_PTR_HELPER_CTOR_CALLER(USoulForgePowerComponent); \
	DEFINE_DEFAULT_CONSTRUCTOR_CALL(USoulForgePowerComponent) \
	NO_API virtual ~USoulForgePowerComponent();


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_10_PROLOG
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_GENERATED_BODY \
PRAGMA_DISABLE_DEPRECATION_WARNINGS \
public: \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_RPC_WRAPPERS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_INCLASS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h_13_ENHANCED_CONSTRUCTORS \
private: \
PRAGMA_ENABLE_DEPRECATION_WARNINGS


class USoulForgePowerComponent;

// ********** End Class USoulForgePowerComponent ***************************************************

#undef CURRENT_FILE_ID
#define CURRENT_FILE_ID FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h

PRAGMA_ENABLE_DEPRECATION_WARNINGS
