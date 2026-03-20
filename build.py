"""
build.py  —  SoulForge PhysX Build Automator
══════════════════════════════════════════════
"El Pegamento" – Un solo comando prepara toda la forja.

Uso:
    python build.py               # compilación release (por defecto)
    python build.py --debug       # compilación debug
    python build.py --check-only  # solo valida el entorno, no compila

Pasos que ejecuta:
    1. Valida que Rust / Cargo estén instalados.
    2. Ejecuta `cargo build --release` en core_rust/.
    3. Copia la DLL / .lib resultante a core_cpp/Binaries/Win64/.
    4. Verifica las dependencias Python de core_python/.
    5. Compila el proyecto Unreal Engine (explosion).
    6. Muestra un resumen del estado de la forja.
"""

from __future__ import annotations

import argparse
import platform
import shutil
import subprocess
import sys
from pathlib import Path

# ─── Rutas del proyecto ───────────────────────────────────────────────────────

ROOT         = Path(__file__).parent.resolve()
CORE_RUST    = ROOT / "core_rust"
CORE_CPP     = ROOT / "core_cpp"
CORE_PYTHON  = ROOT / "core_python"

# Dónde Cargo deja los artefactos
RUST_RELEASE = CORE_RUST / "target" / "release"
RUST_DEBUG   = CORE_RUST / "target" / "debug"

# Destino dentro del plugin de Unreal (ahora en core_cpp)
CPP_BINARIES = CORE_CPP / "Binaries" / "Win64"

# Nombre de la librería por plataforma
_PLATFORM = platform.system()
if _PLATFORM == "Windows":
    DLL_NAME = "soulforge_physx.dll"
    LIB_NAME = "soulforge_physx.lib"   # import lib (solo Windows)
elif _PLATFORM == "Darwin":
    DLL_NAME = "libsoulforge_physx.dylib"
    LIB_NAME = None
else:
    DLL_NAME = "libsoulforge_physx.so"
    LIB_NAME = None


# ─── Utilidades ───────────────────────────────────────────────────────────────

def _banner(msg: str) -> None:
    sep = "=" * 60
    print(f"\n{sep}\n  {msg}\n{sep}")

def _ok(msg: str)   -> None: print(f"  OK  {msg}")
def _warn(msg: str) -> None: print(f"  WARN  {msg}")
def _fail(msg: str) -> None: print(f"  FAIL  {msg}", file=sys.stderr)

def _run(cmd: list[str], cwd: Path) -> bool:
    """Ejecuta un comando y retorna True si tuvo éxito."""
    print(f"\n  $ {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd)
    return result.returncode == 0


# ─── Paso 1: Validar entorno ──────────────────────────────────────────────────

def validate_environment() -> bool:
    _banner("Paso 1 – Validando entorno")
    ok = True

    # Cargo
    cargo = shutil.which("cargo")
    if cargo:
        _ok(f"Cargo encontrado: {cargo}")
    else:
        _fail("Cargo no encontrado. Instala Rust desde https://rustup.rs/")
        ok = False

    # Python
    _ok(f"Python {sys.version.split()[0]} en {sys.executable}")

    # Directorios
    for path in [CORE_RUST, CORE_CPP, CORE_PYTHON]:
        if path.is_dir():
            _ok(f"Directorio OK: {path.relative_to(ROOT)}")
        else:
            _fail(f"Directorio faltante: {path.relative_to(ROOT)}")
            ok = False

    return ok


# ─── Paso 2: Compilar Rust ────────────────────────────────────────────────────

def build_rust(debug: bool = False) -> Path | None:
    profile = "debug" if debug else "release"
    _banner(f"Paso 2 – Compilando núcleo Rust ({profile})")

    cargo_args = ["cargo", "build"]
    if not debug:
        cargo_args.append("--release")

    if not _run(cargo_args, cwd=CORE_RUST):
        _fail("cargo build falló. Revisa los errores arriba.")
        return None

    out_dir = RUST_DEBUG if debug else RUST_RELEASE
    dll_path = out_dir / DLL_NAME

    if dll_path.exists():
        size_kb = dll_path.stat().st_size // 1024
        _ok(f"DLL generada: {dll_path.name}  ({size_kb} KB)")
        return dll_path
    else:
        _fail(f"No se encontró la DLL en: {dll_path}")
        return None


# ─── Paso 3: Copiar artefactos al plugin C++ ─────────────────────────────────

def copy_to_plugin(dll_path: Path) -> bool:
    _banner("Paso 3 – Copiando artefactos al plugin de Unreal")

    CPP_BINARIES.mkdir(parents=True, exist_ok=True)

    # Copiar DLL
    dest_dll = CPP_BINARIES / DLL_NAME
    shutil.copy2(dll_path, dest_dll)
    _ok(f"Copiado: {DLL_NAME}  →  {dest_dll.relative_to(ROOT)}")

    # Copiar .lib de importación (solo Windows)
    if LIB_NAME:
        src_lib = dll_path.parent / LIB_NAME
        if src_lib.exists():
            dest_lib = CPP_BINARIES / LIB_NAME
            shutil.copy2(src_lib, dest_lib)
            _ok(f"Copiado: {LIB_NAME}  →  {dest_lib.relative_to(ROOT)}")
        else:
            _warn(f"{LIB_NAME} no encontrado (se necesita para enlace estático en UBT).")

    return True


# ─── Paso 4: Verificar dependencias Python ───────────────────────────────────

def check_python_deps() -> bool:
    _banner("Paso 4 – Verificando dependencias Python")

    req_file = CORE_PYTHON / "requirements.txt"
    if not req_file.exists():
        _warn("requirements.txt no encontrado en core_python/")
        return True  # no es un error fatal

    result = subprocess.run(
        [sys.executable, "-m", "pip", "install", "-r", str(req_file), "--quiet"],
        capture_output=True, text=True
    )
    if result.returncode == 0:
        _ok("Dependencias Python instaladas / actualizadas.")
    else:
        _warn("Algunas dependencias Python no se pudieron instalar:")
        print(result.stderr[:500])

    return True


# ─── Paso 5: Resumen ─────────────────────────────────────────────────────────

def build_unreal(debug: bool = False) -> bool:
    _banner("Paso 5 – Compilando Proyecto Unreal (explosion)")
    
    # El proyecto está dos niveles arriba del plugin
    project_root = ROOT.parent.parent
    uproject_files = list(project_root.glob("*.uproject"))
    if not uproject_files:
        _fail("No se encontró el archivo .uproject en la raíz del proyecto.")
        return False
    
    uproject_path = uproject_files[0]
    project_name = uproject_path.stem
    
    # Intentar encontrar Build.bat (ruta estándar en este sistema)
    engine_path = Path(r"C:\Program Files\Epic Games\UE_5.7")
    build_bat = engine_path / "Engine" / "Build" / "BatchFiles" / "Build.bat"
    
    if not build_bat.exists():
        _warn(f"No se encontró Build.bat en {build_bat}. Asegúrate de que la ruta del motor sea correcta.")
        return False
        
    config = "DebugGame" if debug else "Development"
    target = f"{project_name}Editor"
    platform_name = "Win64"
    
    # En Windows, llamamos a Build.bat
    cmd = [str(build_bat), target, platform_name, config, str(uproject_path), "-waitmutex"]
    
    return _run(cmd, cwd=project_root)


def print_summary(dll_path: Path | None, unreal_ok: bool = False) -> None:
    _banner("Resumen de la Forja")

    dll_dest = CPP_BINARIES / DLL_NAME
    items = [
        ("Núcleo Rust compilado",     dll_path is not None and dll_path.exists()),
        ("DLL en plugin C++",         dll_dest.exists()),
        ("Dependencias Python",       True),
        ("Proyecto Unreal compilado", unreal_ok),
    ]
    for label, status in items:
        mark = "[OK]" if status else "[FAIL]"
        print(f"  {mark}  {label}")

    if all(s for _, s in items):
        print("\n  [READY] SoulForge PhysX está lista. Abre tu proyecto en Unreal Engine.\n")
    else:
        print("\n  [WARN] Algunos pasos fallaron:")
        for label, status in items:
            if not status:
                print(f"      - Frituras detectadas en: {label}")
        print("\n")


# ─── Punto de entrada ─────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(description="SoulForge PhysX Build Automator")
    parser.add_argument("--debug",      action="store_true", help="Compilar en modo debug")
    parser.add_argument("--check-only", action="store_true", help="Solo validar entorno")
    args = parser.parse_args()

    print("\n  [BUILD] SoulForge PhysX Build Automator")
    print(f"     Plataforma: {_PLATFORM} | Raíz: {ROOT}\n")

    env_ok = validate_environment()
    if not env_ok:
        sys.exit(1)

    if args.check_only:
        _ok("--check-only: Entorno válido. Nada compilado.")
        sys.exit(0)

    dll_path = build_rust(debug=args.debug)
    if dll_path:
        copy_to_plugin(dll_path)

    check_python_deps()
    
    unreal_ok = build_unreal(debug=args.debug)
    
    print_summary(dll_path, unreal_ok)


if __name__ == "__main__":
    main()
