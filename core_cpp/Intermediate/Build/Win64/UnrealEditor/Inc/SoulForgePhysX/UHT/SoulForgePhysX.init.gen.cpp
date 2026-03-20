// Copyright Epic Games, Inc. All Rights Reserved.
/*===========================================================================
	Generated code exported from UnrealHeaderTool.
	DO NOT modify this manually! Edit the corresponding .h files instead!
===========================================================================*/

#include "UObject/GeneratedCppIncludes.h"
PRAGMA_DISABLE_DEPRECATION_WARNINGS
void EmptyLinkFunctionForGeneratedCodeSoulForgePhysX_init() {}
static_assert(!UE_WITH_CONSTINIT_UOBJECT, "This generated code can only be compiled with !UE_WITH_CONSTINIT_OBJECT");	SOULFORGEPHYSX_API UFunction* Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature();
	static FPackageRegistrationInfo Z_Registration_Info_UPackage__Script_SoulForgePhysX;
	FORCENOINLINE UPackage* Z_Construct_UPackage__Script_SoulForgePhysX()
	{
		if (!Z_Registration_Info_UPackage__Script_SoulForgePhysX.OuterSingleton)
		{
		static UObject* (*const SingletonFuncArray[])() = {
			(UObject* (*)())Z_Construct_UDelegateFunction_SoulForgePhysX_OnPowerVisualEffect__DelegateSignature,
		};
		static const UECodeGen_Private::FPackageParams PackageParams = {
			"/Script/SoulForgePhysX",
			SingletonFuncArray,
			UE_ARRAY_COUNT(SingletonFuncArray),
			PKG_CompiledIn | 0x00000000,
			0x760DF5D0,
			0x8BA83068,
			METADATA_PARAMS(0, nullptr)
		};
		UECodeGen_Private::ConstructUPackage(Z_Registration_Info_UPackage__Script_SoulForgePhysX.OuterSingleton, PackageParams);
	}
	return Z_Registration_Info_UPackage__Script_SoulForgePhysX.OuterSingleton;
}
static FRegisterCompiledInInfo Z_CompiledInDeferPackage_UPackage__Script_SoulForgePhysX(Z_Construct_UPackage__Script_SoulForgePhysX, TEXT("/Script/SoulForgePhysX"), Z_Registration_Info_UPackage__Script_SoulForgePhysX, CONSTRUCT_RELOAD_VERSION_INFO(FPackageReloadVersionInfo, 0x760DF5D0, 0x8BA83068));
PRAGMA_ENABLE_DEPRECATION_WARNINGS
