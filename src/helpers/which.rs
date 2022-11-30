use std::{env, path::PathBuf};

pub fn which(program: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|dir| {
        let path = dir.join(program);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    })
}
