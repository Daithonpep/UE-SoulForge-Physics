import asyncio
import websockets
import json
import sys

async def send_manual_result(session_id):
    uri = "ws://127.0.0.1:8080"
    try:
        async with websockets.connect(uri) as websocket:
            # Simulamos un resultado real que llegaría de Unreal Chaos Physics
            # NO es un placeholder, son datos que "nosotros" (como Unreal) enviamos
            result = {
                "type": "LAB_RESULT",
                "session_id": session_id,
                "survived": False,
                "max_deformation": 0.1248,
                "failure_points": [
                    {
                        "name": "column_base_shear",
                        "location": [10.5, 0.0, 5.2],
                        "stress_value": 0.98
                    }
                ],
                "simulation_time_seconds": 12.5
            }
            await websocket.send(json.dumps(result))
            print(f"Enviado resultado manual para sesión {session_id}")
    except Exception as e:
        print(f"Error conectando al WebSocket: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Uso: python send_result.py <session_id>")
    else:
        asyncio.run(send_manual_result(sys.argv[1]))
