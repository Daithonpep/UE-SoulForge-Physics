@echo off
setlocal enabledelayedexpansion

:: ============================================================
:: SOULFORGE: DAITHON STARTUP SCRIPT
:: ============================================================
:: Este script inicia el Bridge de Rust cargando el cerebro
:: más avanzado de Daithon y prepara el entorno.

title SoulForge Bridge - Daithon Engine
mode con: cols=100 lines=30
color 0b

echo.
echo  ############################################################
echo  #                                                          #
echo  #          DAITHON BRIDGE - HARDWARE ACCELERATED           #
echo  #           (RDNA3 iGPU + AVX-512 + WORLD MODEL)           #
echo  #                                                          #
echo  ############################################################
echo.

:: 1. Localizar el checkpoint más avanzado
echo [SYSTEM] Escaneando red neuronal...
set "CHECKPOINT_DIR=checkpoints"
set "LATEST_CP="

for /f "delims=" %%a in ('dir /b /o-n "%CHECKPOINT_DIR%\epoch_*.json"') do (
    set "LATEST_CP=%CHECKPOINT_DIR%\%%a"
    goto :found
)

:found
if defined LATEST_CP (
    echo [BRAIN] Detectado cerebro avanzado: !LATEST_CP!
) else (
    echo [BRAIN] No se detectaron checkpoints. Iniciando cerebro base.
    set "LATEST_CP="
)

:: 2. Verificar compilacion
echo [BUILD] Verificando integridad del binario...
cargo check
if %errorlevel% neq 0 (
    echo [ERROR] Error de integridad en Rust. Revisa los mensajes arriba.
    pause
    exit /b %errorlevel%
)

:: 3. Iniciar consola y servidor
echo [READY] Despegue de Daithon en T-3...
timeout /t 3 > nul

:: Lanzar el dashboard en el navegador predeterminado (esperamos 5s a que el server suba)
echo [WEB] Abriendo Consola del World Model...
start http://localhost:8765/dashboard/world_model.html

echo [GO] Iniciando Servidor de Rust...
echo ------------------------------------------------------------
if defined LATEST_CP (
    cargo run --bin daithon_bridge -- "!LATEST_CP!"
) else (
    cargo run --bin daithon_bridge
)

if %errorlevel% neq 0 (
    echo [FATAL] El servidor se ha detenido abruptamente.
    pause
)

endlocal
