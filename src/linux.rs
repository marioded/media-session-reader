use crate::{Cover, RepeatMode, Track};

use std::fs;

use mpris::{PlaybackStatus, PlayerFinder};

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

pub fn current_track() -> Option<Track> {
    let finder = PlayerFinder::new().ok()?;

    let player = finder.find_active().ok()?;

    let metadata = player.get_metadata().ok()?;

    let position = player.get_position().ok()?.as_millis() as u64;

    let duration = metadata.length()?.as_millis() as u64;

    let title = metadata.title().unwrap_or("Unknown").to_string();

    let artist = metadata
        .artists()
        .and_then(|a| a.first())
        .map(|x| x.to_string())
        .unwrap_or("Unknown".to_string());

    let album = metadata.album().map(|x| x.to_string());

    let album_artist = metadata
        .album_artists()
        .and_then(|a| a.first())
        .map(|x| x.to_string());

    let cover = get_cover(metadata.art_url());

    let playing = player.get_playback_status().ok()? == PlaybackStatus::Playing;

    let shuffle = player.get_shuffle().ok();

    let repeat = player.get_loop_status().ok().map(|mode| match mode {
        mpris::LoopStatus::None => RepeatMode::None,

        mpris::LoopStatus::Track => RepeatMode::Track,

        mpris::LoopStatus::Playlist => RepeatMode::Playlist,

        _ => RepeatMode::None,
    });

    let can_next = player.can_go_next().unwrap_or(false);

    let can_previous = player.can_go_previous().unwrap_or(false);

    let position_ms = position.min(duration);

    let track_number = metadata.track_number().map(|x| x as u32);

    let genre = metadata
        .genre()
        .and_then(|x| x.first())
        .map(|x| x.to_string());

    Some(Track {
        title,
        artist,
        album,
        album_artist,
        cover,
        duration_ms: duration,
        position_ms,
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
