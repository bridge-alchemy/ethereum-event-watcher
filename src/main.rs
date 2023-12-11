mod cron;
mod errors;
mod primitives;

use log::{ error, info };
use primitives::{ ServerResult, AppState };

use crate::cron::serve;

#[tokio::main]
async fn main() -> ServerResult<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let app_state = AppState {
        app_name: String::from("Ethereum event watcher."),
        // env,
    };
    info!("app_state: {:?}", app_state);
    if let Err(err) = serve(&app_state).await {
        error!("system error: {}", err);
    }

    Ok(())
}
