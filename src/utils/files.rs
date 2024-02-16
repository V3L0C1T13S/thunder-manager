pub fn is_excluded_file(name: &str) -> bool {
    let excluded_files = ["icon.png", "manifest.json", "README.md"];

    excluded_files.into_iter().any(|v| name == v)
}
