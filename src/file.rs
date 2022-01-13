use std::path::{Path, PathBuf};

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

pub fn add_prefix(prefix: &str, path: &str) -> std::path::PathBuf {
    let basename = Path::new(&path).file_stem().unwrap().to_str().unwrap();
    let sorted_file_name = format!("{}{}", prefix, &basename);
    let new_path = change_file_name(&path, &sorted_file_name);
    new_path
}
