#!/usr/bin/env bash
set -euo pipefail

REPO_DEFAULT="radjathaher/seo-cli-rs"
REPO="${REPO:-$REPO_DEFAULT}"
TAG="${TAG:-}"
BIN_DIR="${BIN_DIR:-${HOME}/.local/bin}"

usage() {
  cat <<'EOF'
install.sh (seo-cli-rs)

Env:
  REPO     GitHub repo owner/name (default: radjathaher/seo-cli-rs)
  TAG      Release tag to install (default: latest)
  BIN_DIR  Install dir (default: ~/.local/bin)

Notes:
  - Private repo installs require GitHub auth:
      - either `gh auth login` (preferred)
      - or set GITHUB_TOKEN with access to the repo.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

uname_s="$(uname -s)"
uname_m="$(uname -m)"

case "${uname_s}" in
  Linux) os="linux" ;;
  Darwin) os="macos" ;;
  *)
    echo "unsupported OS: ${uname_s}" >&2
    exit 1
    ;;
esac

case "${uname_m}" in
  x86_64|amd64) arch="x86_64" ;;
  arm64|aarch64) arch="aarch64" ;;
  *)
    echo "unsupported arch: ${uname_m}" >&2
    exit 1
    ;;
esac

case "${os}-${arch}" in
  linux-x86_64) target="x86_64-unknown-linux-gnu" ;;
  macos-x86_64) target="x86_64-apple-darwin" ;;
  macos-aarch64) target="aarch64-apple-darwin" ;;
  *)
    echo "unsupported target combo: ${os}-${arch}" >&2
    exit 1
    ;;
esac

tmp="$(mktemp -d)"
cleanup() { rm -rf "${tmp}" >/dev/null 2>&1 || true; }
trap cleanup EXIT

have() { command -v "$1" >/dev/null 2>&1; }

resolve_latest_tag_via_api() {
  local url="https://api.github.com/repos/${REPO}/releases/latest"
  local auth=()
  if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    auth=(-H "Authorization: token ${GITHUB_TOKEN}")
  fi
  curl -fsSL "${auth[@]}" "${url}" | python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"])'
}

download_with_gh() {
  local tag="$1"
  local pattern="seo-${tag}-${target}*"
  (cd "${tmp}" && gh release download "${tag}" --repo "${REPO}" -p "${pattern}" -D .)
}

download_with_curl_api() {
  local tag="$1"
  local base="https://api.github.com/repos/${REPO}/releases/tags/${tag}"
  local auth=()
  if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    auth=(-H "Authorization: token ${GITHUB_TOKEN}")
  fi

  local json_path="${tmp}/release.json"
  curl -fsSL "${auth[@]}" "${base}" -o "${json_path}"

  python3 - "${json_path}" "${tmp}" "seo-${tag}-${target}" <<'PY'
import json, sys, os, re, subprocess
release_json, out_dir, prefix = sys.argv[1], sys.argv[2], sys.argv[3]
data = json.load(open(release_json, "r", encoding="utf-8"))
assets = data.get("assets", [])
want = []
for a in assets:
  name = a.get("name", "")
  if name.startswith(prefix) and (name.endswith(".tar.gz") or name.endswith(".zip") or name.endswith(".sha256")):
    want.append((name, a.get("url")))
if not want:
  raise SystemExit("no matching assets found for prefix: " + prefix)
for name, url in want:
  p = os.path.join(out_dir, name)
  print("downloading", name, file=sys.stderr)
  cmd = ["curl", "-fsSL", "-H", "Accept: application/octet-stream", url, "-o", p]
  token = os.environ.get("GITHUB_TOKEN")
  if token:
    cmd[2:2] = ["-H", f"Authorization: token {token}"]
  subprocess.check_call(cmd)
PY
}

if [[ -z "${TAG}" ]]; then
  if have gh; then
    TAG="$(gh release view --repo "${REPO}" --json tagName -q .tagName)"
  else
    TAG="$(resolve_latest_tag_via_api)"
  fi
fi

echo "repo: ${REPO}"
echo "tag:  ${TAG}"
echo "target: ${target}"

if have gh; then
  download_with_gh "${TAG}"
else
  download_with_curl_api "${TAG}"
fi

archive=""
if ls "${tmp}"/seo-"${TAG}"-"${target}".tar.gz >/dev/null 2>&1; then
  archive="${tmp}/seo-${TAG}-${target}.tar.gz"
elif ls "${tmp}"/seo-"${TAG}"-"${target}".zip >/dev/null 2>&1; then
  archive="${tmp}/seo-${TAG}-${target}.zip"
else
  echo "missing archive for target ${target} in ${tmp}" >&2
  ls -la "${tmp}" >&2 || true
  exit 1
fi

sha_file="${tmp}/seo-${TAG}-${target}.sha256"
if [[ -f "${sha_file}" ]]; then
  if have shasum; then
    (cd "${tmp}" && shasum -a 256 -c "$(basename "${sha_file}")") >/dev/null
  elif have sha256sum; then
    (cd "${tmp}" && sha256sum -c "$(basename "${sha_file}")") >/dev/null
  else
    echo "warning: no sha256 verifier found (need shasum or sha256sum)" >&2
  fi
else
  echo "warning: missing sha256 file: ${sha_file}" >&2
fi

extract_dir="${tmp}/extract"
mkdir -p "${extract_dir}"

case "${archive}" in
  *.tar.gz) tar -xzf "${archive}" -C "${extract_dir}" ;;
  *.zip)
    if ! have unzip; then
      echo "need unzip for .zip archives" >&2
      exit 1
    fi
    unzip -q "${archive}" -d "${extract_dir}"
    ;;
esac

bin_path=""
if [[ -f "${extract_dir}/seo" ]]; then
  bin_path="${extract_dir}/seo"
else
  bin_path="$(find "${extract_dir}" -maxdepth 2 -type f -name seo -print -quit || true)"
fi

if [[ -z "${bin_path}" || ! -f "${bin_path}" ]]; then
  echo "could not find extracted binary 'seo' in ${extract_dir}" >&2
  find "${extract_dir}" -maxdepth 3 -type f -print >&2 || true
  exit 1
fi

mkdir -p "${BIN_DIR}"
install -m 0755 "${bin_path}" "${BIN_DIR}/seo"

echo "installed: ${BIN_DIR}/seo"
echo "try: seo -h"
if [[ ":$PATH:" != *":${BIN_DIR}:"* ]]; then
  echo "note: add to PATH: export PATH=\"${BIN_DIR}:\$PATH\""
fi

