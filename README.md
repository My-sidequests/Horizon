# Horizon

> Learn security through hands-on exploitation.

Horizon is a CLI companion for working through security exercises. It tracks where you are, shows you the brief, gives you hints one at a time, and gets out of your way. The actual work — running, exploiting, debugging — happens in your own terminal.

The first module covers **memory corruption in C**: eleven exercises building from GDB and Valgrind fundamentals up through shellcode injection and ROP chains.

---

## Install

```bash
cargo install --git https://github.com/My-sidequests/Horizon.git
```

Then run `horizon start` to compile the exercises and begin.

---

## Prerequisites

| Tool | Purpose |
|------|---------|
| Rust + Cargo | Install Horizon |
| GCC | Compile exercise targets |
| GDB | Inspect memory, debug crashes |
| Valgrind | Detect memory errors |
| Python 3 | Write exploit scripts |
| pwntools *(optional)* | Shellcode / ROP helpers — `pip3 install pwntools` |
| ROPgadget *(optional)* | Find ROP gadgets — `pip3 install ropgadget` |

---

## Usage

```
horizon              Show the current exercise
horizon start        Compile all exercises and reset progress
horizon done <flag>  Submit a flag to validate the current exercise
horizon hint         Reveal the next hint (progressive)
horizon skip         Skip the current exercise
horizon list         See all exercises and your progress
horizon reset        Start over (asks for confirmation)
horizon update       Pull the latest version from GitHub
```

### Typical workflow

```
# Terminal 1 — Horizon
horizon

# Terminal 2 — your work
cd ~/.local/share/horizon/exercises/01_gdb_basics
gdb ./target

# When you find the flag —
horizon done HRZ{...}
```

---

## Exercises

| # | Name | Core concept |
|---|------|-------------|
| 01 | GDB Basics | `info functions`, call a hidden function |
| 02 | GDB Variables | `break`, `set variable`, satisfy a condition |
| 03 | GDB Crash Analysis | Segfault, `backtrace`, read a static variable |
| 04 | Valgrind — Memory Leak | `malloc` without `free`, read the leak report |
| 05 | Valgrind — Use-After-Free | Access after `free`, dangling pointer |
| 06 | Valgrind — Invalid Read | Buffer overread, off-by-one loop |
| 07 | Return-Oriented Hijacking | Stack overflow, return address overwrite |
| 08 | Shellcode Injection | Executable stack, injected machine code |
| 09 | Off-by-One | Single-byte overflow, saved RBP corruption |
| 10 | Format String Attack | `printf` without format string, `%n` write |
| 11 | Return-to-libc | NX bypass, ROP chain, `system("/bin/sh")` |

---

## How it works

Each exercise ships as a compiled binary — source code is never written to disk. The brief, hints, and step-by-step solution are all available locally once you run `horizon start`.

Flags are in the format `HRZ{...}`. Finding them requires actually using the tools — GDB, Valgrind, or a working exploit. Submit with `horizon done HRZ{...}`.

---

## Progress

Saved to `~/.local/share/horizon/progress.json` (Linux/macOS) or `%LOCALAPPDATA%\horizon\progress.json` (Windows). Reset anytime with `horizon reset`.

---

## Security note

Exercise targets are compiled with protections intentionally disabled. Run them in a VM or isolated environment.
