use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Path to the Applications directory
    let apps_dir = "/Applications";

    // Read the directory
    let entries = fs::read_dir(apps_dir);
    match entries {
        Ok(entries) => {
            let mut app_names = Vec::new();

            for entry in entries {
                let entry = entry.expect("Failed to read entry");
                let app_name = format!("{}\n", entry.path().display());
                app_names.push(app_name);
            }

            let count = app_names.len();

            // Open a file for writing
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("apps_list.txt")
                .expect("Failed to open file");

            // Write the message and total count at the beginning
            file.write_all(b"please check the list for any applications you do not recognise\n").expect("Failed to write to file");
            let total_message = format!("Total number of applications: {}\n\n", count);
            file.write_all(total_message.as_bytes()).expect("Failed to write total to file");

            // Write the list of applications
            for app_name in app_names {
                file.write_all(app_name.as_bytes()).expect("Failed to write to file");
            }

            println!("List saved to apps_list.txt");


        },
        Err(e) => {
            eprintln!("Error reading {}: {}", apps_dir, e);
        }
    }
}
