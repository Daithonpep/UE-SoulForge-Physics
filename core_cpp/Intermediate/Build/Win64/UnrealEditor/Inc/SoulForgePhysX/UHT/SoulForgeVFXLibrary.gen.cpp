// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeVFXLibrary.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeVFXLibrary() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UClass* Z_Construct_UClass_UObject_NoRegister();
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
ENGINE_API UClass* Z_Construct_UClass_UBlueprintFunctionLibrary();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeVFXLibrary();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeVFXLibrary_NoRegister();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Class USoulForgeVFXLibrary Function GenerarOndaReactiva ************************
struct Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics
{
	struct SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms
	{
		UObject* WorldContextObject;
		FVector Epicentro;
		float Energia;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge | Efectos Visuales" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n     * Genera una onda reactiva basada en el terreno.\n     * @param WorldContextObject Objeto de contexto (ej. el componente o actor detonante)\n     * @param Epicentro Posici\xc3\xb3n de la detonaci\xc3\xb3n\n     * @param Energia Fuerza de la explosi\xc3\xb3n para escalar efectos\n     */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeVFXLibrary.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Genera una onda reactiva basada en el terreno.\n@param WorldContextObject Objeto de contexto (ej. el componente o actor detonante)\n@param Epicentro Posici\xc3\xb3n de la detonaci\xc3\xb3n\n@param Energia Fuerza de la explosi\xc3\xb3n para escalar efectos" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function GenerarOndaReactiva constinit property declarations *******************
	static const UECodeGen_Private::FObjectPropertyParams NewProp_WorldContextObject;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Epicentro;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Energia;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function GenerarOndaReactiva constinit property declarations *********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function GenerarOndaReactiva Property Definitions ******************************
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_WorldContextObject = { "WorldContextObject", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms, WorldContextObject), Z_Construct_UClass_UObject_NoRegister, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_Epicentro = { "Epicentro", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms, Epicentro), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_Energia = { "Energia", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms, Energia), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_WorldContextObject,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_Epicentro,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::NewProp_Energia,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::PropPointers) < 2048);
// ********** End Function GenerarOndaReactiva Property Definitions ********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeVFXLibrary, nullptr, "GenerarOndaReactiva", 	Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::SoulForgeVFXLibrary_eventGenerarOndaReactiva_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeVFXLibrary::execGenerarOndaReactiva)
{
	P_GET_OBJECT(UObject,Z_Param_WorldContextObject);
	P_GET_STRUCT(FVector,Z_Param_Epicentro);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Energia);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeVFXLibrary::GenerarOndaReactiva(Z_Param_WorldContextObject,Z_Param_Epicentro,Z_Param_Energia);
	P_NATIVE_END;
}
// ********** End Class USoulForgeVFXLibrary Function GenerarOndaReactiva **************************

// ********** Begin Class USoulForgeVFXLibrary *****************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeVFXLibrary;
UClass* USoulForgeVFXLibrary::GetPrivateStaticClass()
{
	using TClass = USoulForgeVFXLibrary;
	if (!Z_Registration_Info_UClass_USoulForgeVFXLibrary.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeVFXLibrary"),
			Z_Registration_Info_UClass_USoulForgeVFXLibrary.InnerSingleton,
			StaticRegisterNativesUSoulForgeVFXLibrary,
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
	return Z_Registration_Info_UClass_USoulForgeVFXLibrary.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeVFXLibrary_NoRegister()
{
	return USoulForgeVFXLibrary::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeVFXLibrary_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "IncludePath", "SoulForgeVFXLibrary.h" },
		{ "ModuleRelativePath", "Public/SoulForgeVFXLibrary.h" },
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeVFXLibrary constinit property declarations *********************
// ********** End Class USoulForgeVFXLibrary constinit property declarations ***********************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("GenerarOndaReactiva"), .Pointer = &USoulForgeVFXLibrary::execGenerarOndaReactiva },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeVFXLibrary_GenerarOndaReactiva, "GenerarOndaReactiva" }, // 389899667
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeVFXLibrary>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeVFXLibrary_Statics
UObject* (*const Z_Construct_UClass_USoulForgeVFXLibrary_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UBlueprintFunctionLibrary,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeVFXLibrary_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeVFXLibrary_Statics::ClassParams = {
	&USoulForgeVFXLibrary::StaticClass,
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
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeVFXLibrary_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeVFXLibrary_Statics::Class_MetaDataParams)
};
void USoulForgeVFXLibrary::StaticRegisterNativesUSoulForgeVFXLibrary()
{
	UClass* Class = USoulForgeVFXLibrary::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeVFXLibrary_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeVFXLibrary()
{
	if (!Z_Registration_Info_UClass_USoulForgeVFXLibrary.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeVFXLibrary.OuterSingleton, Z_Construct_UClass_USoulForgeVFXLibrary_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeVFXLibrary.OuterSingleton;
}
USoulForgeVFXLibrary::USoulForgeVFXLibrary(const FObjectInitializer& ObjectInitializer) : Super(ObjectInitializer) {}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeVFXLibrary);
USoulForgeVFXLibrary::~USoulForgeVFXLibrary() {}
// ********** End Class USoulForgeVFXLibrary *******************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeVFXLibrary_h__Script_SoulForgePhysX_Statics
{
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeVFXLibrary, USoulForgeVFXLibrary::StaticClass, TEXT("USoulForgeVFXLibrary"), &Z_Registration_Info_UClass_USoulForgeVFXLibrary, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeVFXLibrary), 3015862961U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeVFXLibrary_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeVFXLibrary_h__Script_SoulForgePhysX_4043424814{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeVFXLibrary_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeVFXLibrary_h__Script_SoulForgePhysX_Statics::ClassInfo),
	nullptr, 0,
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
