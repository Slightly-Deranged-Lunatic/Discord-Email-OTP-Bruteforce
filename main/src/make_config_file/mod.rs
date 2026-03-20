use std::io;
use std::path::Path;

struct ConfigValues {
    email: String,
    password: String,
    new_email: String
}

pub fn make_config_file() {
    let user_config_values = get_config_values();
}

fn get_config_values() -> ConfigValues {
    println!("It looks like you didn't have any configuration file, no worries we'll get you set up with one.");
    println!("First, what is your email used to login? This is necessary because we have to login");

    let mut email = String::new();
    io::stdin()
        .read_line(& mut email)
        .expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    println!("What is your password? Please note this WILL be stored in plaintext, however, should you be uncomfortable with this, you can put it in yourself at the login screen.");
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
        email : email,
        password : password,
        new_email : new_email
    };

    let config_file_path = Path::new("../../configs/config.toml");
    let config_file_display = config_file_path.display();
    println!("Should you ever want to redo these configurations, you can delete the configs folder found in {config_file_display}");

    return user_config_values;
}

fn create_config_file() {
    // Function that actually makes the configuration file, here because where it is depends on the OS and stuff for best practice
    let username = whoami::username_os(); // I'm not going to lie I don't even know if this the right thing to get but uh yeah
}