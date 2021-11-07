import sys
import shutil
import subprocess

if len(sys.argv) != 2:
    raise Exception("Expected version")

version = sys.argv[1]

shutil.copy("target/debug/terraform-hello-world", f"target/terraform-provider-hello-world_v{version}")
subprocess.check_call(["zip", f"terraform-provider-hello-world_{version}_linux_amd64.zip", f"terraform-provider-hello-world_v{version}"], cwd="target")
shas = subprocess.check_output(["shasum", "-a", "256", f"terraform-provider-hello-world_{version}_linux_amd64.zip"], cwd="target").decode('utf-8')
open(f"target/terraform-provider-hello-world_{version}_SHA256SUMS", "w").write(shas)
subprocess.check_output(["gpg", "--detach-sign", f"terraform-provider-hello-world_{version}_SHA256SUMS"], cwd="target")