import subprocess
import time
import os
import signal
import json
import asyncio
import websockets
import sys

def tail(filename, n=50):
    try:
        with open(filename, "r", encoding='utf-8', errors='ignore') as f:
            lines = f.readlines()
            return lines[-n:]
    except:
        return []

async def send_manual_result(session_id):
    # URI corregida según main.rs (Sin slash final)
    uri = "ws://127.0.0.1:8765/ws"
    async with websockets.connect(uri) as websocket:
        payload = {
            "session_id": session_id,
            "survived": False,
            "max_deformation": 0.1245,
            "failure_points": [
                {"name": "column_base_shear", "location": [10.5, 0.0, 5.2], "stress_value": 0.98}
            ],
            "stress_distribution": {"shear_peak": 0.95, "compression": 0.45},
            "simulation_time_seconds": 12.5
        }
        omni_msg = {
            "stream_type": "LAB_RESULT",
            "payload": payload
        }
        await websocket.send(json.dumps(omni_msg))

def main():
    log_file = "daithon_master.log"
    # Kill existing
    subprocess.run(["taskkill", "/F", "/IM", "daithon_bridge.exe"], capture_output=True)
    
    print("Iniciando Daithon...")
    with open(log_file, "w", encoding='utf-8') as f:
        process = subprocess.Popen(["cargo", "run", "--bin", "daithon_bridge"], 
                                   stdout=f, stderr=subprocess.STDOUT, text=True)
    
    time.sleep(50) # Esperar a que compile y arranque
    
    session_id = None
    output = tail(log_file, 300)
    for line in output:
        if "[LAB] Sesión de experimento creada:" in line:
            parts = line.split(":")
            if len(parts) > 1:
                session_id = parts[-1].strip()
                break
    
    if session_id:
        print(f"Detectada sesión: {session_id}. Enviando feedback de Unreal...")
        try:
            asyncio.run(send_manual_result(session_id))
            time.sleep(12) # Esperar a que procese feedback
        except Exception as e:
            print(f"Error enviando resultado: {e}")
    else:
        print("No se detectó sesión de experimento.")

    # Kill server after test
    process.terminate()
    time.sleep(2)
    process.kill()

    # Re-leer log final
    print("\n\n--- LOG FINAL CRUDO (Birth of Empirical Anchor) ---")
    final_output = tail(log_file, 100)
    for line in final_output:
        sys.stdout.buffer.write(line.encode('utf-8', errors='replace'))
    sys.stdout.flush()

if __name__ == "__main__":
    main()
