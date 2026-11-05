#!/usr/bin/env bash
# Install a libtdjson the crate can link (unversioned libtdjson.so).
# Prefers the distro package. Ubuntu 24.04 and older do not ship it, so the
# script falls back to the Ubuntu archive builds of libsqlcipher2 and libtdjson.
set -euo pipefail

have_linker_name() {
  ldconfig -p 2>/dev/null | grep -q 'libtdjson\.so '
}

if have_linker_name; then
  echo "libtdjson already on the loader path"
  ldconfig -p | grep tdjson || true
  exit 0
fi

sudo apt-get update
pkg="$(apt-cache search --names-only '^libtdjson' | awk '{print $1}' | sort -V | tail -n 1 || true)"
if [[ -n "${pkg}" ]]; then
  sudo apt-get install -y "${pkg}"
  dev="$(apt-cache search --names-only '^libtd-dev$' | awk '{print $1}' | head -n 1 || true)"
  if [[ -n "${dev}" ]]; then
    sudo apt-get install -y "${dev}" || true
  fi
else
  arch="$(dpkg --print-architecture)"
  if [[ "${arch}" != "amd64" ]]; then
    echo "No distro libtdjson package, and the fallback .deb is amd64-only (this host is ${arch})." >&2
    exit 1
  fi
  sudo apt-get install -y ca-certificates curl libssl3t64 zlib1g
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' EXIT
  curl -fsSL -o "${tmp}/sqlcipher.deb" \
    "http://archive.ubuntu.com/ubuntu/pool/universe/s/sqlcipher/libsqlcipher2_4.13.0-1_amd64.deb"
  curl -fsSL -o "${tmp}/tdjson.deb" \
    "http://archive.ubuntu.com/ubuntu/pool/universe/t/td/libtdjson1.8.38_1.8.38~git20251126.f0d04d3+dfsg-1_amd64.deb"
  sudo dpkg -i "${tmp}/sqlcipher.deb" "${tmp}/tdjson.deb"
fi

shopt -s nullglob
for dir in /usr/lib/x86_64-linux-gnu /usr/lib/aarch64-linux-gnu /usr/lib /usr/local/lib; do
  if [[ -e "${dir}/libtdjson.so" ]]; then
    continue
  fi
  soname="$(find "${dir}" -maxdepth 1 -name 'libtdjson.so.*' -printf '%f\n' | head -n 1 || true)"
  if [[ -n "${soname}" ]]; then
    sudo ln -sfn "${soname}" "${dir}/libtdjson.so"
  fi
done

sudo ldconfig
if ! have_linker_name; then
  echo "libtdjson.so is still not visible to the linker" >&2
  ldconfig -p | grep tdjson || true
  exit 1
fi
ldconfig -p | grep tdjson
