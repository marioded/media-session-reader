use crate::{Cover, RepeatMode, Track};

use std::collections::HashMap;
use std::fs;

use zbus::blocking::{Connection, Proxy};
use zvariant::OwnedValue;

fn get_cover(url: Option<&str>) -> Option<Cover> {
    let url = url?;

    let data = if url.starts_with("file://") {
        let path = url.strip_prefix("file://")?;

        fs::read(path).ok()?
    } else {
        reqwest::blocking::get(url).ok()?.bytes().ok()?.to_vec()
    };

    Some(Cover {
        data,
        mime: "image/jpeg".to_string(),
    })
}

fn get_players(connection: &Connection) -> Option<Vec<String>> {
    let proxy = Proxy::new(
        connection,
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
    )
    .ok()?;

    let names: Vec<String> = proxy.call("ListNames", &()).ok()?;

    Some(
        names
            .into_iter()
            .filter(|n| n.starts_with("org.mpris.MediaPlayer2."))
            .collect(),
    )
}

pub fn current_track() -> Option<Track> {
    let connection = Connection::session().ok()?;

    let player_name = get_players(&connection)?.first()?.clone();

    let player = Proxy::new(
        &connection,
        player_name,
        "/org/mpris/MediaPlayer2",
        "org.mpris.MediaPlayer2.Player",
    )
    .ok()?;

    let metadata: HashMap<String, OwnedValue> = player.get_property("Metadata").ok()?;

    let title = metadata
        .get("xesam:title")
        .and_then(|v| {
            let r: Result<String, _> = v.clone().try_into();
            r.ok()
        })
        .unwrap_or_else(|| "Unknown".into());

    let artist = metadata
        .get("xesam:artist")
        .and_then(|v| {
            let r: Result<Vec<String>, _> = v.clone().try_into();
            r.ok()
        })
        .and_then(|v| v.first().cloned())
        .unwrap_or_else(|| "Unknown".into());

    let album = metadata.get("xesam:album").and_then(|v| {
        let r: Result<String, _> = v.clone().try_into();
        r.ok()
    });

    let album_artist = metadata
        .get("xesam:albumArtist")
        .and_then(|v| {
            let r: Result<Vec<String>, _> = v.clone().try_into();
            r.ok()
        })
        .and_then(|v| v.first().cloned());

    let genre = metadata
        .get("xesam:genre")
        .and_then(|v| {
            let r: Result<Vec<String>, _> = v.clone().try_into();
            r.ok()
        })
        .and_then(|v| v.first().cloned());

    let cover = metadata
        .get("mpris:artUrl")
        .and_then(|v| {
            let r: Result<String, _> = v.clone().try_into();
            r.ok()
        })
        .and_then(|url| get_cover(Some(&url)));

    let duration = metadata
        .get("mpris:length")
        .and_then(|v| {
            let r: Result<i64, _> = v.clone().try_into();
            r.ok()
        })
        .unwrap_or(0);

    let position: i64 = player.get_property("Position").unwrap_or(0);

    let playing = player
        .get_property::<String>("PlaybackStatus")
        .map(|x| x == "Playing")
        .unwrap_or(false);

    let shuffle = player.get_property::<bool>("Shuffle").ok();

    let repeat = player
        .get_property::<String>("LoopStatus")
        .ok()
        .map(|x| match x.as_str() {
            "Track" => RepeatMode::Track,
            "Playlist" => RepeatMode::Playlist,
            _ => RepeatMode::None,
        });

    let can_next = player.get_property::<bool>("CanGoNext").unwrap_or(false);

    let can_previous = player
        .get_property::<bool>("CanGoPrevious")
        .unwrap_or(false);

    let track_number = metadata
        .get("xesam:trackNumber")
        .and_then(|v| {
            let r: Result<i32, _> = v.clone().try_into();
            r.ok()
        })
        .map(|x| x as u32);

    Some(Track {
        title,
        artist,
        album,
        album_artist,
        cover,
        duration_ms: (duration / 1000) as u64,
        position_ms: (position / 1000) as u64,
        playing,
        playback_rate: None,
        shuffle,
        repeat,
        can_next,
        can_previous,
        track_number,
        genre,
    })
}
