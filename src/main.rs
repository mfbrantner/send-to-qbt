use anyhow::{Result, anyhow, ensure};
use notify_rust::Notification;
use qbittorrent_rust::{
    core::{api::QbitApi, creds::Credentials},
    *,
};
use std::{env, path::PathBuf};
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
    // expects config file at $HOME/.config/send-to-qbt/config.toml
    fn from_toml() -> Result<Config> {
        let home = env::var("HOME")?;
        let path = PathBuf::from(format!("{}/.config/send-to-qbt/config.toml", home));

        let settings = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?;

        let username = settings.get_string("username")?;
        let password = settings.get_string("password")?;
        let host_name = settings.get_string("host_name")?;
        Ok(Config {
            creds: Credentials::new(username, password),
            host_name,
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
    //let config = Config::from_dotenv()?;
    let config = Config::from_toml()?;

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
