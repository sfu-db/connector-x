import platform
import sys
import sysconfig
from shutil import copyfile


def main() -> None:
    
    # copied from the maturin project
    metadata = {
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

    if metadata["platform"] == "win32":
        suffix = ".dll"
    elif metadata["platform"] == "linux":
        suffix = ".so"
    elif metadata["platform"] == "darwin":
        suffix = ".dylib"
    else:
        raise NotImplementedError(f"platform '{platform}' not supported")

    copyfile(f"../target/release/libconnector_agent_python{suffix}", f"connector_agent/connector_agent_python{metadata['ext_suffix']}")

if __name__ == "__main__":
    main()