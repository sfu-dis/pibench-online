use cmake::Config;
use std::process::Command;

fn main() {
    Command::new("git")
        .args(&[
            "clone",
            "https://github.com/wangtzh/pibench.git",
            "target/pibench_src",
            "--recursive",
        ])
        .status()
        .unwrap();
    Command::new("mkdir")
        .args(&["-p", "target/pibench_build"])
        .status()
        .unwrap();
    let dst = Config::new("target/pibench_src")
        .out_dir("target/pibench_build")
        .profile("Release")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}
