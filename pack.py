import subprocess
import zipfile
import os

subprocess.run(["cargo", "build", "-r"], check=True)

with zipfile.ZipFile("./output.zip", "w") as zipf:
    zipf.write("./target/release/CpuOptimizer.dll",
               "bin/NativeMods/CpuOptimizer.dll")
    zipf.write("./CpuOptimizer.ini", "bin/NativeMods/CpuOptimizer.ini")

print("Done")
