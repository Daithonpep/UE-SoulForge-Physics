// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

// IWYU pragma: private, include "SoulForgeBridge.h"

#ifdef SOULFORGEPHYSX_SoulForgeBridge_generated_h
#error "SoulForgeBridge.generated.h already included, missing '#pragma once' in SoulForgeBridge.h"
#endif
#define SOULFORGEPHYSX_SoulForgeBridge_generated_h

#include "UObject/ObjectMacros.h"
#include "UObject/ScriptMacros.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
enum class EExplosiveType : uint8;
enum class EMaterialType : uint8;
enum class EPhysicsLOD : uint8;
enum class EPowerType : uint8;
struct FActivePowerPayload;
struct FEngineState;
struct FShockwaveData;
struct FSoulForgeFragmentData;

// ********** Begin ScriptStruct FSoulForgeFragmentData ********************************************
struct Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_77_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FSoulForgeFragmentData;
// ********** End ScriptStruct FSoulForgeFragmentData **********************************************

// ********** Begin ScriptStruct FEngineState ******************************************************
struct Z_Construct_UScriptStruct_FEngineState_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_89_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FEngineState_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FEngineState;
// ********** End ScriptStruct FEngineState ********************************************************

// ********** Begin ScriptStruct FShockwaveData ****************************************************
struct Z_Construct_UScriptStruct_FShockwaveData_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_98_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FShockwaveData_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FShockwaveData;
// ********** End ScriptStruct FShockwaveData ******************************************************

// ********** Begin ScriptStruct FActivePowerPayload ***********************************************
struct Z_Construct_UScriptStruct_FActivePowerPayload_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_106_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FActivePowerPayload_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FActivePowerPayload;
// ********** End ScriptStruct FActivePowerPayload *************************************************

// ********** Begin ScriptStruct FRenderData *******************************************************
struct Z_Construct_UScriptStruct_FRenderData_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_118_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FRenderData_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FRenderData;
// ********** End ScriptStruct FRenderData *********************************************************

// ********** Begin ScriptStruct FActivePowerInstance **********************************************
struct Z_Construct_UScriptStruct_FActivePowerInstance_Statics;
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_127_GENERATED_BODY \
	friend struct ::Z_Construct_UScriptStruct_FActivePowerInstance_Statics; \
	SOULFORGEPHYSX_API static class UScriptStruct* StaticStruct();


struct FActivePowerInstance;
// ********** End ScriptStruct FActivePowerInstance ************************************************

// ********** Begin Delegate FOnPowerVisualEffect **************************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_137_DELEGATE \
SOULFORGEPHYSX_API void FOnPowerVisualEffect_DelegateWrapper(const FMulticastScriptDelegate& OnPowerVisualEffect, float BloomIntensity);


// ********** End Delegate FOnPowerVisualEffect ****************************************************

// ********** Begin Class USoulForgeBridge *********************************************************
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_RPC_WRAPPERS_NO_PURE_DECLS \
	DECLARE_FUNCTION(execSpawnNodeOptimized); \
	DECLARE_FUNCTION(execUpdateAllPowers); \
	DECLARE_FUNCTION(execActivarPoder); \
	DECLARE_FUNCTION(execSetGroundZ); \
	DECLARE_FUNCTION(execCalcularOnda); \
	DECLARE_FUNCTION(execGetExplosionPreview); \
	DECLARE_FUNCTION(execSpawnNodesBulk); \
	DECLARE_FUNCTION(execDetonarMilitar); \
	DECLARE_FUNCTION(execSetProxyMilitar); \
	DECLARE_FUNCTION(execApplySettings); \
	DECLARE_FUNCTION(execLimpiarNucleo); \
	DECLARE_FUNCTION(execSetGlobalPower); \
	DECLARE_FUNCTION(execShutdown); \
	DECLARE_FUNCTION(execTickPhysics); \
	DECLARE_FUNCTION(execBorrarProxyEnRust); \
	DECLARE_FUNCTION(execAnclarObjeto); \
	DECLARE_FUNCTION(execRegisterProxy); \
	DECLARE_FUNCTION(execDetonarNativo); \
	DECLARE_FUNCTION(execFiltrarInstancias); \
	DECLARE_FUNCTION(execGetPerformanceState); \
	DECLARE_FUNCTION(execInitialize);


struct Z_Construct_UClass_USoulForgeBridge_Statics;
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeBridge_NoRegister();

#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_INCLASS_NO_PURE_DECLS \
private: \
	static void StaticRegisterNativesUSoulForgeBridge(); \
	friend struct ::Z_Construct_UClass_USoulForgeBridge_Statics; \
	static UClass* GetPrivateStaticClass(); \
	friend SOULFORGEPHYSX_API UClass* ::Z_Construct_UClass_USoulForgeBridge_NoRegister(); \
public: \
	DECLARE_CLASS2(USoulForgeBridge, UObject, COMPILED_IN_FLAGS(0), CASTCLASS_None, TEXT("/Script/SoulForgePhysX"), Z_Construct_UClass_USoulForgeBridge_NoRegister) \
	DECLARE_SERIALIZER(USoulForgeBridge)


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_ENHANCED_CONSTRUCTORS \
	/** Standard constructor, called after all reflected properties have been initialized */ \
	NO_API USoulForgeBridge(const FObjectInitializer& ObjectInitializer = FObjectInitializer::Get()); \
	/** Deleted move- and copy-constructors, should never be used */ \
	USoulForgeBridge(USoulForgeBridge&&) = delete; \
	USoulForgeBridge(const USoulForgeBridge&) = delete; \
	DECLARE_VTABLE_PTR_HELPER_CTOR(NO_API, USoulForgeBridge); \
	DEFINE_VTABLE_PTR_HELPER_CTOR_CALLER(USoulForgeBridge); \
	DEFINE_DEFAULT_OBJECT_INITIALIZER_CONSTRUCTOR_CALL(USoulForgeBridge) \
	NO_API virtual ~USoulForgeBridge();


#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_197_PROLOG
#define FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_GENERATED_BODY \
PRAGMA_DISABLE_DEPRECATION_WARNINGS \
public: \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_RPC_WRAPPERS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_INCLASS_NO_PURE_DECLS \
	FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h_200_ENHANCED_CONSTRUCTORS \
private: \
PRAGMA_ENABLE_DEPRECATION_WARNINGS


class USoulForgeBridge;

// ********** End Class USoulForgeBridge ***********************************************************

#undef CURRENT_FILE_ID
#define CURRENT_FILE_ID FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h

// ********** Begin Enum ESoulForgeDamagePreset ****************************************************
#define FOREACH_ENUM_ESOULFORGEDAMAGEPRESET(op) \
	op(ESoulForgeDamagePreset::Explosion) \
	op(ESoulForgeDamagePreset::Collapse) \
	op(ESoulForgeDamagePreset::Impact) \
	op(ESoulForgeDamagePreset::ShapedCharge) 

enum class ESoulForgeDamagePreset : uint8;
template<> struct TIsUEnumClass<ESoulForgeDamagePreset> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgeDamagePreset>();
// ********** End Enum ESoulForgeDamagePreset ******************************************************

// ********** Begin Enum EPhysicsLOD ***************************************************************
#define FOREACH_ENUM_EPHYSICSLOD(op) \
	op(EPhysicsLOD::Primary) \
	op(EPhysicsLOD::Secondary) \
	op(EPhysicsLOD::Tertiary) 

enum class EPhysicsLOD : uint8;
template<> struct TIsUEnumClass<EPhysicsLOD> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EPhysicsLOD>();
// ********** End Enum EPhysicsLOD *****************************************************************

// ********** Begin Enum EMaterialType *************************************************************
#define FOREACH_ENUM_EMATERIALTYPE(op) \
	op(EMaterialType::Steel4340) \
	op(EMaterialType::Aluminum6061) \
	op(EMaterialType::Concrete35MPa) \
	op(EMaterialType::WoodOak) \
	op(EMaterialType::GlassTempered) 

enum class EMaterialType : uint8;
template<> struct TIsUEnumClass<EMaterialType> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EMaterialType>();
// ********** End Enum EMaterialType ***************************************************************

// ********** Begin Enum EExplosiveType ************************************************************
#define FOREACH_ENUM_EEXPLOSIVETYPE(op) \
	op(EExplosiveType::TNT) \
	op(EExplosiveType::AgujeroNegro) \
	op(EExplosiveType::RDX) \
	op(EExplosiveType::PETN) \
	op(EExplosiveType::Matrix) \
	op(EExplosiveType::Collapse) 

enum class EExplosiveType : uint8;
template<> struct TIsUEnumClass<EExplosiveType> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EExplosiveType>();
// ********** End Enum EExplosiveType **************************************************************

// ********** Begin Enum EPowerType ****************************************************************
#define FOREACH_ENUM_EPOWERTYPE(op) \
	op(EPowerType::EnergyBeam) \
	op(EPowerType::WaterBending) \
	op(EPowerType::FireBending) \
	op(EPowerType::EarthBending) \
	op(EPowerType::AirBending) \
	op(EPowerType::Lightning) \
	op(EPowerType::GravityZero) \
	op(EPowerType::GravityBlackHole) \
	op(EPowerType::IceControl) \
	op(EPowerType::FireStorm) \
	op(EPowerType::Electrocution) \
	op(EPowerType::LavaBurst) \
	op(EPowerType::Blizzard) \
	op(EPowerType::Supernova) 

enum class EPowerType : uint8;
template<> struct TIsUEnumClass<EPowerType> { enum { Value = true }; };
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EPowerType>();
// ********** End Enum EPowerType ******************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
