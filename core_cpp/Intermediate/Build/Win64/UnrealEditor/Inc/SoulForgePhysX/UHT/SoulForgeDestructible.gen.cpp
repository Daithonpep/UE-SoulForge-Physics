// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeDestructible.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeDestructible() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FGuid();
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
ENGINE_API UClass* Z_Construct_UClass_UActorComponent();
ENGINE_API UClass* Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister();
ENGINE_API UClass* Z_Construct_UClass_UMaterialInterface_NoRegister();
ENGINE_API UClass* Z_Construct_UClass_UStaticMesh_NoRegister();
NIAGARA_API UClass* Z_Construct_UClass_UNiagaraComponent_NoRegister();
NIAGARA_API UClass* Z_Construct_UClass_UNiagaraSystem_NoRegister();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeDestructible();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeDestructible_NoRegister();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EExplosiveType();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EMaterialType();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeNode();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Enum ESoulForgePattern *********************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_ESoulForgePattern;
static UEnum* ESoulForgePattern_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_ESoulForgePattern.OuterSingleton)
	{
		Z_Registration_Info_UEnum_ESoulForgePattern.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ESoulForgePattern"));
	}
	return Z_Registration_Info_UEnum_ESoulForgePattern.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgePattern>()
{
	return ESoulForgePattern_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Las formas en las que Rust romper\xc3\xa1 el objeto\n" },
#endif
		{ "Directional.DisplayName", "Perforaci\xc3\xb3n (Bala)" },
		{ "Directional.Name", "ESoulForgePattern::Directional" },
		{ "Implosion.DisplayName", "Implosi\xc3\xb3n (Colapso Interno)" },
		{ "Implosion.Name", "ESoulForgePattern::Implosion" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
		{ "Radial.DisplayName", "Explosi\xc3\xb3n Radial (Esfera)" },
		{ "Radial.Name", "ESoulForgePattern::Radial" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Las formas en las que Rust romper\xc3\xa1 el objeto" },
#endif
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "ESoulForgePattern::Radial", (int64)ESoulForgePattern::Radial },
		{ "ESoulForgePattern::Directional", (int64)ESoulForgePattern::Directional },
		{ "ESoulForgePattern::Implosion", (int64)ESoulForgePattern::Implosion },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"ESoulForgePattern",
	"ESoulForgePattern",
	Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern()
{
	if (!Z_Registration_Info_UEnum_ESoulForgePattern.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_ESoulForgePattern.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_ESoulForgePattern.InnerSingleton;
}
// ********** End Enum ESoulForgePattern ***********************************************************

// ********** Begin ScriptStruct FSoulForgeNode ****************************************************
struct Z_Construct_UScriptStruct_FSoulForgeNode_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FSoulForgeNode); }
	static inline consteval int16 GetStructAlignment() { return alignof(FSoulForgeNode); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Esta es la estructura s\xc3\xbaper optimizada (Solo n\xc3\xbameros puros)\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Esta es la estructura s\xc3\xbaper optimizada (Solo n\xc3\xbameros puros)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FSoulForgeNode constinit property declarations ********************
// ********** End ScriptStruct FSoulForgeNode constinit property declarations **********************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FSoulForgeNode>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FSoulForgeNode_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FSoulForgeNode;
class UScriptStruct* FSoulForgeNode::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeNode.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FSoulForgeNode.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FSoulForgeNode, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("SoulForgeNode"));
	}
	return Z_Registration_Info_UScriptStruct_FSoulForgeNode.OuterSingleton;
	}
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FSoulForgeNode_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"SoulForgeNode",
	nullptr,
	0,
	sizeof(FSoulForgeNode),
	alignof(FSoulForgeNode),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeNode_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FSoulForgeNode_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeNode()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeNode.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FSoulForgeNode.InnerSingleton, Z_Construct_UScriptStruct_FSoulForgeNode_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FSoulForgeNode.InnerSingleton);
}
// ********** End ScriptStruct FSoulForgeNode ******************************************************

// ********** Begin Class USoulForgeDestructible Function ActivarInsight ***************************
struct Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge|Rendimiento" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// El bot\xc3\xb3n maestro para encender el monitor de Insight\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "El bot\xc3\xb3n maestro para encender el monitor de Insight" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ActivarInsight constinit property declarations ************************
// ********** End Function ActivarInsight constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "ActivarInsight", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x00020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execActivarInsight)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ActivarInsight();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function ActivarInsight *****************************

// ********** Begin Class USoulForgeDestructible Function ActivateKamehameha ***********************
struct Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics
{
	struct SoulForgeDestructible_eventActivateKamehameha_Parms
	{
		FVector Origin;
		FVector Direction;
		float Intensity;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function ActivateKamehameha constinit property declarations ********************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origin;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direction;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensity;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ActivateKamehameha constinit property declarations **********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ActivateKamehameha Property Definitions *******************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventActivateKamehameha_Parms, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventActivateKamehameha_Parms, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventActivateKamehameha_Parms, Intensity), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::NewProp_Intensity,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::PropPointers) < 2048);
// ********** End Function ActivateKamehameha Property Definitions *********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "ActivateKamehameha", 	Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::SoulForgeDestructible_eventActivateKamehameha_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::SoulForgeDestructible_eventActivateKamehameha_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execActivateKamehameha)
{
	P_GET_STRUCT(FVector,Z_Param_Origin);
	P_GET_STRUCT(FVector,Z_Param_Direction);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Intensity);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ActivateKamehameha(Z_Param_Origin,Z_Param_Direction,Z_Param_Intensity);
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function ActivateKamehameha *************************

// ********** Begin Class USoulForgeDestructible Function ApplyDamage ******************************
struct Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics
{
	struct SoulForgeDestructible_eventApplyDamage_Parms
	{
		float DamageAmount;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Destruction" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Aplica da\xc3\xb1o al objeto. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Aplica da\xc3\xb1o al objeto." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ApplyDamage constinit property declarations ***************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DamageAmount;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ApplyDamage constinit property declarations *****************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ApplyDamage Property Definitions **************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::NewProp_DamageAmount = { "DamageAmount", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventApplyDamage_Parms, DamageAmount), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::NewProp_DamageAmount,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::PropPointers) < 2048);
// ********** End Function ApplyDamage Property Definitions ****************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "ApplyDamage", 	Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::SoulForgeDestructible_eventApplyDamage_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::SoulForgeDestructible_eventApplyDamage_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execApplyDamage)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_DamageAmount);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ApplyDamage(Z_Param_DamageAmount);
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function ApplyDamage ********************************

// ********** Begin Class USoulForgeDestructible Function AutoLinkToRust ***************************
struct Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n\x09 * Re-vincula el objeto con el n\xc3\xba""cleo de Rust si el ProxyId se ha perdido o es inv\xc3\xa1lido.\n\x09 */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Re-vincula el objeto con el n\xc3\xba""cleo de Rust si el ProxyId se ha perdido o es inv\xc3\xa1lido." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function AutoLinkToRust constinit property declarations ************************
// ********** End Function AutoLinkToRust constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "AutoLinkToRust", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execAutoLinkToRust)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->AutoLinkToRust();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function AutoLinkToRust *****************************

// ********** Begin Class USoulForgeDestructible Function DeactivateOriginal ***********************
struct Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Actions" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Funci\xc3\xb3n para que el objeto se oculte y desactive colisiones solo */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Funci\xc3\xb3n para que el objeto se oculte y desactive colisiones solo" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function DeactivateOriginal constinit property declarations ********************
// ********** End Function DeactivateOriginal constinit property declarations **********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "DeactivateOriginal", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execDeactivateOriginal)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->DeactivateOriginal();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function DeactivateOriginal *************************

// ********** Begin Class USoulForgeDestructible Function ForzarEncendidoRust **********************
struct Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge|Debug" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Bot\xc3\xb3n m\xc3\xa1gico que aparecer\xc3\xa1 en el editor de Unreal */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Bot\xc3\xb3n m\xc3\xa1gico que aparecer\xc3\xa1 en el editor de Unreal" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ForzarEncendidoRust constinit property declarations *******************
// ********** End Function ForzarEncendidoRust constinit property declarations *********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "ForzarEncendidoRust", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x00020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execForzarEncendidoRust)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ForzarEncendidoRust();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function ForzarEncendidoRust ************************

// ********** Begin Class USoulForgeDestructible Function GetHealth ********************************
struct Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics
{
	struct SoulForgeDestructible_eventGetHealth_Parms
	{
		float ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|State" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Salud actual del objeto [0.0 = destruido, 1.0 = intacto]. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Salud actual del objeto [0.0 = destruido, 1.0 = intacto]." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function GetHealth constinit property declarations *****************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function GetHealth constinit property declarations *******************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function GetHealth Property Definitions ****************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventGetHealth_Parms, ReturnValue), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::PropPointers) < 2048);
// ********** End Function GetHealth Property Definitions ******************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "GetHealth", 	Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::SoulForgeDestructible_eventGetHealth_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x54020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::SoulForgeDestructible_eventGetHealth_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeDestructible_GetHealth()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_GetHealth_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execGetHealth)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	*(float*)Z_Param__Result=P_THIS->GetHealth();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function GetHealth **********************************

// ********** Begin Class USoulForgeDestructible Function PreparaRenderizadoFragmentos *************
struct Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Visual" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Captura la textura del objeto original y la inyecta en los fragmentos */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Captura la textura del objeto original y la inyecta en los fragmentos" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function PreparaRenderizadoFragmentos constinit property declarations **********
// ********** End Function PreparaRenderizadoFragmentos constinit property declarations ************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "PreparaRenderizadoFragmentos", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execPreparaRenderizadoFragmentos)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->PreparaRenderizadoFragmentos();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function PreparaRenderizadoFragmentos ***************

// ********** Begin Class USoulForgeDestructible Function PreviewExplosion *************************
struct Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge|Laboratorio" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\xa7\xaa LABORATORIO: Pre-visualizar patr\xc3\xb3n de explosi\xc3\xb3n sin simular nada */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\xa7\xaa LABORATORIO: Pre-visualizar patr\xc3\xb3n de explosi\xc3\xb3n sin simular nada" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function PreviewExplosion constinit property declarations **********************
// ********** End Function PreviewExplosion constinit property declarations ************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "PreviewExplosion", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x00020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execPreviewExplosion)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->PreviewExplosion();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function PreviewExplosion ***************************

// ********** Begin Class USoulForgeDestructible Function RecibirImpacto ***************************
struct Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics
{
	struct SoulForgeDestructible_eventRecibirImpacto_Parms
	{
		FVector PuntoDeGolpe;
		FVector Direccion;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Acci\xc3\xb3n" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** LA \xc3\x9aNICA FUNCI\xc3\x93N QUE USAR\xc3\x81S EN BLUEPRINTS (Panel Maestro) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "LA \xc3\x9aNICA FUNCI\xc3\x93N QUE USAR\xc3\x81S EN BLUEPRINTS (Panel Maestro)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function RecibirImpacto constinit property declarations ************************
	static const UECodeGen_Private::FStructPropertyParams NewProp_PuntoDeGolpe;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direccion;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function RecibirImpacto constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function RecibirImpacto Property Definitions ***********************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::NewProp_PuntoDeGolpe = { "PuntoDeGolpe", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventRecibirImpacto_Parms, PuntoDeGolpe), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::NewProp_Direccion = { "Direccion", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventRecibirImpacto_Parms, Direccion), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::NewProp_PuntoDeGolpe,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::NewProp_Direccion,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::PropPointers) < 2048);
// ********** End Function RecibirImpacto Property Definitions *************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "RecibirImpacto", 	Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::SoulForgeDestructible_eventRecibirImpacto_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::SoulForgeDestructible_eventRecibirImpacto_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execRecibirImpacto)
{
	P_GET_STRUCT(FVector,Z_Param_PuntoDeGolpe);
	P_GET_STRUCT(FVector,Z_Param_Direccion);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->RecibirImpacto(Z_Param_PuntoDeGolpe,Z_Param_Direccion);
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function RecibirImpacto *****************************

// ********** Begin Class USoulForgeDestructible Function ResetInternalState ***********************
struct Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "CallInEditor", "true" },
		{ "Category", "SoulForge|Debug" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Bot\xc3\xb3n en el Editor para resetear el estado interno. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Bot\xc3\xb3n en el Editor para resetear el estado interno." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ResetInternalState constinit property declarations ********************
// ********** End Function ResetInternalState constinit property declarations **********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "ResetInternalState", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x00020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execResetInternalState)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ResetInternalState();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function ResetInternalState *************************

// ********** Begin Class USoulForgeDestructible Function SpawnNodeGrid ****************************
struct Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics
{
	struct SoulForgeDestructible_eventSpawnNodeGrid_Parms
	{
		FVector Center;
		int32 Width;
		int32 Height;
		int32 Depth;
		float Spacing;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\x9a\x80 NUEVA API OPTIMIZADA (Llamado masivo de nodos) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x9a\x80 NUEVA API OPTIMIZADA (Llamado masivo de nodos)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SpawnNodeGrid constinit property declarations *************************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Center;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Width;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Height;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Depth;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Spacing;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SpawnNodeGrid constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SpawnNodeGrid Property Definitions ************************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Center = { "Center", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventSpawnNodeGrid_Parms, Center), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Width = { "Width", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventSpawnNodeGrid_Parms, Width), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Height = { "Height", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventSpawnNodeGrid_Parms, Height), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Depth = { "Depth", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventSpawnNodeGrid_Parms, Depth), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Spacing = { "Spacing", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeDestructible_eventSpawnNodeGrid_Parms, Spacing), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Center,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Width,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Height,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Depth,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::NewProp_Spacing,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::PropPointers) < 2048);
// ********** End Function SpawnNodeGrid Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "SpawnNodeGrid", 	Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::SoulForgeDestructible_eventSpawnNodeGrid_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::SoulForgeDestructible_eventSpawnNodeGrid_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execSpawnNodeGrid)
{
	P_GET_STRUCT(FVector,Z_Param_Center);
	P_GET_PROPERTY(FIntProperty,Z_Param_Width);
	P_GET_PROPERTY(FIntProperty,Z_Param_Height);
	P_GET_PROPERTY(FIntProperty,Z_Param_Depth);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Spacing);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->SpawnNodeGrid(Z_Param_Center,Z_Param_Width,Z_Param_Height,Z_Param_Depth,Z_Param_Spacing);
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function SpawnNodeGrid ******************************

// ********** Begin Class USoulForgeDestructible Function TriggerDestruction ***********************
struct Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n\x09 * Destruye el objeto: oculta el mesh original y activa la simulaci\xc3\xb3n.\n\x09 */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Destruye el objeto: oculta el mesh original y activa la simulaci\xc3\xb3n." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function TriggerDestruction constinit property declarations ********************
// ********** End Function TriggerDestruction constinit property declarations **********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeDestructible, nullptr, "TriggerDestruction", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeDestructible::execTriggerDestruction)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->TriggerDestruction();
	P_NATIVE_END;
}
// ********** End Class USoulForgeDestructible Function TriggerDestruction *************************

// ********** Begin Class USoulForgeDestructible ***************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeDestructible;
UClass* USoulForgeDestructible::GetPrivateStaticClass()
{
	using TClass = USoulForgeDestructible;
	if (!Z_Registration_Info_UClass_USoulForgeDestructible.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeDestructible"),
			Z_Registration_Info_UClass_USoulForgeDestructible.InnerSingleton,
			StaticRegisterNativesUSoulForgeDestructible,
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
	return Z_Registration_Info_UClass_USoulForgeDestructible.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeDestructible_NoRegister()
{
	return USoulForgeDestructible::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeDestructible_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "BlueprintSpawnableComponent", "" },
		{ "ClassGroupNames", "SoulForge" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Componente Destruible \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\n" },
#endif
		{ "IncludePath", "SoulForgeDestructible.h" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Componente Destruible \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ProxyName_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// \xe2\x94\x80\xe2\x94\x80 Configuraci\xc3\xb3n (Mejorada para Panel de Detalles) \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xe2\x94\x80\xe2\x94\x80 Configuraci\xc3\xb3n (Mejorada para Panel de Detalles) \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ProxyId_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PersistentDNI_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** DNI persistente calculado por el componente */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "DNI persistente calculado por el componente" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PersistentGUID_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** ADN \xc3\xbanico del objeto (Persistente entre sesiones) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "ADN \xc3\xbanico del objeto (Persistente entre sesiones)" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_MaterialType_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PhysicsLOD_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ExplosiveType_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_HalfExtent_MetaData[] = {
		{ "Category", "SoulForge|Geometry" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n\x09 * Semi-extensiones de la caja AABB del objeto (cm).\n\x09 * X=ancho/2, Y=profundidad/2, Z=altura/2.\n\x09 */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Semi-extensiones de la caja AABB del objeto (cm).\nX=ancho/2, Y=profundidad/2, Z=altura/2." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_DamagePreset_MetaData[] = {
		{ "Category", "SoulForge|Destruction" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Preset de da\xc3\xb1o aplicado cuando se destruya este objeto. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Preset de da\xc3\xb1o aplicado cuando se destruya este objeto." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_DustVFX_MetaData[] = {
		{ "Category", "SoulForge|VFX" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Sistema Niagara para el polvo y part\xc3\xad""culas finas (Dust category). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Sistema Niagara para el polvo y part\xc3\xad""culas finas (Dust category)." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FragmentRenderer_MetaData[] = {
		{ "Category", "SoulForge|Renderer" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n\x09 * Referencia al renderer de fragmentos del mismo Actor.\n\x09 * Se busca autom\xc3\xa1ticamente en BeginPlay.\n\x09 */" },
#endif
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Referencia al renderer de fragmentos del mismo Actor.\nSe busca autom\xc3\xa1ticamente en BeginPlay." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_StabilityThreshold_MetaData[] = {
		{ "Category", "SoulForge|Config" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Sensibilidad de la simulaci\xc3\xb3n de fragmentos (0.0 = ca\xc3\xb3tico, 1.0 = r\xc3\xadgido). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Sensibilidad de la simulaci\xc3\xb3n de fragmentos (0.0 = ca\xc3\xb3tico, 1.0 = r\xc3\xadgido)." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FuerzaDeImpacto_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// --- CONFIGURACI\xc3\x93N DE IMPACTO (Panel Maestro) ---\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "--- CONFIGURACI\xc3\x93N DE IMPACTO (Panel Maestro) ---" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_RadioDeImpacto_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// 500 Millones por defecto\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "500 Millones por defecto" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FormaDeExplosion_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_CantidadDeNodos_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Cantidad de fragmentos. Poner en 0 para que Rust calcule el n\xc3\xbamero real seg\xc3\xban la f\xc3\xadsica de Grady-Kipp." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_CantidadDeNodosHecatombe_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Nodos que genera el Kamehameha para el estr\xc3\xa9s test." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bUsarRenderizadoRustico_MetaData[] = {
		{ "Category", "SoulForge | Visuals" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_SharedNiagaraVFX_MetaData[] = {
		{ "Category", "SoulForge | Visuals" },
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bActivarMiraLaser_MetaData[] = {
		{ "Category", "SoulForge|Asistencia" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bIsEngineReady_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Indica si el componente tiene conexi\xc3\xb3n activa con el n\xc3\xba""cleo Rust. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Indica si el componente tiene conexi\xc3\xb3n activa con el n\xc3\xba""cleo Rust." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bDestroyed_MetaData[] = {
		{ "Category", "SoulForge|Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PreviewDuration_MetaData[] = {
		{ "Category", "SoulForge|Laboratorio" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_NucleosCPU_MetaData[] = {
		{ "Category", "SoulForge|Rendimiento" },
		{ "ClampMax", "32" },
		{ "ClampMin", "1" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// --- PANEL DE RENDIMIENTO INSIGHT ---\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "--- PANEL DE RENDIMIENTO INSIGHT ---" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bUsarAceleracionGPU_MetaData[] = {
		{ "Category", "SoulForge|Rendimiento" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// T\xc3\xba decides cu\xc3\xa1ntos n\xc3\xba""cleos dedicarle\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "T\xc3\xba decides cu\xc3\xa1ntos n\xc3\xba""cleos dedicarle" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_InstancedRenderer_MetaData[] = {
#if !UE_BUILD_SHIPPING
		{ "Comment", "// El componente que dibujar\xc3\xa1 los pedazos a la velocidad de la luz\n" },
#endif
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "El componente que dibujar\xc3\xa1 los pedazos a la velocidad de la luz" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_MaxNodos_MetaData[] = {
		{ "Category", "SoulForge|Memoria" },
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_MallaDelNodo_MetaData[] = {
		{ "Category", "SoulForge|Visual" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Tu malla gen\xc3\xa9rica (El cubito o roca que usaremos como \"Nodo\")\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Tu malla gen\xc3\xa9rica (El cubito o roca que usaremos como \"Nodo\")" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_MasterShardMaterial_MetaData[] = {
		{ "Category", "SoulForge|Visual" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// El material maestro del fragmento (donde inyectaremos la textura)\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeDestructible.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "El material maestro del fragmento (donde inyectaremos la textura)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeDestructible constinit property declarations *******************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyName;
	static const UECodeGen_Private::FIntPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FIntPropertyParams NewProp_PersistentDNI;
	static const UECodeGen_Private::FStructPropertyParams NewProp_PersistentGUID;
	static const UECodeGen_Private::FBytePropertyParams NewProp_MaterialType_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_MaterialType;
	static const UECodeGen_Private::FBytePropertyParams NewProp_PhysicsLOD_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_PhysicsLOD;
	static const UECodeGen_Private::FBytePropertyParams NewProp_ExplosiveType_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_ExplosiveType;
	static const UECodeGen_Private::FStructPropertyParams NewProp_HalfExtent;
	static const UECodeGen_Private::FBytePropertyParams NewProp_DamagePreset_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_DamagePreset;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_DustVFX;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_FragmentRenderer;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_StabilityThreshold;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_FuerzaDeImpacto;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_RadioDeImpacto;
	static const UECodeGen_Private::FBytePropertyParams NewProp_FormaDeExplosion_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_FormaDeExplosion;
	static const UECodeGen_Private::FIntPropertyParams NewProp_CantidadDeNodos;
	static const UECodeGen_Private::FIntPropertyParams NewProp_CantidadDeNodosHecatombe;
	static void NewProp_bUsarRenderizadoRustico_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bUsarRenderizadoRustico;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_SharedNiagaraVFX;
	static void NewProp_bActivarMiraLaser_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bActivarMiraLaser;
	static void NewProp_bIsEngineReady_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bIsEngineReady;
	static void NewProp_bDestroyed_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bDestroyed;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_PreviewDuration;
	static const UECodeGen_Private::FIntPropertyParams NewProp_NucleosCPU;
	static void NewProp_bUsarAceleracionGPU_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bUsarAceleracionGPU;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_InstancedRenderer;
	static const UECodeGen_Private::FIntPropertyParams NewProp_MaxNodos;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_MallaDelNodo;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_MasterShardMaterial;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Class USoulForgeDestructible constinit property declarations *********************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ActivarInsight"), .Pointer = &USoulForgeDestructible::execActivarInsight },
		{ .NameUTF8 = UTF8TEXT("ActivateKamehameha"), .Pointer = &USoulForgeDestructible::execActivateKamehameha },
		{ .NameUTF8 = UTF8TEXT("ApplyDamage"), .Pointer = &USoulForgeDestructible::execApplyDamage },
		{ .NameUTF8 = UTF8TEXT("AutoLinkToRust"), .Pointer = &USoulForgeDestructible::execAutoLinkToRust },
		{ .NameUTF8 = UTF8TEXT("DeactivateOriginal"), .Pointer = &USoulForgeDestructible::execDeactivateOriginal },
		{ .NameUTF8 = UTF8TEXT("ForzarEncendidoRust"), .Pointer = &USoulForgeDestructible::execForzarEncendidoRust },
		{ .NameUTF8 = UTF8TEXT("GetHealth"), .Pointer = &USoulForgeDestructible::execGetHealth },
		{ .NameUTF8 = UTF8TEXT("PreparaRenderizadoFragmentos"), .Pointer = &USoulForgeDestructible::execPreparaRenderizadoFragmentos },
		{ .NameUTF8 = UTF8TEXT("PreviewExplosion"), .Pointer = &USoulForgeDestructible::execPreviewExplosion },
		{ .NameUTF8 = UTF8TEXT("RecibirImpacto"), .Pointer = &USoulForgeDestructible::execRecibirImpacto },
		{ .NameUTF8 = UTF8TEXT("ResetInternalState"), .Pointer = &USoulForgeDestructible::execResetInternalState },
		{ .NameUTF8 = UTF8TEXT("SpawnNodeGrid"), .Pointer = &USoulForgeDestructible::execSpawnNodeGrid },
		{ .NameUTF8 = UTF8TEXT("TriggerDestruction"), .Pointer = &USoulForgeDestructible::execTriggerDestruction },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeDestructible_ActivarInsight, "ActivarInsight" }, // 2265005253
		{ &Z_Construct_UFunction_USoulForgeDestructible_ActivateKamehameha, "ActivateKamehameha" }, // 3222836936
		{ &Z_Construct_UFunction_USoulForgeDestructible_ApplyDamage, "ApplyDamage" }, // 1657719209
		{ &Z_Construct_UFunction_USoulForgeDestructible_AutoLinkToRust, "AutoLinkToRust" }, // 1141706354
		{ &Z_Construct_UFunction_USoulForgeDestructible_DeactivateOriginal, "DeactivateOriginal" }, // 1371365178
		{ &Z_Construct_UFunction_USoulForgeDestructible_ForzarEncendidoRust, "ForzarEncendidoRust" }, // 2324364540
		{ &Z_Construct_UFunction_USoulForgeDestructible_GetHealth, "GetHealth" }, // 3346019867
		{ &Z_Construct_UFunction_USoulForgeDestructible_PreparaRenderizadoFragmentos, "PreparaRenderizadoFragmentos" }, // 626005550
		{ &Z_Construct_UFunction_USoulForgeDestructible_PreviewExplosion, "PreviewExplosion" }, // 2121753330
		{ &Z_Construct_UFunction_USoulForgeDestructible_RecibirImpacto, "RecibirImpacto" }, // 119252405
		{ &Z_Construct_UFunction_USoulForgeDestructible_ResetInternalState, "ResetInternalState" }, // 2554793391
		{ &Z_Construct_UFunction_USoulForgeDestructible_SpawnNodeGrid, "SpawnNodeGrid" }, // 3358682910
		{ &Z_Construct_UFunction_USoulForgeDestructible_TriggerDestruction, "TriggerDestruction" }, // 3552338102
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeDestructible>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeDestructible_Statics

// ********** Begin Class USoulForgeDestructible Property Definitions ******************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ProxyName = { "ProxyName", nullptr, (EPropertyFlags)0x0010000000020015, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, ProxyName), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ProxyName_MetaData), NewProp_ProxyName_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000020015, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, ProxyId), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ProxyId_MetaData), NewProp_ProxyId_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PersistentDNI = { "PersistentDNI", nullptr, (EPropertyFlags)0x0010000000020015, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, PersistentDNI), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PersistentDNI_MetaData), NewProp_PersistentDNI_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PersistentGUID = { "PersistentGUID", nullptr, (EPropertyFlags)0x0010000000020001, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, PersistentGUID), Z_Construct_UScriptStruct_FGuid, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PersistentGUID_MetaData), NewProp_PersistentGUID_MetaData) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaterialType_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaterialType = { "MaterialType", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, MaterialType), Z_Construct_UEnum_SoulForgePhysX_EMaterialType, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_MaterialType_MetaData), NewProp_MaterialType_MetaData) }; // 497896940
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PhysicsLOD_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PhysicsLOD = { "PhysicsLOD", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, PhysicsLOD), Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PhysicsLOD_MetaData), NewProp_PhysicsLOD_MetaData) }; // 2707074426
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ExplosiveType_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ExplosiveType = { "ExplosiveType", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, ExplosiveType), Z_Construct_UEnum_SoulForgePhysX_EExplosiveType, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ExplosiveType_MetaData), NewProp_ExplosiveType_MetaData) }; // 4168153385
const UECodeGen_Private::FStructPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_HalfExtent = { "HalfExtent", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, HalfExtent), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_HalfExtent_MetaData), NewProp_HalfExtent_MetaData) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DamagePreset_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DamagePreset = { "DamagePreset", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, DamagePreset), Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_DamagePreset_MetaData), NewProp_DamagePreset_MetaData) }; // 3946375310
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DustVFX = { "DustVFX", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, DustVFX), Z_Construct_UClass_UNiagaraSystem_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_DustVFX_MetaData), NewProp_DustVFX_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FragmentRenderer = { "FragmentRenderer", nullptr, (EPropertyFlags)0x011400000008000d, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, FragmentRenderer), Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FragmentRenderer_MetaData), NewProp_FragmentRenderer_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_StabilityThreshold = { "StabilityThreshold", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, StabilityThreshold), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_StabilityThreshold_MetaData), NewProp_StabilityThreshold_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FuerzaDeImpacto = { "FuerzaDeImpacto", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, FuerzaDeImpacto), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FuerzaDeImpacto_MetaData), NewProp_FuerzaDeImpacto_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_RadioDeImpacto = { "RadioDeImpacto", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, RadioDeImpacto), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_RadioDeImpacto_MetaData), NewProp_RadioDeImpacto_MetaData) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FormaDeExplosion_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FormaDeExplosion = { "FormaDeExplosion", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, FormaDeExplosion), Z_Construct_UEnum_SoulForgePhysX_ESoulForgePattern, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FormaDeExplosion_MetaData), NewProp_FormaDeExplosion_MetaData) }; // 1577948756
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_CantidadDeNodos = { "CantidadDeNodos", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, CantidadDeNodos), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_CantidadDeNodos_MetaData), NewProp_CantidadDeNodos_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_CantidadDeNodosHecatombe = { "CantidadDeNodosHecatombe", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, CantidadDeNodosHecatombe), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_CantidadDeNodosHecatombe_MetaData), NewProp_CantidadDeNodosHecatombe_MetaData) };
void Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarRenderizadoRustico_SetBit(void* Obj)
{
	((USoulForgeDestructible*)Obj)->bUsarRenderizadoRustico = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarRenderizadoRustico = { "bUsarRenderizadoRustico", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeDestructible), &Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarRenderizadoRustico_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bUsarRenderizadoRustico_MetaData), NewProp_bUsarRenderizadoRustico_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_SharedNiagaraVFX = { "SharedNiagaraVFX", nullptr, (EPropertyFlags)0x001000000008000d, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, SharedNiagaraVFX), Z_Construct_UClass_UNiagaraComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_SharedNiagaraVFX_MetaData), NewProp_SharedNiagaraVFX_MetaData) };
void Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bActivarMiraLaser_SetBit(void* Obj)
{
	((USoulForgeDestructible*)Obj)->bActivarMiraLaser = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bActivarMiraLaser = { "bActivarMiraLaser", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeDestructible), &Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bActivarMiraLaser_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bActivarMiraLaser_MetaData), NewProp_bActivarMiraLaser_MetaData) };
void Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bIsEngineReady_SetBit(void* Obj)
{
	((USoulForgeDestructible*)Obj)->bIsEngineReady = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bIsEngineReady = { "bIsEngineReady", nullptr, (EPropertyFlags)0x0010000000020015, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeDestructible), &Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bIsEngineReady_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bIsEngineReady_MetaData), NewProp_bIsEngineReady_MetaData) };
void Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bDestroyed_SetBit(void* Obj)
{
	((USoulForgeDestructible*)Obj)->bDestroyed = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bDestroyed = { "bDestroyed", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeDestructible), &Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bDestroyed_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bDestroyed_MetaData), NewProp_bDestroyed_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PreviewDuration = { "PreviewDuration", nullptr, (EPropertyFlags)0x0010000000000001, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, PreviewDuration), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PreviewDuration_MetaData), NewProp_PreviewDuration_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_NucleosCPU = { "NucleosCPU", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, NucleosCPU), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_NucleosCPU_MetaData), NewProp_NucleosCPU_MetaData) };
void Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarAceleracionGPU_SetBit(void* Obj)
{
	((USoulForgeDestructible*)Obj)->bUsarAceleracionGPU = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarAceleracionGPU = { "bUsarAceleracionGPU", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeDestructible), &Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarAceleracionGPU_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bUsarAceleracionGPU_MetaData), NewProp_bUsarAceleracionGPU_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_InstancedRenderer = { "InstancedRenderer", nullptr, (EPropertyFlags)0x0010000000080008, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, InstancedRenderer), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_InstancedRenderer_MetaData), NewProp_InstancedRenderer_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaxNodos = { "MaxNodos", nullptr, (EPropertyFlags)0x0010000000000001, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, MaxNodos), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_MaxNodos_MetaData), NewProp_MaxNodos_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MallaDelNodo = { "MallaDelNodo", nullptr, (EPropertyFlags)0x0010000000000001, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, MallaDelNodo), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_MallaDelNodo_MetaData), NewProp_MallaDelNodo_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MasterShardMaterial = { "MasterShardMaterial", nullptr, (EPropertyFlags)0x0010000000000001, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeDestructible, MasterShardMaterial), Z_Construct_UClass_UMaterialInterface_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_MasterShardMaterial_MetaData), NewProp_MasterShardMaterial_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UClass_USoulForgeDestructible_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ProxyName,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PersistentDNI,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PersistentGUID,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaterialType_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaterialType,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PhysicsLOD_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PhysicsLOD,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ExplosiveType_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_ExplosiveType,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_HalfExtent,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DamagePreset_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DamagePreset,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_DustVFX,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FragmentRenderer,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_StabilityThreshold,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FuerzaDeImpacto,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_RadioDeImpacto,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FormaDeExplosion_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_FormaDeExplosion,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_CantidadDeNodos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_CantidadDeNodosHecatombe,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarRenderizadoRustico,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_SharedNiagaraVFX,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bActivarMiraLaser,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bIsEngineReady,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bDestroyed,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_PreviewDuration,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_NucleosCPU,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_bUsarAceleracionGPU,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_InstancedRenderer,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MaxNodos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MallaDelNodo,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeDestructible_Statics::NewProp_MasterShardMaterial,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeDestructible_Statics::PropPointers) < 2048);
// ********** End Class USoulForgeDestructible Property Definitions ********************************
UObject* (*const Z_Construct_UClass_USoulForgeDestructible_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UActorComponent,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeDestructible_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeDestructible_Statics::ClassParams = {
	&USoulForgeDestructible::StaticClass,
	"Engine",
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	Z_Construct_UClass_USoulForgeDestructible_Statics::PropPointers,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeDestructible_Statics::PropPointers),
	0,
	0x00B000A4u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeDestructible_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeDestructible_Statics::Class_MetaDataParams)
};
void USoulForgeDestructible::StaticRegisterNativesUSoulForgeDestructible()
{
	UClass* Class = USoulForgeDestructible::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeDestructible_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeDestructible()
{
	if (!Z_Registration_Info_UClass_USoulForgeDestructible.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeDestructible.OuterSingleton, Z_Construct_UClass_USoulForgeDestructible_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeDestructible.OuterSingleton;
}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeDestructible);
USoulForgeDestructible::~USoulForgeDestructible() {}
// ********** End Class USoulForgeDestructible *****************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics
{
	static constexpr FEnumRegisterCompiledInInfo EnumInfo[] = {
		{ ESoulForgePattern_StaticEnum, TEXT("ESoulForgePattern"), &Z_Registration_Info_UEnum_ESoulForgePattern, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 1577948756U) },
	};
	static constexpr FStructRegisterCompiledInInfo ScriptStructInfo[] = {
		{ FSoulForgeNode::StaticStruct, Z_Construct_UScriptStruct_FSoulForgeNode_Statics::NewStructOps, TEXT("SoulForgeNode"),&Z_Registration_Info_UScriptStruct_FSoulForgeNode, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FSoulForgeNode), 3800629931U) },
	};
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeDestructible, USoulForgeDestructible::StaticClass, TEXT("USoulForgeDestructible"), &Z_Registration_Info_UClass_USoulForgeDestructible, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeDestructible), 1736090007U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_3737368310{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::ClassInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::ScriptStructInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::ScriptStructInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::EnumInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeDestructible_h__Script_SoulForgePhysX_Statics::EnumInfo),
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
