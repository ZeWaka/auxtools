import os
import re
import sys
import argparse

def find_signatures_in_repo(repo_path):
    """
    Scans the repository for signature! macro calls or hex string patterns.
    """
    signatures = {}
    # Pattern to match: signature!("name", "hex bytes")
    # Also matches common auxtools-style signature strings
    sig_regex = re.compile(r'signature!\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')

    for root, _, files in os.walk(repo_path):
        for file in files:
            if file.endswith('.rs'):
                with open(os.path.join(root, file), 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                    matches = sig_regex.findall(content)
                    for name, pattern in matches:
                        signatures[name] = pattern
    return signatures

def signature_to_regex(sig_str):
    """
    Converts a space-separated hex string with '??' wildcards into a regex pattern.
    """
    parts = sig_str.split()
    re_pattern = b""
    for part in parts:
        if part == "??" or part == "?":
            re_pattern += b"."
        else:
            re_pattern += re.escape(bytes.fromhex(part))
    return re_pattern

def scan_binary(binary_path, signatures):
    """
    Compares the signatures against the actual binary file.
    """
    if not os.path.exists(binary_path):
        print(f"Error: Binary not found at {binary_path}")
        return

    with open(binary_path, 'rb') as f:
        data = f.read()

    print(f"{'Signature Name':<30} | {'Status':<10} | {'Address/Match'}")
    print("-" * 60)

    for name, pattern_str in signatures.items():
        try:
            regex_pattern = signature_to_regex(pattern_str)
            match = re.search(regex_pattern, data, re.DOTALL)

            if match:
                print(f"{name:<30} | OK         | {hex(match.start())}")
            else:
                print(f"{name:<30} | FAILED     | No match found")
        except Exception as e:
            print(f"{name:<30} | ERROR      | {str(e)}")

def discover_file_type_from_header(filepath):
	with open(filepath, "rb") as byond_lib:
		magic_value = byond_lib.read(3)

		if magic_value.starts_with("MZ"):


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("repo", help="Path to the auxtools repository")
    parser.add_argument("lib", help="Path to libbyond.so or byondcore.dll")
	parser.add_argument("-f", "--format", choices=["so", "dll"], help="Format of the library (can be inferred from the filename)")

    args = parser.parse_args()

	lib_name = os.path.basename(args.binary)
	lib_format = args.format or infer_from_libname(lib_name)



    print(f"Extracting signatures from: {args.repo}")
    sigs = find_signatures_in_repo(args.repo)

    if not sigs:
        print("[-] No signatures found. Check if the path is correct or if the macro format changed.")
        return

    print(f"Found {len(sigs)} signatures. Scanning {args.binary}...")
    scan_binary(args.binary, sigs)

if __name__ == "__main__":
    main()
