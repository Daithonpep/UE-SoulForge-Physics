// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeHelper.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeHelper() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FTransform();
ENGINE_API UClass* Z_Construct_UClass_UBlueprintFunctionLibrary();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeHelper();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeHelper_NoRegister();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Class USoulForgeHelper Function ParseRustJson **********************************
struct Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics
{
	struct SoulForgeHelper_eventParseRustJson_Parms
	{
		FString JsonData;
		TArray<FTransform> ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Helpers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n\x09 * Toma el JSON crudo de Rust y lo convierte en una lista de transformadas.\n\x09 * \xc3\x9atil para visualizar o pre-procesar fragmentos en Blueprints.\n\x09 * \n\x09 * @param JsonData Texto en formato JSON devuelto por el n\xc3\xba""cleo de Rust.\n\x09 * @return Array de transformadas listas para usar en Unreal.\n\x09 */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeHelper.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Toma el JSON crudo de Rust y lo convierte en una lista de transformadas.\n\xc3\x9atil para visualizar o pre-procesar fragmentos en Blueprints.\n\n@param JsonData Texto en formato JSON devuelto por el n\xc3\xba""cleo de Rust.\n@return Array de transformadas listas para usar en Unreal." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ParseRustJson constinit property declarations *************************
	static const UECodeGen_Private::FStrPropertyParams NewProp_JsonData;
	static const UECodeGen_Private::FStructPropertyParams NewProp_ReturnValue_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ParseRustJson constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ParseRustJson Property Definitions ************************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_JsonData = { "JsonData", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeHelper_eventParseRustJson_Parms, JsonData), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_ReturnValue_Inner = { "ReturnValue", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FTransform, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeHelper_eventParseRustJson_Parms, ReturnValue), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_JsonData,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_ReturnValue_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::PropPointers) < 2048);
// ********** End Function ParseRustJson Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeHelper, nullptr, "ParseRustJson", 	Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::SoulForgeHelper_eventParseRustJson_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::SoulForgeHelper_eventParseRustJson_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeHelper_ParseRustJson()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeHelper_ParseRustJson_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeHelper::execParseRustJson)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_JsonData);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(TArray<FTransform>*)Z_Param__Result=USoulForgeHelper::ParseRustJson(Z_Param_JsonData);
	P_NATIVE_END;
}
// ********** End Class USoulForgeHelper Function ParseRustJson ************************************

// ********** Begin Class USoulForgeHelper *********************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeHelper;
UClass* USoulForgeHelper::GetPrivateStaticClass()
{
	using TClass = USoulForgeHelper;
	if (!Z_Registration_Info_UClass_USoulForgeHelper.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeHelper"),
			Z_Registration_Info_UClass_USoulForgeHelper.InnerSingleton,
			StaticRegisterNativesUSoulForgeHelper,
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
	return Z_Registration_Info_UClass_USoulForgeHelper.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeHelper_NoRegister()
{
	return USoulForgeHelper::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeHelper_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n * Biblioteca de funciones auxiliares para el ecosistema SoulForge.\n * El \"Eslab\xc3\xb3n Perdido\" que facilita la traducci\xc3\xb3n de datos entre Rust y Unreal.\n */" },
#endif
		{ "IncludePath", "SoulForgeHelper.h" },
		{ "ModuleRelativePath", "Public/SoulForgeHelper.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Biblioteca de funciones auxiliares para el ecosistema SoulForge.\nEl \"Eslab\xc3\xb3n Perdido\" que facilita la traducci\xc3\xb3n de datos entre Rust y Unreal." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeHelper constinit property declarations *************************
// ********** End Class USoulForgeHelper constinit property declarations ***************************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ParseRustJson"), .Pointer = &USoulForgeHelper::execParseRustJson },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeHelper_ParseRustJson, "ParseRustJson" }, // 1388772574
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeHelper>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeHelper_Statics
UObject* (*const Z_Construct_UClass_USoulForgeHelper_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UBlueprintFunctionLibrary,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeHelper_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeHelper_Statics::ClassParams = {
	&USoulForgeHelper::StaticClass,
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
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeHelper_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeHelper_Statics::Class_MetaDataParams)
};
void USoulForgeHelper::StaticRegisterNativesUSoulForgeHelper()
{
	UClass* Class = USoulForgeHelper::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeHelper_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeHelper()
{
	if (!Z_Registration_Info_UClass_USoulForgeHelper.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeHelper.OuterSingleton, Z_Construct_UClass_USoulForgeHelper_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeHelper.OuterSingleton;
}
USoulForgeHelper::USoulForgeHelper(const FObjectInitializer& ObjectInitializer) : Super(ObjectInitializer) {}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeHelper);
USoulForgeHelper::~USoulForgeHelper() {}
// ********** End Class USoulForgeHelper ***********************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeHelper_h__Script_SoulForgePhysX_Statics
{
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeHelper, USoulForgeHelper::StaticClass, TEXT("USoulForgeHelper"), &Z_Registration_Info_UClass_USoulForgeHelper, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeHelper), 3541222617U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeHelper_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeHelper_h__Script_SoulForgePhysX_1623525075{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeHelper_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeHelper_h__Script_SoulForgePhysX_Statics::ClassInfo),
	nullptr, 0,
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
