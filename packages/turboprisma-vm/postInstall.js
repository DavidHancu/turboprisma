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
    "x86_64-unknown-linux-gnu": ["@turboprisma", "vm-cli-linux-x64"],
    "aarch64-unknown-linux-gnu": ["@turboprisma", "vm-cli-linux-arm64"],
    "x86_64-apple-darwin": ["@turboprisma", "vm-cli-darwin-x64"],
    "aarch64-apple-darwin": ["@turboprisma", "vm-cli-darwin-arm64"],
    "x86_64-pc-windows-msvc": ["@turboprisma", "vm-cli-windows-x64"],
    "aarch64-pc-windows-msvc": ["@turboprisma", "vm-cli-linux-arm64"],
};

const TURBO_VERSION =
    require("./package.json").optionalDependencies[
    "@turboprisma/vm-cli-windows-x64"
    ];

let os = process.platform;
let extension = "";
let arch = process.arch;

if (["win32", "cygwin"].includes(os)) {
    os = "pc-windows-msvc";
    extension = ".exe";
} else if (os == "darwin") {
    os = "apple-darwin";
} else {
    os = "unknown-linux-gnu";
}
if (arch == "arm64") {
    arch = "aarch64";
} else if (arch == "x64") {
    arch = "x86_64";
} else {
    throw new Error(
        "Invalid arch found. Turboprisma only supports arm64 and x64."
    );
}

const path = require("path");
const {
    renameSync,
    unlinkSync,
    mkdirSync,
    writeFileSync,
    rmSync
} = require("fs");
const child_process = require("child_process");

let packRoot = path.resolve(__dirname, "node_modules", ...map[`${arch}-${os}`]);
try {
    process.stdout.write(
        `Attempting to read binary from ${path.join(
            packRoot,
            `turboprisma_cli${extension}`
        )}...\n`
    );
    unlinkSync(path.join(__dirname, "turboprisma_cli"));
    renameSync(
        path.join(packRoot, `turboprisma_cli${extension}`),
        path.join(__dirname, `turboprisma_cli${extension}`)
    );
} catch {
    packRoot = path.resolve(__dirname, "../", ...map[`${arch}-${os}`]);
    try {
        process.stdout.write(
            `Attempting to read binary from ${require.resolve(
                map[`${arch}-${os}`].join("/")
            )}...\n`
        );
        renameSync(
            path.join(packRoot, `turboprisma_cli${extension}`),
            path.join(__dirname, `turboprisma_cli${extension}`)
        );
    } catch {
        try {
            packRoot = path.resolve(__dirname, "npm-install");
            mkdirSync(packRoot);
            writeFileSync(path.join(packRoot, "package.json"), "{}");
            child_process.execSync(
                `npm install --loglevel=error --prefer-offline --no-audit --progress=false ${map[
                    `${arch}-${os}`
                ].join("/")}@${TURBO_VERSION}`,
                { cwd: packRoot, stdio: "pipe", env: process.env }
            );
            renameSync(
                path.join(
                    packRoot,
                    "node_modules",
                    ...map[`${arch}-${os}`],
                    `turboprisma_cli${extension}`
                ),
                path.join(__dirname, `turboprisma_cli${extension}`)
            );
        } finally {
            rmSync(packRoot, { recursive: true, force: true });
        }
    }
}
