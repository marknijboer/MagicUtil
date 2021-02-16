import zipfile
import hashlib
import re
import sys
import json

WINDOWS_RELEASE_EXEC = 'target/x86_64-pc-windows-gnu/release/MagicUtil.exe'
WINDOWS_RELEASE_ZIP = 'dist/magicutil-x86_64.zip'

def create_zip():
    zf = zipfile.ZipFile(WINDOWS_RELEASE_ZIP, mode='w')
    try:
        zf.write(WINDOWS_RELEASE_EXEC, arcname='MagicUtil.exe')
    finally:
        zf.close()

def calculate_zip_hash():
    with open(WINDOWS_RELEASE_ZIP,"rb") as f:
        bytes = f.read() # read entire file as bytes
        readable_hash = hashlib.sha256(bytes).hexdigest();
        return readable_hash

def find_app_version():
    version_reg = re.compile('^version? =? "([0-9\.]+)"', flags=re.MULTILINE)
    with open('Cargo.toml', 'r') as file:
        data = file.read()
        matches = version_reg.findall(data)
        if len(matches) < 0:
            sys.stderr.write('Could not find the MagicUtil version number')
            exit(1)
        return matches[0]

def create_release_json(ziphash, version):
    with open('scoop/magicutil-base.json', 'r') as myfile:
        data = myfile.read()
    # parse file
    release_obj = json.loads(data)
    release_obj['version'] = version
    release_obj['architecture']['64bit']['hash'] = ziphash

    release_str = json.dumps(release_obj, indent=4)

    with open("dist/magicutil.json", "w") as release_file:
        release_file.write(release_str)

def main():
    print("Bundling the executable in a zip file...")
    create_zip()
    ziphash = calculate_zip_hash()
    version = find_app_version()

    print("Writing the release json file...")
    create_release_json(ziphash, version)

    print("Done!")

if __name__ == "__main__":
    main()