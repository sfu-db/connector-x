"""Check an installed Linux wheel in a disposable manylinux build container."""

import argparse
import ctypes
import os
from pathlib import Path
import subprocess
import sys
import tempfile


def wheel_libraries():
    # Import first so the extension's RPATH loads the wheel dependency closure.
    import connectorx

    return Path(connectorx.__file__).resolve().parent.parent / "connectorx.libs"


def probe(expected):
    (library,) = wheel_libraries().glob("libkrb5-*.so*")
    krb5 = ctypes.CDLL(str(library))
    krb5.krb5_init_context.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    krb5.krb5_init_context.restype = ctypes.c_int32
    krb5.krb5_get_default_realm.argtypes = [
        ctypes.c_void_p,
        ctypes.POINTER(ctypes.c_char_p),
    ]
    krb5.krb5_get_default_realm.restype = ctypes.c_int32
    krb5.krb5_free_default_realm.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
    krb5.krb5_free_default_realm.restype = None
    krb5.krb5_free_context.argtypes = [ctypes.c_void_p]
    krb5.krb5_free_context.restype = None

    for _ in range(2):
        context = ctypes.c_void_p()
        assert krb5.krb5_init_context(ctypes.byref(context)) == 0
        try:
            for _ in range(2):
                realm = ctypes.c_char_p()
                try:
                    result = krb5.krb5_get_default_realm(context, ctypes.byref(realm))
                    if expected == "missing":
                        assert result != 0 and realm.value is None, (
                            result,
                            realm.value,
                        )
                    else:
                        assert result == 0 and realm.value == expected.encode(), (
                            result,
                            realm.value,
                        )
                finally:
                    if realm:
                        krb5.krb5_free_default_realm(
                            context, ctypes.cast(realm, ctypes.c_void_p)
                        )
        finally:
            krb5.krb5_free_context(context)
    print(f"Plugin lookup: {expected} (two contexts, two calls each)", flush=True)


def check_plugins(include_dir):
    base = Path("/usr/lib64/krb5/plugins")
    base.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="connectorx-test-", dir=base) as default:
        with tempfile.TemporaryDirectory(prefix="connectorx-plugins-") as temporary:
            temporary = Path(temporary)
            relative = Path(default).name + "/hostrealm.so"
            override_base = temporary / "override"
            override = override_base / relative
            override.parent.mkdir(parents=True)
            fixtures = [
                (base / relative, "DEFAULT.EXAMPLE", ""),
                (override, "OVERRIDE.EXAMPLE", f" plugin_base_dir = {override_base}\n"),
            ]
            config = temporary / "krb5.conf"
            env = dict(os.environ, KRB5_CONFIG=str(config))
            for plugin, realm, setting in fixtures:
                subprocess.run(
                    [
                        "cc",
                        "-shared",
                        "-fPIC",
                        f"-I{include_dir}",
                        f'-DTEST_REALM="{realm}"',
                        str(Path(__file__).parent / "fixtures" / "hostrealm-plugin.c"),
                        "-o",
                        str(plugin),
                    ],
                    check=True,
                    timeout=30,
                )
                assert plugin.is_file()
                config.write_text(
                    "[libdefaults]\n dns_lookup_realm = false\n"
                    + setting
                    + "[plugins]\n hostrealm = {\n"
                    + f"  module = connectorx_test:{relative}\n"
                    + "  enable_only = connectorx_test\n }\n"
                )
                command = [sys.executable, str(Path(__file__).resolve()), "--probe"]
                subprocess.run(command + [realm], env=env, check=True, timeout=30)
                # Prove the result depends on this file, not another realm source.
                disabled = plugin.with_suffix(".disabled")
                plugin.rename(disabled)
                try:
                    subprocess.run(
                        command + ["missing"], env=env, check=True, timeout=30
                    )
                finally:
                    disabled.rename(plugin)

    # Also check legacy Kerberos loaders and relative GSS mechanism paths.
    for pattern, paths in [
        (
            "libkrb5-*.so*",
            ["/usr/lib64/krb5/plugins/authdata", "/usr/lib64/krb5/plugins/libkrb5"],
        ),
        ("libgssapi_krb5-*.so*", ["/usr/lib64/gss/"]),
    ]:
        (library,) = wheel_libraries().glob(pattern)
        contents = library.read_bytes()
        for path in paths:
            assert path.encode() + b"\0" in contents, (library.name, path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--include-dir", default="/opt/connectorx-krb5/include")
    parser.add_argument(
        "--probe", choices=["DEFAULT.EXAMPLE", "OVERRIDE.EXAMPLE", "missing"]
    )
    args = parser.parse_args()
    if args.probe:
        probe(args.probe)
    else:
        check_plugins(args.include_dir)
