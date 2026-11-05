#!/usr/bin/env python3
"""Package only the native launcher and notices, never game files or player data."""
import os
from pathlib import Path
import shutil
import tarfile
import tempfile
import zipfile


def main():
    root = Path(__file__).resolve().parents[1]
    platform = os.environ["LOCAL_PLATFORM"]
    if platform not in ("windows-x64", "linux-x64"):
        raise SystemExit("Unsupported local launcher platform")
    binary = Path(os.environ["LOCAL_BINARY"]).resolve(strict=True)
    out = root / "dist"
    out.mkdir(exist_ok=True)
    stem = f"horizon-free-local-{platform}"
    with tempfile.TemporaryDirectory(prefix="horizon-local-package-") as work:
        folder = Path(work)
        name = "horizon-launcher.exe" if platform == "windows-x64" else "horizon-launcher"
        shutil.copy2(binary, folder / name)
        for source, target in (
            ("LICENSE", "LICENSE"),
            ("THIRD_PARTY_NOTICES.md", "THIRD_PARTY_NOTICES.md"),
            ("docs/launcher-local.md", "LEIA-ME.md"),
        ):
            shutil.copy2(root / source, folder / target)
        if platform == "windows-x64":
            shutil.copy2(binary, out / f"{stem}.exe")
            with zipfile.ZipFile(out / f"{stem}.zip", "w", zipfile.ZIP_DEFLATED) as archive:
                for item in sorted(folder.iterdir()):
                    archive.write(item, item.name)
        else:
            (folder / name).chmod(0o755)
            with tarfile.open(out / f"{stem}.tar.gz", "w:gz") as archive:
                for item in sorted(folder.iterdir()):
                    archive.add(item, arcname=item.name)


if __name__ == "__main__":
    main()
