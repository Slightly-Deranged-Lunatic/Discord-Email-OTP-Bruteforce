use thirtyfour::prelude::*;
use std::{error::Error, path::Path};
use ftail::Ftail;
use log::LevelFilter;
use std::fs::File;
pub mod make_config_file;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ftail::new()
    .console(LevelFilter::Info)
    .daily_file(Path::new("../../logs"), LevelFilter::Info)
    .init()?;

    if ! Path::exists(Path::new("../../configs/")) {
        log::info!("User had no config file, starting the creation process.");
        make_config_file::make_config_file()
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