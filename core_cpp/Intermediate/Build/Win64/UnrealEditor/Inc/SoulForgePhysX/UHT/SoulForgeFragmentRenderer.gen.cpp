// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeFragmentRenderer.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeFragmentRenderer() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FTransform();
ENGINE_API UClass* Z_Construct_UClass_UInstancedStaticMeshComponent();
ENGINE_API UClass* Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister();
ENGINE_API UClass* Z_Construct_UClass_UMaterialInterface_NoRegister();
ENGINE_API UClass* Z_Construct_UClass_UStaticMesh_NoRegister();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeFragmentRenderer();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeFragmentInstance();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin ScriptStruct FSoulForgeFragmentInstance ****************************************
struct Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FSoulForgeFragmentInstance); }
	static inline consteval int16 GetStructAlignment() { return alignof(FSoulForgeFragmentInstance); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Info de un fragmento individual \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Info de un fragmento individual \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FragmentSimId_MetaData[] = {
		{ "Category", "SoulForgeFragmentInstance" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** ID del fragmento en la simulaci\xc3\xb3n de Rust (fragment_sim). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "ID del fragmento en la simulaci\xc3\xb3n de Rust (fragment_sim)." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_InstanceIndex_MetaData[] = {
		{ "Category", "SoulForgeFragmentInstance" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xc3\x8dndice de la instancia en su HISM. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xc3\x8dndice de la instancia en su HISM." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Category_MetaData[] = {
		{ "Category", "SoulForgeFragmentInstance" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Categor\xc3\xad""a: 0=Slab, 1=Chunk, 2=Gravel, 3=Dust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Categor\xc3\xad""a: 0=Slab, 1=Chunk, 2=Gravel, 3=Dust" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bSleeping_MetaData[] = {
		{ "Category", "SoulForgeFragmentInstance" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Si es true, el fragmento est\xc3\xa1 en reposo y no se consulta Rust. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Si es true, el fragmento est\xc3\xa1 en reposo y no se consulta Rust." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FSoulForgeFragmentInstance constinit property declarations ********
	static const UECodeGen_Private::FIntPropertyParams NewProp_FragmentSimId;
	static const UECodeGen_Private::FIntPropertyParams NewProp_InstanceIndex;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Category;
	static void NewProp_bSleeping_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bSleeping;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FSoulForgeFragmentInstance constinit property declarations **********
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FSoulForgeFragmentInstance>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance;
class UScriptStruct* FSoulForgeFragmentInstance::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FSoulForgeFragmentInstance, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("SoulForgeFragmentInstance"));
	}
	return Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.OuterSingleton;
	}

// ********** Begin ScriptStruct FSoulForgeFragmentInstance Property Definitions *******************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_FragmentSimId = { "FragmentSimId", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentInstance, FragmentSimId), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FragmentSimId_MetaData), NewProp_FragmentSimId_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_InstanceIndex = { "InstanceIndex", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentInstance, InstanceIndex), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_InstanceIndex_MetaData), NewProp_InstanceIndex_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_Category = { "Category", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentInstance, Category), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Category_MetaData), NewProp_Category_MetaData) };
void Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_bSleeping_SetBit(void* Obj)
{
	((FSoulForgeFragmentInstance*)Obj)->bSleeping = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_bSleeping = { "bSleeping", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(FSoulForgeFragmentInstance), &Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_bSleeping_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bSleeping_MetaData), NewProp_bSleeping_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_FragmentSimId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_InstanceIndex,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_Category,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewProp_bSleeping,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FSoulForgeFragmentInstance Property Definitions *********************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"SoulForgeFragmentInstance",
	Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::PropPointers),
	sizeof(FSoulForgeFragmentInstance),
	alignof(FSoulForgeFragmentInstance),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeFragmentInstance()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.InnerSingleton, Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance.InnerSingleton);
}
// ********** End ScriptStruct FSoulForgeFragmentInstance ******************************************

// ********** Begin Class USoulForgeFragmentRenderer Function Cleanup ******************************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Renderer" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Elimina todas las instancias HISM y limpia el estado. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Elimina todas las instancias HISM y limpia el estado." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function Cleanup constinit property declarations *******************************
// ********** End Function Cleanup constinit property declarations *********************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "Cleanup", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execCleanup)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->Cleanup();
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function Cleanup ********************************

// ********** Begin Class USoulForgeFragmentRenderer Function ClearAllFragments ********************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function ClearAllFragments constinit property declarations *********************
// ********** End Function ClearAllFragments constinit property declarations ***********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "ClearAllFragments", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execClearAllFragments)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ClearAllFragments();
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function ClearAllFragments **********************

// ********** Begin Class USoulForgeFragmentRenderer Function ExplodeIntoFragments *****************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics
{
	struct SoulForgeFragmentRenderer_eventExplodeIntoFragments_Parms
	{
		FString JsonData;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Destruction" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \n\x09 * ESTA ES LA FUNCI\xc3\x93N QUE BUSCABAS: Aparecer\xc3\xa1 al arrastrar el cable del componente.\n\x09 * Procesa los datos de explosi\xc3\xb3n en formato JSON y registra los fragmentos.\n\x09 */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "ESTA ES LA FUNCI\xc3\x93N QUE BUSCABAS: Aparecer\xc3\xa1 al arrastrar el cable del componente.\nProcesa los datos de explosi\xc3\xb3n en formato JSON y registra los fragmentos." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ExplodeIntoFragments constinit property declarations ******************
	static const UECodeGen_Private::FStrPropertyParams NewProp_JsonData;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ExplodeIntoFragments constinit property declarations ********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ExplodeIntoFragments Property Definitions *****************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::NewProp_JsonData = { "JsonData", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeFragmentRenderer_eventExplodeIntoFragments_Parms, JsonData), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::NewProp_JsonData,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::PropPointers) < 2048);
// ********** End Function ExplodeIntoFragments Property Definitions *******************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "ExplodeIntoFragments", 	Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::SoulForgeFragmentRenderer_eventExplodeIntoFragments_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::SoulForgeFragmentRenderer_eventExplodeIntoFragments_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execExplodeIntoFragments)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_JsonData);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ExplodeIntoFragments(Z_Param_JsonData);
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function ExplodeIntoFragments *******************

// ********** Begin Class USoulForgeFragmentRenderer Function GetActiveFragmentCount ***************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics
{
	struct SoulForgeFragmentRenderer_eventGetActiveFragmentCount_Parms
	{
		int32 ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Renderer" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** N\xc3\xbamero de fragmentos a\xc3\xban en vuelo (no dormidos). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "N\xc3\xbamero de fragmentos a\xc3\xban en vuelo (no dormidos)." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function GetActiveFragmentCount constinit property declarations ****************
	static const UECodeGen_Private::FIntPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function GetActiveFragmentCount constinit property declarations ******************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function GetActiveFragmentCount Property Definitions ***************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeFragmentRenderer_eventGetActiveFragmentCount_Parms, ReturnValue), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::PropPointers) < 2048);
// ********** End Function GetActiveFragmentCount Property Definitions *****************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "GetActiveFragmentCount", 	Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::SoulForgeFragmentRenderer_eventGetActiveFragmentCount_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x54020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::SoulForgeFragmentRenderer_eventGetActiveFragmentCount_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execGetActiveFragmentCount)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	*(int32*)Z_Param__Result=P_THIS->GetActiveFragmentCount();
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function GetActiveFragmentCount *****************

// ********** Begin Class USoulForgeFragmentRenderer Function NotifyExplosion **********************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Renderer" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Se\xc3\xb1""aliza que ha ocurrido una explosi\xc3\xb3n para activar efectos internos */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Se\xc3\xb1""aliza que ha ocurrido una explosi\xc3\xb3n para activar efectos internos" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function NotifyExplosion constinit property declarations ***********************
// ********** End Function NotifyExplosion constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "NotifyExplosion", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execNotifyExplosion)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->NotifyExplosion();
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function NotifyExplosion ************************

// ********** Begin Class USoulForgeFragmentRenderer Function TickUpdate ***************************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics
{
	struct SoulForgeFragmentRenderer_eventTickUpdate_Parms
	{
		float DeltaTime;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Renderer" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Actualiza las transformadas de todos los fragmentos activos. Llamar en Tick. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Actualiza las transformadas de todos los fragmentos activos. Llamar en Tick." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function TickUpdate constinit property declarations ****************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DeltaTime;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function TickUpdate constinit property declarations ******************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function TickUpdate Property Definitions ***************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::NewProp_DeltaTime = { "DeltaTime", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeFragmentRenderer_eventTickUpdate_Parms, DeltaTime), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::NewProp_DeltaTime,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::PropPointers) < 2048);
// ********** End Function TickUpdate Property Definitions *****************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "TickUpdate", 	Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::SoulForgeFragmentRenderer_eventTickUpdate_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::SoulForgeFragmentRenderer_eventTickUpdate_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execTickUpdate)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_DeltaTime);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->TickUpdate(Z_Param_DeltaTime);
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function TickUpdate *****************************

// ********** Begin Class USoulForgeFragmentRenderer Function UpdateHISMTransforms *****************
struct Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics
{
	struct SoulForgeFragmentRenderer_eventUpdateHISMTransforms_Parms
	{
		int32 Category;
		TArray<FTransform> NewTransforms;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Renderer" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_NewTransforms_MetaData[] = {
		{ "NativeConst", "" },
	};
#endif // WITH_METADATA

// ********** Begin Function UpdateHISMTransforms constinit property declarations ******************
	static const UECodeGen_Private::FIntPropertyParams NewProp_Category;
	static const UECodeGen_Private::FStructPropertyParams NewProp_NewTransforms_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_NewTransforms;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function UpdateHISMTransforms constinit property declarations ********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function UpdateHISMTransforms Property Definitions *****************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_Category = { "Category", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeFragmentRenderer_eventUpdateHISMTransforms_Parms, Category), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_NewTransforms_Inner = { "NewTransforms", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FTransform, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_NewTransforms = { "NewTransforms", nullptr, (EPropertyFlags)0x0010000008000182, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeFragmentRenderer_eventUpdateHISMTransforms_Parms, NewTransforms), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_NewTransforms_MetaData), NewProp_NewTransforms_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_Category,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_NewTransforms_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::NewProp_NewTransforms,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::PropPointers) < 2048);
// ********** End Function UpdateHISMTransforms Property Definitions *******************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeFragmentRenderer, nullptr, "UpdateHISMTransforms", 	Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::SoulForgeFragmentRenderer_eventUpdateHISMTransforms_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04420401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::SoulForgeFragmentRenderer_eventUpdateHISMTransforms_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeFragmentRenderer::execUpdateHISMTransforms)
{
	P_GET_PROPERTY(FIntProperty,Z_Param_Category);
	P_GET_TARRAY_REF(FTransform,Z_Param_Out_NewTransforms);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->UpdateHISMTransforms(Z_Param_Category,Z_Param_Out_NewTransforms);
	P_NATIVE_END;
}
// ********** End Class USoulForgeFragmentRenderer Function UpdateHISMTransforms *******************

// ********** Begin Class USoulForgeFragmentRenderer ***********************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeFragmentRenderer;
UClass* USoulForgeFragmentRenderer::GetPrivateStaticClass()
{
	using TClass = USoulForgeFragmentRenderer;
	if (!Z_Registration_Info_UClass_USoulForgeFragmentRenderer.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeFragmentRenderer"),
			Z_Registration_Info_UClass_USoulForgeFragmentRenderer.InnerSingleton,
			StaticRegisterNativesUSoulForgeFragmentRenderer,
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
	return Z_Registration_Info_UClass_USoulForgeFragmentRenderer.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeFragmentRenderer_NoRegister()
{
	return USoulForgeFragmentRenderer::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeFragmentRenderer_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "BlueprintSpawnableComponent", "" },
		{ "ClassGroupNames", "SoulForge" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Renderer Component \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\n" },
#endif
		{ "HideCategories", "Object Activation Components|Activation Trigger" },
		{ "IncludePath", "SoulForgeFragmentRenderer.h" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80 Renderer Component \xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_SlabMesh_MetaData[] = {
		{ "Category", "SoulForge|Meshes" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Mesh para fragmentos tipo Losa (grandes, pesados). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Mesh para fragmentos tipo Losa (grandes, pesados)." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ChunkMesh_MetaData[] = {
		{ "Category", "SoulForge|Meshes" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Mesh para cascotes medianos. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Mesh para cascotes medianos." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_GravelMesh_MetaData[] = {
		{ "Category", "SoulForge|Meshes" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Mesh para grava fina. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Mesh para grava fina." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_DustMesh_MetaData[] = {
		{ "Category", "SoulForge|Meshes" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Mesh para polvo/part\xc3\xad""culas muy peque\xc3\xb1""as. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Mesh para polvo/part\xc3\xad""culas muy peque\xc3\xb1""as." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FragmentMaterial_MetaData[] = {
		{ "Category", "SoulForge|Meshes" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Material base compartido por todos los fragmentos. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Material base compartido por todos los fragmentos." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_HISMSlab_MetaData[] = {
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Cuatro ISM: uno por categor\xc3\xad""a (Slab, Chunk, Gravel, Dust).\n" },
#endif
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Cuatro ISM: uno por categor\xc3\xad""a (Slab, Chunk, Gravel, Dust)." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_HISMChunk_MetaData[] = {
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_HISMGravel_MetaData[] = {
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_HISMDust_MetaData[] = {
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeFragmentRenderer.h" },
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeFragmentRenderer constinit property declarations ***************
	static const UECodeGen_Private::FObjectPropertyParams NewProp_SlabMesh;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_ChunkMesh;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_GravelMesh;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_DustMesh;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_FragmentMaterial;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_HISMSlab;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_HISMChunk;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_HISMGravel;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_HISMDust;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Class USoulForgeFragmentRenderer constinit property declarations *****************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("Cleanup"), .Pointer = &USoulForgeFragmentRenderer::execCleanup },
		{ .NameUTF8 = UTF8TEXT("ClearAllFragments"), .Pointer = &USoulForgeFragmentRenderer::execClearAllFragments },
		{ .NameUTF8 = UTF8TEXT("ExplodeIntoFragments"), .Pointer = &USoulForgeFragmentRenderer::execExplodeIntoFragments },
		{ .NameUTF8 = UTF8TEXT("GetActiveFragmentCount"), .Pointer = &USoulForgeFragmentRenderer::execGetActiveFragmentCount },
		{ .NameUTF8 = UTF8TEXT("NotifyExplosion"), .Pointer = &USoulForgeFragmentRenderer::execNotifyExplosion },
		{ .NameUTF8 = UTF8TEXT("TickUpdate"), .Pointer = &USoulForgeFragmentRenderer::execTickUpdate },
		{ .NameUTF8 = UTF8TEXT("UpdateHISMTransforms"), .Pointer = &USoulForgeFragmentRenderer::execUpdateHISMTransforms },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_Cleanup, "Cleanup" }, // 3623544978
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_ClearAllFragments, "ClearAllFragments" }, // 1174759602
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_ExplodeIntoFragments, "ExplodeIntoFragments" }, // 1397567751
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_GetActiveFragmentCount, "GetActiveFragmentCount" }, // 3939083673
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_NotifyExplosion, "NotifyExplosion" }, // 3624358274
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_TickUpdate, "TickUpdate" }, // 1713791674
		{ &Z_Construct_UFunction_USoulForgeFragmentRenderer_UpdateHISMTransforms, "UpdateHISMTransforms" }, // 4258942432
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeFragmentRenderer>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeFragmentRenderer_Statics

// ********** Begin Class USoulForgeFragmentRenderer Property Definitions **************************
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_SlabMesh = { "SlabMesh", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, SlabMesh), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_SlabMesh_MetaData), NewProp_SlabMesh_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_ChunkMesh = { "ChunkMesh", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, ChunkMesh), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ChunkMesh_MetaData), NewProp_ChunkMesh_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_GravelMesh = { "GravelMesh", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, GravelMesh), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_GravelMesh_MetaData), NewProp_GravelMesh_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_DustMesh = { "DustMesh", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, DustMesh), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_DustMesh_MetaData), NewProp_DustMesh_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_FragmentMaterial = { "FragmentMaterial", nullptr, (EPropertyFlags)0x0114000000000005, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, FragmentMaterial), Z_Construct_UClass_UMaterialInterface_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FragmentMaterial_MetaData), NewProp_FragmentMaterial_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMSlab = { "HISMSlab", nullptr, (EPropertyFlags)0x0144000000080008, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, HISMSlab), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_HISMSlab_MetaData), NewProp_HISMSlab_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMChunk = { "HISMChunk", nullptr, (EPropertyFlags)0x0144000000080008, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, HISMChunk), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_HISMChunk_MetaData), NewProp_HISMChunk_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMGravel = { "HISMGravel", nullptr, (EPropertyFlags)0x0144000000080008, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, HISMGravel), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_HISMGravel_MetaData), NewProp_HISMGravel_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMDust = { "HISMDust", nullptr, (EPropertyFlags)0x0144000000080008, UECodeGen_Private::EPropertyGenFlags::Object | UECodeGen_Private::EPropertyGenFlags::ObjectPtr, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeFragmentRenderer, HISMDust), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_HISMDust_MetaData), NewProp_HISMDust_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_SlabMesh,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_ChunkMesh,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_GravelMesh,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_DustMesh,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_FragmentMaterial,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMSlab,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMChunk,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMGravel,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::NewProp_HISMDust,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::PropPointers) < 2048);
// ********** End Class USoulForgeFragmentRenderer Property Definitions ****************************
UObject* (*const Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UInstancedStaticMeshComponent,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::ClassParams = {
	&USoulForgeFragmentRenderer::StaticClass,
	"Engine",
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::PropPointers,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::PropPointers),
	0,
	0x00B010A4u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::Class_MetaDataParams)
};
void USoulForgeFragmentRenderer::StaticRegisterNativesUSoulForgeFragmentRenderer()
{
	UClass* Class = USoulForgeFragmentRenderer::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeFragmentRenderer()
{
	if (!Z_Registration_Info_UClass_USoulForgeFragmentRenderer.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeFragmentRenderer.OuterSingleton, Z_Construct_UClass_USoulForgeFragmentRenderer_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeFragmentRenderer.OuterSingleton;
}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeFragmentRenderer);
USoulForgeFragmentRenderer::~USoulForgeFragmentRenderer() {}
// ********** End Class USoulForgeFragmentRenderer *************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics
{
	static constexpr FStructRegisterCompiledInInfo ScriptStructInfo[] = {
		{ FSoulForgeFragmentInstance::StaticStruct, Z_Construct_UScriptStruct_FSoulForgeFragmentInstance_Statics::NewStructOps, TEXT("SoulForgeFragmentInstance"),&Z_Registration_Info_UScriptStruct_FSoulForgeFragmentInstance, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FSoulForgeFragmentInstance), 949133894U) },
	};
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeFragmentRenderer, USoulForgeFragmentRenderer::StaticClass, TEXT("USoulForgeFragmentRenderer"), &Z_Registration_Info_UClass_USoulForgeFragmentRenderer, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeFragmentRenderer), 100560509U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_2921585913{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics::ClassInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics::ScriptStructInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeFragmentRenderer_h__Script_SoulForgePhysX_Statics::ScriptStructInfo),
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
