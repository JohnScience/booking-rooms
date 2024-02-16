use which::which;

const TRACKED_FRONTEND_PATHS: &[&str] = &[
    "package.json",
    "package-lock.json",
    "tsconfig.json",
    "tsconfig.node.json",
    "vite.config.ts",
    "index.html",
    "public",
    "src",
];

pub fn main() {
    for p in TRACKED_FRONTEND_PATHS.iter() {
        println!("cargo:rerun-if-changed=../front/{}", p);
    }
    let npm = which("npm").expect("npm not found");

    std::process::Command::new(&npm)
        .args(&["install"])
        .current_dir("../front")
        .output()
        .unwrap();
    std::process::Command::new(&npm)
        .args(&["run", "build"])
        .current_dir("../front")
        .output()
        .unwrap();
}
