use thirtyfour::{common::{config, print}, prelude::*};
use std::{ error::Error, fs, path::{Path, PathBuf}};
use ftail::Ftail;
use log::{LevelFilter, info};
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
    .console(LevelFilter::Info)
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
    navigate_to_email_code_entry(driver, config_values).await?;

    Ok(())
}

async fn navigate_to_email_code_entry(driver: &WebDriver, config_values: ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Started navigating to email code entry");
    login_to_discord(driver, config_values).await?;
    click_settings_button(driver).await?;
    click_email_edit_button(driver).await?;

    Ok(())
}

async fn login_to_discord(driver: &WebDriver, config_values: ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    driver.goto("https://discord.com/login").await?;
    log::info!("Opened and navigated to https://discord.com/login");

    let input_group_class = "animatedDiv_b97385"; // The box that has things like log in and text entries, grabbed because this will be frequently referenced and it feels easier to just grab from this
    driver.query(By::ClassName(input_group_class)).first().await?; // wait until the input group class is loaded to do anything
    let input_group = driver.find(By::ClassName(input_group_class)).await?;
    log::info!("Found input group");

    // Find email entry field
    let email_entry_id = "uid_15";
    driver.query(By::Id(email_entry_id)).first().await?;
    let email_entry_field = input_group.find(By::Id(email_entry_id)).await?;
    log::info!("Found email entry field");

    // Find password entry field
    let password_entry_id = "uid_17";
    driver.query(By::Id(password_entry_id)).first().await?;
    let password_entry_field = input_group.find(By::Id(password_entry_id)).await?;
    log::info!("Found password entry field");
    
    if config_values.email != "" {
        email_entry_field.send_keys(config_values.email).await?;
        log::info!("Typed in email")
    }
    
    if config_values.password != "" {
        password_entry_field.send_keys(config_values.password).await?;
        log::info!("Typed in password")
    }

    log::info!("Please input any data missing and login.");

    Ok(())
}

async fn click_settings_button(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Find settings dock
    let settings_dock_css_selector = ".container__37e49"; // I had NO idea what to call this but its the little panel with the mute, deafen settings button etc
    driver.query(By::Css(settings_dock_css_selector)).first().await?;
    log::info!("Found settings dock");

    // Find settings button
    let settings_button_css_selector = ".buttons__37e49 > button:nth-child(3)";
    let settings_button = driver.find(By::Css(settings_button_css_selector)).await?;
    log::info!("Found settings button");

    settings_button.click().await?;

    Ok(())
}

async fn click_email_edit_button(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Find account settings group stuff
    let account_settings_group_css_selector = ".categories__6131a > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)";
    driver.query(By::Css(account_settings_group_css_selector)).first().await?;
    log::info!("Found account settings group");

    // Find email edit button
    let email_edit_button_css_selector = "div.field_a27e58:nth-child(3) > div:nth-child(2) > button:nth-child(1)";
    driver.query(By::Css(email_edit_button_css_selector));
    let email_edit_button = driver.find(By::Css(email_edit_button_css_selector)).await?;
    log::info!("Found email edit button");

    email_edit_button.click().await?;

    Ok(())
}

async fn click_send_verification_code_button(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Find button group with send verification code and cancel
    let button_group = ".actionBar__8a031";
    driver.query(By::Css(button_group)).first().await;
    log::info!("Found send verification code dock");

    // Find send verfiication code button
    let send_verfiication_code_button_css_selector = "button.md_a22cb0:nth-child(2)";
    let send_verification_code_button = driver.find(By::Css(send_verfiication_code_button_css_selector)).await?;

    send_verification_code_button.click();

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