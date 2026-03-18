#pragma once

#include "Modules/ModuleManager.h"

// Tipos de función FFI
typedef const char* (*FDestroyProxyFunc)(const char*, float, float, float, float);
typedef const char* (*FDestroyProxyAvanzadoFunc)(const char*, float, float, int32, int32, float, float, float);

class FSoulForgePhysXModule : public IModuleInterface
{
public:
    virtual void StartupModule() override;
    virtual void ShutdownModule() override;

    static FDestroyProxyFunc GetDestroyProxyFunc() { return CppDestroyProxy; }
    static FDestroyProxyAvanzadoFunc GetDestroyProxyAvanzadoFunc() { return CppDestroyProxyAvanzado; }

private:
    void* PhysXHandle = nullptr;

    static FDestroyProxyFunc CppDestroyProxy;
    static FDestroyProxyAvanzadoFunc CppDestroyProxyAvanzado;
};
