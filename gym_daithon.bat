@echo off
title SOULFORGE GYM - DAITHON BRIDGE
cls
echo ===================================================
echo   🧪 SOULFORGE TRAINING GYMNASIUM IS STARTING
echo ===================================================
echo.
echo [1/3] Verificando entorno de Rust...
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Cargo no esta instalado o no esta en el PATH.
    pause
    exit /b
)

echo [2/3] Preparando directorios de datos...
if not exist "references" mkdir "references"
if not exist "checkpoints" mkdir "checkpoints"
if not exist "static" mkdir "static"

echo [3/3] Compilando y ejecutando Daithon Bridge...
echo.
echo URL del Dashboard: http://localhost:8765/dashboard
echo.
echo Lanzando Dashboard en el navegador...
start http://localhost:8765/dashboard
echo.
echo Presiona Ctrl+C para detener el entrenamiento.
echo ---------------------------------------------------
echo.

cargo run --release --bin daithon_bridge

pause
