#!/usr/bin/env python3
"""Archive the exact NRO and its unstripped ELF with a SHA-256 manifest.

Modelled on Online Deluxe's tools/package_debug.py. Writes
<output>/<first 16 hex digits of the NRO SHA-256>/ containing the NRO, the ELF
(debug symbols), Cargo.lock and manifest.json. Never deploys anything. Keep
the output outside the repository (dist/ is gitignored).
"""
import argparse
import hashlib
import json
import re
import shutil
import struct
import subprocess
from pathlib import Path

NAME = "libssbu_fx_control"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_text(binary, symbols):
    """The NRO's executable bytes must equal the ELF's PT_LOAD text segment."""
    if binary[0x10:0x14] != b"NRO0" or symbols[:4] != b"\x7fELF":
        raise ValueError("expected a Skyline NRO and its ELF")
    if struct.unpack_from("<H", symbols, 18)[0] != 183:
        raise ValueError("ELF is not AArch64")
    phoff = struct.unpack_from("<Q", symbols, 32)[0]
    phsize, phnum = struct.unpack_from("<HH", symbols, 54)
    text_offset, text_size = struct.unpack_from("<II", binary, 0x20)
    matched = False
    for index in range(phnum):
        kind, flags, offset, vaddr, _, filesz, _, _ = struct.unpack_from(
            "<IIQQQQQQ", symbols, phoff + index * phsize)
        if kind == 1 and flags & 1:
            # Linkle adds/fills the first 0x80-byte NRO header in .text.
            start = max(vaddr, 0x80)
            end = min(vaddr + filesz, text_offset + text_size)
            if end > start:
                elf_bytes = symbols[offset + start - vaddr:offset + end - vaddr]
                if binary[start:end] != elf_bytes:
                    raise ValueError("NRO and ELF executable bytes differ")
                matched = True
    if not matched:
        raise ValueError("could not validate matching executable segments")


def package(root, output=None):
    build = root / "target/aarch64-skyline-switch/release"
    nro = build / f"{NAME}.nro"
    elf = build / f"{NAME}.so"
    binary = nro.read_bytes()
    symbols = elf.read_bytes()
    verify_text(binary, symbols)
    ids = set(re.findall(
        rb"\d+\.\d+\.\d+-(?:[0-9a-f]{12}|uncommitted)(?:-dirty)?-\d{10}", symbols))
    if len(ids) != 1:
        raise ValueError(f"expected one embedded build identity, found {len(ids)}")
    build_id = ids.pop().decode()
    digest = sha(nro)
    dest = (output if output is not None else root / "dist") / digest[:16]
    dest.mkdir(parents=True, exist_ok=True)
    shutil.copy2(nro, dest / nro.name)
    shutil.copy2(elf, dest / f"{NAME}.elf")
    shutil.copy2(root / "Cargo.lock", dest / "Cargo.lock")
    commit = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()
    dirty = bool(subprocess.check_output(
        ["git", "-C", str(root), "status", "--porcelain"], text=True).strip())
    rustup = Path.home() / ".cargo/bin/rustup"
    try:
        compiler = subprocess.check_output(
            [str(rustup), "run", "skyline-v3", "rustc", "--version", "--verbose"], text=True)
    except (OSError, subprocess.CalledProcessError):
        compiler = "unknown"
    manifest = {
        "schema": 1,
        "build_id": build_id,
        "commit": commit,
        "dirty": dirty,
        "nro": nro.name,
        "nro_sha256": digest,
        "nro_size": len(binary),
        "module_id": binary[0x40:0x60].hex(),
        "elf": f"{NAME}.elf",
        "elf_sha256": sha(elf),
        "elf_size": len(symbols),
        "cargo_lock_sha256": sha(root / "Cargo.lock"),
        "compiler": compiler,
        "game_version": "13.0.5",
        "text_verified": True,
        "runtime_tested": False,
    }
    (dest / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(dest)
    return dest


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--output", type=Path,
                        help="Artifact directory, preferably outside the source checkout")
    args = parser.parse_args()
    package(args.root, args.output)
