"""
sf_unreal_bridge.py
═══════════════════
SoulForge PhysX – Orquestador de Datos (Python)
"El Arquitecto"

Responsabilidades:
  1. Leer el JSON de destrucción / snapshotde escena que emite el núcleo Rust.
  2. Crear automáticamente los Actores y sistemas Niagara en la escena de Unreal.
  3. Importar assets USD generados por SoulForge.

NOTA: Este script se ejecuta dentro del editor de Unreal Engine 5.
      El módulo 'unreal' está disponible solo en ese entorno.
"""

from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Any

# ── El módulo 'unreal' viene del editor de UE5 ──────────────────────────────
try:
    import unreal
    _UNREAL_AVAILABLE = True
except ImportError:
    _UNREAL_AVAILABLE = False
    print("[SoulForge] Advertencia: módulo 'unreal' no disponible. "
          "Ejecuta este script desde el editor de UE5.")

# ── Logging ──────────────────────────────────────────────────────────────────

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s – %(message)s",
)
log = logging.getLogger("sf_unreal_bridge")


# ─────────────────────────────────────────────────────────────────────────────
# 1. Importación de escena desde JSON
# ─────────────────────────────────────────────────────────────────────────────

def import_soulforge_scene(json_path: str | Path) -> None:
    """
    Lee el snapshot de escena generado por el núcleo Rust y crea los Actores
    correspondientes en el nivel activo de Unreal Engine.

    Args:
        json_path: Ruta al archivo JSON emitido por Rust (DestructionResult o
                   SceneSnapshot).
    """
    json_path = Path(json_path)
    log.info("Importando escena de SoulForge: %s", json_path)

    if not json_path.exists():
        log.error("Archivo no encontrado: %s", json_path)
        return

    with json_path.open("r", encoding="utf-8") as f:
        data: dict[str, Any] = json.load(f)

    scene_type = _detect_scene_type(data)

    if scene_type == "destruction":
        _spawn_destruction_fragments(data)
    elif scene_type == "snapshot":
        _sync_swarm_agents(data)
    else:
        log.warning("Tipo de escena desconocido en: %s", json_path)


def _detect_scene_type(data: dict) -> str:
    """Detecta si el JSON es un DestructionResult o un SceneSnapshot."""
    if "fragments" in data:
        return "destruction"
    if "agent_count" in data:
        return "snapshot"
    return "unknown"


# ─────────────────────────────────────────────────────────────────────────────
# 2. Destrucción: Spawn de fragmentos
# ─────────────────────────────────────────────────────────────────────────────

def _spawn_destruction_fragments(data: dict) -> None:
    """
    Crea un StaticMeshActor por cada fragmento del resultado de destrucción.
    También activa un sistema Niagara de explosión en la posición del proxy.
    """
    proxy_id: int = data.get("proxy_id", -1)
    fragments: list[dict] = data.get("fragments", [])

    log.info("Proxy #%d destruido → %d fragmentos", proxy_id, len(fragments))

    if not _UNREAL_AVAILABLE:
        log.debug("(Simulando spawn en modo sin editor)")
        for frag in fragments:
            log.debug("  Fragmento %d @ %s", frag["fragment_id"], frag["position"])
        return

    editor_subsystem = unreal.get_editor_subsystem(unreal.UnrealEditorSubsystem)
    world = editor_subsystem.get_editor_world()

    for frag in fragments:
        pos = frag["position"]
        location = unreal.Vector(pos[0], pos[1], pos[2])

        # Spawn de Actor vacío (reemplazar con tu mesh de fragmento)
        actor = unreal.EditorLevelLibrary.spawn_actor_from_class(
            unreal.StaticMeshActor,
            location
        )
        if actor:
            actor.set_actor_label(f"SF_Fragment_{proxy_id}_{frag['fragment_id']}")
            log.info("  Fragmento %d spawneado en %s", frag["fragment_id"], location)

    _spawn_niagara_explosion(world, data)


def _spawn_niagara_explosion(world: Any, data: dict) -> None:
    """
    Activa el sistema Niagara de explosión en la posición del proxy destruido.
    (Placeholder: asigna el asset Niagara real de tu proyecto)
    """
    if not _UNREAL_AVAILABLE:
        return

    # TODO: Reemplazar '/Game/SoulForge/VFX/NS_ProxyExplosion' con tu asset real
    niagara_asset_path = "/Game/SoulForge/VFX/NS_ProxyExplosion"
    niagara_system = unreal.load_asset(niagara_asset_path)

    if niagara_system is None:
        log.warning("Asset Niagara no encontrado: %s", niagara_asset_path)
        return

    origin = unreal.Vector(0.0, 0.0, 0.0)
    unreal.NiagaraFunctionLibrary.spawn_system_at_location(
        world,
        niagara_system,
        origin,
        unreal.Rotator(0, 0, 0)
    )
    log.info("Sistema Niagara de explosión activado en %s", origin)


# ─────────────────────────────────────────────────────────────────────────────
# 3. Swarm Snapshot: Sincronización de agentes
# ─────────────────────────────────────────────────────────────────────────────

def _sync_swarm_agents(data: dict) -> None:
    """
    Sincroniza el estado del Cerebro de Enjambre (snapshot de Rust) con el
    nivel activo de Unreal. Por ahora solo registra el estado en el log.

    Args:
        data: SceneSnapshot deserializado desde JSON.
    """
    tick        = data.get("tick", 0)
    elapsed     = data.get("elapsed_secs", 0.0)
    agent_count = data.get("agent_count", 0)

    log.info(
        "SceneSnapshot | tick=%d | tiempo=%.2fs | agentes=%d",
        tick, elapsed, agent_count
    )

    # TODO: Iterar sobre los agentes cuando el snapshot incluya sus posiciones.


# ─────────────────────────────────────────────────────────────────────────────
# 4. Importación de USD
# ─────────────────────────────────────────────────────────────────────────────

def import_usd_scene(usd_path: str | Path, destination: str = "/Game/SoulForge/Scenes") -> None:
    """
    Importa un archivo USD generado por SoulForge al Content Browser de Unreal.

    Args:
        usd_path:    Ruta al archivo .usd o .usda.
        destination: Ruta de destino en el Content Browser de Unreal.
    """
    usd_path = Path(usd_path)
    log.info("Importando USD: %s → %s", usd_path, destination)

    if not usd_path.exists():
        log.error("Archivo USD no encontrado: %s", usd_path)
        return

    if not _UNREAL_AVAILABLE:
        log.debug("(Simulando importación USD en modo sin editor)")
        return

    task = unreal.AssetImportTask()
    task.set_editor_property("filename",      str(usd_path))
    task.set_editor_property("destination_path", destination)
    task.set_editor_property("automated",     True)
    task.set_editor_property("save",          True)

    unreal.AssetToolsHelpers.get_asset_tools().import_asset_tasks([task])
    log.info("USD importado correctamente.")


# ─────────────────────────────────────────────────────────────────────────────
# Punto de entrada (para pruebas fuera de Unreal)
# ─────────────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Uso: python sf_unreal_bridge.py <ruta_al_json>")
        sys.exit(1)

    import_soulforge_scene(sys.argv[1])
