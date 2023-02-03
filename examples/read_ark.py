import grim
import sys

def main(args: list[str]):
    # Open ark
    ark = grim.Ark.from_file_path(args[0])

    # Print version + key info
    print("Version:", ark.version)
    print("Key:", hex(ark.encryption))

    # Print entries
    for entry in ark.entries:
        print(entry.path)

    print("Finished!")

if __name__ == '__main__':
    main(sys.argv[1:])