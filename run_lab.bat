@echo off
title SoulForge PhysX Lab — Launcher
mode con: cols=90 lines=25
color 0b

echo.
echo   ══════════════════════════════════════════════════════
echo    SoulForge PhysX Lab — Iniciando Laboratorio...
echo   ══════════════════════════════════════════════════════
echo.

cd /d "%~dp0"

:: Verificar si Python está en el PATH
where python >nul 2>1
if %errorlevel% neq 0 (
    echo [ERROR] No se encuentra Python en el sistema. Asegurate de instalarlo.
    pause
    exit /b
)

echo [1/3] Verificando dependencias...
pip install websockets >nul 2>1

echo [2/3] Arrancando servidor Backend (Rust Core Bridge)...
start /b python lab/server.py

timeout /t 2 /nobreak >nul

echo [3/3] Arrancando servidor Frontend (HTTP Static)...
start /b python -m http.server 8000 --directory lab

timeout /t 2 /nobreak >nul

echo [4/4] Abriendo interfaz en el navegador...
start http://localhost:8000/index.html

echo.
echo   [OK] !Todo listo! El laboratorio esta corriendo.
echo   Para cerrar: Cierra esta ventana o presiona Ctrl+C.
echo.
echo   ══════════════════════════════════════════════════════
echo.
pause
