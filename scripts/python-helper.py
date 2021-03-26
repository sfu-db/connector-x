"""
Usage:
  python-helper.py (copy-extension|rename-wheel)

Options:
  -h --help     Show this screen.
  --version     Show version.
"""
import platform
import sys
import sysconfig
from shutil import copyfile
from pathlib import Path
import os
from docopt import docopt

# copied from the maturin project
METADATA = {
    "major": sys.version_info.major,
    "minor": sys.version_info.minor,
    "abiflags": sysconfig.get_config_var("ABIFLAGS"),
    "interpreter": platform.python_implementation().lower(),
    "ext_suffix": sysconfig.get_config_var("EXT_SUFFIX"),
    "abi_tag": (sysconfig.get_config_var("SOABI") or "-").split("-")[1] or None,
    "m": sysconfig.get_config_var("WITH_PYMALLOC") == 1,
    "u": sysconfig.get_config_var("Py_UNICODE_SIZE") == 4,
    "d": sysconfig.get_config_var("Py_DEBUG") == 1,
    # This one isn't technically necessary, but still very useful for sanity checks
    "platform": platform.system().lower(),
    # We need this one for windows abi3 builds
    "base_prefix": sys.base_prefix,
}



def main() -> None:
    args = docopt(__doc__)
    if args["copy-extension"]:
        if METADATA["platform"] == "windows":
            suffix = ".dll"
        elif METADATA["platform"] == "linux":
            suffix = ".so"
        elif METADATA["platform"] == "darwin":
            suffix = ".dylib"
        else:
            raise NotImplementedError(f"platform '{METADATA['platform']}' not supported")

        src = Path("../target/release/libconnector_agent_python")
        dst = Path("./connector_agent/connector_agent_python")
        copyfile(src.with_suffix(suffix), dst.with_suffix(METADATA["ext_suffix"]))
    elif args["rename-wheel"]:
        if METADATA["platform"] == "windows":
            arch = "win_amd64"
        elif METADATA["platform"] == "linux":
            arch = "manylinux2014_x86_64"
        elif METADATA["platform"] == "darwin":
            arch = "macosx_10_15_intel"
        else:
            raise NotImplementedError(f"platform '{platform}' not supported")

        for p in Path("./dist").iterdir():
            if p.suffix == ".whl":
                pkgname, version, *rest = p.stem.split("-")
                break

        pyver = f"{METADATA['major']}{METADATA['minor']}"
        os.rename(
            p,
            f"./dist/{pkgname}-{version}-cp{pyver}-cp{METADATA['abi_tag']}-{arch}.whl",
        )
    else:
        raise ValueError(f"args not understand {args}")

if __name__ == "__main__":
    main()
    
