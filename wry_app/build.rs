use std::io::{Read, Write};

use sha2::{Digest, Sha256};
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

fn build_frontend() {
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

fn compute_package_lock_hash() -> [u8; 32] {
    let package_lock = std::fs::File::open("../front/package-lock.json")
        .expect("../front/package-lock.json not found");

    let mut package_lock_reader = std::io::BufReader::new(package_lock);

    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let count = package_lock_reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    hasher.finalize().into()
}

fn write_package_lock_hash(hash: [u8; 32]) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let hash_path = std::path::Path::new(&out_dir).join("package-lock-hash");

    let mut hash_file = std::fs::File::create(hash_path).unwrap();
    hash_file.write_all(&hash).unwrap();
}

mod compare_and_set_package_lock_hash {
    pub(crate) enum Outcome {
        Equal,
        Updated,
        FreshlyWritten,
    }

    impl Outcome {
        pub(crate) fn has_changed(&self) -> bool {
            !matches!(self, Self::Equal)
        }
    }
}

// Returns true if equal. Returns false if not equal or file does not exist.
fn compare_and_set_package_lock_hash(
    computed: [u8; 32],
) -> compare_and_set_package_lock_hash::Outcome {
    use compare_and_set_package_lock_hash::Outcome::{Equal, FreshlyWritten, Updated};

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let hash_path = std::path::Path::new(&out_dir).join("package-lock-hash");

    if !hash_path.exists() {
        write_package_lock_hash(computed);
        return FreshlyWritten;
    }

    let mut hash_file = std::fs::File::open(hash_path).unwrap();
    let mut stored_hash = [0; 32];
    hash_file.read_exact(&mut stored_hash).unwrap();
    if stored_hash == computed {
        Equal
    } else {
        write_package_lock_hash(computed);
        Updated
    }
}

pub fn main() {
    for p in TRACKED_FRONTEND_PATHS.iter() {
        println!("cargo:rerun-if-changed=../front/{}", p);
    }

    if !std::path::Path::new("../front/node_modules").exists() {
        build_frontend();
        return;
    };

    let computed_package_lock_hash = compute_package_lock_hash();
    if compare_and_set_package_lock_hash(computed_package_lock_hash).has_changed() {
        build_frontend();
    }
}
