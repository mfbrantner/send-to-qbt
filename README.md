# send-to-qbt - Send to qBittorrent
Send a `magnet` link or `.torrent` file to a remote qBittorrent instance.
My main use case for this is to be able to click on `magnet` links in Firefox and have my qBittorrent instance immediately download my ~~movies~~ Linux ISOs :)

## Setup

Create the file `$HOME/.config/send-to-qbt/config.toml` and add your credentials:
```
host_name = "localhost"
username = "qbt_username"
password = "qbt_password"
```

Make sure to make the file permissions as restrictive as possible to avoid leaking your credentials.

---

To test if it works, invoke the program with a single argument pointing to a `magnet` link or a `.torrent` file:
```
./send-to-qbt <magnet>
```
If it is set up correctly, it should display a desktop notification saying the torrent was added successfully.

---

Copy the `send-to-qbt.desktop` file to `$HOME/.local/share/applications/` and make sure the `Exec` points to the binary.
I put a symlink to the release build of the program in `~/.local/bin/`.
Then, run `update-desktop-database $HOME/.local/share/applications`.
The program should now show up in the app launcher.

---

Setting up Firefox to let me select a program to handle `magnet` links was not really that straightforward :/
You might have to add `network.protocol-handler.expose.magnet` to `about:config` and set it to `false`.

When clicking a `magnet` link, Firefox should now ask which program to use.
Now, select "Send to qBittorrent" from the list.

If everything works, the same notification as before should pop up.
