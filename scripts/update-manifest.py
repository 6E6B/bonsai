#!/usr/bin/env python3
"""Build the Tauri updater manifest only when every supported package is signed."""
import argparse
import datetime
import json
from pathlib import Path
from urllib.parse import quote


def build_manifest(directory, tag, repository, notes):
    platforms = {}
    packages = [
        ("*.AppImage", ["linux-x86_64", "linux-x86_64-appimage"]),
        ("*.exe", ["windows-x86_64", "windows-x86_64-nsis"]),
        ("*.msi", ["windows-x86_64-msi"]),
        ("*.app.tar.gz", ["darwin-aarch64", "darwin-x86_64"]),
    ]
    for pattern, targets in packages:
        matches = list(directory.glob(pattern))
        if len(matches) != 1:
            raise ValueError(f"Expected one {pattern} package, found {len(matches)}")
        package = matches[0]
        signature = Path(str(package) + ".sig").read_text().strip()
        if not signature:
            raise ValueError(f"Empty signature for {package.name}")
        url = f"https://github.com/{repository}/releases/download/{quote(tag, safe='')}/{quote(package.name, safe='')}"
        for target in targets:
            platforms[target] = {"signature": signature, "url": url}
    return {
        "version": tag.removeprefix("v"),
        "notes": notes,
        "pub_date": datetime.datetime.now(datetime.timezone.utc).isoformat().replace("+00:00", "Z"),
        "platforms": platforms,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--directory", required=True, type=Path)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--notes-file", required=True, type=Path)
    args = parser.parse_args()
    manifest = build_manifest(args.directory, args.tag, args.repository, args.notes_file.read_text())
    (args.directory / "latest.json").write_text(json.dumps(manifest, indent=2) + "\n")


if __name__ == "__main__":
    main()
