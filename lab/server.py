"""
server.py - SoulForge PhysX Lab Backend
========================================
Servidor WebSocket que:
  1. Carga la DLL de Rust va ctypes.
  2. Inicializa el motor de fsica.
  3. Registra agentes en el Cerebro de Enjambre.
  4. Ejecuta el loop de fsica a 60 Hz.
  5. Transmite posiciones de agentes al Lab (index.html) por WebSocket.
  6. Acepta comandos del Lab (preset, param, purge).

Uso:
  pip install websockets
  python lab/server.py
   Abrir lab/index.html en el navegador
"""

import asyncio
import ctypes
import json
import logging
import math
import os
import sys
import time
from pathlib import Path

# --- Logging ------------------------------------------------------------------
logging.basicConfig(
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%H:%M:%S",
    level=logging.INFO,
)
log = logging.getLogger("sf-lab")

# --- Rutas ------------------------------------------------------------------
_ROOT    = Path(__file__).parent.parent.resolve()
_DLL     = _ROOT / "core_cpp" / "Binaries" / "Win64" / "soulforge_physx.dll"
_PORT    = 8765
_TICK_HZ = 60          # Hz de transmisin al cliente
_AGENT_N = 50          # agentes iniciales

# --- Estado global ------------------------------------------------------------
g_sf       = None       # CDLL
g_agents   = []         # lista de AgentIDs en Rust
g_clients  = set()      # websockets conectados
g_panics   = 0
g_preset   = "swarm"
g_params   = {
    "gravity":    981.0,
    "swarm":      0.5,
    "speed":      300.0,
    "cohesion":   0.8,
    "sep":        1.8,
    "agents":     50,
}

# --- Carga de la DLL ---------------------------------------------------------

def load_dll() -> ctypes.CDLL:
    if not _DLL.exists():
        log.error(f"DLL no encontrada: {_DLL}")
        log.error("Ejecuta: python build.py")
        sys.exit(1)

    sf = ctypes.CDLL(str(_DLL))

    # sf_init_physics() -> i32
    sf.sf_init_physics.argtypes = []
    sf.sf_init_physics.restype  = ctypes.c_int

    # sf_shutdown_physics() -> i32
    sf.sf_shutdown_physics.argtypes = []
    sf.sf_shutdown_physics.restype  = ctypes.c_int

    # sf_tick_physics(dt: f32) -> i32
    sf.sf_tick_physics.argtypes = [ctypes.c_float]
    sf.sf_tick_physics.restype  = ctypes.c_int

    # sf_register_swarm_agent(x,y,z: f32, dormant: i32) -> i32
    sf.sf_register_swarm_agent.argtypes = [
        ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_int
    ]
    sf.sf_register_swarm_agent.restype  = ctypes.c_int

    # sf_get_swarm_agent_pos(id, *x, *y, *z) -> bool
    sf.sf_get_swarm_agent_pos.argtypes = [
        ctypes.c_int,
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
    ]
    sf.sf_get_swarm_agent_pos.restype = ctypes.c_bool

    # sf_swarm_agent_count() -> i32
    sf.sf_swarm_agent_count.argtypes = []
    sf.sf_swarm_agent_count.restype  = ctypes.c_int

    # Fragment sim
    sf.sf_fragment_active_count.argtypes = []
    sf.sf_fragment_active_count.restype  = ctypes.c_int
    sf.sf_fragment_total_count.argtypes  = []
    sf.sf_fragment_total_count.restype   = ctypes.c_int
    sf.sf_purge_sleeping_fragments.argtypes = []
    sf.sf_purge_sleeping_fragments.restype  = ctypes.c_int

    # sf_register_proxy(id, x,y,z, ex,ey,ez, dens) -> bool
    sf.sf_register_proxy.argtypes = [
        ctypes.c_char_p,
        ctypes.c_float, ctypes.c_float, ctypes.c_float,
        ctypes.c_float, ctypes.c_float, ctypes.c_float,
        ctypes.c_float,
    ]
    sf.sf_register_proxy.restype = ctypes.c_bool

    # sf_activar_poder(type, x,y,z, dx,dy,dz, int, dur) -> void
    sf.sf_activar_poder.argtypes = [
        ctypes.c_int,
        ctypes.c_float, ctypes.c_float, ctypes.c_float,
        ctypes.c_float, ctypes.c_float, ctypes.c_float,
        ctypes.c_float, ctypes.c_float
    ]
    sf.sf_activar_poder.restype = None

    # sf_destroy_proxy(id, preset, energy, dx, dy, dz) -> *char
    sf.sf_destroy_proxy.argtypes = [
        ctypes.c_int, ctypes.c_int, ctypes.c_float,
        ctypes.c_float, ctypes.c_float, ctypes.c_float
    ]
    sf.sf_destroy_proxy.restype  = ctypes.c_char_p

    # sf_free_string(*char) -> void
    sf.sf_free_string.argtypes = [ctypes.c_char_p]
    sf.sf_free_string.restype  = None

    # sf_get_node_positions(*count) -> *f32
    sf.sf_get_node_positions.argtypes = [ctypes.POINTER(ctypes.c_int)]
    sf.sf_get_node_positions.restype  = ctypes.POINTER(ctypes.c_float)

    # sf_get_node_temperatures(*count) -> *f32
    sf.sf_get_node_temperatures.argtypes = [ctypes.POINTER(ctypes.c_int)]
    sf.sf_get_node_temperatures.restype  = ctypes.POINTER(ctypes.c_float)

    log.info(f"DLL cargada: {_DLL.name}")
    return sf


def init_engine(sf: ctypes.CDLL, n: int) -> list[int]:
    ret = sf.sf_init_physics()
    if ret == 0:
        raise RuntimeError("sf_init_physics() fallo.")
    log.info("Motor de fisica iniciado (60 Hz).")

    ids = []
    for i in range(n):
        angle = i * 2.399
        r = 80 * math.sqrt(i / max(n, 1))
        x, y, z = math.cos(angle) * r, math.sin(angle) * r, 200.0
        aid = sf.sf_register_swarm_agent(
            ctypes.c_float(x), ctypes.c_float(y), ctypes.c_float(z), 0
        )
        if aid >= 0:
            ids.append(aid)

    log.info(f"Agentes en el Cerebro de Enjambre: {len(ids)}")
    return ids


def get_all_positions(sf) -> list[dict]:
    """Lee TODAS las posiciones de los nodos activos (Agentes + Poderes + Fragmentos)."""
    count = ctypes.c_int(0)
    ptr_pos = sf.sf_get_node_positions(ctypes.byref(count))
    ptr_temp = sf.sf_get_node_temperatures(ctypes.byref(count))
    
    if not ptr_pos:
        return []
        
    results = []
    n = count.value
    for i in range(n):
        results.append({
            "id": i,
            "x": ptr_pos[i*3],
            "y": ptr_pos[i*3+1],
            "z": ptr_pos[i*3+2],
            "temp": ptr_temp[i] if ptr_temp else 293.0
        })
    return results


# --- WebSocket handlers ------------------------------------------------------

async def handle_client(websocket):
    global g_clients, g_params, g_preset, g_agents, g_panics

    g_clients.add(websocket)
    addr = websocket.remote_address
    log.info(f"Cliente conectado: {addr}")

    try:
        async for raw in websocket:
            try:
                msg = json.loads(raw)
            except json.JSONDecodeError:
                continue

            cmd = msg.get("cmd", "")

            if cmd == "preset":
                g_preset = msg.get("preset", "swarm")
                log.info(f"Preset: {g_preset}")
                if g_sf:
                    _reset_agents_for_preset(g_preset)

            elif cmd == "param":
                key = msg.get("key", "")
                val = msg.get("value", 0)
                if key in g_params:
                    g_params[key] = val
                    if key == "agents" and g_sf:
                        _resize_agents(int(val))

            elif cmd == "impact":
                if g_sf and g_preset == "wall":
                    x_click = msg.get("x", 0)
                    y_click = msg.get("y", 0)
                    rad     = msg.get("radius", 80)
                    
                    # Convertimos click a coordenadas Rust
                    rx = (x_click - 550) * 2.0
                    ry = (y_click - 350) * 2.0
                    
                    if hasattr(g_sf, "wall_proxy_id") and g_sf.wall_proxy_id >= 0:
                        # Preset codes: 2=Impact, 3=ShapedCharge
                        preset_code = 3 if rad > 150 else 2
                        energy      = 3000.0 if rad > 150 else 800.0
                        
                        log.info(f"Impacto: ({rx:.1f}, {ry:.1f}) | Preset: {preset_code}")
                        
                        # Direccin del impacto (desde la cmara hacia +Z en este lab 2D)
                        res = g_sf.sf_destroy_proxy(
                            g_sf.wall_proxy_id, 
                            preset_code, 
                            ctypes.c_float(energy),
                            ctypes.c_float(0.0), ctypes.c_float(0.0), ctypes.c_float(1.0)
                        )
                        g_sf.sf_free_string(res)
                        g_sf.wall_proxy_id = -1

            elif cmd == "power":
                if g_sf:
                    p_type = msg.get("type", 0)
                    x = msg.get("x", 0.0)
                    y = msg.get("y", 0.0)
                    z = msg.get("z", 200.0)
                    dx = msg.get("dx", 1.0)
                    dy = msg.get("dy", 0.0)
                    dz = msg.get("dz", 0.0)
                    intensity = msg.get("intensity", 1.0)
                    duration = msg.get("duration", 8.0)
                    
                    log.info(f"Poder activado: Tipo {p_type} en ({x}, {y}, {z})")
                    g_sf.sf_activar_poder(
                        ctypes.c_int(p_type),
                        ctypes.c_float(x), ctypes.c_float(y), ctypes.c_float(z),
                        ctypes.c_float(dx), ctypes.c_float(dy), ctypes.c_float(dz),
                        ctypes.c_float(intensity),
                        ctypes.c_float(duration)
                    )
                
            elif cmd == "purge":
                if g_sf:
                    purged = g_sf.sf_purge_sleeping_fragments()
                    log.info(f"Purge: {purged} fragmentos dormidos eliminados.")
                    await websocket.send(json.dumps({
                        "type": "telemetry",
                        "frag_active": g_sf.sf_fragment_active_count(),
                        "frag_sleep":  g_sf.sf_fragment_total_count() - g_sf.sf_fragment_active_count(),
                        "panics": g_panics,
                    }))

    except Exception as e:
        log.error(f"WS error: {e}")
    finally:
        g_clients.discard(websocket)
        log.info(f"Cliente desconectado: {addr}")


def _reset_agents_for_preset(preset: str):
    """Fuerza una redistribucin de agentes para el preset dado."""
    if preset == "wall" and g_sf:
        # Registrar una pared fsica (AABB) en Rust para que el impacto sea real
        if hasattr(g_sf, "wall_proxy_id") and g_sf.wall_proxy_id >= 0:
            return # ya existe
            
        # Posicin (0,0,200), Dimensiones (300, 200, 200), Densidad Concrete
        muro_id = b"wall_lab"
        res = g_sf.sf_register_proxy(
            muro_id,
            0.0, 0.0, 200.0,   # Center
            150.0, 100.0, 10.0, # HalfExtents (una pared flaca)
            0.0024             # Density
        )
        if res:
            g_sf.wall_proxy_id = 999 # Placeholder id or tracking
            log.info(f"Muro fsico registrado en Rust: {muro_id.decode()}")
        else:
            log.error(f"Fallo al registrar muro fsico.")


def _resize_agents(target_n: int):
    global g_agents
    if not g_sf:
        return
    # Deberan estar dormidos? (Si estamos en modo muro, s)
    dormant = 1 if g_preset == "wall" else 0
    
    # Aadir agentes faltantes
    while len(g_agents) < target_n:
        angle = len(g_agents) * 2.399
        r = 80 * math.sqrt(len(g_agents) / max(target_n, 1))
        aid = g_sf.sf_register_swarm_agent(
            ctypes.c_float(math.cos(angle) * r),
            ctypes.c_float(math.sin(angle) * r),
            ctypes.c_float(200.0),
            ctypes.c_int(dormant)
        )
        if aid >= 0:
            g_agents.append(aid)
    # Eliminar excedentes
    if len(g_agents) > target_n:
        g_agents = g_agents[:target_n]
    log.info(f"Agentes ajustados: {len(g_agents)} (Dormant={dormant})")


# --- Physics Loop (corre en background) ------------------------------------

async def physics_broadcast_loop():
    global g_panics, g_clients, g_sf

    FIXED_DT = 1.0 / _TICK_HZ
    BROADCAST_EVERY = 2   # ticks por broadcast (30 Hz al cliente)
    tick = 0

    if not g_sf:
        log.warning("DLL no cargada  modo demo (sin Rust).")
        return

    while True:
        t0 = time.perf_counter()

        #  1. Tick de Rust ---------------------------------------
        sub_steps = g_sf.sf_tick_physics(ctypes.c_float(FIXED_DT))
        if sub_steps < 0:
            g_panics += 1
            log.error(f"sf_tick_physics panic #{g_panics}")

        tick += 1

        #  2. Broadcast al cliente cada 2 ticks (30 Hz) ---------
        if tick % BROADCAST_EVERY == 0 and g_clients:
            positions = get_all_positions(g_sf)

            frag_active = g_sf.sf_fragment_active_count()
            frag_total  = g_sf.sf_fragment_total_count()
            frag_sleep  = frag_total - frag_active

            payload = json.dumps({
                "type":   "positions",
                "agents": positions,
                "tick":   tick,
            })
            tel_payload = json.dumps({
                "type":        "telemetry",
                "frag_active": frag_active,
                "frag_sleep":  frag_sleep,
                "panics":      g_panics,
            })

            dead = set()
            for ws in list(g_clients):
                try:
                    await ws.send(payload)
                    if tick % 60 == 0:
                        await ws.send(tel_payload)
                except Exception:
                    dead.add(ws)
            g_clients -= dead

        #  3. Mantener el ritmo de 60 Hz ------------------------
        elapsed = time.perf_counter() - t0
        sleep_t = FIXED_DT - elapsed
        if sleep_t > 0:
            await asyncio.sleep(sleep_t)


# --- Entry point ------------------------------------------------------------

async def main():
    global g_sf, g_agents

    print()
    print("  ======================================================")
    print("   SoulForge PhysX Lab - Backend Server")
    print("  ======================================================")
    print(f"  DLL  : {_DLL}")
    print(f"  WS   : ws://localhost:{_PORT}")
    print(f"  UI   : Abre lab/index.html en tu navegador")
    print("  ======================================================")
    print()

    # Cargar DLL y arrancar motor
    try:
        g_sf     = load_dll()
        g_agents = init_engine(g_sf, _AGENT_N)
    except Exception as e:
        import traceback
        traceback.print_exc()
        log.error(f"Error inicializando el motor: {e}")
        log.warning("Iniciando en modo DEMO (sin Rust). La UI sigue funcionando.")
        g_sf = None

    # Importar websockets
    try:
        import websockets
    except ImportError:
        log.error("Instala websockets: pip install websockets")
        sys.exit(1)

    # Lanzar physics loop y servidor WS en paralelo
    async with websockets.serve(handle_client, "localhost", _PORT):
        log.info(f"Server escuchando en ws://localhost:{_PORT}")
        await physics_broadcast_loop()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n  [SoulForge Lab] Servidor detenido.")
        if g_sf:
            g_sf.sf_shutdown_physics()
