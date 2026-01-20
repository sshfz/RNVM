use crate::node::platform::platform;

pub(crate) fn url(version: &str) -> String {
    let (os, arch) = platform();
    return format!(
        "https://nodejs.org/dist/v{0}/node-v{0}-{1}-{2}.tar.xz",
        version, os, arch
    )
}