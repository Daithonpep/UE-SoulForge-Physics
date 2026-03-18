// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeInsightBridge.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeInsightBridge() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
ENGINE_API UClass* Z_Construct_UClass_UBlueprintFunctionLibrary();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeInsightBridge();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeInsightBridge_NoRegister();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Class USoulForgeInsightBridge Function FiltrarInstanciasSoulForge **************
struct Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics
{
	struct SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms
	{
		TArray<FVector> Posiciones;
		FVector CamaraPos;
		float DistanciaVision;
		TArray<int32> ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Insight" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Nodo para pasar el arreglo de Unreal a Rust y recibir los visibles\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeInsightBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Nodo para pasar el arreglo de Unreal a Rust y recibir los visibles" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Posiciones_MetaData[] = {
		{ "NativeConst", "" },
	};
#endif // WITH_METADATA

// ********** Begin Function FiltrarInstanciasSoulForge constinit property declarations ************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Posiciones_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_Posiciones;
	static const UECodeGen_Private::FStructPropertyParams NewProp_CamaraPos;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DistanciaVision;
	static const UECodeGen_Private::FIntPropertyParams NewProp_ReturnValue_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function FiltrarInstanciasSoulForge constinit property declarations **************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function FiltrarInstanciasSoulForge Property Definitions ***********************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_Posiciones_Inner = { "Posiciones", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_Posiciones = { "Posiciones", nullptr, (EPropertyFlags)0x0010000008000182, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms, Posiciones), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Posiciones_MetaData), NewProp_Posiciones_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_CamaraPos = { "CamaraPos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms, CamaraPos), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_DistanciaVision = { "DistanciaVision", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms, DistanciaVision), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_ReturnValue_Inner = { "ReturnValue", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms, ReturnValue), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_Posiciones_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_Posiciones,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_CamaraPos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_DistanciaVision,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_ReturnValue_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::PropPointers) < 2048);
// ********** End Function FiltrarInstanciasSoulForge Property Definitions *************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeInsightBridge, nullptr, "FiltrarInstanciasSoulForge", 	Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04C22401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::SoulForgeInsightBridge_eventFiltrarInstanciasSoulForge_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeInsightBridge::execFiltrarInstanciasSoulForge)
{
	P_GET_TARRAY_REF(FVector,Z_Param_Out_Posiciones);
	P_GET_STRUCT(FVector,Z_Param_CamaraPos);
	P_GET_PROPERTY(FFloatProperty,Z_Param_DistanciaVision);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(TArray<int32>*)Z_Param__Result=USoulForgeInsightBridge::FiltrarInstanciasSoulForge(Z_Param_Out_Posiciones,Z_Param_CamaraPos,Z_Param_DistanciaVision);
	P_NATIVE_END;
}
// ********** End Class USoulForgeInsightBridge Function FiltrarInstanciasSoulForge ****************

// ********** Begin Class USoulForgeInsightBridge Function IniciarSoulForgeInsight *****************
struct Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics
{
	struct SoulForgeInsightBridge_eventIniciarSoulForgeInsight_Parms
	{
		bool ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Insight" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Este nodo de Blueprint cargar\xc3\xa1 el n\xc3\xba""cleo y probar\xc3\xa1 la conexi\xc3\xb3n\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeInsightBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Este nodo de Blueprint cargar\xc3\xa1 el n\xc3\xba""cleo y probar\xc3\xa1 la conexi\xc3\xb3n" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function IniciarSoulForgeInsight constinit property declarations ***************
	static void NewProp_ReturnValue_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function IniciarSoulForgeInsight constinit property declarations *****************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function IniciarSoulForgeInsight Property Definitions **************************
void Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::NewProp_ReturnValue_SetBit(void* Obj)
{
	((SoulForgeInsightBridge_eventIniciarSoulForgeInsight_Parms*)Obj)->ReturnValue = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(SoulForgeInsightBridge_eventIniciarSoulForgeInsight_Parms), &Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::NewProp_ReturnValue_SetBit, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::PropPointers) < 2048);
// ********** End Function IniciarSoulForgeInsight Property Definitions ****************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeInsightBridge, nullptr, "IniciarSoulForgeInsight", 	Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::SoulForgeInsightBridge_eventIniciarSoulForgeInsight_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::SoulForgeInsightBridge_eventIniciarSoulForgeInsight_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeInsightBridge::execIniciarSoulForgeInsight)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	*(bool*)Z_Param__Result=USoulForgeInsightBridge::IniciarSoulForgeInsight();
	P_NATIVE_END;
}
// ********** End Class USoulForgeInsightBridge Function IniciarSoulForgeInsight *******************

// ********** Begin Class USoulForgeInsightBridge **************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeInsightBridge;
UClass* USoulForgeInsightBridge::GetPrivateStaticClass()
{
	using TClass = USoulForgeInsightBridge;
	if (!Z_Registration_Info_UClass_USoulForgeInsightBridge.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeInsightBridge"),
			Z_Registration_Info_UClass_USoulForgeInsightBridge.InnerSingleton,
			StaticRegisterNativesUSoulForgeInsightBridge,
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
	return Z_Registration_Info_UClass_USoulForgeInsightBridge.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeInsightBridge_NoRegister()
{
	return USoulForgeInsightBridge::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeInsightBridge_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "IncludePath", "SoulForgeInsightBridge.h" },
		{ "ModuleRelativePath", "Public/SoulForgeInsightBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeInsightBridge constinit property declarations ******************
// ********** End Class USoulForgeInsightBridge constinit property declarations ********************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("FiltrarInstanciasSoulForge"), .Pointer = &USoulForgeInsightBridge::execFiltrarInstanciasSoulForge },
		{ .NameUTF8 = UTF8TEXT("IniciarSoulForgeInsight"), .Pointer = &USoulForgeInsightBridge::execIniciarSoulForgeInsight },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeInsightBridge_FiltrarInstanciasSoulForge, "FiltrarInstanciasSoulForge" }, // 3139475243
		{ &Z_Construct_UFunction_USoulForgeInsightBridge_IniciarSoulForgeInsight, "IniciarSoulForgeInsight" }, // 2259030398
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeInsightBridge>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeInsightBridge_Statics
UObject* (*const Z_Construct_UClass_USoulForgeInsightBridge_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UBlueprintFunctionLibrary,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeInsightBridge_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeInsightBridge_Statics::ClassParams = {
	&USoulForgeInsightBridge::StaticClass,
	nullptr,
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	nullptr,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	0,
	0,
	0x001000A0u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeInsightBridge_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeInsightBridge_Statics::Class_MetaDataParams)
};
void USoulForgeInsightBridge::StaticRegisterNativesUSoulForgeInsightBridge()
{
	UClass* Class = USoulForgeInsightBridge::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeInsightBridge_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeInsightBridge()
{
	if (!Z_Registration_Info_UClass_USoulForgeInsightBridge.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeInsightBridge.OuterSingleton, Z_Construct_UClass_USoulForgeInsightBridge_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeInsightBridge.OuterSingleton;
}
USoulForgeInsightBridge::USoulForgeInsightBridge(const FObjectInitializer& ObjectInitializer) : Super(ObjectInitializer) {}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeInsightBridge);
USoulForgeInsightBridge::~USoulForgeInsightBridge() {}
// ********** End Class USoulForgeInsightBridge ****************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeInsightBridge_h__Script_SoulForgePhysX_Statics
{
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeInsightBridge, USoulForgeInsightBridge::StaticClass, TEXT("USoulForgeInsightBridge"), &Z_Registration_Info_UClass_USoulForgeInsightBridge, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeInsightBridge), 4151854934U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeInsightBridge_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeInsightBridge_h__Script_SoulForgePhysX_177309744{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeInsightBridge_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeInsightBridge_h__Script_SoulForgePhysX_Statics::ClassInfo),
	nullptr, 0,
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
