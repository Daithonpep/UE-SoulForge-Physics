// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
#include "SoulForgeBridge.h"

PRAGMA_DISABLE_DEPRECATION_WARNINGS
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");
void EmptyLinkFunctionForGeneratedCodeSoulForgeBridge() {}

// ********** Begin Cross Module References ********************************************************
COREUOBJECT_API UClass* Z_Construct_UClass_UObject();
COREUOBJECT_API UScriptStruct* Z_Construct_UScriptStruct_FVector();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeBridge();
SOULFORGEPHYSX_API UClass* Z_Construct_UClass_USoulForgeBridge_NoRegister();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EExplosiveType();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EMaterialType();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_EPowerType();
SOULFORGEPHYSX_API UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset();
SOULFORGEPHYSX_API UFunction* Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FActivePowerInstance();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FActivePowerPayload();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FEngineState();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FRenderData();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FShockwaveData();
SOULFORGEPHYSX_API UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeFragmentData();
UPackage* Z_Construct_UPackage__Script_SoulForgePhysX();
// ********** End Cross Module References **********************************************************

// ********** Begin Enum ESoulForgeDamagePreset ****************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_ESoulForgeDamagePreset;
static UEnum* ESoulForgeDamagePreset_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_ESoulForgeDamagePreset.OuterSingleton)
	{
		Z_Registration_Info_UEnum_ESoulForgeDamagePreset.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ESoulForgeDamagePreset"));
	}
	return Z_Registration_Info_UEnum_ESoulForgeDamagePreset.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<ESoulForgeDamagePreset>()
{
	return ESoulForgeDamagePreset_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "BlueprintType", "true" },
		{ "Collapse.DisplayName", "Collapse (Gravity)" },
		{ "Collapse.Name", "ESoulForgeDamagePreset::Collapse" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Presets de da\xc3\xb1o disponibles para DestroyProxy. */" },
#endif
		{ "Explosion.DisplayName", "Explosion (Radial)" },
		{ "Explosion.Name", "ESoulForgeDamagePreset::Explosion" },
		{ "Impact.DisplayName", "Impact (Cone)" },
		{ "Impact.Name", "ESoulForgeDamagePreset::Impact" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
		{ "RealityShatter.DisplayName", "Reality Shatter (Epic)" },
		{ "RealityShatter.Name", "ESoulForgeDamagePreset::RealityShatter" },
		{ "ShapedCharge.DisplayName", "Shaped Charge (Penetrator)" },
		{ "ShapedCharge.Name", "ESoulForgeDamagePreset::ShapedCharge" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Presets de da\xc3\xb1o disponibles para DestroyProxy." },
#endif
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "ESoulForgeDamagePreset::Explosion", (int64)ESoulForgeDamagePreset::Explosion },
		{ "ESoulForgeDamagePreset::Collapse", (int64)ESoulForgeDamagePreset::Collapse },
		{ "ESoulForgeDamagePreset::Impact", (int64)ESoulForgeDamagePreset::Impact },
		{ "ESoulForgeDamagePreset::ShapedCharge", (int64)ESoulForgeDamagePreset::ShapedCharge },
		{ "ESoulForgeDamagePreset::RealityShatter", (int64)ESoulForgeDamagePreset::RealityShatter },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"ESoulForgeDamagePreset",
	"ESoulForgeDamagePreset",
	Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset()
{
	if (!Z_Registration_Info_UEnum_ESoulForgeDamagePreset.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_ESoulForgeDamagePreset.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_ESoulForgeDamagePreset_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_ESoulForgeDamagePreset.InnerSingleton;
}
// ********** End Enum ESoulForgeDamagePreset ******************************************************

// ********** Begin Enum EPhysicsLOD ***************************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_EPhysicsLOD;
static UEnum* EPhysicsLOD_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_EPhysicsLOD.OuterSingleton)
	{
		Z_Registration_Info_UEnum_EPhysicsLOD.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("EPhysicsLOD"));
	}
	return Z_Registration_Info_UEnum_EPhysicsLOD.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EPhysicsLOD>()
{
	return EPhysicsLOD_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "BlueprintType", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
		{ "Primary.DisplayName", "Primario (Grady-Kipp)" },
		{ "Primary.Name", "EPhysicsLOD::Primary" },
		{ "Secondary.DisplayName", "Secundario (Gurney)" },
		{ "Secondary.Name", "EPhysicsLOD::Secondary" },
		{ "Tertiary.DisplayName", "Terciario (Cinematogr\xc3\xa1""fico)" },
		{ "Tertiary.Name", "EPhysicsLOD::Tertiary" },
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "EPhysicsLOD::Primary", (int64)EPhysicsLOD::Primary },
		{ "EPhysicsLOD::Secondary", (int64)EPhysicsLOD::Secondary },
		{ "EPhysicsLOD::Tertiary", (int64)EPhysicsLOD::Tertiary },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"EPhysicsLOD",
	"EPhysicsLOD",
	Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD()
{
	if (!Z_Registration_Info_UEnum_EPhysicsLOD.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_EPhysicsLOD.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_EPhysicsLOD.InnerSingleton;
}
// ********** End Enum EPhysicsLOD *****************************************************************

// ********** Begin Enum EMaterialType *************************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_EMaterialType;
static UEnum* EMaterialType_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_EMaterialType.OuterSingleton)
	{
		Z_Registration_Info_UEnum_EMaterialType.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_EMaterialType, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("EMaterialType"));
	}
	return Z_Registration_Info_UEnum_EMaterialType.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EMaterialType>()
{
	return EMaterialType_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "Aluminum6061.DisplayName", "Aluminio 6061" },
		{ "Aluminum6061.Name", "EMaterialType::Aluminum6061" },
		{ "BlueprintType", "true" },
		{ "Concrete35MPa.DisplayName", "Concreto 35MPa" },
		{ "Concrete35MPa.Name", "EMaterialType::Concrete35MPa" },
		{ "GlassTempered.DisplayName", "Vidrio Templado" },
		{ "GlassTempered.Name", "EMaterialType::GlassTempered" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
		{ "Steel4340.DisplayName", "Acero 4340 (Militar)" },
		{ "Steel4340.Name", "EMaterialType::Steel4340" },
		{ "WoodOak.DisplayName", "Madera de Roble" },
		{ "WoodOak.Name", "EMaterialType::WoodOak" },
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "EMaterialType::Steel4340", (int64)EMaterialType::Steel4340 },
		{ "EMaterialType::Aluminum6061", (int64)EMaterialType::Aluminum6061 },
		{ "EMaterialType::Concrete35MPa", (int64)EMaterialType::Concrete35MPa },
		{ "EMaterialType::WoodOak", (int64)EMaterialType::WoodOak },
		{ "EMaterialType::GlassTempered", (int64)EMaterialType::GlassTempered },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"EMaterialType",
	"EMaterialType",
	Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_EMaterialType()
{
	if (!Z_Registration_Info_UEnum_EMaterialType.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_EMaterialType.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_EMaterialType_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_EMaterialType.InnerSingleton;
}
// ********** End Enum EMaterialType ***************************************************************

// ********** Begin Enum EExplosiveType ************************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_EExplosiveType;
static UEnum* EExplosiveType_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_EExplosiveType.OuterSingleton)
	{
		Z_Registration_Info_UEnum_EExplosiveType.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_EExplosiveType, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("EExplosiveType"));
	}
	return Z_Registration_Info_UEnum_EExplosiveType.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EExplosiveType>()
{
	return EExplosiveType_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "AgujeroNegro.DisplayName", "Agujero Negro (Implosi\xc3\xb3n \xc3\x89pica)" },
		{ "AgujeroNegro.Name", "EExplosiveType::AgujeroNegro" },
		{ "BlueprintType", "true" },
		{ "Collapse.DisplayName", "Colapso (Gravedad Real)" },
		{ "Collapse.Name", "EExplosiveType::Collapse" },
		{ "Matrix.DisplayName", "Matrix (Gravedad Cero)" },
		{ "Matrix.Name", "EExplosiveType::Matrix" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
		{ "PETN.DisplayName", "PETN (Carga Hueca T\xc3\xa1""ctica)" },
		{ "PETN.Name", "EExplosiveType::PETN" },
		{ "RDX.DisplayName", "RDX (Ca\xc3\xb1\xc3\xb3n de Riel)" },
		{ "RDX.Name", "EExplosiveType::RDX" },
		{ "RealityShatter.DisplayName", "Realidad Fracturada (\xc3\x89pico)" },
		{ "RealityShatter.Name", "EExplosiveType::RealityShatter" },
		{ "TNT.DisplayName", "TNT (Explosi\xc3\xb3n Convencional)" },
		{ "TNT.Name", "EExplosiveType::TNT" },
		{ "Vortex.DisplayName", "V\xc3\xb3rtice (Tornado)" },
		{ "Vortex.Name", "EExplosiveType::Vortex" },
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "EExplosiveType::TNT", (int64)EExplosiveType::TNT },
		{ "EExplosiveType::AgujeroNegro", (int64)EExplosiveType::AgujeroNegro },
		{ "EExplosiveType::RDX", (int64)EExplosiveType::RDX },
		{ "EExplosiveType::PETN", (int64)EExplosiveType::PETN },
		{ "EExplosiveType::Matrix", (int64)EExplosiveType::Matrix },
		{ "EExplosiveType::Collapse", (int64)EExplosiveType::Collapse },
		{ "EExplosiveType::RealityShatter", (int64)EExplosiveType::RealityShatter },
		{ "EExplosiveType::Vortex", (int64)EExplosiveType::Vortex },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"EExplosiveType",
	"EExplosiveType",
	Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_EExplosiveType()
{
	if (!Z_Registration_Info_UEnum_EExplosiveType.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_EExplosiveType.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_EExplosiveType_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_EExplosiveType.InnerSingleton;
}
// ********** End Enum EExplosiveType **************************************************************

// ********** Begin Enum EPowerType ****************************************************************
static FEnumRegistrationInfo Z_Registration_Info_UEnum_EPowerType;
static UEnum* EPowerType_StaticEnum()
{
	if (!Z_Registration_Info_UEnum_EPowerType.OuterSingleton)
	{
		Z_Registration_Info_UEnum_EPowerType.OuterSingleton = GetStaticEnum(Z_Construct_UEnum_SoulForgePhysX_EPowerType, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("EPowerType"));
	}
	return Z_Registration_Info_UEnum_EPowerType.OuterSingleton;
}
template<> SOULFORGEPHYSX_NON_ATTRIBUTED_API UEnum* StaticEnum<EPowerType>()
{
	return EPowerType_StaticEnum();
}
struct Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Enum_MetaDataParams[] = {
		{ "AirBending.DisplayName", "Control de Aire" },
		{ "AirBending.Name", "EPowerType::AirBending" },
		{ "Blizzard.DisplayName", "Ventisca \xc3\x81rtica" },
		{ "Blizzard.Name", "EPowerType::Blizzard" },
		{ "BlueprintType", "true" },
		{ "EarthBending.DisplayName", "Control de Tierra" },
		{ "EarthBending.Name", "EPowerType::EarthBending" },
		{ "Electrocution.DisplayName", "Electrocuci\xc3\xb3n Masiva" },
		{ "Electrocution.Name", "EPowerType::Electrocution" },
		{ "EnergyBeam.DisplayName", "Kamehameha / Rayo de Energ\xc3\xad""a" },
		{ "EnergyBeam.Name", "EPowerType::EnergyBeam" },
		{ "FireBending.DisplayName", "Piroquinesis" },
		{ "FireBending.Name", "EPowerType::FireBending" },
		{ "FireStorm.Comment", "// Combos\n" },
		{ "FireStorm.DisplayName", "Tormenta de Fuego" },
		{ "FireStorm.Name", "EPowerType::FireStorm" },
		{ "FireStorm.ToolTip", "Combos" },
		{ "GravityBlackHole.DisplayName", "Agujero Negro" },
		{ "GravityBlackHole.Name", "EPowerType::GravityBlackHole" },
		{ "GravityZero.DisplayName", "Gravedad Cero" },
		{ "GravityZero.Name", "EPowerType::GravityZero" },
		{ "IceControl.DisplayName", "Control de Hielo" },
		{ "IceControl.Name", "EPowerType::IceControl" },
		{ "LavaBurst.DisplayName", "Explosi\xc3\xb3n de Lava" },
		{ "LavaBurst.Name", "EPowerType::LavaBurst" },
		{ "Lightning.DisplayName", "Rayo / Electricidad" },
		{ "Lightning.Name", "EPowerType::Lightning" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
		{ "Supernova.DisplayName", "Supernova Gravitacional" },
		{ "Supernova.Name", "EPowerType::Supernova" },
		{ "WaterBending.DisplayName", "Control de Agua" },
		{ "WaterBending.Name", "EPowerType::WaterBending" },
	};
#endif // WITH_METADATA
	static constexpr UECodeGen_Private::FEnumeratorParam Enumerators[] = {
		{ "EPowerType::EnergyBeam", (int64)EPowerType::EnergyBeam },
		{ "EPowerType::WaterBending", (int64)EPowerType::WaterBending },
		{ "EPowerType::FireBending", (int64)EPowerType::FireBending },
		{ "EPowerType::EarthBending", (int64)EPowerType::EarthBending },
		{ "EPowerType::AirBending", (int64)EPowerType::AirBending },
		{ "EPowerType::Lightning", (int64)EPowerType::Lightning },
		{ "EPowerType::GravityZero", (int64)EPowerType::GravityZero },
		{ "EPowerType::GravityBlackHole", (int64)EPowerType::GravityBlackHole },
		{ "EPowerType::IceControl", (int64)EPowerType::IceControl },
		{ "EPowerType::FireStorm", (int64)EPowerType::FireStorm },
		{ "EPowerType::Electrocution", (int64)EPowerType::Electrocution },
		{ "EPowerType::LavaBurst", (int64)EPowerType::LavaBurst },
		{ "EPowerType::Blizzard", (int64)EPowerType::Blizzard },
		{ "EPowerType::Supernova", (int64)EPowerType::Supernova },
	};
	static const UECodeGen_Private::FEnumParams EnumParams;
}; // struct Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics 
const UECodeGen_Private::FEnumParams Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::EnumParams = {
	(UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	"EPowerType",
	"EPowerType",
	Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::Enumerators,
	RF_Public|RF_Transient|RF_MarkAsNative,
	UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::Enumerators),
	EEnumFlags::None,
	(uint8)UEnum::ECppForm::EnumClass,
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::Enum_MetaDataParams), Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::Enum_MetaDataParams)
};
UEnum* Z_Construct_UEnum_SoulForgePhysX_EPowerType()
{
	if (!Z_Registration_Info_UEnum_EPowerType.InnerSingleton)
	{
		UECodeGen_Private::ConstructUEnum(Z_Registration_Info_UEnum_EPowerType.InnerSingleton, Z_Construct_UEnum_SoulForgePhysX_EPowerType_Statics::EnumParams);
	}
	return Z_Registration_Info_UEnum_EPowerType.InnerSingleton;
}
// ********** End Enum EPowerType ******************************************************************

// ********** Begin ScriptStruct FSoulForgeFragmentData ********************************************
struct Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FSoulForgeFragmentData); }
	static inline consteval int16 GetStructAlignment() { return alignof(FSoulForgeFragmentData); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// ESTRUCTURA FFI PURA (SIN REFLEXI\xc3\x93N DE UNREAL PARA EVITAR VTABLE / PADDING)\n" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "ESTRUCTURA FFI PURA (SIN REFLEXI\xc3\x93N DE UNREAL PARA EVITAR VTABLE / PADDING)" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_X_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Y_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Z_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Pitch_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Yaw_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Roll_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ScaleX_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ScaleY_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ScaleZ_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Category_MetaData[] = {
		{ "Category", "SoulForgeFragmentData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FSoulForgeFragmentData constinit property declarations ************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_X;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Y;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Z;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Pitch;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Yaw;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Roll;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ScaleX;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ScaleY;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ScaleZ;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Category;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FSoulForgeFragmentData constinit property declarations **************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FSoulForgeFragmentData>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData;
class UScriptStruct* FSoulForgeFragmentData::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FSoulForgeFragmentData, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("SoulForgeFragmentData"));
	}
	return Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.OuterSingleton;
	}

// ********** Begin ScriptStruct FSoulForgeFragmentData Property Definitions ***********************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_X = { "X", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, X), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_X_MetaData), NewProp_X_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Y = { "Y", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Y), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Y_MetaData), NewProp_Y_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Z = { "Z", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Z), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Z_MetaData), NewProp_Z_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Pitch = { "Pitch", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Pitch), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Pitch_MetaData), NewProp_Pitch_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Yaw = { "Yaw", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Yaw), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Yaw_MetaData), NewProp_Yaw_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Roll = { "Roll", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Roll), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Roll_MetaData), NewProp_Roll_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleX = { "ScaleX", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, ScaleX), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ScaleX_MetaData), NewProp_ScaleX_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleY = { "ScaleY", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, ScaleY), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ScaleY_MetaData), NewProp_ScaleY_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleZ = { "ScaleZ", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, ScaleZ), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ScaleZ_MetaData), NewProp_ScaleZ_MetaData) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Category = { "Category", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FSoulForgeFragmentData, Category), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Category_MetaData), NewProp_Category_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_X,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Y,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Z,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Pitch,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Yaw,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Roll,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleX,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleY,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_ScaleZ,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewProp_Category,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FSoulForgeFragmentData Property Definitions *************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"SoulForgeFragmentData",
	Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::PropPointers),
	sizeof(FSoulForgeFragmentData),
	alignof(FSoulForgeFragmentData),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FSoulForgeFragmentData()
{
	if (!Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.InnerSingleton, Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData.InnerSingleton);
}
// ********** End ScriptStruct FSoulForgeFragmentData **********************************************

// ********** Begin ScriptStruct FEngineState ******************************************************
struct Z_Construct_UScriptStruct_FEngineState_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FEngineState); }
	static inline consteval int16 GetStructAlignment() { return alignof(FEngineState); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_ActiveNodes_MetaData[] = {
		{ "Category", "Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_FPS_MetaData[] = {
		{ "Category", "Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Efficiency_MetaData[] = {
		{ "Category", "Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_SimTime_MetaData[] = {
		{ "Category", "Stats" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FEngineState constinit property declarations **********************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ActiveNodes;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_FPS;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Efficiency;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_SimTime;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FEngineState constinit property declarations ************************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FEngineState>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FEngineState_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FEngineState;
class UScriptStruct* FEngineState::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FEngineState.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FEngineState.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FEngineState, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("EngineState"));
	}
	return Z_Registration_Info_UScriptStruct_FEngineState.OuterSingleton;
	}

// ********** Begin ScriptStruct FEngineState Property Definitions *********************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_ActiveNodes = { "ActiveNodes", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FEngineState, ActiveNodes), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_ActiveNodes_MetaData), NewProp_ActiveNodes_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_FPS = { "FPS", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FEngineState, FPS), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_FPS_MetaData), NewProp_FPS_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_Efficiency = { "Efficiency", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FEngineState, Efficiency), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Efficiency_MetaData), NewProp_Efficiency_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_SimTime = { "SimTime", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FEngineState, SimTime), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_SimTime_MetaData), NewProp_SimTime_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FEngineState_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_ActiveNodes,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_FPS,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_Efficiency,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FEngineState_Statics::NewProp_SimTime,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FEngineState_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FEngineState Property Definitions ***********************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FEngineState_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"EngineState",
	Z_Construct_UScriptStruct_FEngineState_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FEngineState_Statics::PropPointers),
	sizeof(FEngineState),
	alignof(FEngineState),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FEngineState_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FEngineState_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FEngineState()
{
	if (!Z_Registration_Info_UScriptStruct_FEngineState.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FEngineState.InnerSingleton, Z_Construct_UScriptStruct_FEngineState_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FEngineState.InnerSingleton);
}
// ********** End ScriptStruct FEngineState ********************************************************

// ********** Begin ScriptStruct FShockwaveData ****************************************************
struct Z_Construct_UScriptStruct_FShockwaveData_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FShockwaveData); }
	static inline consteval int16 GetStructAlignment() { return alignof(FShockwaveData); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_RadiusCM_MetaData[] = {
		{ "Category", "ShockwaveData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_SpeedCMS_MetaData[] = {
		{ "Category", "ShockwaveData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FShockwaveData constinit property declarations ********************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_RadiusCM;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_SpeedCMS;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FShockwaveData constinit property declarations **********************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FShockwaveData>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FShockwaveData_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FShockwaveData;
class UScriptStruct* FShockwaveData::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FShockwaveData.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FShockwaveData.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FShockwaveData, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ShockwaveData"));
	}
	return Z_Registration_Info_UScriptStruct_FShockwaveData.OuterSingleton;
	}

// ********** Begin ScriptStruct FShockwaveData Property Definitions *******************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FShockwaveData_Statics::NewProp_RadiusCM = { "RadiusCM", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FShockwaveData, RadiusCM), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_RadiusCM_MetaData), NewProp_RadiusCM_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FShockwaveData_Statics::NewProp_SpeedCMS = { "SpeedCMS", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FShockwaveData, SpeedCMS), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_SpeedCMS_MetaData), NewProp_SpeedCMS_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FShockwaveData_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FShockwaveData_Statics::NewProp_RadiusCM,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FShockwaveData_Statics::NewProp_SpeedCMS,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FShockwaveData_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FShockwaveData Property Definitions *********************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FShockwaveData_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"ShockwaveData",
	Z_Construct_UScriptStruct_FShockwaveData_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FShockwaveData_Statics::PropPointers),
	sizeof(FShockwaveData),
	alignof(FShockwaveData),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FShockwaveData_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FShockwaveData_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FShockwaveData()
{
	if (!Z_Registration_Info_UScriptStruct_FShockwaveData.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FShockwaveData.InnerSingleton, Z_Construct_UScriptStruct_FShockwaveData_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FShockwaveData.InnerSingleton);
}
// ********** End ScriptStruct FShockwaveData ******************************************************

// ********** Begin ScriptStruct FActivePowerPayload ***********************************************
struct Z_Construct_UScriptStruct_FActivePowerPayload_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FActivePowerPayload); }
	static inline consteval int16 GetStructAlignment() { return alignof(FActivePowerPayload); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Payload plano para env\xc3\xado masivo de poderes a Rust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Payload plano para env\xc3\xado masivo de poderes a Rust" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_PowerType_MetaData[] = {
		{ "Category", "ActivePowerPayload" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Origin_MetaData[] = {
		{ "Category", "ActivePowerPayload" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Direction_MetaData[] = {
		{ "Category", "ActivePowerPayload" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Intensity_MetaData[] = {
		{ "Category", "ActivePowerPayload" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Phase_MetaData[] = {
		{ "Category", "ActivePowerPayload" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FActivePowerPayload constinit property declarations ***************
	static const UECodeGen_Private::FIntPropertyParams NewProp_PowerType;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origin;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direction;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensity;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Phase;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FActivePowerPayload constinit property declarations *****************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FActivePowerPayload>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FActivePowerPayload_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FActivePowerPayload;
class UScriptStruct* FActivePowerPayload::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FActivePowerPayload.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FActivePowerPayload.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FActivePowerPayload, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ActivePowerPayload"));
	}
	return Z_Registration_Info_UScriptStruct_FActivePowerPayload.OuterSingleton;
	}

// ********** Begin ScriptStruct FActivePowerPayload Property Definitions **************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_PowerType = { "PowerType", nullptr, (EPropertyFlags)0x0010000000000004, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerPayload, PowerType), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_PowerType_MetaData), NewProp_PowerType_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000004, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerPayload, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Origin_MetaData), NewProp_Origin_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000004, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerPayload, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Direction_MetaData), NewProp_Direction_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000004, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerPayload, Intensity), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Intensity_MetaData), NewProp_Intensity_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Phase = { "Phase", nullptr, (EPropertyFlags)0x0010000000000004, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerPayload, Phase), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Phase_MetaData), NewProp_Phase_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FActivePowerPayload_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_PowerType,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Intensity,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewProp_Phase,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerPayload_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FActivePowerPayload Property Definitions ****************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FActivePowerPayload_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"ActivePowerPayload",
	Z_Construct_UScriptStruct_FActivePowerPayload_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerPayload_Statics::PropPointers),
	sizeof(FActivePowerPayload),
	alignof(FActivePowerPayload),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerPayload_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FActivePowerPayload_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FActivePowerPayload()
{
	if (!Z_Registration_Info_UScriptStruct_FActivePowerPayload.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FActivePowerPayload.InnerSingleton, Z_Construct_UScriptStruct_FActivePowerPayload_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FActivePowerPayload.InnerSingleton);
}
// ********** End ScriptStruct FActivePowerPayload *************************************************

// ********** Begin ScriptStruct FRenderData *******************************************************
struct Z_Construct_UScriptStruct_FRenderData_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FRenderData); }
	static inline consteval int16 GetStructAlignment() { return alignof(FRenderData); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Datos compactos para el renderizado masivo */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Datos compactos para el renderizado masivo" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Count_MetaData[] = {
		{ "Category", "RenderData" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FRenderData constinit property declarations ***********************
	static const UECodeGen_Private::FIntPropertyParams NewProp_Count;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FRenderData constinit property declarations *************************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FRenderData>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FRenderData_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FRenderData;
class UScriptStruct* FRenderData::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FRenderData.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FRenderData.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FRenderData, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("RenderData"));
	}
	return Z_Registration_Info_UScriptStruct_FRenderData.OuterSingleton;
	}

// ********** Begin ScriptStruct FRenderData Property Definitions **********************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UScriptStruct_FRenderData_Statics::NewProp_Count = { "Count", nullptr, (EPropertyFlags)0x0010000000000014, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FRenderData, Count), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Count_MetaData), NewProp_Count_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FRenderData_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FRenderData_Statics::NewProp_Count,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderData_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FRenderData Property Definitions ************************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FRenderData_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"RenderData",
	Z_Construct_UScriptStruct_FRenderData_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderData_Statics::PropPointers),
	sizeof(FRenderData),
	alignof(FRenderData),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FRenderData_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FRenderData_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FRenderData()
{
	if (!Z_Registration_Info_UScriptStruct_FRenderData.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FRenderData.InnerSingleton, Z_Construct_UScriptStruct_FRenderData_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FRenderData.InnerSingleton);
}
// ********** End ScriptStruct FRenderData *********************************************************

// ********** Begin ScriptStruct FActivePowerInstance **********************************************
struct Z_Construct_UScriptStruct_FActivePowerInstance_Statics
{
	static inline consteval int32 GetStructSize() { return sizeof(FActivePowerInstance); }
	static inline consteval int16 GetStructAlignment() { return alignof(FActivePowerInstance); }
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Struct_MetaDataParams[] = {
		{ "BlueprintType", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Type_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Origin_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Direction_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Intensity_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_TimeRemaining_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_TotalDuration_MetaData[] = {
		{ "Category", "ActivePowerInstance" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin ScriptStruct FActivePowerInstance constinit property declarations **************
	static const UECodeGen_Private::FBytePropertyParams NewProp_Type_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_Type;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origin;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direction;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensity;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_TimeRemaining;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_TotalDuration;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End ScriptStruct FActivePowerInstance constinit property declarations ****************
	static void* NewStructOps()
	{
		return (UScriptStruct::ICppStructOps*)new UScriptStruct::TCppStructOps<FActivePowerInstance>();
	}
	static const UECodeGen_Private::FStructParams StructParams;
}; // struct Z_Construct_UScriptStruct_FActivePowerInstance_Statics
static FStructRegistrationInfo Z_Registration_Info_UScriptStruct_FActivePowerInstance;
class UScriptStruct* FActivePowerInstance::StaticStruct()
{
	if (!Z_Registration_Info_UScriptStruct_FActivePowerInstance.OuterSingleton)
	{
		Z_Registration_Info_UScriptStruct_FActivePowerInstance.OuterSingleton = GetStaticStruct(Z_Construct_UScriptStruct_FActivePowerInstance, (UObject*)Z_Construct_UPackage__Script_SoulForgePhysX(), TEXT("ActivePowerInstance"));
	}
	return Z_Registration_Info_UScriptStruct_FActivePowerInstance.OuterSingleton;
	}

// ********** Begin ScriptStruct FActivePowerInstance Property Definitions *************************
const UECodeGen_Private::FBytePropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Type_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Type = { "Type", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, Type), Z_Construct_UEnum_SoulForgePhysX_EPowerType, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Type_MetaData), NewProp_Type_MetaData) }; // 1062630852
const UECodeGen_Private::FStructPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Origin = { "Origin", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, Origin), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Origin_MetaData), NewProp_Origin_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Direction = { "Direction", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, Direction), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Direction_MetaData), NewProp_Direction_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Intensity = { "Intensity", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, Intensity), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Intensity_MetaData), NewProp_Intensity_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_TimeRemaining = { "TimeRemaining", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, TimeRemaining), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_TimeRemaining_MetaData), NewProp_TimeRemaining_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_TotalDuration = { "TotalDuration", nullptr, (EPropertyFlags)0x0010000000000005, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(FActivePowerInstance, TotalDuration), METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_TotalDuration_MetaData), NewProp_TotalDuration_MetaData) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UScriptStruct_FActivePowerInstance_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Type_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Type,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Origin,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Direction,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_Intensity,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_TimeRemaining,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewProp_TotalDuration,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerInstance_Statics::PropPointers) < 2048);
// ********** End ScriptStruct FActivePowerInstance Property Definitions ***************************
const UECodeGen_Private::FStructParams Z_Construct_UScriptStruct_FActivePowerInstance_Statics::StructParams = {
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
	nullptr,
	&NewStructOps,
	"ActivePowerInstance",
	Z_Construct_UScriptStruct_FActivePowerInstance_Statics::PropPointers,
	UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerInstance_Statics::PropPointers),
	sizeof(FActivePowerInstance),
	alignof(FActivePowerInstance),
	RF_Public|RF_Transient|RF_MarkAsNative,
	EStructFlags(0x00000001),
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UScriptStruct_FActivePowerInstance_Statics::Struct_MetaDataParams), Z_Construct_UScriptStruct_FActivePowerInstance_Statics::Struct_MetaDataParams)
};
UScriptStruct* Z_Construct_UScriptStruct_FActivePowerInstance()
{
	if (!Z_Registration_Info_UScriptStruct_FActivePowerInstance.InnerSingleton)
	{
		UECodeGen_Private::ConstructUScriptStruct(Z_Registration_Info_UScriptStruct_FActivePowerInstance.InnerSingleton, Z_Construct_UScriptStruct_FActivePowerInstance_Statics::StructParams);
	}
	return CastChecked<UScriptStruct>(Z_Registration_Info_UScriptStruct_FActivePowerInstance.InnerSingleton);
}
// ********** End ScriptStruct FActivePowerInstance ************************************************

// ********** Begin Delegate FOnPowerVisualEffect **************************************************
struct Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics
{
	struct _Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms
	{
		float BloomIntensity;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Delegate FOnPowerVisualEffect constinit property declarations ******************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_BloomIntensity;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Delegate FOnPowerVisualEffect constinit property declarations ********************
	static const UECodeGen_Private::FDelegateFunctionParams FuncParams;
};

// ********** Begin Delegate FOnPowerVisualEffect Property Definitions *****************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::NewProp_BloomIntensity = { "BloomIntensity", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(_Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms, BloomIntensity), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::NewProp_BloomIntensity,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::PropPointers) < 2048);
// ********** End Delegate FOnPowerVisualEffect Property Definitions *******************************
const UECodeGen_Private::FDelegateFunctionParams Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UPackage__Script_SoulForgePhysX, nullptr, "OnPowerVisualEffect__DelegateSignature", 	Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::PropPointers), 
sizeof(Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::_Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x00130000, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::Function_MetaDataParams), Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::_Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms) < MAX_uint16);
UFunction* Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUDelegateFunction(&ReturnFunction, Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature_Statics::FuncParams);
	}
	return ReturnFunction;
}
void FOnPowerVisualEffect_DelegateWrapper(const FMulticastScriptDelegate& OnPowerVisualEffect, float BloomIntensity)
{
	struct _Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms
	{
		float BloomIntensity;
	};
	_Script_SoulForgePhysX_eventOnPowerVisualEffect_Parms Parms;
	Parms.BloomIntensity=BloomIntensity;
	OnPowerVisualEffect.ProcessMulticastDelegate<UObject>(&Parms);
}
// ********** End Delegate FOnPowerVisualEffect ****************************************************

// ********** Begin Class USoulForgeBridge Function ActivarPoder ***********************************
struct Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics
{
	struct SoulForgeBridge_eventActivarPoder_Parms
	{
		EPowerType Tipo;
		FVector Origen;
		FVector Direccion;
		float Intensidad;
		float Duracion;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Powers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Activa un poder elemental en el motor f\xc3\xadsico */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Activa un poder elemental en el motor f\xc3\xadsico" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ActivarPoder constinit property declarations **************************
	static const UECodeGen_Private::FBytePropertyParams NewProp_Tipo_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_Tipo;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Origen;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Direccion;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Intensidad;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Duracion;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ActivarPoder constinit property declarations ****************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ActivarPoder Property Definitions *************************************
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Tipo_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Tipo = { "Tipo", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventActivarPoder_Parms, Tipo), Z_Construct_UEnum_SoulForgePhysX_EPowerType, METADATA_PARAMS(0, nullptr) }; // 1062630852
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Origen = { "Origen", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventActivarPoder_Parms, Origen), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Direccion = { "Direccion", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventActivarPoder_Parms, Direccion), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Intensidad = { "Intensidad", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventActivarPoder_Parms, Intensidad), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Duracion = { "Duracion", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventActivarPoder_Parms, Duracion), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Tipo_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Tipo,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Origen,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Direccion,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Intensidad,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::NewProp_Duracion,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::PropPointers) < 2048);
// ********** End Function ActivarPoder Property Definitions ***************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "ActivarPoder", 	Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::SoulForgeBridge_eventActivarPoder_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::SoulForgeBridge_eventActivarPoder_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_ActivarPoder()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_ActivarPoder_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execActivarPoder)
{
	P_GET_ENUM(EPowerType,Z_Param_Tipo);
	P_GET_STRUCT(FVector,Z_Param_Origen);
	P_GET_STRUCT(FVector,Z_Param_Direccion);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Intensidad);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Duracion);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::ActivarPoder(EPowerType(Z_Param_Tipo),Z_Param_Origen,Z_Param_Direccion,Z_Param_Intensidad,Z_Param_Duracion);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function ActivarPoder *************************************

// ********** Begin Class USoulForgeBridge Function AnclarObjeto ***********************************
struct Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics
{
	struct SoulForgeBridge_eventAnclarObjeto_Parms
	{
		FString ProxyId;
		FVector Posicion;
		bool ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Proxy" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Actualiza la posici\xc3\xb3n del objeto en Rust antes de la detonaci\xc3\xb3n (GPS Update) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Actualiza la posici\xc3\xb3n del objeto en Rust antes de la detonaci\xc3\xb3n (GPS Update)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function AnclarObjeto constinit property declarations **************************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Posicion;
	static void NewProp_ReturnValue_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function AnclarObjeto constinit property declarations ****************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function AnclarObjeto Property Definitions *************************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventAnclarObjeto_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_Posicion = { "Posicion", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventAnclarObjeto_Parms, Posicion), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
void Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ReturnValue_SetBit(void* Obj)
{
	((SoulForgeBridge_eventAnclarObjeto_Parms*)Obj)->ReturnValue = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(SoulForgeBridge_eventAnclarObjeto_Parms), &Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ReturnValue_SetBit, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_Posicion,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::PropPointers) < 2048);
// ********** End Function AnclarObjeto Property Definitions ***************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "AnclarObjeto", 	Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::SoulForgeBridge_eventAnclarObjeto_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::SoulForgeBridge_eventAnclarObjeto_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execAnclarObjeto)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_STRUCT(FVector,Z_Param_Posicion);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(bool*)Z_Param__Result=USoulForgeBridge::AnclarObjeto(Z_Param_ProxyId,Z_Param_Posicion);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function AnclarObjeto *************************************

// ********** Begin Class USoulForgeBridge Function ApplySettings **********************************
struct Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics
{
	struct SoulForgeBridge_eventApplySettings_Parms
	{
		int32 NumThreads;
		bool bUseGPU;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Rendimiento" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Configura el rendimiento (Hilos y GPU) del n\xc3\xba""cleo */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Configura el rendimiento (Hilos y GPU) del n\xc3\xba""cleo" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function ApplySettings constinit property declarations *************************
	static const UECodeGen_Private::FIntPropertyParams NewProp_NumThreads;
	static void NewProp_bUseGPU_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_bUseGPU;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function ApplySettings constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function ApplySettings Property Definitions ************************************
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_NumThreads = { "NumThreads", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventApplySettings_Parms, NumThreads), METADATA_PARAMS(0, nullptr) };
void Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_bUseGPU_SetBit(void* Obj)
{
	((SoulForgeBridge_eventApplySettings_Parms*)Obj)->bUseGPU = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_bUseGPU = { "bUseGPU", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(SoulForgeBridge_eventApplySettings_Parms), &Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_bUseGPU_SetBit, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_NumThreads,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::NewProp_bUseGPU,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::PropPointers) < 2048);
// ********** End Function ApplySettings Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "ApplySettings", 	Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::SoulForgeBridge_eventApplySettings_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::SoulForgeBridge_eventApplySettings_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_ApplySettings()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_ApplySettings_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execApplySettings)
{
	P_GET_PROPERTY(FIntProperty,Z_Param_NumThreads);
	P_GET_UBOOL(Z_Param_bUseGPU);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::ApplySettings(Z_Param_NumThreads,Z_Param_bUseGPU);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function ApplySettings ************************************

// ********** Begin Class USoulForgeBridge Function BorrarProxyEnRust ******************************
struct Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics
{
	struct SoulForgeBridge_eventBorrarProxyEnRust_Parms
	{
		FString ProxyId;
		bool ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Proxy" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function BorrarProxyEnRust constinit property declarations *********************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static void NewProp_ReturnValue_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function BorrarProxyEnRust constinit property declarations ***********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function BorrarProxyEnRust Property Definitions ********************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventBorrarProxyEnRust_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
void Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ReturnValue_SetBit(void* Obj)
{
	((SoulForgeBridge_eventBorrarProxyEnRust_Parms*)Obj)->ReturnValue = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(SoulForgeBridge_eventBorrarProxyEnRust_Parms), &Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ReturnValue_SetBit, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::PropPointers) < 2048);
// ********** End Function BorrarProxyEnRust Property Definitions **********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "BorrarProxyEnRust", 	Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::SoulForgeBridge_eventBorrarProxyEnRust_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::SoulForgeBridge_eventBorrarProxyEnRust_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execBorrarProxyEnRust)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(bool*)Z_Param__Result=USoulForgeBridge::BorrarProxyEnRust(Z_Param_ProxyId);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function BorrarProxyEnRust ********************************

// ********** Begin Class USoulForgeBridge Function CalcularOnda ***********************************
struct Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics
{
	struct SoulForgeBridge_eventCalcularOnda_Parms
	{
		float Energia;
		FShockwaveData ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Physics" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Calcula datos f\xc3\xadsicos de la onda expansiva usando Rust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Calcula datos f\xc3\xadsicos de la onda expansiva usando Rust" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function CalcularOnda constinit property declarations **************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Energia;
	static const UECodeGen_Private::FStructPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function CalcularOnda constinit property declarations ****************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function CalcularOnda Property Definitions *************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::NewProp_Energia = { "Energia", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventCalcularOnda_Parms, Energia), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventCalcularOnda_Parms, ReturnValue), Z_Construct_UScriptStruct_FShockwaveData, METADATA_PARAMS(0, nullptr) }; // 3033931910
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::NewProp_Energia,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::PropPointers) < 2048);
// ********** End Function CalcularOnda Property Definitions ***************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "CalcularOnda", 	Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::SoulForgeBridge_eventCalcularOnda_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::SoulForgeBridge_eventCalcularOnda_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_CalcularOnda()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_CalcularOnda_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execCalcularOnda)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_Energia);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(FShockwaveData*)Z_Param__Result=USoulForgeBridge::CalcularOnda(Z_Param_Energia);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function CalcularOnda *************************************

// ********** Begin Class USoulForgeBridge Function DetonarMilitar *********************************
struct Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics
{
	struct SoulForgeBridge_eventDetonarMilitar_Parms
	{
		FString ProxyId;
		FVector PosicionActual;
		float Energia;
		EExplosiveType Explosivo;
		int32 CantidadNodos;
		float Radio;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Proxy" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Detonaci\xc3\xb3n con actualizaci\xc3\xb3n de GPS en tiempo real y n\xc3\xbamero de fragmentos configurable */" },
#endif
		{ "CPP_Default_CantidadNodos", "0" },
		{ "CPP_Default_Radio", "250.000000" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Detonaci\xc3\xb3n con actualizaci\xc3\xb3n de GPS en tiempo real y n\xc3\xbamero de fragmentos configurable" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function DetonarMilitar constinit property declarations ************************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FStructPropertyParams NewProp_PosicionActual;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Energia;
	static const UECodeGen_Private::FBytePropertyParams NewProp_Explosivo_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_Explosivo;
	static const UECodeGen_Private::FIntPropertyParams NewProp_CantidadNodos;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Radio;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function DetonarMilitar constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function DetonarMilitar Property Definitions ***********************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_PosicionActual = { "PosicionActual", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, PosicionActual), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Energia = { "Energia", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, Energia), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Explosivo_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Explosivo = { "Explosivo", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, Explosivo), Z_Construct_UEnum_SoulForgePhysX_EExplosiveType, METADATA_PARAMS(0, nullptr) }; // 1116665067
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_CantidadNodos = { "CantidadNodos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, CantidadNodos), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Radio = { "Radio", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarMilitar_Parms, Radio), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_PosicionActual,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Energia,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Explosivo_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Explosivo,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_CantidadNodos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::NewProp_Radio,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::PropPointers) < 2048);
// ********** End Function DetonarMilitar Property Definitions *************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "DetonarMilitar", 	Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::SoulForgeBridge_eventDetonarMilitar_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::SoulForgeBridge_eventDetonarMilitar_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execDetonarMilitar)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_STRUCT(FVector,Z_Param_PosicionActual);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Energia);
	P_GET_ENUM(EExplosiveType,Z_Param_Explosivo);
	P_GET_PROPERTY(FIntProperty,Z_Param_CantidadNodos);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Radio);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::DetonarMilitar(Z_Param_ProxyId,Z_Param_PosicionActual,Z_Param_Energia,EExplosiveType(Z_Param_Explosivo),Z_Param_CantidadNodos,Z_Param_Radio);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function DetonarMilitar ***********************************

// ********** Begin Class USoulForgeBridge Function DetonarNativo **********************************
struct Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics
{
	struct SoulForgeBridge_eventDetonarNativo_Parms
	{
		FString ProxyId;
		float Energy;
		int32 Preset;
		float FragLevel;
		TArray<FSoulForgeFragmentData> OutData;
		int32 CantidadNodos;
		int32 ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Proxy" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\x92\xa5 DESTRUCCI\xc3\x93N NATIVA */" },
#endif
		{ "CPP_Default_CantidadNodos", "0" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x92\xa5 DESTRUCCI\xc3\x93N NATIVA" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function DetonarNativo constinit property declarations *************************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Energy;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Preset;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_FragLevel;
	static const UECodeGen_Private::FStructPropertyParams NewProp_OutData_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_OutData;
	static const UECodeGen_Private::FIntPropertyParams NewProp_CantidadNodos;
	static const UECodeGen_Private::FIntPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function DetonarNativo constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function DetonarNativo Property Definitions ************************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_Energy = { "Energy", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, Energy), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_Preset = { "Preset", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, Preset), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_FragLevel = { "FragLevel", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, FragLevel), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_OutData_Inner = { "OutData", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FSoulForgeFragmentData, METADATA_PARAMS(0, nullptr) }; // 3758934569
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_OutData = { "OutData", nullptr, (EPropertyFlags)0x0010000000000180, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, OutData), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) }; // 3758934569
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_CantidadNodos = { "CantidadNodos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, CantidadNodos), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventDetonarNativo_Parms, ReturnValue), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_Energy,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_Preset,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_FragLevel,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_OutData_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_OutData,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_CantidadNodos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::PropPointers) < 2048);
// ********** End Function DetonarNativo Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "DetonarNativo", 	Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::SoulForgeBridge_eventDetonarNativo_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04422401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::SoulForgeBridge_eventDetonarNativo_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_DetonarNativo()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_DetonarNativo_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execDetonarNativo)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Energy);
	P_GET_PROPERTY(FIntProperty,Z_Param_Preset);
	P_GET_PROPERTY(FFloatProperty,Z_Param_FragLevel);
	P_GET_TARRAY_REF(FSoulForgeFragmentData,Z_Param_Out_OutData);
	P_GET_PROPERTY(FIntProperty,Z_Param_CantidadNodos);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(int32*)Z_Param__Result=USoulForgeBridge::DetonarNativo(Z_Param_ProxyId,Z_Param_Energy,Z_Param_Preset,Z_Param_FragLevel,Z_Param_Out_OutData,Z_Param_CantidadNodos);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function DetonarNativo ************************************

// ********** Begin Class USoulForgeBridge Function FiltrarInstancias ******************************
struct Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics
{
	struct SoulForgeBridge_eventFiltrarInstancias_Parms
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
		{ "Comment", "/** \xf0\x9f\x91\x81\xef\xb8\x8f INSIGHT FILTER (Culling Nativo) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x91\x81\xef\xb8\x8f INSIGHT FILTER (Culling Nativo)" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Posiciones_MetaData[] = {
		{ "NativeConst", "" },
	};
#endif // WITH_METADATA

// ********** Begin Function FiltrarInstancias constinit property declarations *********************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Posiciones_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_Posiciones;
	static const UECodeGen_Private::FStructPropertyParams NewProp_CamaraPos;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DistanciaVision;
	static const UECodeGen_Private::FIntPropertyParams NewProp_ReturnValue_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function FiltrarInstancias constinit property declarations ***********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function FiltrarInstancias Property Definitions ********************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_Posiciones_Inner = { "Posiciones", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_Posiciones = { "Posiciones", nullptr, (EPropertyFlags)0x0010000008000182, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventFiltrarInstancias_Parms, Posiciones), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Posiciones_MetaData), NewProp_Posiciones_MetaData) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_CamaraPos = { "CamaraPos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventFiltrarInstancias_Parms, CamaraPos), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_DistanciaVision = { "DistanciaVision", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventFiltrarInstancias_Parms, DistanciaVision), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_ReturnValue_Inner = { "ReturnValue", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventFiltrarInstancias_Parms, ReturnValue), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_Posiciones_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_Posiciones,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_CamaraPos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_DistanciaVision,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_ReturnValue_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::PropPointers) < 2048);
// ********** End Function FiltrarInstancias Property Definitions **********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "FiltrarInstancias", 	Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::SoulForgeBridge_eventFiltrarInstancias_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04C22401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::SoulForgeBridge_eventFiltrarInstancias_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execFiltrarInstancias)
{
	P_GET_TARRAY_REF(FVector,Z_Param_Out_Posiciones);
	P_GET_STRUCT(FVector,Z_Param_CamaraPos);
	P_GET_PROPERTY(FFloatProperty,Z_Param_DistanciaVision);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(TArray<int32>*)Z_Param__Result=USoulForgeBridge::FiltrarInstancias(Z_Param_Out_Posiciones,Z_Param_CamaraPos,Z_Param_DistanciaVision);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function FiltrarInstancias ********************************

// ********** Begin Class USoulForgeBridge Function GetExplosionPreview ****************************
struct Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics
{
	struct SoulForgeBridge_eventGetExplosionPreview_Parms
	{
		FString ProxyId;
		float Energia;
		int32 ExplosiveType;
		int32 CantidadNodos;
		TArray<FVector> ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Lab" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\xa7\xaa LABORATORIO DE PREVISUALIZACI\xc3\x93N */" },
#endif
		{ "CPP_Default_CantidadNodos", "0" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\xa7\xaa LABORATORIO DE PREVISUALIZACI\xc3\x93N" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function GetExplosionPreview constinit property declarations *******************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Energia;
	static const UECodeGen_Private::FIntPropertyParams NewProp_ExplosiveType;
	static const UECodeGen_Private::FIntPropertyParams NewProp_CantidadNodos;
	static const UECodeGen_Private::FStructPropertyParams NewProp_ReturnValue_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function GetExplosionPreview constinit property declarations *********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function GetExplosionPreview Property Definitions ******************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetExplosionPreview_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_Energia = { "Energia", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetExplosionPreview_Parms, Energia), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ExplosiveType = { "ExplosiveType", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetExplosionPreview_Parms, ExplosiveType), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_CantidadNodos = { "CantidadNodos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetExplosionPreview_Parms, CantidadNodos), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ReturnValue_Inner = { "ReturnValue", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetExplosionPreview_Parms, ReturnValue), EArrayPropertyFlags::None, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_Energia,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ExplosiveType,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_CantidadNodos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ReturnValue_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::PropPointers) < 2048);
// ********** End Function GetExplosionPreview Property Definitions ********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "GetExplosionPreview", 	Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::SoulForgeBridge_eventGetExplosionPreview_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::SoulForgeBridge_eventGetExplosionPreview_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execGetExplosionPreview)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Energia);
	P_GET_PROPERTY(FIntProperty,Z_Param_ExplosiveType);
	P_GET_PROPERTY(FIntProperty,Z_Param_CantidadNodos);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(TArray<FVector>*)Z_Param__Result=USoulForgeBridge::GetExplosionPreview(Z_Param_ProxyId,Z_Param_Energia,Z_Param_ExplosiveType,Z_Param_CantidadNodos);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function GetExplosionPreview ******************************

// ********** Begin Class USoulForgeBridge Function GetPerformanceState ****************************
struct Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics
{
	struct SoulForgeBridge_eventGetPerformanceState_Parms
	{
		FEngineState ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Stats" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\x93\x8a TELEMETR\xc3\x8d""A DE ALTA VELOCIDAD */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x93\x8a TELEMETR\xc3\x8d""A DE ALTA VELOCIDAD" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function GetPerformanceState constinit property declarations *******************
	static const UECodeGen_Private::FStructPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function GetPerformanceState constinit property declarations *********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function GetPerformanceState Property Definitions ******************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventGetPerformanceState_Parms, ReturnValue), Z_Construct_UScriptStruct_FEngineState, METADATA_PARAMS(0, nullptr) }; // 3212734741
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::PropPointers) < 2048);
// ********** End Function GetPerformanceState Property Definitions ********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "GetPerformanceState", 	Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::SoulForgeBridge_eventGetPerformanceState_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::SoulForgeBridge_eventGetPerformanceState_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execGetPerformanceState)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	*(FEngineState*)Z_Param__Result=USoulForgeBridge::GetPerformanceState();
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function GetPerformanceState ******************************

// ********** Begin Class USoulForgeBridge Function Initialize *************************************
struct Z_Construct_UFunction_USoulForgeBridge_Initialize_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Core" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\x94\x8c INICIALIZACI\xc3\x93N UNIFICADA */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x94\x8c INICIALIZACI\xc3\x93N UNIFICADA" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function Initialize constinit property declarations ****************************
// ********** End Function Initialize constinit property declarations ******************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_Initialize_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "Initialize", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_Initialize_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_Initialize_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeBridge_Initialize()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_Initialize_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execInitialize)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::Initialize();
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function Initialize ***************************************

// ********** Begin Class USoulForgeBridge Function LimpiarNucleo **********************************
struct Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "Soul Forge|Core" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Limpia la memoria del n\xc3\xba""cleo de Rust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Limpia la memoria del n\xc3\xba""cleo de Rust" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function LimpiarNucleo constinit property declarations *************************
// ********** End Function LimpiarNucleo constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "LimpiarNucleo", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execLimpiarNucleo)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::LimpiarNucleo();
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function LimpiarNucleo ************************************

// ********** Begin Class USoulForgeBridge Function RegisterProxy **********************************
struct Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics
{
	struct SoulForgeBridge_eventRegisterProxy_Parms
	{
		FString ProxyId;
		FVector Center;
		FVector HalfExtent;
		float Density;
		bool ReturnValue;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Proxy" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// M\xc3\xa9todos Legacy / Auxiliares\n" },
#endif
		{ "CPP_Default_Density", "0.002400" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "M\xc3\xa9todos Legacy / Auxiliares" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function RegisterProxy constinit property declarations *************************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FStructPropertyParams NewProp_Center;
	static const UECodeGen_Private::FStructPropertyParams NewProp_HalfExtent;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Density;
	static void NewProp_ReturnValue_SetBit(void* Obj);
	static const UECodeGen_Private::FBoolPropertyParams NewProp_ReturnValue;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function RegisterProxy constinit property declarations ***************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function RegisterProxy Property Definitions ************************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventRegisterProxy_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_Center = { "Center", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventRegisterProxy_Parms, Center), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_HalfExtent = { "HalfExtent", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventRegisterProxy_Parms, HalfExtent), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_Density = { "Density", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventRegisterProxy_Parms, Density), METADATA_PARAMS(0, nullptr) };
void Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ReturnValue_SetBit(void* Obj)
{
	((SoulForgeBridge_eventRegisterProxy_Parms*)Obj)->ReturnValue = 1;
}
const UECodeGen_Private::FBoolPropertyParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ReturnValue = { "ReturnValue", nullptr, (EPropertyFlags)0x0010000000000580, UECodeGen_Private::EPropertyGenFlags::Bool | UECodeGen_Private::EPropertyGenFlags::NativeBool, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, sizeof(bool), sizeof(SoulForgeBridge_eventRegisterProxy_Parms), &Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ReturnValue_SetBit, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_Center,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_HalfExtent,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_Density,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::NewProp_ReturnValue,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::PropPointers) < 2048);
// ********** End Function RegisterProxy Property Definitions **************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "RegisterProxy", 	Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::SoulForgeBridge_eventRegisterProxy_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::SoulForgeBridge_eventRegisterProxy_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_RegisterProxy()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_RegisterProxy_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execRegisterProxy)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_STRUCT(FVector,Z_Param_Center);
	P_GET_STRUCT(FVector,Z_Param_HalfExtent);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Density);
	P_FINISH;
	P_NATIVE_BEGIN;
	*(bool*)Z_Param__Result=USoulForgeBridge::RegisterProxy(Z_Param_ProxyId,Z_Param_Center,Z_Param_HalfExtent,Z_Param_Density);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function RegisterProxy ************************************

// ********** Begin Class USoulForgeBridge Function SetGlobalChaos *********************************
struct Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics
{
	struct SoulForgeBridge_eventSetGlobalChaos_Parms
	{
		float ChaosFactor;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Ajusta qu\xc3\xa9 tan deformados/ca\xc3\xb3ticos son los fragmentos (0.0 = Cubos perfectos, 1.0 = Caos total) */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Ajusta qu\xc3\xa9 tan deformados/ca\xc3\xb3ticos son los fragmentos (0.0 = Cubos perfectos, 1.0 = Caos total)" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SetGlobalChaos constinit property declarations ************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_ChaosFactor;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SetGlobalChaos constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SetGlobalChaos Property Definitions ***********************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::NewProp_ChaosFactor = { "ChaosFactor", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetGlobalChaos_Parms, ChaosFactor), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::NewProp_ChaosFactor,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::PropPointers) < 2048);
// ********** End Function SetGlobalChaos Property Definitions *************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SetGlobalChaos", 	Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::SoulForgeBridge_eventSetGlobalChaos_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::SoulForgeBridge_eventSetGlobalChaos_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSetGlobalChaos)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_ChaosFactor);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SetGlobalChaos(Z_Param_ChaosFactor);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SetGlobalChaos ***********************************

// ********** Begin Class USoulForgeBridge Function SetGlobalPower *********************************
struct Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics
{
	struct SoulForgeBridge_eventSetGlobalPower_Parms
	{
		float Multiplier;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Admin" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function SetGlobalPower constinit property declarations ************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Multiplier;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SetGlobalPower constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SetGlobalPower Property Definitions ***********************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::NewProp_Multiplier = { "Multiplier", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetGlobalPower_Parms, Multiplier), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::NewProp_Multiplier,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::PropPointers) < 2048);
// ********** End Function SetGlobalPower Property Definitions *************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SetGlobalPower", 	Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::SoulForgeBridge_eventSetGlobalPower_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::SoulForgeBridge_eventSetGlobalPower_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSetGlobalPower)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_Multiplier);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SetGlobalPower(Z_Param_Multiplier);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SetGlobalPower ***********************************

// ********** Begin Class USoulForgeBridge Function SetGroundZ *************************************
struct Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics
{
	struct SoulForgeBridge_eventSetGroundZ_Parms
	{
		float Z;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|PhysX" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Configura la altura del suelo en el motor de Rust */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Configura la altura del suelo en el motor de Rust" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SetGroundZ constinit property declarations ****************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Z;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SetGroundZ constinit property declarations ******************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SetGroundZ Property Definitions ***************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::NewProp_Z = { "Z", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetGroundZ_Parms, Z), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::NewProp_Z,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::PropPointers) < 2048);
// ********** End Function SetGroundZ Property Definitions *****************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SetGroundZ", 	Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::SoulForgeBridge_eventSetGroundZ_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::SoulForgeBridge_eventSetGroundZ_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SetGroundZ()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SetGroundZ_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSetGroundZ)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_Z);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SetGroundZ(Z_Param_Z);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SetGroundZ ***************************************

// ********** Begin Class USoulForgeBridge Function SetProxyMilitar ********************************
struct Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics
{
	struct SoulForgeBridge_eventSetProxyMilitar_Parms
	{
		FString ProxyId;
		EMaterialType Material;
		EPhysicsLOD LOD;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Core" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Configura las propiedades militares de un proxy */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Configura las propiedades militares de un proxy" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SetProxyMilitar constinit property declarations ***********************
	static const UECodeGen_Private::FStrPropertyParams NewProp_ProxyId;
	static const UECodeGen_Private::FBytePropertyParams NewProp_Material_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_Material;
	static const UECodeGen_Private::FBytePropertyParams NewProp_LOD_Underlying;
	static const UECodeGen_Private::FEnumPropertyParams NewProp_LOD;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SetProxyMilitar constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SetProxyMilitar Property Definitions **********************************
const UECodeGen_Private::FStrPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_ProxyId = { "ProxyId", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Str, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetProxyMilitar_Parms, ProxyId), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_Material_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_Material = { "Material", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetProxyMilitar_Parms, Material), Z_Construct_UEnum_SoulForgePhysX_EMaterialType, METADATA_PARAMS(0, nullptr) }; // 497896940
const UECodeGen_Private::FBytePropertyParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_LOD_Underlying = { "UnderlyingType", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Byte, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, nullptr, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FEnumPropertyParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_LOD = { "LOD", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Enum, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSetProxyMilitar_Parms, LOD), Z_Construct_UEnum_SoulForgePhysX_EPhysicsLOD, METADATA_PARAMS(0, nullptr) }; // 2707074426
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_ProxyId,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_Material_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_Material,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_LOD_Underlying,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::NewProp_LOD,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::PropPointers) < 2048);
// ********** End Function SetProxyMilitar Property Definitions ************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SetProxyMilitar", 	Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::SoulForgeBridge_eventSetProxyMilitar_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::SoulForgeBridge_eventSetProxyMilitar_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSetProxyMilitar)
{
	P_GET_PROPERTY(FStrProperty,Z_Param_ProxyId);
	P_GET_ENUM(EMaterialType,Z_Param_Material);
	P_GET_ENUM(EPhysicsLOD,Z_Param_LOD);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SetProxyMilitar(Z_Param_ProxyId,EMaterialType(Z_Param_Material),EPhysicsLOD(Z_Param_LOD));
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SetProxyMilitar **********************************

// ********** Begin Class USoulForgeBridge Function Shutdown ***************************************
struct Z_Construct_UFunction_USoulForgeBridge_Shutdown_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Core" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function Shutdown constinit property declarations ******************************
// ********** End Function Shutdown constinit property declarations ********************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_Shutdown_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "Shutdown", 	nullptr, 
	0, 
0,
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_Shutdown_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_Shutdown_Statics::Function_MetaDataParams)},  };
UFunction* Z_Construct_UFunction_USoulForgeBridge_Shutdown()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_Shutdown_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execShutdown)
{
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::Shutdown();
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function Shutdown *****************************************

// ********** Begin Class USoulForgeBridge Function SpawnNodeOptimized *****************************
struct Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics
{
	struct SoulForgeBridge_eventSpawnNodeOptimized_Parms
	{
		FVector Pos;
		float Mass;
		int32 Material;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** \xf0\x9f\x9a\x80 NUEVA API DE NODOS */" },
#endif
		{ "CPP_Default_Mass", "1.000000" },
		{ "CPP_Default_Material", "0" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "\xf0\x9f\x9a\x80 NUEVA API DE NODOS" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Function SpawnNodeOptimized constinit property declarations ********************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Pos;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Mass;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Material;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SpawnNodeOptimized constinit property declarations **********************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SpawnNodeOptimized Property Definitions *******************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Pos = { "Pos", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodeOptimized_Parms, Pos), Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Mass = { "Mass", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodeOptimized_Parms, Mass), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Material = { "Material", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodeOptimized_Parms, Material), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Pos,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Mass,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::NewProp_Material,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::PropPointers) < 2048);
// ********** End Function SpawnNodeOptimized Property Definitions *********************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SpawnNodeOptimized", 	Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::SoulForgeBridge_eventSpawnNodeOptimized_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04822401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::SoulForgeBridge_eventSpawnNodeOptimized_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSpawnNodeOptimized)
{
	P_GET_STRUCT(FVector,Z_Param_Pos);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Mass);
	P_GET_PROPERTY(FIntProperty,Z_Param_Material);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SpawnNodeOptimized(Z_Param_Pos,Z_Param_Mass,Z_Param_Material);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SpawnNodeOptimized *******************************

// ********** Begin Class USoulForgeBridge Function SpawnNodesBulk *********************************
struct Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics
{
	struct SoulForgeBridge_eventSpawnNodesBulk_Parms
	{
		TArray<FVector> Posiciones;
		float Masa;
		int32 Material;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|PhysX" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Inyecci\xc3\xb3n masiva de nodos para evitar lag en el FFI (1000+ nodos en una sola llamada) */" },
#endif
		{ "CPP_Default_Masa", "1.000000" },
		{ "CPP_Default_Material", "0" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Inyecci\xc3\xb3n masiva de nodos para evitar lag en el FFI (1000+ nodos en una sola llamada)" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Posiciones_MetaData[] = {
		{ "NativeConst", "" },
	};
#endif // WITH_METADATA

// ********** Begin Function SpawnNodesBulk constinit property declarations ************************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Posiciones_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_Posiciones;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_Masa;
	static const UECodeGen_Private::FIntPropertyParams NewProp_Material;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function SpawnNodesBulk constinit property declarations **************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function SpawnNodesBulk Property Definitions ***********************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Posiciones_Inner = { "Posiciones", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FVector, METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Posiciones = { "Posiciones", nullptr, (EPropertyFlags)0x0010000008000182, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodesBulk_Parms, Posiciones), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Posiciones_MetaData), NewProp_Posiciones_MetaData) };
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Masa = { "Masa", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodesBulk_Parms, Masa), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FIntPropertyParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Material = { "Material", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Int, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventSpawnNodesBulk_Parms, Material), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Posiciones_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Posiciones,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Masa,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::NewProp_Material,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::PropPointers) < 2048);
// ********** End Function SpawnNodesBulk Property Definitions *************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "SpawnNodesBulk", 	Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::SoulForgeBridge_eventSpawnNodesBulk_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04422401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::SoulForgeBridge_eventSpawnNodesBulk_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execSpawnNodesBulk)
{
	P_GET_TARRAY_REF(FVector,Z_Param_Out_Posiciones);
	P_GET_PROPERTY(FFloatProperty,Z_Param_Masa);
	P_GET_PROPERTY(FIntProperty,Z_Param_Material);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::SpawnNodesBulk(Z_Param_Out_Posiciones,Z_Param_Masa,Z_Param_Material);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function SpawnNodesBulk ***********************************

// ********** Begin Class USoulForgeBridge Function TickPhysics ************************************
struct Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics
{
	struct SoulForgeBridge_eventTickPhysics_Parms
	{
		float DeltaSeconds;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|PhysX" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
	};
#endif // WITH_METADATA

// ********** Begin Function TickPhysics constinit property declarations ***************************
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DeltaSeconds;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function TickPhysics constinit property declarations *****************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function TickPhysics Property Definitions **************************************
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::NewProp_DeltaSeconds = { "DeltaSeconds", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventTickPhysics_Parms, DeltaSeconds), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::NewProp_DeltaSeconds,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::PropPointers) < 2048);
// ********** End Function TickPhysics Property Definitions ****************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "TickPhysics", 	Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::SoulForgeBridge_eventTickPhysics_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04022401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::SoulForgeBridge_eventTickPhysics_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_TickPhysics()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_TickPhysics_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execTickPhysics)
{
	P_GET_PROPERTY(FFloatProperty,Z_Param_DeltaSeconds);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::TickPhysics(Z_Param_DeltaSeconds);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function TickPhysics **************************************

// ********** Begin Class USoulForgeBridge Function UpdateAllPowers ********************************
struct Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics
{
	struct SoulForgeBridge_eventUpdateAllPowers_Parms
	{
		TArray<FActivePowerPayload> Payloads;
		float DeltaTime;
	};
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Function_MetaDataParams[] = {
		{ "Category", "SoulForge|Powers" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "/** Actualizaci\xc3\xb3n masiva de todos los poderes activos en un solo viaje FFI */" },
#endif
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "Actualizaci\xc3\xb3n masiva de todos los poderes activos en un solo viaje FFI" },
#endif
	};
	static constexpr UECodeGen_Private::FMetaDataPairParam NewProp_Payloads_MetaData[] = {
		{ "NativeConst", "" },
	};
#endif // WITH_METADATA

// ********** Begin Function UpdateAllPowers constinit property declarations ***********************
	static const UECodeGen_Private::FStructPropertyParams NewProp_Payloads_Inner;
	static const UECodeGen_Private::FArrayPropertyParams NewProp_Payloads;
	static const UECodeGen_Private::FFloatPropertyParams NewProp_DeltaTime;
	static const UECodeGen_Private::FPropertyParamsBase* const PropPointers[];
// ********** End Function UpdateAllPowers constinit property declarations *************************
	static const UECodeGen_Private::FFunctionParams FuncParams;
};

// ********** Begin Function UpdateAllPowers Property Definitions **********************************
const UECodeGen_Private::FStructPropertyParams Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_Payloads_Inner = { "Payloads", nullptr, (EPropertyFlags)0x0000000000000000, UECodeGen_Private::EPropertyGenFlags::Struct, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, 0, Z_Construct_UScriptStruct_FActivePowerPayload, METADATA_PARAMS(0, nullptr) }; // 442066524
const UECodeGen_Private::FArrayPropertyParams Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_Payloads = { "Payloads", nullptr, (EPropertyFlags)0x0010000008000182, UECodeGen_Private::EPropertyGenFlags::Array, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventUpdateAllPowers_Parms, Payloads), EArrayPropertyFlags::None, METADATA_PARAMS(UE_ARRAY_COUNT(NewProp_Payloads_MetaData), NewProp_Payloads_MetaData) }; // 442066524
const UECodeGen_Private::FFloatPropertyParams Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_DeltaTime = { "DeltaTime", nullptr, (EPropertyFlags)0x0010000000000080, UECodeGen_Private::EPropertyGenFlags::Float, RF_Public|RF_Transient|RF_MarkAsNative, nullptr, nullptr, 1, STRUCT_OFFSET(SoulForgeBridge_eventUpdateAllPowers_Parms, DeltaTime), METADATA_PARAMS(0, nullptr) };
const UECodeGen_Private::FPropertyParamsBase* const Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::PropPointers[] = {
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_Payloads_Inner,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_Payloads,
	(const UECodeGen_Private::FPropertyParamsBase*)&Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::NewProp_DeltaTime,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::PropPointers) < 2048);
// ********** End Function UpdateAllPowers Property Definitions ************************************
const UECodeGen_Private::FFunctionParams Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::FuncParams = { { (UObject*(*)())Z_Construct_UClass_USoulForgeBridge, nullptr, "UpdateAllPowers", 	Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::PropPointers, 
	UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::PropPointers), 
sizeof(Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::SoulForgeBridge_eventUpdateAllPowers_Parms),
RF_Public|RF_Transient|RF_MarkAsNative, (EFunctionFlags)0x04422401, 0, 0, METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::Function_MetaDataParams), Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::Function_MetaDataParams)},  };
static_assert(sizeof(Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::SoulForgeBridge_eventUpdateAllPowers_Parms) < MAX_uint16);
UFunction* Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers()
{
	static UFunction* ReturnFunction = nullptr;
	if (!ReturnFunction)
	{
		UECodeGen_Private::ConstructUFunction(&ReturnFunction, Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers_Statics::FuncParams);
	}
	return ReturnFunction;
}
DEFINE_FUNCTION(USoulForgeBridge::execUpdateAllPowers)
{
	P_GET_TARRAY_REF(FActivePowerPayload,Z_Param_Out_Payloads);
	P_GET_PROPERTY(FFloatProperty,Z_Param_DeltaTime);
	P_FINISH;
	P_NATIVE_BEGIN;
	USoulForgeBridge::UpdateAllPowers(Z_Param_Out_Payloads,Z_Param_DeltaTime);
	P_NATIVE_END;
}
// ********** End Class USoulForgeBridge Function UpdateAllPowers **********************************

// ********** Begin Class USoulForgeBridge *********************************************************
FClassRegistrationInfo Z_Registration_Info_UClass_USoulForgeBridge;
UClass* USoulForgeBridge::GetPrivateStaticClass()
{
	using TClass = USoulForgeBridge;
	if (!Z_Registration_Info_UClass_USoulForgeBridge.InnerSingleton)
	{
		GetPrivateStaticClassBody(
			TClass::StaticPackage(),
			TEXT("SoulForgeBridge"),
			Z_Registration_Info_UClass_USoulForgeBridge.InnerSingleton,
			StaticRegisterNativesUSoulForgeBridge,
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
	return Z_Registration_Info_UClass_USoulForgeBridge.InnerSingleton;
}
UClass* Z_Construct_UClass_USoulForgeBridge_NoRegister()
{
	return USoulForgeBridge::GetPrivateStaticClass();
}
struct Z_Construct_UClass_USoulForgeBridge_Statics
{
#if WITH_METADATA
	static constexpr UECodeGen_Private::FMetaDataPairParam Class_MetaDataParams[] = {
		{ "BlueprintType", "true" },
#if !UE_BUILD_SHIPPING
		{ "Comment", "// ==================================================\n// 4. CLASE PUENTE\n// ==================================================\n" },
#endif
		{ "IncludePath", "SoulForgeBridge.h" },
		{ "IsBlueprintBase", "true" },
		{ "ModuleRelativePath", "Public/SoulForgeBridge.h" },
#if !UE_BUILD_SHIPPING
		{ "ToolTip", "4. CLASE PUENTE" },
#endif
	};
#endif // WITH_METADATA

// ********** Begin Class USoulForgeBridge constinit property declarations *************************
// ********** End Class USoulForgeBridge constinit property declarations ***************************
	static constexpr UE::CodeGen::FClassNativeFunction Funcs[] = {
		{ .NameUTF8 = UTF8TEXT("ActivarPoder"), .Pointer = &USoulForgeBridge::execActivarPoder },
		{ .NameUTF8 = UTF8TEXT("AnclarObjeto"), .Pointer = &USoulForgeBridge::execAnclarObjeto },
		{ .NameUTF8 = UTF8TEXT("ApplySettings"), .Pointer = &USoulForgeBridge::execApplySettings },
		{ .NameUTF8 = UTF8TEXT("BorrarProxyEnRust"), .Pointer = &USoulForgeBridge::execBorrarProxyEnRust },
		{ .NameUTF8 = UTF8TEXT("CalcularOnda"), .Pointer = &USoulForgeBridge::execCalcularOnda },
		{ .NameUTF8 = UTF8TEXT("DetonarMilitar"), .Pointer = &USoulForgeBridge::execDetonarMilitar },
		{ .NameUTF8 = UTF8TEXT("DetonarNativo"), .Pointer = &USoulForgeBridge::execDetonarNativo },
		{ .NameUTF8 = UTF8TEXT("FiltrarInstancias"), .Pointer = &USoulForgeBridge::execFiltrarInstancias },
		{ .NameUTF8 = UTF8TEXT("GetExplosionPreview"), .Pointer = &USoulForgeBridge::execGetExplosionPreview },
		{ .NameUTF8 = UTF8TEXT("GetPerformanceState"), .Pointer = &USoulForgeBridge::execGetPerformanceState },
		{ .NameUTF8 = UTF8TEXT("Initialize"), .Pointer = &USoulForgeBridge::execInitialize },
		{ .NameUTF8 = UTF8TEXT("LimpiarNucleo"), .Pointer = &USoulForgeBridge::execLimpiarNucleo },
		{ .NameUTF8 = UTF8TEXT("RegisterProxy"), .Pointer = &USoulForgeBridge::execRegisterProxy },
		{ .NameUTF8 = UTF8TEXT("SetGlobalChaos"), .Pointer = &USoulForgeBridge::execSetGlobalChaos },
		{ .NameUTF8 = UTF8TEXT("SetGlobalPower"), .Pointer = &USoulForgeBridge::execSetGlobalPower },
		{ .NameUTF8 = UTF8TEXT("SetGroundZ"), .Pointer = &USoulForgeBridge::execSetGroundZ },
		{ .NameUTF8 = UTF8TEXT("SetProxyMilitar"), .Pointer = &USoulForgeBridge::execSetProxyMilitar },
		{ .NameUTF8 = UTF8TEXT("Shutdown"), .Pointer = &USoulForgeBridge::execShutdown },
		{ .NameUTF8 = UTF8TEXT("SpawnNodeOptimized"), .Pointer = &USoulForgeBridge::execSpawnNodeOptimized },
		{ .NameUTF8 = UTF8TEXT("SpawnNodesBulk"), .Pointer = &USoulForgeBridge::execSpawnNodesBulk },
		{ .NameUTF8 = UTF8TEXT("TickPhysics"), .Pointer = &USoulForgeBridge::execTickPhysics },
		{ .NameUTF8 = UTF8TEXT("UpdateAllPowers"), .Pointer = &USoulForgeBridge::execUpdateAllPowers },
	};
	static UObject* (*const DependentSingletons[])();
	static constexpr FClassFunctionLinkInfo FuncInfo[] = {
		{ &Z_Construct_UFunction_USoulForgeBridge_ActivarPoder, "ActivarPoder" }, // 3167119067
		{ &Z_Construct_UFunction_USoulForgeBridge_AnclarObjeto, "AnclarObjeto" }, // 2344394911
		{ &Z_Construct_UFunction_USoulForgeBridge_ApplySettings, "ApplySettings" }, // 1087382506
		{ &Z_Construct_UFunction_USoulForgeBridge_BorrarProxyEnRust, "BorrarProxyEnRust" }, // 614299429
		{ &Z_Construct_UFunction_USoulForgeBridge_CalcularOnda, "CalcularOnda" }, // 3728353418
		{ &Z_Construct_UFunction_USoulForgeBridge_DetonarMilitar, "DetonarMilitar" }, // 332403523
		{ &Z_Construct_UFunction_USoulForgeBridge_DetonarNativo, "DetonarNativo" }, // 195762033
		{ &Z_Construct_UFunction_USoulForgeBridge_FiltrarInstancias, "FiltrarInstancias" }, // 932500953
		{ &Z_Construct_UFunction_USoulForgeBridge_GetExplosionPreview, "GetExplosionPreview" }, // 2246220172
		{ &Z_Construct_UFunction_USoulForgeBridge_GetPerformanceState, "GetPerformanceState" }, // 576613424
		{ &Z_Construct_UFunction_USoulForgeBridge_Initialize, "Initialize" }, // 3711600770
		{ &Z_Construct_UFunction_USoulForgeBridge_LimpiarNucleo, "LimpiarNucleo" }, // 363529589
		{ &Z_Construct_UFunction_USoulForgeBridge_RegisterProxy, "RegisterProxy" }, // 3352854138
		{ &Z_Construct_UFunction_USoulForgeBridge_SetGlobalChaos, "SetGlobalChaos" }, // 511066451
		{ &Z_Construct_UFunction_USoulForgeBridge_SetGlobalPower, "SetGlobalPower" }, // 1912902864
		{ &Z_Construct_UFunction_USoulForgeBridge_SetGroundZ, "SetGroundZ" }, // 3071926169
		{ &Z_Construct_UFunction_USoulForgeBridge_SetProxyMilitar, "SetProxyMilitar" }, // 558783063
		{ &Z_Construct_UFunction_USoulForgeBridge_Shutdown, "Shutdown" }, // 394150731
		{ &Z_Construct_UFunction_USoulForgeBridge_SpawnNodeOptimized, "SpawnNodeOptimized" }, // 1074832370
		{ &Z_Construct_UFunction_USoulForgeBridge_SpawnNodesBulk, "SpawnNodesBulk" }, // 3866671945
		{ &Z_Construct_UFunction_USoulForgeBridge_TickPhysics, "TickPhysics" }, // 529359267
		{ &Z_Construct_UFunction_USoulForgeBridge_UpdateAllPowers, "UpdateAllPowers" }, // 3932686753
	};
	static_assert(UE_ARRAY_COUNT(FuncInfo) < 2048);
	static constexpr FCppClassTypeInfoStatic StaticCppClassTypeInfo = {
		TCppClassTypeTraits<USoulForgeBridge>::IsAbstract,
	};
	static const UECodeGen_Private::FClassParams ClassParams;
}; // struct Z_Construct_UClass_USoulForgeBridge_Statics
UObject* (*const Z_Construct_UClass_USoulForgeBridge_Statics::DependentSingletons[])() = {
	(UObject* (*)())Z_Construct_UClass_UObject,
	(UObject* (*)())Z_Construct_UPackage__Script_SoulForgePhysX,
};
static_assert(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeBridge_Statics::DependentSingletons) < 16);
const UECodeGen_Private::FClassParams Z_Construct_UClass_USoulForgeBridge_Statics::ClassParams = {
	&USoulForgeBridge::StaticClass,
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
	METADATA_PARAMS(UE_ARRAY_COUNT(Z_Construct_UClass_USoulForgeBridge_Statics::Class_MetaDataParams), Z_Construct_UClass_USoulForgeBridge_Statics::Class_MetaDataParams)
};
void USoulForgeBridge::StaticRegisterNativesUSoulForgeBridge()
{
	UClass* Class = USoulForgeBridge::StaticClass();
	FNativeFunctionRegistrar::RegisterFunctions(Class, MakeConstArrayView(Z_Construct_UClass_USoulForgeBridge_Statics::Funcs));
}
UClass* Z_Construct_UClass_USoulForgeBridge()
{
	if (!Z_Registration_Info_UClass_USoulForgeBridge.OuterSingleton)
	{
		UECodeGen_Private::ConstructUClass(Z_Registration_Info_UClass_USoulForgeBridge.OuterSingleton, Z_Construct_UClass_USoulForgeBridge_Statics::ClassParams);
	}
	return Z_Registration_Info_UClass_USoulForgeBridge.OuterSingleton;
}
USoulForgeBridge::USoulForgeBridge(const FObjectInitializer& ObjectInitializer) : Super(ObjectInitializer) {}
DEFINE_VTABLE_PTR_HELPER_CTOR_NS(, USoulForgeBridge);
USoulForgeBridge::~USoulForgeBridge() {}
// ********** End Class USoulForgeBridge ***********************************************************

// ********** Begin Registration *******************************************************************
struct Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics
{
	static constexpr FEnumRegisterCompiledInInfo EnumInfo[] = {
		{ ESoulForgeDamagePreset_StaticEnum, TEXT("ESoulForgeDamagePreset"), &Z_Registration_Info_UEnum_ESoulForgeDamagePreset, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 391389145U) },
		{ EPhysicsLOD_StaticEnum, TEXT("EPhysicsLOD"), &Z_Registration_Info_UEnum_EPhysicsLOD, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 2707074426U) },
		{ EMaterialType_StaticEnum, TEXT("EMaterialType"), &Z_Registration_Info_UEnum_EMaterialType, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 497896940U) },
		{ EExplosiveType_StaticEnum, TEXT("EExplosiveType"), &Z_Registration_Info_UEnum_EExplosiveType, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 1116665067U) },
		{ EPowerType_StaticEnum, TEXT("EPowerType"), &Z_Registration_Info_UEnum_EPowerType, CONSTRUCT_RELOAD_VERSION_INFO(FEnumReloadVersionInfo, 1062630852U) },
	};
	static constexpr FStructRegisterCompiledInInfo ScriptStructInfo[] = {
		{ FSoulForgeFragmentData::StaticStruct, Z_Construct_UScriptStruct_FSoulForgeFragmentData_Statics::NewStructOps, TEXT("SoulForgeFragmentData"),&Z_Registration_Info_UScriptStruct_FSoulForgeFragmentData, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FSoulForgeFragmentData), 3758934569U) },
		{ FEngineState::StaticStruct, Z_Construct_UScriptStruct_FEngineState_Statics::NewStructOps, TEXT("EngineState"),&Z_Registration_Info_UScriptStruct_FEngineState, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FEngineState), 3212734741U) },
		{ FShockwaveData::StaticStruct, Z_Construct_UScriptStruct_FShockwaveData_Statics::NewStructOps, TEXT("ShockwaveData"),&Z_Registration_Info_UScriptStruct_FShockwaveData, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FShockwaveData), 3033931910U) },
		{ FActivePowerPayload::StaticStruct, Z_Construct_UScriptStruct_FActivePowerPayload_Statics::NewStructOps, TEXT("ActivePowerPayload"),&Z_Registration_Info_UScriptStruct_FActivePowerPayload, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FActivePowerPayload), 442066524U) },
		{ FRenderData::StaticStruct, Z_Construct_UScriptStruct_FRenderData_Statics::NewStructOps, TEXT("RenderData"),&Z_Registration_Info_UScriptStruct_FRenderData, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FRenderData), 1047554499U) },
		{ FActivePowerInstance::StaticStruct, Z_Construct_UScriptStruct_FActivePowerInstance_Statics::NewStructOps, TEXT("ActivePowerInstance"),&Z_Registration_Info_UScriptStruct_FActivePowerInstance, CONSTRUCT_RELOAD_VERSION_INFO(FStructReloadVersionInfo, sizeof(FActivePowerInstance), 3403567033U) },
	};
	static constexpr FClassRegisterCompiledInInfo ClassInfo[] = {
		{ Z_Construct_UClass_USoulForgeBridge, USoulForgeBridge::StaticClass, TEXT("USoulForgeBridge"), &Z_Registration_Info_UClass_USoulForgeBridge, CONSTRUCT_RELOAD_VERSION_INFO(FClassReloadVersionInfo, sizeof(USoulForgeBridge), 3865647248U) },
	};
}; // Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics 
static FRegisterCompiledInInfo Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_1184881266{
	TEXT("/Script/SoulForgePhysX"),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::ClassInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::ClassInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::ScriptStructInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::ScriptStructInfo),
	Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::EnumInfo, UE_ARRAY_COUNT(Z_CompiledInDeferFile_FID_proyectos_unreal_explosion_Plugins_SoulForgePhysX_core_cpp_Source_SoulForgePhysX_Public_SoulForgeBridge_h__Script_SoulForgePhysX_Statics::EnumInfo),
};
// ********** End Registration *********************************************************************

PRAGMA_ENABLE_DEPRECATION_WARNINGS
