use std::path::PathBuf;

pub(crate) fn rnvm_dirs() -> PathBuf {
    dirs::home_dir() 
     .expect("Couldn't find home directory")
     .join("rnvm")
}

pub(crate) fn versions_dir() -> PathBuf {
    rnvm_dirs().join("versions")
}

pub(crate) fn bin_dir() -> PathBuf {
    rnvm_dirs().join("bin")
}

pub(crate) fn config_file() -> PathBuf {
    rnvm_dirs().join("config.json")
}