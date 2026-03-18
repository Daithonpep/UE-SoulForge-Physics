// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

// IWYU pragma: private, include "SoulForgeSubsystem.h"

#ifdef SOULFORGEPHYSX_SoulForgeSubsystem_generated_h
#error "SoulForgeSubsystem.generated.h already included, missing '#pragma once' in SoulForgeSubsystem.h"
#endif
#define SOULFORGEPHYSX_SoulForgeSubsystem_generated_h

#include "UObject/ObjectMacros.h"
#include "UObject/ScriptMacros.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS

// ********** Begin Class USoulForgeSubsystem ******************************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_RPC_WRAPPERS_NO_PURE_DECLS \
	DECLARE_FUNCTION(execApplyPerformanceSettings); \
	DECLARE_FUNCTION(execSyncGlobalPower); \
	DECLARE_FUNCTION(execEmergencyReset);


struct Z_Construct_UClass_USoulForgeSubsystem_Statics;
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeSubsystem_NoRegister();

#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_INCLASS_NO_PURE_DECLS \
private: \
	static void StaticRegisterNativesUSoulForgeSubsystem(); \
	friend struct ::Z_Construct_UClass_USoulForgeSubsystem_Statics; \
	static UClass* GetPrivateStaticClass(); \
	friend SOULFORGEPHYSX_API UClass* ::Z_Construct_UClass_USoulForgeSubsystem_NoRegister(); \
public: \
	DECLARE_CLASS2(USoulForgeSubsystem, UWorldSubsystem, COMPILED_IN_FLAGS(0), CASTCLASS_None, TEXT("/Script/SoulForgePhysX"), Z_Construct_UClass_USoulForgeSubsystem_NoRegister) \
	DECLARE_SERIALIZER(USoulForgeSubsystem)


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_ENHANCED_CONSTRUCTORS \
	/** Standard constructor, called after all reflected properties have been initialized */ \
	NO_API USoulForgeSubsystem(); \
	/** Deleted move- and copy-constructors, should never be used */ \
	USoulForgeSubsystem(USoulForgeSubsystem&&) = delete; \
	USoulForgeSubsystem(const USoulForgeSubsystem&) = delete; \
	DECLARE_VTABLE_PTR_HELPER_CTOR(NO_API, USoulForgeSubsystem); \
	DEFINE_VTABLE_PTR_HELPER_CTOR_CALLER(USoulForgeSubsystem); \
	DEFINE_DEFAULT_CONSTRUCTOR_CALL(USoulForgeSubsystem) \
	NO_API virtual ~USoulForgeSubsystem();


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_21_PROLOG
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_GENERATED_BODY \
PRAGMA_DISABLE_DEPRECATION_WARNINGS \
public: \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_RPC_WRAPPERS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_INCLASS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h_24_ENHANCED_CONSTRUCTORS \
private: \
PRAGMA_ENABLE_DEPRECATION_WARNINGS


class USoulForgeSubsystem;

// ********** End Class USoulForgeSubsystem ********************************************************

#undef CURRENT_FILE_ID
#define CURRENT_FILE_ID FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h

// ********** Begin Enum ESoulForgePerformanceMode *************************************************
#define FOREACH_ENUM_ESOULFORGEPERFORMANCEMODE(op) \
	op(ESoulForgePerformanceMode::Eco) \
	op(ESoulForgePerformanceMode::Balanced) \
	op(ESoulForgePerformanceMode::Insight) 

enum class ESoulForgePerformanceMode : uint8;
template<> struct TIsUEnumClass<ESoulForgePerformanceMode> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgePerformanceMode>();
// ********** End Enum ESoulForgePerformanceMode ***************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
