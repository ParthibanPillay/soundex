use discord_rich_presence::{
    activity::{Activity, ActivityType, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use serde_json::{json, Value};
use std::{
    io::Read,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

// Create an app at discord.com/developers, paste its Application ID here.
const CLIENT_ID: &str = "1563908474339852289";
// Matches the `key` in extension/manifest.json, so every install gets the same extension ID.
const ORIGIN: &str = "chrome-extension://enmkpjohffhmjocegfnioggikmbjihoa/";

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("install") {
        return install();
    }

    // As a native messaging host the browser holds our stdin open and sends the tab's state
    // (title, artists, album, art, url, artist_url, playing, pos, len, private) as
    // length-prefixed JSON about once a second; EOF means the YouTube Music tab closed.
    // Each message is stored with its arrival time so playback start can be derived from it.
    let tab: Arc<Mutex<(Value, i64)>> = Arc::new(Mutex::new((Value::Null, 0)));
    let tab_w = tab.clone();
    std::thread::spawn(move || {
        let mut i = std::io::stdin().lock();
        loop {
            let mut len = [0u8; 4];
            if i.read_exact(&mut len).is_err() {
                std::process::exit(0);
            }
            let mut buf = vec![0; u32::from_le_bytes(len) as usize];
            if i.read_exact(&mut buf).is_err() {
                std::process::exit(0);
            }
            if let Ok(v) = serde_json::from_slice(&buf) {
                *tab_w.lock().unwrap() = (v, now());
            }
        }
    });

    let mut rpc = DiscordIpcClient::new(CLIENT_ID);
    let mut connected = false;
    let mut last = String::new();
    let mut last_start = 0i64;
    let mut paused_since: Option<Instant> = None;

    loop {
        sleep(Duration::from_secs(2));

        let (t, at) = tab.lock().unwrap().clone();
        let s = |k: &str| t[k].as_str().unwrap_or_default().to_string();
        let n = |k: &str| t[k].as_f64().unwrap_or_default() as i64;
        let (title, artists, album, art) = (s("title"), s("artists"), s("album"), s("art"));
        let (url, artist_url) = (s("url"), s("artist_url"));
        let playing = t["playing"].as_bool() == Some(true);

        // Clear when nothing is loaded, privacy mode is on, or a pause has gone stale.
        paused_since = if playing { None } else { paused_since.or_else(|| Some(Instant::now())) };
        let stale = paused_since.is_some_and(|p| p.elapsed() > Duration::from_secs(600));
        if title.is_empty() || stale || t["private"].as_bool() == Some(true) {
            if !last.is_empty() {
                let _ = rpc.clear_activity();
                last.clear();
            }
            continue;
        }

        let (pos, len) = (n("pos"), n("len"));
        let start = at - pos;
        let key = format!("{title}|{artists}|{art}|{album}|{url}|{artist_url}|{playing}");
        // A start-time jump beyond rounding jitter means the user seeked.
        if key == last && (!playing || (start - last_start).abs() <= 3) {
            continue;
        }

        if !connected {
            connected = rpc.connect().is_ok();
            if !connected {
                continue; // Discord not running yet; retry next tick
            }
        }

        let fmt = |s: i64| format!("{}:{:02}", s / 60, s % 60);
        let paused_state = if len > 0 {
            format!("{artists} • Paused {} / {}", fmt(pos), fmt(len))
        } else {
            format!("{artists} • Paused {}", fmt(pos))
        };
        let mut act = Activity::new().activity_type(ActivityType::Listening).details(&title);
        act = if playing {
            let mut ts = Timestamps::new().start(start);
            if len > 0 {
                ts = ts.end(start + len);
            }
            act.state(&artists).timestamps(ts)
        } else {
            // No timestamps while paused, otherwise the bar keeps counting.
            act.state(&paused_state)
        };
        // "logo", "play" and "pause" are asset keys uploaded in the Discord dev portal (assets/).
        let mut assets = Assets::new().large_image(if art.starts_with("http") { &art } else { "logo" });
        if !album.is_empty() {
            assets = assets.large_text(&album);
        }
        act = act.assets(if playing {
            assets.small_image("play").small_text("Playing")
        } else {
            assets.small_image("pause").small_text("Paused")
        });
        let mut buttons = vec![];
        if url.starts_with("https://") {
            buttons.push(Button::new("Listen on YouTube Music", &url));
        }
        if artist_url.starts_with("https://") {
            buttons.push(Button::new("View Artist", &artist_url));
        }
        if !buttons.is_empty() {
            act = act.buttons(buttons);
        }

        match rpc.set_activity(act) {
            Ok(_) => (last, last_start) = (key, start),
            Err(_) => {
                let _ = rpc.close();
                connected = false; // Discord restarted; reconnect next tick
            }
        }
    }
}

// `soundex install`: register this binary as the native messaging host for installed browsers.
// ponytail: Chromium-family only; Firefox needs its own manifest format (allowed_extensions).
fn install() {
    let exe = std::env::current_exe().expect("no exe path");
    let manifest = json!({
        "name": "dev.soundex",
        "description": "Soundex Discord presence",
        "path": exe,
        "type": "stdio",
        "allowed_origins": [ORIGIN],
    })
    .to_string();

    if cfg!(windows) {
        // Windows reads a registry key whose value is the manifest's path.
        let file = exe.with_file_name("dev.soundex.json");
        std::fs::write(&file, manifest).expect("can't write manifest");
        for b in ["Google\\Chrome", "Chromium", "BraveSoftware\\Brave-Browser", "Microsoft\\Edge"] {
            let key = format!("HKCU\\Software\\{b}\\NativeMessagingHosts\\dev.soundex");
            let ok = std::process::Command::new("reg")
                .args(["add", &key, "/ve", "/t", "REG_SZ", "/d"])
                .arg(&file)
                .arg("/f")
                .output()
                .is_ok_and(|o| o.status.success());
            println!("{} {b}", if ok { "installed for" } else { "failed for" });
        }
        return;
    }

    let home = std::path::PathBuf::from(std::env::var("HOME").expect("no HOME"));
    let (base, browsers): (_, &[&str]) = if cfg!(target_os = "macos") {
        (
            home.join("Library/Application Support"),
            &["Google/Chrome", "Chromium", "BraveSoftware/Brave-Browser", "Microsoft Edge"],
        )
    } else {
        (
            home.join(".config"),
            &["google-chrome", "chromium", "BraveSoftware/Brave-Browser", "microsoft-edge", "net.imput.helium"],
        )
    };
    for b in browsers {
        let dir = base.join(b);
        if !dir.is_dir() {
            continue;
        }
        let dir = dir.join("NativeMessagingHosts");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("dev.soundex.json"), &manifest).unwrap();
        println!("installed for {b}");
    }
}
