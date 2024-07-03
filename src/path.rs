use std::path::{Path, PathBuf};

pub fn find_exec(path_var: &str, exec: &str) -> Option<PathBuf> {
    if exec.contains("/") {
        let path = Path::new(exec);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    let path_vars: Vec<&str> = path_var.split(":").collect();

    for p in path_vars {
        let path = Path::new(p).join(exec);
        if path.exists() {
            return Some(path);
        }
    }

    return None;
}
