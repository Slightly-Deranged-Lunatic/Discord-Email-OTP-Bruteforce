use thirtyfour::prelude::*;
use std::{error::Error, path::Path};
use ftail::Ftail;
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ftail::new()
    .console(LevelFilter::Info)
    .daily_file(Path::new("../../logs"), LevelFilter::Error)
    .init()?;

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto("https://discord.com/login").await?;
    log::info!("Opened and navigated to https://discord.com/login");

    driver.quit().await?;
    Ok(())
}