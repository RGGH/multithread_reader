use std::io::Read;
use std::thread;
use std::path::PathBuf;
use std::fs::File;

fn main() {
    let file_list = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.txt"),
        PathBuf::from("file3.txt"),
        PathBuf::from("file4.txt"),
    ];
    
    let mut handles = Vec::new();

    for file_path in file_list {
        let handle = thread::spawn(move || {
            let mut content = String::new();
            match File::open(&file_path) {
                Ok(mut file) => {
                    if let Ok(_) = file.read_to_string(&mut content) {
                        println!("Content of {}: {}", file_path.display(), content);
                    }
                },
                Err(_) => {
                    eprintln!("Error opening or reading {}", file_path.display());
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

