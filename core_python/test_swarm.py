"""
test_swarm.py  —  SoulForge PhysX Swarm Test (ctypes + Unreal)
═══════════════════════════════════════════════════════════════
Prueba real del Cerebro de Enjambre:
  1. Carga la DLL de Rust vía ctypes (sin Unreal como intermediario).
  2. Registra N agentes directamente en el núcleo de Rust.
  3. Crea N StaticMeshActors en el nivel activo de Unreal.
  4. Ejecuta un loop donde Rust calcula la física y Python mueve
     los Actores de Unreal a las posiciones que Rust le indica.

Ejecución:
  - Desde el editor de Unreal: Herramientas → Ejecutar script de Python
  - Desde línea de comandos (sin Unreal): python test_swarm.py --dry-run
"""

from __future__ import annotations

import argparse
import ctypes
import os
import sys
import time
from pathlib import Path

# ─── Detección del entorno ────────────────────────────────────────────────────

try:
    import unreal
    _UNREAL = True
except ImportError:
    _UNREAL = False

# ─── Rutas ───────────────────────────────────────────────────────────────────

_SCRIPT_DIR = Path(__file__).parent.resolve()
_DLL_PATH   = _SCRIPT_DIR.parent / "core_cpp" / "Binaries" / "Win64" / "soulforge_physx.dll"

# ─── Carga de la DLL de Rust ──────────────────────────────────────────────────

def load_dll(dll_path: Path) -> ctypes.CDLL:
    """Carga la DLL de SoulForge-PhysX y define los tipos de sus funciones."""
    if not dll_path.exists():
        raise FileNotFoundError(
            f"DLL no encontrada: {dll_path}\n"
            f"Ejecuta 'python build.py' en la raíz del proyecto primero."
        )

    sf = ctypes.CDLL(str(dll_path))

    # ── Contratos FFI (deben coincidir EXACTAMENTE con ffi.rs) ───────────────

    # sf_init_physics() -> i32  (1=ok, 0=error)
    sf.sf_init_physics.argtypes = []
    sf.sf_init_physics.restype  = ctypes.c_int

    # sf_shutdown_physics() -> i32
    sf.sf_shutdown_physics.argtypes = []
    sf.sf_shutdown_physics.restype  = ctypes.c_int

    # sf_tick_physics(delta_seconds: f32) -> i32  (nº sub-pasos, -1=error)
    sf.sf_tick_physics.argtypes = [ctypes.c_float]
    sf.sf_tick_physics.restype  = ctypes.c_int

    # sf_register_swarm_agent(x, y, z: f32) -> i32  (AgentID ≥ 0, -1=error)
    sf.sf_register_swarm_agent.argtypes = [ctypes.c_float, ctypes.c_float, ctypes.c_float]
    sf.sf_register_swarm_agent.restype  = ctypes.c_int

    # sf_get_swarm_agent_pos(id: i32, out_x, out_y, out_z: *f32) -> bool
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

    print(f"  ✓ DLL cargada: {dll_path.name}")
    return sf


# ─── Inicialización del enjambre en Rust ──────────────────────────────────────

def init_swarm(sf: ctypes.CDLL, count: int) -> list[int]:
    """
    Inicializa el motor y registra `count` agentes en el Cerebro de Enjambre.

    Los agentes se colocan en una línea a lo largo del eje Y, separados 150 cm
    (unidades de Unreal), a 200 cm de altura (Z).

    Retorna la lista de AgentIDs asignados por Rust.
    """
    ret = sf.sf_init_physics()
    if ret == 0:
        raise RuntimeError("sf_init_physics() falló. Revisa los logs de la DLL.")
    print(f"  ✓ Motor de física iniciado.")

    agent_ids: list[int] = []
    for i in range(count):
        x = 0.0
        y = float(i) * 150.0   # separación inicial: 150 cm entre agentes
        z = 200.0               # altura inicial sobre el suelo
        aid = sf.sf_register_swarm_agent(
            ctypes.c_float(x),
            ctypes.c_float(y),
            ctypes.c_float(z),
        )
        if aid < 0:
            print(f"  ⚠  Agente #{i} no pudo registrarse.")
        else:
            agent_ids.append(aid)

    print(f"  ✓ {len(agent_ids)}/{count} agentes registrados en el enjambre de Rust.")
    return agent_ids


# ─── Lectura de posición desde Rust ──────────────────────────────────────────

def get_agent_position(sf: ctypes.CDLL, agent_id: int) -> tuple[float, float, float] | None:
    """Pide a Rust la posición actual del agente con el ID dado."""
    x, y, z = ctypes.c_float(0), ctypes.c_float(0), ctypes.c_float(0)
    found = sf.sf_get_swarm_agent_pos(
        ctypes.c_int(agent_id),
        ctypes.byref(x),
        ctypes.byref(y),
        ctypes.byref(z),
    )
    return (x.value, y.value, z.value) if found else None


# ─── Spawn de Actores en Unreal ───────────────────────────────────────────────

def spawn_sphere_actors(count: int) -> list:
    """Crea `count` StaticMeshActors esfera en el nivel activo de Unreal."""
    if not _UNREAL:
        return []

    editor_subs = unreal.get_editor_subsystem(unreal.EditorActorSubsystem)
    mesh = unreal.load_asset("/Engine/BasicShapes/Sphere")

    actors = []
    for i in range(count):
        loc = unreal.Vector(0.0, float(i) * 150.0, 200.0)
        actor = editor_subs.spawn_actor_from_class(unreal.StaticMeshActor, loc)
        if actor and mesh:
            actor.static_mesh_component.set_static_mesh(mesh)
            actor.set_actor_label(f"SF_SwarmAgent_{i:03d}")
        actors.append(actor)

    print(f"  ✓ {len(actors)} Actores esfera creados en Unreal.")
    return actors


# ─── El Loop de la Verdad ─────────────────────────────────────────────────────

def run_simulation_loop(
    sf:        ctypes.CDLL,
    agent_ids: list[int],
    actors:    list,
    steps:     int = 200,
    fixed_dt:  float = 1.0 / 60.0,
    dry_run:   bool = False,
) -> None:
    """
    Loop principal de la prueba:
      - Rust calcula la física (Boids: Separación, Alineación, Cohesión).
      - Python lee las posiciones desde Rust y mueve los Actores de Unreal.

    Args:
        steps:    Número de frames/pasos a simular.
        fixed_dt: Tiempo de frame a pasar a Rust (simula 60 FPS constantes).
        dry_run:  Si True, no usa Unreal; solo mide el rendimiento de Rust.
    """
    print(f"\n  ▸ Iniciando loop: {steps} pasos | dt={fixed_dt:.4f}s | agentes={len(agent_ids)}")

    t_start    = time.perf_counter()
    panic_count = 0

    for step in range(steps):
        # ── 1. Rust calcula el siguiente estado del enjambre ─────────────────
        sub_steps = sf.sf_tick_physics(ctypes.c_float(fixed_dt))

        if sub_steps < 0:
            panic_count += 1
            print(f"  ⚠  Paso {step}: sf_tick_physics retornó error (panic capturado).")
            if panic_count >= 3:
                print("  ✗ Demasiados errores. Abortando simulación.")
                break
            continue

        # ── 2. Python lee posiciones desde Rust y mueve los Actores ──────────
        for i, agent_id in enumerate(agent_ids):
            pos = get_agent_position(sf, agent_id)
            if pos is None:
                continue

            rx, ry, rz = pos   # posición calculada por los Boids de Rust

            if not dry_run and _UNREAL and i < len(actors) and actors[i]:
                new_loc = unreal.Vector(rx, ry, rz)
                actors[i].set_actor_location(new_loc, False, False)

        # ── 3. Refrescar viewports de Unreal cada 10 pasos ───────────────────
        if not dry_run and _UNREAL and step % 10 == 0:
            unreal.EditorLevelLibrary.editor_invalidate_viewports()

    t_end     = time.perf_counter()
    elapsed   = t_end - t_start
    fps_equiv = steps / elapsed if elapsed > 0 else float('inf')

    print(f"\n  ══ Resultado de la prueba ══")
    print(f"     Pasos simulados : {steps}")
    print(f"     Tiempo total    : {elapsed:.3f}s")
    print(f"     Equivalente FPS : {fps_equiv:.1f}")
    print(f"     Agentes activos : {sf.sf_swarm_agent_count()}")
    print(f"     Panics capturados: {panic_count}")
    print(f"  {'✓ Prueba exitosa.' if panic_count == 0 else '⚠ Prueba con errores.'}")


# ─── Cleanup ──────────────────────────────────────────────────────────────────

def cleanup(sf: ctypes.CDLL, actors: list) -> None:
    """Apaga el motor de Rust y elimina los Actores de Unreal."""
    sf.sf_shutdown_physics()
    print("  ✓ Motor de física apagado.")

    if _UNREAL and actors:
        editor_subs = unreal.get_editor_subsystem(unreal.EditorActorSubsystem)
        destroyed = 0
        for actor in actors:
            if actor:
                editor_subs.destroy_actor(actor)
                destroyed += 1
        print(f"  ✓ {destroyed} Actores eliminados del nivel.")


# ─── Punto de entrada ─────────────────────────────────────────────────────────

def main(agent_count: int = 50, steps: int = 200, dry_run: bool = False) -> None:
    print("\n" + "═" * 60)
    print("  SoulForge PhysX  —  Prueba del Cerebro de Enjambre")
    print("═" * 60)
    print(f"  Agentes : {agent_count}")
    print(f"  Pasos   : {steps}")
    print(f"  Modo    : {'Dry-run (sin Unreal)' if dry_run else 'Unreal Editor'}")
    print()

    sf      = load_dll(_DLL_PATH)
    ids     = init_swarm(sf, agent_count)
    actors  = [] if dry_run else spawn_sphere_actors(agent_count)

    try:
        run_simulation_loop(
            sf       = sf,
            agent_ids= ids,
            actors   = actors,
            steps    = steps,
            dry_run  = dry_run,
        )
    finally:
        cleanup(sf, actors)


# ─── CLI (para pruebas fuera de Unreal) ──────────────────────────────────────

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="SoulForge Swarm Test")
    parser.add_argument("--agents",   type=int, default=50,  help="Número de agentes (default: 50)")
    parser.add_argument("--steps",    type=int, default=200, help="Pasos de simulación (default: 200)")
    parser.add_argument("--dry-run",  action="store_true",   help="Ejecutar sin Unreal")
    args = parser.parse_args()

    main(agent_count=args.agents, steps=args.steps, dry_run=args.dry_run)
