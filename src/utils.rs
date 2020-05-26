use std::error::Error;
use std::fs;

pub fn get_path(path_option: Option<String>) -> String {
    let path = match path_option {
        Some(path) => format!("./{}", path),
        None => "./src/components".into(),
    };

    path
}

pub fn create_component_path(path: String) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path)?;

    Ok(())
}
