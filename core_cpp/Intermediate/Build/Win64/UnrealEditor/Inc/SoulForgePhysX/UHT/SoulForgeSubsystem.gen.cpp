// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeSubsystem.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeSubsystem() {}

// ********** Begin Cross Module References ********************************************************
ENGINE_API UClass* Z_Construct_UClass_UWorldSubsystem();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeSubsystem();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeSubsystem_NoRegister();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Enum ESoulForgePerformanceMode *************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_ESoulForgePerformanceMode;
static UEnum* ESoulForgePerformanceMode_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_ESoulForgePerformanceMode.OuterSingleton)
	{
		Z_Registration_Info_UEnum_ESoulForgePerformanceMode.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ESoulForgePerformanceMode"));
	}
	return Z_Registration_Info_UEnum_ESoulForgePerformanceMode.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgePerformanceMode>()
{
	return ESoulForgePerformanceMode_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "Balanced.DisplayName", "Equilibrado (Autom\xc3\xa1tico)" },
		{ "Balanced.Name", "ESoulForgePerformanceMode::Balanced" },
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Modos de rendimiento para el usuario\n" },
#endif
		{ "Eco.DisplayName", "Ahorro (1-2 N\xc3\xba""cleos)" },
		{ "Eco.Name", "ESoulForgePerformanceMode::Eco" },
		{ "Insight.DisplayName", "Extreme Insight (GPU + Todos los N\xc3\xba""cleos)" },
		{ "Insight.Name", "ESoulForgePerformanceMode::Insight" },
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Modos de rendimiento para el usuario" },
#endif
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "ESoulForgePerformanceMode::Eco", (int64)ESoulForgePerformanceMode::Eco },
		{ "ESoulForgePerformanceMode::Balanced", (int64)ESoulForgePerformanceMode::Balanced },
		{ "ESoulForgePerformanceMode::Insight", (int64)ESoulForgePerformanceMode::Insight },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"ESoulForgePerformanceMode",
	"ESoulForgePerformanceMode",
	Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode()
{
	if (!Z_Registration_Info_UEnum_ESoulForgePerformanceMode.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_ESoulForgePerformanceMode.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_ESoulForgePerformanceMode.InnerSingleton;
}
// ********** End Enum ESoulForgePerformanceMode ***************************************************

// ********** Begin Class USoulForgeSubsystem Function ApplyPerformanceSettings ********************
struct Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Env\xc3\xad""a los ajustes de rendimiento a la DLL de Rust. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Env\xc3\xad""a los ajustes de rendimiento a la DLL de Rust." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ApplyPerformanceSettings constinit property declarations **************
// ********** End Function ApplyPerformanceSettings constinit property declarations ****************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeSubsystem, nullptr, "ApplyPerformanceSettings", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeSubsystem::execApplyPerformanceSettings)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ApplyPerformanceSettings();
	P_NATIVE_END;
}
// ********** End Class USoulForgeSubsystem Function ApplyPerformanceSettings **********************

// ********** Begin Class USoulForgeSubsystem Function EmergencyReset ******************************
struct Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \n     * Bot\xc3\xb3n de P\xc3\xa1nico: Limpia Rust y Unreal a la vez.\n     */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Bot\xc3\xb3n de P\xc3\xa1nico: Limpia Rust y Unreal a la vez." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function EmergencyReset constinit property declarations ************************
// ********** End Function EmergencyReset constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeSubsystem, nullptr, "EmergencyReset", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeSubsystem::execEmergencyReset)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->EmergencyReset();
	P_NATIVE_END;
}
// ********** End Class USoulForgeSubsystem Function EmergencyReset ********************************

// ********** Begin Class USoulForgeSubsystem Function SyncGlobalPower *****************************
struct Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Global" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Sincroniza la escala de energ\xc3\xad""a con el n\xc3\xba""cleo de Rust. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Sincroniza la escala de energ\xc3\xad""a con el n\xc3\xba""cleo de Rust." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SyncGlobalPower constinit property declarations ***********************
// ********** End Function SyncGlobalPower constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeSubsystem, nullptr, "SyncGlobalPower", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04020401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeSubsystem::execSyncGlobalPower)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->SyncGlobalPower();
	P_NATIVE_END;
}
// ********** End Class USoulForgeSubsystem Function SyncGlobalPower *******************************

// ********** Begin Class USoulForgeSubsystem ******************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeSubsystem;
UClass* USoulForgeSubsystem::GetPrivateStaticClass()
{
	using TClass = USoulForgeSubsystem;
	if (!Z_Registration_Info_UClass_USoulForgeSubsystem.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeSubsystem"),
			Z_Registration_Info_UClass_USoulForgeSubsystem.InnerSingleton,
			StaticRegisterNativesUSoulForgeSubsystem,
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
	return Z_Registration_Info_UClass_USoulForgeSubsystem.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeSubsystem_NoRegister()
{
	return USoulForgeSubsystem::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeSubsystem_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
#if !UE_BUILD_SHIPPING
		{ "Comment", "/**\n * Cerebro Global de SoulForge.\n * Gestiona el Modo Dios, el rendimiento y la limpieza del motor.\n */" },
#endif
		{ "IncludePath", "SoulForgeSubsystem.h" },
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Cerebro Global de SoulForge.\nGestiona el Modo Dios, el rendimiento y la limpieza del motor." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_GlobalEnergyScale_MetaData[] = {
		{ "Category", "SoulForge|Global" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Escala global de energ\xc3\xad""a para todas las explosiones. */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Escala global de energ\xc3\xad""a para todas las explosiones." },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PerformanceMode_MetaData[] = {
		{ "Category", "SoulForge|Performance" },
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bEnableGPUAcceleration_MetaData[] = {
		{ "Category", "SoulForge|Performance" },
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bDrawDebugIDs_MetaData[] = {
		{ "Category", "SoulForge|Debug" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Si true, se dibujar\xc3\xa1 telemetr\xc3\xad""a visual de IDs en el mundo (pr\xc3\xb3ximamente). */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeSubsystem.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Si true, se dibujar\xc3\xa1 telemetr\xc3\xad""a visual de IDs en el mundo (pr\xc3\xb3ximamente)." },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeSubsystem constinit property declarations **********************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_GlobalEnergyScale;
	static const UECodeGen_Private::FBytePropertyParams NewProp_PerformanceMode_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_PerformanceMode;
	static void NewProp_bEnableGPUAcceleration_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bEnableGPUAcceleration;
	static void NewProp_bDrawDebugIDs_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bDrawDebugIDs;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Class USoulForgeSubsystem constinit property declarations ************************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ApplyPerformanceSettings"), .Pointer = &USoulForgeSubsystem::execApplyPerformanceSettings },
		{ .NameUTF8 = UTF8TEXT("EmergencyReset"), .Pointer = &USoulForgeSubsystem::execEmergencyReset },
		{ .NameUTF8 = UTF8TEXT("SyncGlobalPower"), .Pointer = &USoulForgeSubsystem::execSyncGlobalPower },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeSubsystem_ApplyPerformanceSettings, "ApplyPerformanceSettings" }, // 1119065234
		{ &Z_Construct_UFunction_USoulForgeSubsystem_EmergencyReset, "EmergencyReset" }, // 2658754273
		{ &Z_Construct_UFunction_USoulForgeSubsystem_SyncGlobalPower, "SyncGlobalPower" }, // 2654485696
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeSubsystem>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeSubsystem_Statics

// ********** Begin Class USoulForgeSubsystem Property Definitions *********************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_GlobalEnergyScale = { "GlobalEnergyScale", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeSubsystem, GlobalEnergyScale), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_GlobalEnergyScale_MetaData), NewProp_GlobalEnergyScale_MetaData) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_PerformanceMode_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_PerformanceMode = { "PerformanceMode", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(USoulForgeSubsystem, PerformanceMode), Z_Construct_UEnum_SoulForgePhysX_ESoulForgePerformanceMode, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PerformanceMode_MetaData), NewProp_PerformanceMode_MetaData) }; // 1693152317
void Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bEnableGPUAcceleration_SetBit(void* Obj)
{
	((USoulForgeSubsystem*)Obj)->bEnableGPUAcceleration = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bEnableGPUAcceleration = { "bEnableGPUAcceleration", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeSubsystem), &Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bEnableGPUAcceleration_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bEnableGPUAcceleration_MetaData), NewProp_bEnableGPUAcceleration_MetaData) };
void Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bDrawDebugIDs_SetBit(void* Obj)
{
	((USoulForgeSubsystem*)Obj)->bDrawDebugIDs = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bDrawDebugIDs = { "bDrawDebugIDs", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(USoulForgeSubsystem), &Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bDrawDebugIDs_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bDrawDebugIDs_MetaData), NewProp_bDrawDebugIDs_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UClass_USoulForgeSubsystem_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_GlobalEnergyScale,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_PerformanceMode_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_PerformanceMode,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bEnableGPUAcceleration,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_USoulForgeSubsystem_Statics::NewProp_bDrawDebugIDs,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeSubsystem_Statics::PropPointers) < 2048);
// ********** End Class USoulForgeSubsystem Property Definitions ***********************************
UObject* (*const Z_Construct_UClass_USoulForgeSubsystem_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UWorldSubsystem,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeSubsystem_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeSubsystem_Statics::ClassParams = {
	&USoulForgeSubsystem::StaticClass,
	nullptr,
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	Z_Construct_UClass_USoulForgeSubsystem_Statics::PropPointers,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeSubsystem_Statics::PropPointers),
	0,
	0x001000A0u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeSubsystem_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeSubsystem_Statics::Class_MetaDataParams)
};
void USoulForgeSubsystem::StaticRegisterNativesUSoulForgeSubsystem()
{
	UClass* Class = USoulForgeSubsystem::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeSubsystem_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeSubsystem()
{
	if (!Z_Registration_Info_UClass_USoulForgeSubsystem.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeSubsystem.OuterSingleton, Z_Construct_UClass_USoulForgeSubsystem_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeSubsystem.OuterSingleton;
}
USoulForgeSubsystem::USoulForgeSubsystem() {}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeSubsystem);
USoulForgeSubsystem::~USoulForgeSubsystem() {}
// ********** End Class USoulForgeSubsystem ********************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics
{
	static constexpr FEnumRegisterCompiledInInfo EnumInfo[] = {
		{ ESoulForgePerformanceMode_StaticEnum, TEXT("ESoulForgePerformanceMode"), &Z_Registration_Info_UEnum_ESoulForgePerformanceMode, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 1693152317U) },
	};
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeSubsystem, USoulForgeSubsystem::StaticClass, TEXT("USoulForgeSubsystem"), &Z_Registration_Info_UClass_USoulForgeSubsystem, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeSubsystem), 3807666089U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_4161683118{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics::ClassInfo),
	nullptr, 0,
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics::EnumInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeSubsystem_h__Script_SoulForgePhysX_Statics::EnumInfo),
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
