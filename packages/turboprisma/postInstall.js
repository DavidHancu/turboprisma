/*
Linux: Save file as turboprisma_cli
MacOS: Save file as turboprisma_cli
Windows: Save file as turboprisma_cli.exe
*/

/*
Windows = x86_64-pc-windows-msvc aarch64-pc-windows-msvc
MacOS = x86_64-apple-darwin aarch64-apple-darwin
Linux = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
*/
const map = {
    "x86_64-unknown-linux-gnu": ["@turboprisma", "cli-linux-x64"],
    "aarch64-unknown-linux-gnu": ["@turboprisma", "cli-linux-arm64"],
    "x86_64-apple-darwin": ["@turboprisma", "cli-darwin-x64"],
    "aarch64-apple-darwin": ["@turboprisma", "cli-darwin-arm64"],
    "x86_64-pc-windows-msvc": ["@turboprisma", "cli-windows-x64"],
    "aarch64-pc-windows-msvc": ["@turboprisma", "cli-linux-arm64"]
}

let os = process.platform;
let extension = "";
let arch = process.arch;

if (["win32", "cygwin"].includes(os)) {
  os = "pc-windows-msvc";
  extension = ".exe";
} else if(os == "darwin") {
    os = "apple-darwin";
} else {
    os = "unknown-linux-gnu";
}
if(arch == "arm64")
{
    arch = "aarch64";
} else if (arch == "x64") {
    arch = "x86_64";
} else {
    throw new Error("Invalid arch found. Turboprisma only supports arm64 and x64.");
}

const path = require("path")
const { renameSync, unlinkSync } = require("fs");

let packRoot = path.resolve(__dirname, "node_modules", ...map[`${arch}-${os}`]);
try {
    process.stdout.write(`Attempting to read binary from ${path.join(packRoot, `turboprisma_cli${extension}`)}...\n`)
    unlinkSync(path.join(__dirname, "turboprisma_cli"));
    renameSync(path.join(packRoot, `turboprisma_cli${extension}`), path.join(__dirname, `turboprisma_cli${extension}`))
} catch {
    packRoot = path.resolve(__dirname, "../", ...map[`${arch}-${os}`]);
    process.stdout.write(`Attempting to read binary from ${path.join(packRoot, `turboprisma_cli${extension}`)}...\n`)
    unlinkSync(path.join(__dirname, "turboprisma_cli"));
    renameSync(path.join(packRoot, `turboprisma_cli${extension}`), path.join(__dirname, `turboprisma_cli${extension}`))
}