
#!/usr/bin/env python3
import subprocess
import sys
import shutil
import os

def run_cargo_command(command):
    try:
        subprocess.run(["cargo", command], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print(f"Cargo {command} completed successfully.")
    except subprocess.CalledProcessError as e:
        print(f"Cargo {command} failed: {e.stderr.decode().strip()}", file=sys.stderr)
        sys.exit(1)


def move_artifact(artifact_name, destination):
    source_path = f"target/debug/{artifact_name}"
    if not os.path.exists(source_path):
        print(f"Artifact '{artifact_name}' not found. Skipping move operation.")
        return

    # Ensure the destination directory exists, create if not, remove and recreate if already exists
    if os.path.exists(destination):
        print(f"Output directory '{destination}' already exists. Cleaning it.")
        shutil.rmtree(destination)
    os.makedirs(destination)

    shutil.move(source_path, destination)
    print(f"Moved '{artifact_name}' to '{destination}'.")


def clean_output_directory(directory):
    if os.path.exists(directory):
        shutil.rmtree(directory)
        print(f"Cleaned output directory '{directory}'.")
    else:
        print(f"Output directory '{directory}' does not exist. No need to clean.")

def main():
    if len(sys.argv) < 2:
        print("Usage: python build.py <command>")
        sys.exit(1)

    command = sys.argv[1]

    if command == "build":
        run_cargo_command("build")
        move_artifact("aes", "./out")  # Replace 'your_artifact_name' with your actual artifact name
    elif command == "clean":
        run_cargo_command("clean")
        clean_output_directory("./out")
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)

if __name__ == "__main__":
    main()
