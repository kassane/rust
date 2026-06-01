import glob
import json
import os
import subprocess
import sys
import re

def create_target(target_spec, platform, vendor, _os):
    opts = target_spec.copy()
    opts.pop("is-builtin", None)
    opts["linker"] = f"mos-{platform}-clang"
    opts["vendor"] = vendor
    opts["os"] = _os
    # Rust 1.94+ expects these width fields as integers, not strings.
    for key in ("target-c-int-width", "target-pointer-width"):
        if key in opts and isinstance(opts[key], str):
            opts[key] = int(opts[key])
    return opts

def vendor_os(platform):
    platform = platform.split('-', 1)
    assert len(platform) <= 2, f"{platform}"
    if len(platform) == 1:
        platform = platform + ['none']
    return platform

def get_mos_platforms():
    # Match on the basename only. The glob yields absolute paths, and if the
    # install prefix itself contains "mos-" (e.g. .../mos-toolchain/bin/...), a
    # regex over the full path would greedily capture the directory components
    # too (vendor="toolchain/bin/mos"), producing a bogus nested output path.
    # Accept an optional .exe suffix so this also works on Windows, where the
    # driver aliases are installed as mos-<platform>-clang.exe. The anchored
    # `$` ensures the C++ drivers (mos-*-clang++[.exe]) are excluded.
    bindir = os.path.dirname(subprocess.getoutput("which mos-sim-clang"))
    platforms = set()
    for i in glob.glob(os.path.join(bindir, 'mos-*-clang')) + \
             glob.glob(os.path.join(bindir, 'mos-*-clang.exe')):
        base = os.path.basename(i)
        if 'common' in base:
            continue
        m = re.search(r"mos-(.*)-clang(\.exe)?$", base)
        if m:
            platforms.add(m.group(1))
    return sorted(platforms)

if __name__ == "__main__":

    target_spec = json.loads(
        subprocess.getoutput('rustc --target mos-unknown-none -Z unstable-options --print target-spec-json')
    )

    dest_dir = sys.argv[1]

    for platform in get_mos_platforms():
        vendor, _os = vendor_os(platform)
        target_def = create_target(target_spec, platform, vendor, _os)
        target_file = os.path.join(dest_dir, f"mos-{vendor}-{_os}.json")
        with open(target_file, "w") as fp:
            json.dump(target_def, fp, indent=2)
            print("created", target_file)