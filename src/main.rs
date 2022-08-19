mod args;
mod cookies;
pub use {args::*, cookies::*};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !handle_args() {
        return Ok(());
    }
    let cookie_store = cookies_from("cookies.json");
    {
        // Examine initial contents
        println!("initial load");
        let store = cookie_store.lock().unwrap();
        for c in store.iter_any() {
            println!("{:?}", c);
        }
    }

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()?;

    client.get("https://www.google.com").send().await?;
    {
        // Examine the contents of the store.
        println!("after google.com GET");
        let store = cookie_store.lock().unwrap();
        save_cookies_to(store, "cookies.json")
    }
    Ok(())
}
