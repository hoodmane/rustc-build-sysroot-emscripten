use clap_derive::Parser;
use clap::Parser;
// use rustc_version::VersionMeta;
use rustc_build_sysroot::{BuildMode, SysrootBuilder, rustc_sysroot_src};
use std::process::{Command};
use std::path::PathBuf;

/// Make an emscripten sysroot
#[derive(Parser)]
struct Cli {
    /// The folder to put the sysroot into
    #[arg(short, long)]
    outdir: PathBuf,
}

fn build_sysroot(b: SysrootBuilder) {
    let src_dir = rustc_sysroot_src(Command::new("rustc")).unwrap();
    b.cargo(Command::new("cargo"))
        .build_from_source(&src_dir)
        .unwrap();
}


fn main() {
    let args = Cli::parse();
    build_sysroot(
        SysrootBuilder::new(&args.outdir, "wasm32-unknown-emscripten")
            .build_mode(BuildMode::Build)
    );
}
