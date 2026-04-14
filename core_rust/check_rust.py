import subprocess
import os

cwd = r"c:\proyectos unreal\explosion\Plugins\SoulForgePhysX\core_rust"
try:
    result = subprocess.run(["cargo", "check"], cwd=cwd, capture_output=True, text=True)
    print("STDOUT:")
    print(result.stdout)
    print("STDERR:")
    print(result.stderr)
except Exception as e:
    print(f"Error: {e}")
