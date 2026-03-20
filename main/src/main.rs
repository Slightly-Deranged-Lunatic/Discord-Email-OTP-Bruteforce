use thirtyfour::prelude::*;
use whoami::platform;
use std::{env, error::Error, fs::{self, create_dir_all}, path::{Path, PathBuf}};
use ftail::Ftail;
use log::LevelFilter;
pub mod make_config_file;
use directories::BaseDirs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut local_data_directory = base_dirs.data_local_dir().to_path_buf();

        local_data_directory.push("slightly_deranged_lunatic");
        local_data_directory.push("discord_brute_force");

        // Make log directory
        if ! Path::exists(& local_data_directory) {
            fs::create_dir_all(&local_data_directory)?;
        }
        // Initalize logs
        Ftail::new()
        .daily_file(&local_data_directory.as_path(), LevelFilter::Info)
        .init()?;


        // Does a config file exist?
        let mut local_config_directory = base_dirs.config_local_dir().to_path_buf();
        local_config_directory.push("slightly_deranged_lunatic");
        local_config_directory.push("discord_brute_force");
        local_config_directory.push("config.json");

        if ! Path::exists(&local_config_directory) {
            log::info!("User had no config file, starting the creation process.");
            make_config_file::make_config_file(&local_config_directory);
        }

    }
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto("https://discord.com/login").await?;
    log::info!("Opened and navigated to https://discord.com/login");

    navigate_to_email_code_entry(driver.clone());

    driver.quit().await?;
    Ok(())
}

async fn navigate_to_email_code_entry(driver: WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    login_to_discord(driver);

    Ok(())
}

async fn login_to_discord(driver: WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_group_class = "animatedDiv_b97385"; // The box that has things like log in and text entries, grabbed because this will be frequently referenced and it feels easier to just grab from this
    let input_group = driver.find(By::ClassName(input_group_class)).await?;

    // Find email entry field
    let email_entry_id = "uid_15";
    let email_entry_field = input_group.find(By::Id(email_entry_id)).await?;

    // Find password entry field
    let password_entry_id = "uid_17";
    let password_entry_field = input_group.find(By::Id(password_entry_id)).await?;

    // Find log in button
    let log_in_button = input_group.find(By::Css("button[type='submit']")).await?; // CSS used because the class is way too long and there is no ID


    // For now this will be it as I need to write code for making config file, I really did not think this through huh
    todo!();
    Ok(())
}