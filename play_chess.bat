@echo off
echo ==========================================
echo   DAITHON AGI - CHESS LEARNING SYSTEM
echo ==========================================
echo.
echo   PASO 1: Daithon lee el manual de ajedrez
echo   PASO 2: Extrae reglas, entidades, objetivos
echo   PASO 3: Integra al Grafo Semantico
echo   PASO 4: Busca analogias con dominios conocidos
echo   PASO 5: Practica jugando contra si mismo
echo   PASO 6: Descubre leyes NO escritas en el manual
echo.
echo ==========================================
echo   ABRIENDO INTERFAZ REACT...
start http://localhost:8765/dashboard/chess.html
cargo run --bin daithon_bridge
pause
