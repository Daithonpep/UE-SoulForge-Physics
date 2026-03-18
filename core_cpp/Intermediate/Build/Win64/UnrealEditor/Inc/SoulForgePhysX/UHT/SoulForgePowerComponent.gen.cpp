// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgePowerComponent.h"
#include "SoulForgeBridge.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgePowerComponent() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UClass* Z_Construct_UClass_UClass_NoRegister();
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
ENGINE_API UClass* Z_Construct_UClass_UActorComponent();
ENGINE_API UClass* Z_Construct_UClass_UCameraShakeBase_NoRegister();
NIAGARA_API UClass* Z_Construct_UClass_UNiagaraComponent_NoRegister();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgePowerComponent();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgePowerComponent_NoRegister();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EPowerType();
SOULFORGEPHYSX_API UFunction* Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FActivePowerInstance();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Class USoulForgePowerComponent Function ActivateCombo **************************
struct Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics
{
	struct SoulForgePowerComponent_eventActivateCombo_Parms
	{
		TArray<EPowerType> Elements;
		FVector Origin;
		FVector Direction;
		float Intensity;
		float Duration;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Powers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Activa una combinaci\xc3\xb3n de elementos */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Activa una combinaci\xc3\xb3n de elementos" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ActivateCombo constinit property declarations *************************
	static const UECodeGen_Private::FBytePropertyParams NewProp_Elements_Inner_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_Elements_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_Elements;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origin;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direction;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensity;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Duration;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ActivateCombo constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ActivateCombo Property Definitions ************************************
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements_Inner_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements_Inner = { "Elements", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UEnum_SoulForgePhysX_EPowerType, METADATA_PARAMS(0, nullptr) }; // 1062630852
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements = { "Elements", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivateCombo_Parms, Elements), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) }; // 1062630852
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivateCombo_Parms, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivateCombo_Parms, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivateCombo_Parms, Intensity), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Duration = { "Duration", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivateCombo_Parms, Duration), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements_Inner_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Elements,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Intensity,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::NewProp_Duration,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::PropPointers) < 2048);
// ********** End Function ActivateCombo Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "ActivateCombo", 	Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::SoulForgePowerComponent_eventActivateCombo_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::SoulForgePowerComponent_eventActivateCombo_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execActivateCombo)
{
	P_GET_TARRAY(EPowerType,Z_Param_Elements);
	P_GET_STRUCT(FVector,Z_Param_Origin);
	P_GET_STRUCT(FVector,Z_Param_Direction);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Intensity);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Duration);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ActivateCombo(Z_Param_Elements,Z_Param_Origin,Z_Param_Direction,Z_Param_Intensity,Z_Param_Duration);
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function ActivateCombo ****************************

// ********** Begin Class USoulForgePowerComponent Function ActivatePower **************************
struct Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics
{
	struct SoulForgePowerComponent_eventActivatePower_Parms
	{
		EPowerType PowerType;
		FVector Origin;
		FVector Direction;
		float Intensity;
		float Duration;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Powers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Activa un poder elemental en el motor f\xc3\xadsico */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Activa un poder elemental en el motor f\xc3\xadsico" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ActivatePower constinit property declarations *************************
	static const UECodeGen_Private::FBytePropertyParams NewProp_PowerType_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_PowerType;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origin;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direction;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensity;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Duration;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ActivatePower constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ActivatePower Property Definitions ************************************
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_PowerType_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_PowerType = { "PowerType", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivatePower_Parms, PowerType), Z_Construct_UEnum_SoulForgePhysX_EPowerType, METADATA_PARAMS(0, nullptr) }; // 1062630852
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivatePower_Parms, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivatePower_Parms, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivatePower_Parms, Intensity), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Duration = { "Duration", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgePowerComponent_eventActivatePower_Parms, Duration), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_PowerType_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_PowerType,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Intensity,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::NewProp_Duration,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::PropPointers) < 2048);
// ********** End Function ActivatePower Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "ActivatePower", 	Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::SoulForgePowerComponent_eventActivatePower_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::SoulForgePowerComponent_eventActivatePower_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execActivatePower)
{
	P_GET_ENUM(EPowerType,Z_Param_PowerType);
	P_GET_STRUCT(FVector,Z_Param_Origin);
	P_GET_STRUCT(FVector,Z_Param_Direction);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Intensity);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Duration);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ActivatePower(EPowerType(Z_Param_PowerType),Z_Param_Origin,Z_Param_Direction,Z_Param_Intensity,Z_Param_Duration);
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function ActivatePower ****************************

// ********** Begin Class USoulForgePowerComponent Function TestFireTornado ************************
struct Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge | Debug" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function TestFireTornado constinit property declarations ***********************
// ********** End Function TestFireTornado constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "TestFireTornado", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execTestFireTornado)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->TestFireTornado();
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function TestFireTornado **************************

// ********** Begin Class USoulForgePowerComponent Function TestGravityWell ************************
struct Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge | Debug" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function TestGravityWell constinit property declarations ***********************
// ********** End Function TestGravityWell constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "TestGravityWell", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execTestGravityWell)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->TestGravityWell();
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function TestGravityWell **************************

// ********** Begin Class USoulForgePowerComponent Function TestKamehameha *************************
struct Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge | Debug" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// --- DEBUG BUTTONS (Para el Panel de Detalles) ---\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "--- DEBUG BUTTONS (Para el Panel de Detalles) ---" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function TestKamehameha constinit property declarations ************************
// ********** End Function TestKamehameha constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "TestKamehameha", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execTestKamehameha)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->TestKamehameha();
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function TestKamehameha ***************************

// ********** Begin Class USoulForgePowerComponent Function UpdateNiagaraVisuals *******************
struct Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Visuals" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Actualiza los visuales de Niagara con las posiciones de los nodos de Rust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Actualiza los visuales de Niagara con las posiciones de los nodos de Rust" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function UpdateNiagaraVisuals constinit property declarations ******************
// ********** End Function UpdateNiagaraVisuals constinit property declarations ********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgePowerComponent, nullptr, "UpdateNiagaraVisuals", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgePowerComponent::execUpdateNiagaraVisuals)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->UpdateNiagaraVisuals();
	P_NATIVE_END;
}
// ********** End Class USoulForgePowerComponent Function UpdateNiagaraVisuals *********************

// ********** Begin Class USoulForgePowerComponent *************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgePowerComponent;
UClass* USoulForgePowerComponent::GetPrivateStaticClass()
{
	using TClass = USoulForgePowerComponent;
	if (!Z_Registration_Info_UClass_USoulForgePowerComponent.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgePowerComponent"),
			Z_Registration_Info_UClass_USoulForgePowerComponent.InnerSingleton,
			StaticRegisterNativesUSoulForgePowerComponent,
			sizeof(TClass),
			alignof(TClass),
			TClass::StaticClassFlags,
			TClass::StaticClassCastFlags(),
			TClass::StaticConfigName(),
			(UClass::ClassConstructorType)InternalConstructor<TClass>,
			(UClass::ClassVTableHelperCtorCallerType)InternalVTableHelperCtorCaller<TClass>,
			UOBJECT_CPPCLASS_STATICFUNCTIONS_FORCLASS(TClass),
			&TClass::Super::StaticClass,
			&TClass::WithinClass::StaticClass
		);
	}
	return Z_Registration_Info_UClass_USoulForgePowerComponent.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgePowerComponent_NoRegister()
{
	return USoulForgePowerComponent::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgePowerComponent_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "BlueprintSpawnableComponent", "" },
		{ "ClassGroupNames", "SoulForge" },
		{ "IncludePath", "SoulForgePowerComponent.h" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_OnPowerVisualPulse_MetaData[] = {
		{ "Category", "SoulForge | Events" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PowerCameraShake_MetaData[] = {
		{ "Category", "SoulForge | Cinematics" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bUsarRenderizadoRustico_MetaData[] = {
		{ "Category", "SoulForge | Visuals" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ActivePowers_MetaData[] = {
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_NiagaraVFX_MetaData[] = {
		{ "Category", "SoulForge | Visuals" },
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgePowerComponent.h" },
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgePowerComponent constinit property declarations *****************
	static const UECodeGen_Private::FMulticastDelegatePropertyParams NewProp_OnPowerVisualPulse;
	static const UECodeGen_Private::FClassPropertyParams NewProp_PowerCameraShake;
	static void NewProp_bUsarRenderizadoRustico_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bUsarRenderizadoRustico;
	static const UECodeGen_Private::FStructPropertyParams NewProp_ActivePowers_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_ActivePowers;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_NiagaraVFX;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Class USoulForgePowerComponent constinit property declarations *******************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ActivateCombo"), .Pointer = &USoulForgePowerComponent::execActivateCombo },
		{ .NameUTF8 = UTF8TEXT("ActivatePower"), .Pointer = &USoulForgePowerComponent::execActivatePower },
		{ .NameUTF8 = UTF8TEXT("TestFireTornado"), .Pointer = &USoulForgePowerComponent::execTestFireTornado },
		{ .NameUTF8 = UTF8TEXT("TestGravityWell"), .Pointer = &USoulForgePowerComponent::execTestGravityWell },
		{ .NameUTF8 = UTF8TEXT("TestKamehameha"), .Pointer = &USoulForgePowerComponent::execTestKamehameha },
		{ .NameUTF8 = UTF8TEXT("UpdateNiagaraVisuals"), .Pointer = &USoulForgePowerComponent::execUpdateNiagaraVisuals },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgePowerComponent_ActivateCombo, "ActivateCombo" }, // 3172986253
		{ &Z_Construct_UFunction_USoulForgePowerComponent_ActivatePower, "ActivatePower" }, // 1055899874
		{ &Z_Construct_UFunction_USoulForgePowerComponent_TestFireTornado, "TestFireTornado" }, // 1092696911
		{ &Z_Construct_UFunction_USoulForgePowerComponent_TestGravityWell, "TestGravityWell" }, // 3898481640
		{ &Z_Construct_UFunction_USoulForgePowerComponent_TestKamehameha, "TestKamehameha" }, // 3814326304
		{ &Z_Construct_UFunction_USoulForgePowerComponent_UpdateNiagaraVisuals, "UpdateNiagaraVisuals" }, // 3611579768
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgePowerComponent>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgePowerComponent_Statics

// ********** Begin Class USoulForgePowerComponent Property Definitions ****************************
const UECodeGen_Private::FMulticastDelegatePropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_OnPowerVisualPulse = { "OnPowerVisualPulse", nullptr, (EPropertyFlags)0x0010000010080000, UECodeGen_Private::EPropertyGenFlags::InlineMulticastDelegate, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgePowerComponent, OnPowerVisualPulse), Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_OnPowerVisualPulse_MetaData), NewProp_OnPowerVisualPulse_MetaData) }; // 4109095153
const UECodeGen_Private::FClassPropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_PowerCameraShake = { "PowerCameraShake", nullptr, (EPropertyFlags)0x0014000000000005, UECodeGen_Private::EPropertyGenFlags::Class, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgePowerComponent, PowerCameraShake), Z_Construct_UClass_UClass_NoRegister, Z_Construct_UClass_UCameraShakeBase_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PowerCameraShake_MetaData), NewProp_PowerCameraShake_MetaData) };
void Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_bUsarRenderizadoRustico_SetBit(void* Obj)
{
	((USoulForgePowerComponent*)Obj)->bUsarRenderizadoRustico = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_bUsarRenderizadoRustico = { "bUsarRenderizadoRustico", nullptr, (EPropertyFlags)0x0010000000000001, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgePowerComponent), &Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_bUsarRenderizadoRustico_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bUsarRenderizadoRustico_MetaData), NewProp_bUsarRenderizadoRustico_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_ActivePowers_Inner = { "ActivePowers", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FActivePowerInstance, METADATA_PARAMS(0, nullptr) }; // 3403567033
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_ActivePowers = { "ActivePowers", nullptr, (EPropertyFlags)0x0040000000000000, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgePowerComponent, ActivePowers), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ActivePowers_MetaData), NewProp_ActivePowers_MetaData) }; // 3403567033
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_NiagaraVFX = { "NiagaraVFX", nullptr, (EPropertyFlags)0x0040000000080009, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgePowerComponent, NiagaraVFX), Z_Construct_UClass_UNiagaraComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_NiagaraVFX_MetaData), NewProp_NiagaraVFX_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UClass_USoulForgePowerComponent_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_OnPowerVisualPulse,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_PowerCameraShake,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_bUsarRenderizadoRustico,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_ActivePowers_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_ActivePowers,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgePowerComponent_Statics::NewProp_NiagaraVFX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgePowerComponent_Statics::PropPointers) < 2048);
// ********** End Class USoulForgePowerComponent Property Definitions ******************************
UObject* (*const Z_Construct_UClass_USoulForgePowerComponent_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UActorComponent,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgePowerComponent_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgePowerComponent_Statics::ClassParams = {
	&USoulForgePowerComponent::StaticClass,
	"Engine",
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	Z_Construct_UClass_USoulForgePowerComponent_Statics::PropPointers,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgePowerComponent_Statics::PropPointers),
	0,
	0x00B000A4u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgePowerComponent_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgePowerComponent_Statics::Class_MetaDataParams)
};
void USoulForgePowerComponent::StaticRegisterNativesUSoulForgePowerComponent()
{
	UClass* Class = USoulForgePowerComponent::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgePowerComponent_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgePowerComponent()
{
	if (!Z_Registration_Info_UClass_USoulForgePowerComponent.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgePowerComponent.OuterSingleton, Z_Construct_UClass_USoulForgePowerComponent_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgePowerComponent.OuterSingleton;
}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgePowerComponent);
USoulForgePowerComponent::~USoulForgePowerComponent() {}
// ********** End Class USoulForgePowerComponent ***************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h__Script_SoulForgePhysX_Statics
{
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgePowerComponent, USoulForgePowerComponent::StaticClass, TEXT("USoulForgePowerComponent"), &Z_Registration_Info_UClass_USoulForgePowerComponent, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgePowerComponent), 1890568210U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h__Script_SoulForgePhysX_1931344970{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgePowerComponent_h__Script_SoulForgePhysX_Statics::ClassInfo),
	nullptr, 0,
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
