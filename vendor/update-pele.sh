#!/usr/bin/env bash
set -euo pipefail

version=${1:-}
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
    echo "usage: $0 VERSION (for example, 1.6.0)" >&2
    exit 1
fi

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d "${TMPDIR:-/tmp}/pele-update.XXXXXX")
trap 'rm -f "$tmp/bundle" "$tmp/license" "$tmp/lib.rs" "$tmp/pele.global.url"; rmdir "$tmp"' EXIT

url="https://cdn.jsdelivr.net/npm/pele@$version"
curl -fsSL "$url/dist/pele.global.js" -o "$tmp/bundle"
curl -fsSL "$url/LICENSE" -o "$tmp/license"

current=$(sed -n 's#.*https://cdn.jsdelivr.net/npm/pele@\([^/" ]*\)/dist/pele.global.js.*#\1#p' "$root/src/lib.rs")
old_hash=$(grep -oE 'sha384-[A-Za-z0-9+/=]+' "$root/src/lib.rs")
if [ -z "$current" ] || [ "$(printf '%s\n' "$old_hash" | wc -l)" -ne 1 ]; then
    echo "expected one Pele CDN URL and SRI hash in src/lib.rs" >&2
    exit 1
fi

hash="sha384-$(openssl dgst -sha384 -binary "$tmp/bundle" | openssl base64 -A)"
sed -e "s#pele@$current/dist/pele.global.js#pele@$version/dist/pele.global.js#" \
    -e "s#$old_hash#$hash#" "$root/src/lib.rs" > "$tmp/lib.rs"
perl -0777 -pe 's/([^A-Za-z0-9._~!\x24\x27()*+,;=:@\/?-])/sprintf("%%%02X", ord($1))/ge' \
    "$tmp/bundle" > "$tmp/pele.global.url"

cp "$tmp/lib.rs" "$root/src/lib.rs"
cp "$tmp/pele.global.url" "$root/vendor/pele.global.url"
cp "$tmp/license" "$root/vendor/LICENSE"
echo "Updated Pele to $version ($hash)"
