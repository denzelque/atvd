use thirtyfour::{prelude::*, stringmatch::StringMatch};
use tokio::time::sleep;
use std::{process::{Command, Stdio}, sync::Once, time::Duration};
use color_eyre::Result;
fn main() {}

// Initialize shared state for tests - spawn driver thread and acquire session
// Then go to target website
async fn init() -> Result<WebDriver> {
    std::env::set_var("RUST_BACKTRACE", "0");
    color_eyre::install().ok();
    
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        // Start driver executable
        std::thread::spawn(|| {
            Command::new("chromedriver.exe")
                .arg("--port=9515")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        });
    });
    
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://nushell.sh/").await?;
    Ok(driver)
}

#[tokio::test]
async fn click_goto() -> Result<()> {
    let driver = init().await?;
   
    let btn = driver.find(By::LinkText("Getting Started")).await?;
    btn.click().await?;
    

    // Wait until button becomes active
    btn.wait_until()
        .has_class(StringMatch::new("route-link-active").partial())
        .await.ok();

    assert_eq!(driver.title().await?, "Getting Started | Nushell");

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
async fn search_input() -> Result<()> {
    let driver = init().await?;
    
    let search_btn = driver.find(By::ClassName("DocSearch-Button")).await?;
    search_btn.click().await?;

    // Query and wait on condition
    let input = driver.query(By::ClassName("DocSearch-Input"))
        .and_displayed()
        .first()
        .await?;

    // Enter some text and check if it was entered
    input.send_keys("list").await?;
    assert_eq!(input.value().await?.as_deref(), Some("list"));
    
    driver.quit().await?;
    
    Ok(())
}
