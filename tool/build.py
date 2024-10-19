#!/usr/bin/python3
import subprocess
import sys
import shutil
import os

def build_rust_project():
     try:
        subprocess.run(["cargo", "build"], check=True)
        print("success to build tools.")
     except subprocess.CalledProcessError as e:
        print("Rust项目构建失败：", e, file=sys.stderr)
        sys.exit(1)

def clean_rust_project():
    try:
        subprocess.run(["cargo", "clean"], check=True)
        print("success to build tools.")
    except subprocess.CalledProcessError as e:
        print("Rust项目构建失败：", e, file=sys.stderr)
        sys.exit(1)

def mov_file(file_p, dest):
  try:
    shutil.move(file_p, dest)
  except shutil.Error as e:
    print(f"无法移动文件。错误: {e}")
  except FileNotFoundError:
    print(f"源文件 '{file_p}' 不存在。")


if len(sys.argv) < 2:
    print("用法：python example.py <参数1> <参数2> <参数3>")
    sys.exit(1)

if sys.argv[1] == "build":
 build_rust_project()
 os.mkdir("./out")
 mov_file("target/debug/aes","./out")
if sys.argv[1] == "clean":
 clean_rust_project()
 shutil.rmtree("./out")