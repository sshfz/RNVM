use core::{arch, panic};

pub(crate) fn platform() -> (&'static str, &'static str) {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let os = match os {
        "macos" => "darwin", 
        "linux" => "linux",
        _ => panic!("unsupported OS")
    };

    let arch = match arch {
        "x86_64" => "x64", 
        "aarch64" => "arm64",
        _ => panic!("unsupported arch")
    };

    return (os, arch);
}