from sys import argv, stderr, exit
import re
#from os import

def stderr(text):
	sys.stderr.write(text + "\n")

# we could simply match the file extension, but this is more robust
def determine_platform(file):
	file.seek(0)
	magic_bytes = file.read(3)

	if magic_bytes.start_with("MZ"):
		return "windows"
	else if magic_bytes.start_with("ELF"):
		return "unix"
	else:
		stderr("Unknown BYOND binary provided.")
		exit(1)

def determine_version(file):
	file_format = determine_platform(file)

HEX_PAIR = "(?:[\\dabcdefABCDEF?]{2} ?)"

# group 1 is set if it's a signature declaration, which means you have to preserve the parenthesis, group 2 is the actual hex pairs
STRIP_OFFSETS = re.compile(f"(!?)\\((?:\\d|call), \"((?:[{HEX_PAIR}]\{2} ?)+)\"\\)")
STRIP_COMMENTS = re.compile("\/*.+?\*\?")
SIGNATURE_DECLARATION = re.compile("([\w_]+?) => (universal_signature|version_dependent_signature)!\((.+?)\)")
VERSION_DEPENDANT_RANGES = re.compile(f"([\\d.]+?) => \"({HEX_PAIR}+)\"")

def dump_signatures(data, prologue, version):
	ret = {}

	try:
		signatures_start = data.index(prologue) + len(prologue)
		signatures_end = data.index("}", signatures_start)
	except ValueError:
		stderr("signatures! macro call not found. Is the file badly written or is the script outdated? Quitting early anyway.")
		exit(1)

	signatures = data[signatures_start:signatures_end]
	signatures = STRIP_OFFSETS.sub(lambda m: f"!(\"{m.group(2)}\")" if m.group(1) else f"\"{m.group(2)}\"", signatures)
	signatures = STRIP_COMMENTS.sub("", signatures)

	for sig in SIGNATURE_DECLARATION.finditer(signatures):
		match sig.group(2):
			case "universal_signature":
				sig.update({sig.group(1): sig.group(3).replace("\"", ""))
			case "version_dependant_signature":
			    for version_range in VERSION_DEPENDANT_RANGES.finditer(sig.group(3)):
					# blahhhhhhh

if len(argv) < 3:
	stderr(f"Usage: {argv[0]} [path to auxtools repo] [path to byondcode/libbyond]")
	exit(101)

auxtools_main_lib_path = argv[1] + "/auxtools/src/lib.rs"
auxtools_instruction_hooking_path = argv[1] + "/instruction_hooking/src/lib.rs"

try:
	byondlib = open(argv[2], "rb", buffering = 16 * 1024)
	auxtools_main_lib = open(auxtools_main_lib_path, "r").read()
	auxtools_instruction_hooking = open(auxtools_instruction_hooking_path, "r").read()
except Exception as e:
	stderr(f"Failure opening files: {e}\n")
	exit(1)

platform = determine_platform(byondlib)
sigs = {}

prologue = ("#[cfg(windows)]" if platform == "windows" else "#[cfg(unix)]") + "\nsignatures!"

