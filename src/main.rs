use file_handler::User;
use std::process;

fn main() {
    let user_name = file_handler::prompt("Enter you username:");
    let file_name = file_handler::prompt("Enter file to save username to:");

    let mut user = User {
        name: user_name,
        file_name: file_name,
        read: false,
    };

    let mut file = match file_handler::open(&user.file_name) {
        Ok(file) => file,
        Err(error) => panic!("{error}"),
    };

    if let Err(error) = file_handler::write_to_file(&mut file, &user.name) {
        println!("Error writing to file: {error}");
        process::exit(1);
    }

    println!(
        "Saved username: {0:?} to file: {1:?}",
        user.name, user.file_name
    );

    let read_content = file_handler::prompt(&format!(
        "Do you want to read the content of {}?",
        user.file_name
    ));

    user.read = file_handler::should_read_content(&read_content);

    if user.read {
        let file_content = match file_handler::read_content_from(&user.file_name) {
            Ok(fc) => fc,
            Err(error) => panic!("Failed to read content: {error}"),
        };

        println!("Content of file {0:?}:", user.file_name);
        println!("{file_content}");
    }
}
