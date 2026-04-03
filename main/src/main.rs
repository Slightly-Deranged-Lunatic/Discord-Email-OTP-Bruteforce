use thirtyfour::{prelude::*};
use core::panic;
use std::{ error::Error, fs, option, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{self, Duration} };
use rand::{RngExt, SeedableRng, rngs::{StdRng, SysRng}};
use ftail::Ftail;
use log::{LevelFilter};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use rand_regex;
use sysinfo::System;
pub mod make_config_file;


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

    let driver = start_webdriver().await;

    let mut rng = StdRng::try_from_rng(&mut SysRng).unwrap(); // From what I could see rng had to be mut to work?? I could just be stupid

    login_to_discord(&driver, &config_values).await?;
    navigate_to_email_code_entry(&driver, &mut rng).await?;
    bruteforce_code(&driver, &mut rng).await?;
    do_survey(&driver, &mut rng).await?;
    change_email(&driver, &mut rng, config_values).await?;

    Ok(())
}

async fn navigate_to_email_code_entry(driver: &WebDriver, rng: &mut StdRng) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Started navigating to email code entry");
    click_settings_button(driver).await?;
    sleep(rng, 1, 5);

    click_email_edit_button(driver).await?;
    sleep(rng, 1, 5);

    click_send_verification_code_button(driver).await?;
    sleep(rng, 1, 5);

    Ok(())
}

async fn login_to_discord(driver: &WebDriver, config_values: &ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    driver.goto("https://discord.com/login").await?;
    log::info!("Opened and navigated to https://discord.com/login");

    let input_group_class = "animatedDiv_b97385"; // The box that has things like log in and text entries, grabbed because this will be frequently referenced and it feels easier to just grab from this. Class was used because didn't have anything better from what I could see.
    driver.query(By::ClassName(input_group_class)).first().await?; // wait until the input group class is loaded to do anything
    let input_group = driver.find(By::ClassName(input_group_class)).await?;
    log::info!("Found input group");

    // Find email entry field
    let email_entry_css = r#"[aria-label="Email or Phone Number"]"#; // CSS used because UID changes randomly for no reason
    driver.query(By::Css(email_entry_css)).first().await?;
    let email_entry_field = input_group.find(By::Css(email_entry_css)).await?;
    log::info!("Found email entry field");

    // Find password entry field
    let password_entry_css = r#"[aria-label="Password"]"#; // CSS used because UID changes randomly for no reason
    driver.query(By::Css(password_entry_css)).first().await?;
    let password_entry_field = input_group.find(By::Css(password_entry_css)).await?;
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
    // Wait for settings button to exist and for the user to login
    let settings_button_css_selector = ".buttons__37e49 > button:nth-child(3)";
    log::info!("Waiting for user to login");

    let settings_button = driver.query(By::Css(settings_button_css_selector))
    .wait(Duration::from_secs(2300), Duration::from_millis(500)) // 2300 was just a random high number
    .first().await?;
    log::info!("Found settings button");

    // Wait for loading screen to fully disappear before clicking, not doing so results in an error
    let loading_screen_class_name = "container_a2f514 fixClipping_efbae7";
    loop {
        let is_loading = driver
        .query(By::ClassName(loading_screen_class_name))
        .wait(Duration::from_secs(3), Duration::from_millis(500)) // 3 because it will timeout faster and thus actually continue the program faster.
        .exists()
        .await?;
        if ! is_loading {
            break;
        }
    }

    settings_button.click().await?;

    Ok(())
}

async fn click_email_edit_button(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Wait for email edit button to exist
    let email_edit_button_css_selector = "div.field_a27e58:nth-child(3) > div:nth-child(2) > button:nth-child(1)";
    let email_edit_button = driver.find(By::Css(email_edit_button_css_selector)).await?;
    log::info!("Found email edit button");

    email_edit_button.click().await?;

    Ok(())
}

async fn click_send_verification_code_button(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Find send verfiication code button
    let send_verfiication_code_button_css_selector = "button.md_a22cb0:nth-child(2)";
    let send_verification_code_button = driver.find(By::Css(send_verfiication_code_button_css_selector)).await?;
    log::info!("Found send verfiication button");

    send_verification_code_button.click().await?;

    Ok(())
}

async fn bruteforce_code(driver: &WebDriver, rng: &mut StdRng) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Find necessary elements
    let code_ui_screen_css_selector = ".size-md__8a031";
    let code_ui_menu = driver.find(By::Css(code_ui_screen_css_selector)).await?;
    log::info!("Found code UI menu");

    let input_box_css_selector = r#"[label="Verification Code"]"#;
    let input_box = driver.find(By::Css(input_box_css_selector)).await?;
    log::info!("Found code input box.");

    let attempt_code_css_selector = ".actionBarTrailing__8a031 > div:nth-child(1) > button:nth-child(1)";
    let attempt_code_button = driver.find(By::Css(attempt_code_css_selector)).await?;
    log::info!("Found attempt code button.");

    let mut code_entry_count = 0;

    // Logic to continuously enter codes until one works
    loop {
        // Detect rate limits and sleep for 30 minutes if rate limits
        let code_ui_text = code_ui_menu.text().await?;
        if code_ui_text.contains("You are being rate limited.") {
            log::info!("Sleeping for 30 minutes due to rate limits being detected.");
            log::info!("Attempted {} codes before rate limits.", code_entry_count);
            
            let time_to_sleep = time::Duration::from_mins(30);
            thread::sleep(time_to_sleep); // We don't use sleep() because we need minutes not seconds, and I want a consistent value.
            code_entry_count = 0;
        } else { // Actually input the code
            let code = create_code()?;
            log::info!("Trying code {}", code);
            input_box.send_keys(code.clone()).await?;
            attempt_code_button.click().await?;
            code_entry_count += 1;

            sleep(rng, 3, 7);

            if code_worked(driver).await? {
                break;
            }

            input_box.clear().await?;

            log::info!("Cleared input box.");
            sleep(rng, 40, 120);
        }
    }

    Ok(())
}

async fn start_webdriver() -> WebDriver {
    let _gecko_driver = Command::new("geckodriver")
    .stdout(Stdio::null())
    .spawn()
    .expect("Geckodriver not installed.");
    log::info!("Trying to start Geckodriver");

    loop {
        let caps = DesiredCapabilities::firefox();

        match WebDriver::new("http://localhost:4444", caps).await {
            Ok(driver) => {
                log::info!("Driver started!");
                return driver;
            }
            Err(err) => {
                let err_string = err.to_string();
                if err_string.contains("Session is already started") {
                    kill_geckodriver_processes();
                    continue;
                    }
                let _gecko_driver = Command::new("geckodriver")
                .stdout(Stdio::null())
                .spawn()
                .expect("Geckodriver not installed.");
                log::info!("Trying to start Geckodriver");
            }
        };
    };
}

async fn code_worked(driver: &WebDriver) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let source = driver.source().await?;
    if source.contains("Why are you changing your email?") || source.contains("Enter an email address") {
        log::info!("Code worked!");
        Ok(true)
    }
    else {
        log::info!("Code failed");
        Ok(false)
    }
}

async fn do_survey(driver: &WebDriver, rng: &mut StdRng) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Does the discord survey asking why you changed your email
    let survey_text_css_selector = ".size-md__8a031";
    let survey = driver
        .query(By::ClassName(survey_text_css_selector))
        .wait(Duration::from_secs(10), Duration::from_millis(500)) // Wait 10 seconds for the survey to appear.
        .exists()
        .await?;
    if ! survey {
        log::info!("No survey found");
    }
    else {
        log::info!("Found survey");
        let option_css_selector = "label.radioGroupOption__64e61:nth-child(4) > span:nth-child(1) > input:nth-child(1)";
        let option_button = driver.find(By::Css(option_css_selector)).await?;
        option_button.click().await?;
        log::info!("Clicked an option button");
        sleep(rng, 3, 5);

        let continue_button_css_selector = "button.md_a22cb0:nth-child(2)";
        let continue_button = driver.find(By::Css(continue_button_css_selector)).await?;
        continue_button.click().await?;
        log::info!("Clicked condinue button");
        sleep(rng, 3, 5);

    }
    Ok(())
}

async fn change_email(driver: &WebDriver, rng: &mut StdRng, config_values: ConfigValues) -> Result<(), Box<dyn Error + Send + Sync>> {
    // The whole field thing as a whole not the input 
    let email_change_ui_css_selector = ".size-md__8a031";
    let email_change_ui = driver.find(By::Css(email_change_ui_css_selector)).await?;
    log::info!("Found email change ui thing");

    let email_entry_field_css = r#"[label="Email"]"#;
    let email_entry_field = email_change_ui.find(By::Css(email_entry_field_css)).await?;
    log::info!("Found email change input");

    let password_entry_field_css = r#"[label="Current Password"]"#;
    let password_entry_field = email_change_ui.find(By::Css(password_entry_field_css)).await?;
    log::info!("Found password input field.");

    if config_values.email != "" {
        email_entry_field.send_keys(config_values.new_email).await?;
        log::info!("Typed in email")
    }
    sleep(rng, 3, 5);

    if config_values.password != "" {
        password_entry_field.send_keys(config_values.password).await?;
        log::info!("Typed in password")
    }

    log::info!("Please input any data missing and press done. Enjoy your account.");
    Ok(())
}

fn kill_geckodriver_processes() {
    log::warn!("Geckodriver was already found, killing Geckodriver");
    let system = System::new_all();
    for process in system.processes_by_exact_name("geckodriver".as_ref()) {
        process.kill();
        log::info!("Killed geckodriver process")
    }
}

fn create_code() -> Result<String, Box<dyn Error + Send + Sync>> {

    let mut rng = rand::rng();
    let regex_gen = rand_regex::Regex::compile("[A-Z0-9]{6}", 100)?;

    let random_string:String = rng.sample(&regex_gen);
    log::info!("Generated code {}", random_string);

    Ok(random_string)
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

fn sleep(rng: &mut StdRng, min: u64, max: u64) {

    let time_to_sleep = rng.random_range(min..max);
    log::info!("Sleeping for {} seconds", time_to_sleep);
    let time_to_sleep = time::Duration::from_secs(time_to_sleep);
    thread::sleep(time_to_sleep);
}