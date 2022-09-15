use std::path::Path;

#[derive(Clone, Copy)]
enum Version {
    Java11,
    Java16,
    Java17,
    Java8,
}

struct JavaInstallation {
    path: Box<Path>,
    version: Version,
}

fn get_installation(version: Version) {
    if let Some(path) = find_java(version) {
        todo!()
    } else {
        get_java(version).unwrap();
    }
}

#[cfg(any(target_os = "windows"))]
fn find_java(version: Version) -> Option<Box<Path>> {
    todo!();
}

#[cfg(any(target_os = "macos"))]
fn find_java(version: Version) -> Option<Box<Path>> {
    todo!();
}

#[cfg(any(target_os = "linux"))]
fn find_java(version: Version) -> Option<Box<Path>> {
    todo!();
}

fn get_java(version: Version) -> Result<Box<Path>, String> {
    // fetch graalvm from github
    todo!();
}
