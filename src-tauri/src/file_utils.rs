use std::fs;

#[tauri::command]
pub fn listfiles(dirpath: String) -> Vec<String> {
    let entries = fs::read_dir(&dirpath)
        .expect("Error reading directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().map(String::from)
            })
        })
        .collect();

    entries
}
