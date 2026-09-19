#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 1 || ! -f "$1" || "$1" != *.whl ]]; then
    echo "Usage: $0 path/to/connectorx-cp312-manylinux-x86_64.whl" >&2
    exit 2
fi
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
context=$(mktemp -d)
trap 'rm -rf "$context"' EXIT
cp -- "$1" "$script_dir/check-wheel-import.py" "$context/"
docker build --platform linux/amd64 --iidfile "$context/image-id" \
    -f "$script_dir/Dockerfile.databricks-import" "$context"
docker run --rm --platform linux/amd64 --network none \
    "$(cat "$context/image-id")"
