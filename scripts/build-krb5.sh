#!/usr/bin/env bash
set -euo pipefail

# The manylinux 2.28 system Kerberos pulls a distro-patched OpenSSL 1.1
# into wheels. Its FIPS initializer aborts in the Databricks runtime.
version=1.22.2
sha256=3243ffbc8ea4d4ac22ddc7dd2a1dc54c57874c40648b60ff97009763554eaf13
prefix=/opt/connectorx-krb5
build_dir=$(mktemp -d)
trap 'rm -rf "$build_dir"' EXIT

curl --fail --location --retry 3 --connect-timeout 30 --max-time 300 \
    "https://kerberos.org/dist/krb5/1.22/krb5-${version}.tar.gz" \
    --output "$build_dir/krb5.tar.gz"
echo "$sha256  $build_dir/krb5.tar.gz" | sha256sum --check
tar -xzf "$build_dir/krb5.tar.gz" -C "$build_dir"
cd "$build_dir/krb5-${version}/src"

CPPFLAGS=-I/usr/include/openssl3 \
LDFLAGS=-L/usr/lib64/openssl3 \
    ./configure --prefix="$prefix" --sysconfdir=/etc \
    --localstatedir=/var --runstatedir=/run --with-crypto-impl=openssl
make -j"$(nproc)"
make -C lib/crypto check
make runenv.py
if ! make -C tests/gssapi check; then
    cat tests/gssapi/testlog >&2
    find tests/gssapi/testdir -maxdepth 1 -name '*.log' -exec cat {} \; >&2
    ldd kdc/krb5kdc >&2
    exit 1
fi
make install
install -D -m 644 ../NOTICE "$prefix/share/licenses/MIT-Kerberos-NOTICE"
install -m 644 /usr/share/licenses/openssl3-libs/LICENSE.txt \
    "$prefix/share/licenses/OpenSSL-LICENSE.txt"

# Check the dependency selected by the linker, not just configure's output.
readelf -d "$prefix/lib/libk5crypto.so" | grep 'Shared library: \[libcrypto.so.3\]'
dependencies=$(ldd "$prefix/lib/libgssapi_krb5.so")
if echo "$dependencies" | grep -E 'libcrypto\.so\.1\.1|not found'; then
    echo "Unexpected Kerberos dependency in the wheel build" >&2
    exit 1
fi
