use thirtyfour::{common::{config, print}, prelude::*};
use std::{ error::Error, fs, path::{Path, PathBuf}};
use ftail::Ftail;
use log::LevelFilter;
pub mod make_config_file;
use directories::BaseDirs;
use std::fs::File;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct ConfigValues {
    email: String,
    password: String,
    new_email: String
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (local_data_directory, local_config_directory) = get_data_directories();

    // Make log directory
    if ! Path::exists(& local_data_directory) {
        fs::create_dir_all(&local_data_directory)?;
    }

    // Initalize logs
    Ftail::new()
    .daily_file(&local_data_directory.as_path(), LevelFilter::Info)
    .init()?;

    // Does a config file exist?
    if ! Path::exists(&local_config_directory) {
        log::info!("User had no config file, starting the creation process.");
        make_config_file::make_config_file(&local_config_directory);
        }
    
    // Get config stuff
    let config_values_result = get_config_values(local_config_directory.as_path());
    let config_values = match config_values_result {
        Ok(config) => {
            let config_values: ConfigValues = config;
            config_values
        }
        Err(e) => {
            log::error!("{}", e);
            ConfigValues {
                email: String::new(),
                new_email: String::new(),
                password: String::new(),
            }
        }
    };
    
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    println!("A");
    navigate_to_email_code_entry(driver, config_values);

    Ok(())
}

fn navigate_to_email_code_entry(driver: WebDriver, config_values: ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Inside navbigate email function");
    login_to_discord(driver, config_values);

    Ok(())
}

async fn login_to_discord(driver: WebDriver, config_values: ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    driver.goto("https://discord.com/login");
    log::info!("Opened and navigated to https://discord.com/login");
    let input_group_class = "animatedDiv_b97385"; // The box that has things like log in and text entries, grabbed because this will be frequently referenced and it feels easier to just grab from this
    driver.query(By::ClassName(input_group_class)); // wait until the input group class is loaded to do anything
    println!("A");
    let input_group = driver.find(By::ClassName(input_group_class)).await?;

    // Find email entry field
    let email_entry_id = "uid_15";
    let email_entry_field = input_group.find(By::Id(email_entry_id)).await?;

    // Find password entry field
    let password_entry_id = "uid_17";
    let password_entry_field = input_group.find(By::Id(password_entry_id)).await?;

    // Find log in button
    let log_in_button = input_group.find(By::Css("button[type='submit']")).await?; // CSS used because the class is way too long and there is no ID
    
    let mut auto_login = true;
    if config_values.email != "" {
        email_entry_field.send_keys(config_values.email).await?;
    } else {
        println!("Please enter your email manually, don't forget to log in");
        auto_login = false;
    }
    
    if config_values.password != "" {
        password_entry_field.send_keys(config_values.password).await?;
    } else {
        println!("Please enter your password manually, don't forget to log in");
        auto_login = false;
    }
    
    if auto_login {
        log_in_button.click().await;
    } else {
        println!("Press login when you are ready to login")
    }

    Ok(())
}

fn get_config_values(local_config_directory: &Path) -> Result<ConfigValues, Box< dyn Error>> {
    let content = fs::read_to_string(local_config_directory)?;
    let config_values: ConfigValues = serde_json::from_str(&content).unwrap();
    return  Ok(config_values);
}

fn get_data_directories() -> (PathBuf, PathBuf) {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut local_data_directory = base_dirs.data_local_dir().to_path_buf();

        local_data_directory.push("slightly_deranged_lunatic");
        local_data_directory.push("discord_brute_force");

        let mut local_config_directory = base_dirs.config_local_dir().to_path_buf();
        local_config_directory.push("slightly_deranged_lunatic");
        local_config_directory.push("discord_brute_force");
        local_config_directory = local_config_directory.join("config.json"); //.join is used here because if it was .push it would have been a directory not a json file

        return (local_data_directory, local_config_directory)
    } else{
        return (PathBuf::new(), PathBuf::new())
    }
}