#!/usr/bin/env python3
"""Download the current Rentman OpenAPI document and refresh the manifest."""

from __future__ import annotations

import re
import subprocess
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS_URL = "https://api.rentman.net/"
OPENAPI = ROOT / "openapi" / "rentman-oas.json"
GENERATOR = ROOT / "scripts" / "generate_endpoint_manifest.py"


def main() -> None:
    html = urllib.request.urlopen(DOCS_URL, timeout=30).read().decode()
    match = re.search(r"Redoc\.init\('([^']+)'", html)
    if not match:
        raise SystemExit("Could not find Redoc OpenAPI URL in Rentman docs")

    openapi_url = match.group(1)
    document = urllib.request.urlopen(openapi_url, timeout=60).read().decode()
    OPENAPI.write_text(document)
    subprocess.run([str(GENERATOR)], cwd=ROOT, check=True)
    print(f"Updated {OPENAPI.relative_to(ROOT)} from {openapi_url}")


if __name__ == "__main__":
    main()
