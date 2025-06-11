use anyhow::{Result, anyhow, ensure};
use notify_rust::Notification;
use qbittorrent_rust::{
    core::{api::QbitApi, creds::Credentials},
    *,
};
use std::env;
use tokio;

// given the API object and the torrent URI, adds the torrent with default options
async fn add_torrent(t: &str, api: &mut QbitApi) -> Result<()> {
    let ttype = if t.starts_with("magnet:?") || t.starts_with("https://") {
        TorrentType::Url(t)
    } else {
        TorrentType::TorrentFile(t)
    };
    let desc = TorrentAddDescriptor::builder(vec![Torrent::new(ttype)])
        .build()
        .map_err(|e| anyhow!(e.message))?;

    api.torrents_add_torrent(desc)
        .await
        .map_err(|e| anyhow!(e.message))?;

    notify("Torrent added", "Added torrent successfully :)")?;
    Ok(())
}

#[derive(Debug)]
struct Config {
    creds: Credentials,
    host_name: String,
}

impl Config {
    // creates the config from a .env file
    // in a debug build, it will look for the .env file in the cwd of the process
    // in a release build, it will try to find the .env file at $HOME/.config/send-to-qbt/.env
    // I'm pretty sure that this is not the right way to do it, but i just need this to work for me
    fn from_dotenv() -> Result<Config> {
        #[cfg(debug_assertions)]
        {
            dotenv::dotenv()
                .map_err(|e| anyhow!(format!("[Debug] Could not open .env file: {}", e)))?;
        }

        #[cfg(not(debug_assertions))]
        {
            let home = std::env::var("HOME")?;
            let path = format!("{}/.config/send-to-qbt/.env", home);
            dbg!(&path);
            dotenv::from_path(path)
                .map_err(|e| anyhow!(format!("Could not open .env file: {}", e)))?;
        }

        let username = env::var("USERNAME")?;
        let password = env::var("PASSWORD")?;
        let host_name = env::var("HOST_NAME")?;
        Ok(Config {
            creds: Credentials::new(username, password),
            host_name: host_name,
        })
    }
}

// sends a desktop notification using notify_rust
// app name is hardcoded to Send to qBittorrent
// summary is more or less the title
// body is the actual text of the message
fn notify(summary: &str, body: &str) -> Result<()> {
    let app_name = "Send to qBittorrent";
    Notification::new()
        .summary(summary)
        .appname(app_name)
        .body(body)
        .show()?;
    Ok(())
}

// checks the args for the URI to the magnet/.torrent
fn get_uri() -> Result<String> {
    let mut args = std::env::args();
    // we want 2 arguments: <binary> <URI>
    // if we get != 2 args, notify user that they invoked the program wrong
    // the arg number in the message is off by one, as to not confuse the user
    ensure!(args.len() == 2, "Invalid number of arguments.");
    let torrent_uri = args.nth(1).ok_or(anyhow!("Could not read argument."))?;
    Ok(torrent_uri)
}

async fn add_to_qbt() -> Result<()> {
    // get the URI from the args
    let uri = get_uri()?;

    // check .env ($HOME/.config/send-to-qbt/.env)
    let config = Config::from_dotenv()?;

    // instantiate API
    let mut api = QbitApi::new(config.host_name, config.creds)
        .await
        .map_err(|e| anyhow!(e.message))?;

    add_torrent(&uri, &mut api).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = add_to_qbt().await {
        notify("Error", &format!("{e}"))?;
        return Err(e);
    }

    Ok(())
}
