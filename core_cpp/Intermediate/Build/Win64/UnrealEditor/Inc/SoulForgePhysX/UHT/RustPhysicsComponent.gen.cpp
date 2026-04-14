// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "RustPhysicsComponent.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeRustPhysicsComponent() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
ENGINE_API UClass* Z_Construct_UClass_UActorComponent();
ENGINE_API UClass* Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister();
ENGINE_API UClass* Z_Construct_UClass_UStaticMesh_NoRegister();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_URustPhysicsComponent();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_URustPhysicsComponent_NoRegister();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FRenderDataOptimized();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin ScriptStruct FRenderDataOptimized **********************************************
struct Z_Construct_UScriptStruct_FRenderDataOptimized_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FRenderDataOptimized); }
	static inline consteval int16 GetStructAlignment() { return alignof(FRenderDataOptimized); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// Datos de renderizado optimizados (Debe coincidir con ffi.rs)\n" },
#endif
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Datos de renderizado optimizados (Debe coincidir con ffi.rs)" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Count_MetaData[] = {
		{ "Category", "RenderDataOptimized" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FRenderDataOptimized constinit property declarations **************
	static const UECodeGen_Private::FIntPropertyParams NewProp_Count;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FRenderDataOptimized constinit property declarations ****************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FRenderDataOptimized>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FRenderDataOptimized_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FRenderDataOptimized;
class UScriptStruct* FRenderDataOptimized::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FRenderDataOptimized.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FRenderDataOptimized.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FRenderDataOptimized, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("RenderDataOptimized"));
	}
	return Z_Registration_Info_UScriptStruct_FRenderDataOptimized.OuterSingleton;
	}

// ********** Begin ScriptStruct FRenderDataOptimized Property Definitions *************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::NewProp_Count = { "Count", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FRenderDataOptimized, Count), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Count_MetaData), NewProp_Count_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::NewProp_Count,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FRenderDataOptimized Property Definitions ***************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"RenderDataOptimized",
	Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::PropPointers),
	sizeof(FRenderDataOptimized),
	alignof(FRenderDataOptimized),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FRenderDataOptimized()
{
	if (!Z_Registration_Info_UScriptStruct_FRenderDataOptimized.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FRenderDataOptimized.InnerSingleton, Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FRenderDataOptimized.InnerSingleton);
}
// ********** End ScriptStruct FRenderDataOptimized ************************************************

// ********** Begin Class URustPhysicsComponent Function ActivateKamehameha ************************
struct Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics
{
	struct RustPhysicsComponent_eventActivateKamehameha_Parms
	{
		FVector Origin;
		FVector Direction;
		float Intensity;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Powers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// === BLUEPRINT API ===\n" },
#endif
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "=== BLUEPRINT API ===" },
#endif
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
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventActivateKamehameha_Parms, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventActivateKamehameha_Parms, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventActivateKamehameha_Parms, Intensity), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::NewProp_Intensity,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::PropPointers) < 2048);
// ********** End Function ActivateKamehameha Property Definitions *********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_URustPhysicsComponent, nullptr, "ActivateKamehameha", 	Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::RustPhysicsComponent_eventActivateKamehameha_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::Function_MetaDataParams), Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::RustPhysicsComponent_eventActivateKamehameha_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(URustPhysicsComponent::execActivateKamehameha)
{
	P_GET_STRUCT(FVector,Z_Param_Origin);
	P_GET_STRUCT(FVector,Z_Param_Direction);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Intensity);
	P_FINISH;
	P_NATIVE_BEGIN;
	P_THIS->ActivateKamehameha(Z_Param_Origin,Z_Param_Direction,Z_Param_Intensity);
	P_NATIVE_END;
}
// ********** End Class URustPhysicsComponent Function ActivateKamehameha **************************

// ********** Begin Class URustPhysicsComponent Function SpawnNodeGrid *****************************
struct Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics
{
	struct RustPhysicsComponent_eventSpawnNodeGrid_Parms
	{
		FVector Center;
		int32 Width;
		int32 Height;
		int32 Depth;
		float Spacing;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Debug" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
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
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Center = { "Center", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventSpawnNodeGrid_Parms, Center), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Width = { "Width", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventSpawnNodeGrid_Parms, Width), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Height = { "Height", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventSpawnNodeGrid_Parms, Height), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Depth = { "Depth", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventSpawnNodeGrid_Parms, Depth), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Spacing = { "Spacing", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(RustPhysicsComponent_eventSpawnNodeGrid_Parms, Spacing), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Center,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Width,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Height,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Depth,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::NewProp_Spacing,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::PropPointers) < 2048);
// ********** End Function SpawnNodeGrid Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_URustPhysicsComponent, nullptr, "SpawnNodeGrid", 	Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::RustPhysicsComponent_eventSpawnNodeGrid_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04820401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::Function_MetaDataParams), Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::RustPhysicsComponent_eventSpawnNodeGrid_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(URustPhysicsComponent::execSpawnNodeGrid)
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
// ********** End Class URustPhysicsComponent Function SpawnNodeGrid *******************************

// ********** Begin Class URustPhysicsComponent ****************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_URustPhysicsComponent;
UClass* URustPhysicsComponent::GetPrivateStaticClass()
{
	using TClass = URustPhysicsComponent;
	if (!Z_Registration_Info_UClass_URustPhysicsComponent.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("RustPhysicsComponent"),
			Z_Registration_Info_UClass_URustPhysicsComponent.InnerSingleton,
			StaticRegisterNativesURustPhysicsComponent,
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
	return Z_Registration_Info_UClass_URustPhysicsComponent.InnerSingleton;
}
UClass* Z_Construct_UClass_URustPhysicsComponent_NoRegister()
{
	return URustPhysicsComponent::GetPrivateStaticClass();
}
struct Z_Construct_UClass_URustPhysicsComponent_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "BlueprintSpawnableComponent", "" },
		{ "ClassGroupNames", "SoulForge" },
		{ "IncludePath", "RustPhysicsComponent.h" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_MaxNodes_MetaData[] = {
		{ "Category", "SoulForge|Rust Physics" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// === CONFIGURACI\xc3\x93N ===\n" },
#endif
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "=== CONFIGURACI\xc3\x93N ===" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_NodeMesh_MetaData[] = {
		{ "Category", "SoulForge|Rust Physics" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Gravity_MetaData[] = {
		{ "Category", "SoulForge|Rust Physics" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_bUseInstancedRendering_MetaData[] = {
		{ "Category", "SoulForge|Rust Physics" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_InstancedMesh_MetaData[] = {
		{ "EditInline", "true" },
		{ "ModuleRelativePath", "Public/RustPhysicsComponent.h" },
	};
#endif // WITH_METADATA

// ********** Begin Class URustPhysicsComponent constinit property declarations ********************
	static const UECodeGen_Private::FIntPropertyParams NewProp_MaxNodes;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_NodeMesh;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Gravity;
	static void NewProp_bUseInstancedRendering_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bUseInstancedRendering;
	static const UECodeGen_Private::FObjectPropertyParams NewProp_InstancedMesh;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Class URustPhysicsComponent constinit property declarations **********************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ActivateKamehameha"), .Pointer = &URustPhysicsComponent::execActivateKamehameha },
		{ .NameUTF8 = UTF8TEXT("SpawnNodeGrid"), .Pointer = &URustPhysicsComponent::execSpawnNodeGrid },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_URustPhysicsComponent_ActivateKamehameha, "ActivateKamehameha" }, // 1334650349
		{ &Z_Construct_UFunction_URustPhysicsComponent_SpawnNodeGrid, "SpawnNodeGrid" }, // 2471385308
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<URustPhysicsComponent>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_URustPhysicsComponent_Statics

// ********** Begin Class URustPhysicsComponent Property Definitions *******************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_MaxNodes = { "MaxNodes", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(URustPhysicsComponent, MaxNodes), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_MaxNodes_MetaData), NewProp_MaxNodes_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_NodeMesh = { "NodeMesh", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(URustPhysicsComponent, NodeMesh), Z_Construct_UClass_UStaticMesh_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_NodeMesh_MetaData), NewProp_NodeMesh_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_Gravity = { "Gravity", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(URustPhysicsComponent, Gravity), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Gravity_MetaData), NewProp_Gravity_MetaData) };
void Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_bUseInstancedRendering_SetBit(void* Obj)
{
	((URustPhysicsComponent*)Obj)->bUseInstancedRendering = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_bUseInstancedRendering = { "bUseInstancedRendering", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(URustPhysicsComponent), &Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_bUseInstancedRendering_SetBit, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_bUseInstancedRendering_MetaData), NewProp_bUseInstancedRendering_MetaData) };
const UECodeGen_Private::FObjectPropertyParams Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_InstancedMesh = { "InstancedMesh", nullptr, (EPropertyFlags)0x0040000000080008, UECodeGen_Private::EPropertyGenFlags::Object, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(URustPhysicsComponent, InstancedMesh), Z_Construct_UClass_UInstancedStaticMeshComponent_NoRegister, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_InstancedMesh_MetaData), NewProp_InstancedMesh_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UClass_URustPhysicsComponent_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_MaxNodes,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_NodeMesh,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_Gravity,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_bUseInstancedRendering,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UClass_URustPhysicsComponent_Statics::NewProp_InstancedMesh,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_URustPhysicsComponent_Statics::PropPointers) < 2048);
// ********** End Class URustPhysicsComponent Property Definitions *********************************
UObject* (*const Z_Construct_UClass_URustPhysicsComponent_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UActorComponent,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_URustPhysicsComponent_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_URustPhysicsComponent_Statics::ClassParams = {
	&URustPhysicsComponent::StaticClass,
	"Engine",
	&StaticCppClassTypeInfo,
	DependentSingletons,
	FuncInfo,
	Z_Construct_UClass_URustPhysicsComponent_Statics::PropPointers,
	nullptr,
	UE_ARRAY_COUNT(DependentSingletons),
	UE_ARRAY_COUNT(FuncInfo),
	UE_ARRAY_COUNT(Z_Construct_UClass_URustPhysicsComponent_Statics::PropPointers),
	0,
	0x00B000A4u,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_URustPhysicsComponent_Statics::Class_MetaDataParams), Z_Construct_UClass_URustPhysicsComponent_Statics::Class_MetaDataParams)
};
void URustPhysicsComponent::StaticRegisterNativesURustPhysicsComponent()
{
	UClass* Class = URustPhysicsComponent::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_URustPhysicsComponent_Statics::Funcs));
}
UClass* Z_Construct_UClass_URustPhysicsComponent()
{
	if (!Z_Registration_Info_UClass_URustPhysicsComponent.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_URustPhysicsComponent.OuterSingleton, Z_Construct_UClass_URustPhysicsComponent_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_URustPhysicsComponent.OuterSingleton;
}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, URustPhysicsComponent);
URustPhysicsComponent::~URustPhysicsComponent() {}
// ********** End Class URustPhysicsComponent ******************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics
{
	static constexpr FStructRegisterCompiledInInfo ScriptStructInfo[] = {
		{ FRenderDataOptimized::StaticStruct, Z_Construct_UScriptStruct_FRenderDataOptimized_Statics::NewStructOps, TEXT("RenderDataOptimized"),&Z_Registration_Info_UScriptStruct_FRenderDataOptimized, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FRenderDataOptimized), 3316139445U) },
	};
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_URustPhysicsComponent, URustPhysicsComponent::StaticClass, TEXT("URustPhysicsComponent"), &Z_Registration_Info_UClass_URustPhysicsComponent, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(URustPhysicsComponent), 2822824953U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_75717033{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics::ClassInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics::ScriptStructInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_RustPhysicsComponent_h__Script_SoulForgePhysX_Statics::ScriptStructInfo),
	nullptr, 0,
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
