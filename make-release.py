import sys
import shutil
import subprocess

if len(sys.argv) != 2:
    raise Exception("Expected version")

version = sys.argv[1]

subprocess.check_call(["strip", "target/debug/terraform-provider-helloworld"])
shutil.copy("target/debug/terraform-provider-helloworld", f"target/terraform-provider-helloworld_v{version}")
subprocess.check_call(["zip", f"terraform-provider-helloworld_{version}_linux_amd64.zip", f"terraform-provider-helloworld_v{version}"], cwd="target")
shas = subprocess.check_output(["shasum", "-a", "256", f"terraform-provider-helloworld_{version}_linux_amd64.zip"], cwd="target").decode('utf-8')
open(f"target/terraform-provider-helloworld_{version}_SHA256SUMS", "w").write(shas)
subprocess.check_output(["gpg", "--detach-sign", f"terraform-provider-helloworld_{version}_SHA256SUMS"], cwd="target")