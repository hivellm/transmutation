#!/bin/bash
# AppImage launcher script for Transmutation

HERE="$(dirname "$(readlink -f "${0}")")"
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

# Run transmutation
exec "${HERE}/usr/bin/transmutation" "$@"

