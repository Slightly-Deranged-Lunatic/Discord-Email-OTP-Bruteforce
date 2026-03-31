use std::{fs::File, io::{self, Write}, fs::{self}, path::{Path}};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct ConfigValues {
    email: String,
    password: String,
    new_email: String
}


pub fn make_config_file(local_config_directory: &Path) {
    let config_values = input_config_values(local_config_directory);
    let _ = create_config_file(local_config_directory, config_values);
}

fn input_config_values(local_config_directory: &Path) -> ConfigValues {
    println!("It looks like you didn't have any configuration file, no worries we'll get you set up with one.");
    println!("Please note that if at anytime you don't wanna put something in, you can just press enter and input it yourself when you have to.");
    println!("First, what is your email used to login?");

    let mut email = String::new();
    io::stdin()
        .read_line(& mut email)
        .expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    println!("What is your password? Please note this WILL be stored in plaintext, however, should you be uncomfortable with this, you can put it in yourself at the login screen and just press enter for right now.");
    let mut password = String::new();
    io::stdin()
        .read_line(& mut password)
        .expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    println!("What is the new email you would like to use for this account?");
    let mut new_email = String::new();
    io::stdin()
        .read_line(& mut new_email)
        .expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    let user_config_values = ConfigValues {
        email : email.trim().to_owned(),
        password : password.trim().to_owned(),
        new_email : new_email.trim().to_owned()
    };


    println!("Should you ever want to redo these configurations, you can delete the configs folder found in {}", local_config_directory.display());

    return user_config_values;
}

fn create_config_file(local_config_directory: &Path, config_values: ConfigValues) -> io::Result<()> {
    // Function that actually makes the configuration file and puts data in it
    let data = serde_json::to_string(&config_values)?;
    fs::create_dir_all(local_config_directory.parent().expect("Man idk what happend"))?;
    let mut file = File::create_new(local_config_directory)?;
    file.write_all(data.as_bytes())?;
    
    Ok(())
}