use std::fs;

pub fn is_excluded_file(name: &str) -> bool {
    let excluded_files = [
        "icon.png",
        "manifest.json",
        "README.md",
        "changelog.txt",
        "CHANGELOG.md",
        "LICENSE",
        "LICENSE.md",
    ];

    excluded_files.into_iter().any(|v| name == v)
}

pub fn create_dir_all_or_fail<P: AsRef<std::path::Path>>(path: P, name: Option<&str>) {
    fs::create_dir_all(path).unwrap_or_else(|_| {
        panic!(
            "failed to create path {name}",
            name = name.unwrap_or_default()
        )
    })
}
