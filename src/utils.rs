use std::{fs::File, io::Read, path::Path};

use crate::command_utils::Package;

#[allow(dead_code)]
pub fn get_package_json(path: &str) -> Package {
    let path = Path::new(path);
    if path.exists() && path.is_file() {
        let file = File::open(path);
        if let Ok(mut file) = file {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                return serde_json::from_str::<Package>(&contents).unwrap_or_default();
            }
            return Package::default();
        }
        return Package::default();
    }
    Package::default()
}
