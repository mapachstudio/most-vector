#!/usr/bin/env bash
# Modificado por Grupo MOST para Most Vector. Obra original: VectorCraft del equipo ArtCraft (https://github.com/storytold/vectorcraft), Apache-2.0.
# Modificado por Grupo MOST para Most Vector. Obra original: Most Vector del equipo ArtCraft (https://github.com/storytold/vectorcraft), Apache-2.0.
# App-specific package assertions retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("com.grupomostmexico.mostvector.yml", "com.grupomostmexico.mostvector.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "com.grupomostmexico.mostvector" and manifest["command"] == "vectorcraft", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("Most Vector package identity ok")
PYEOF
