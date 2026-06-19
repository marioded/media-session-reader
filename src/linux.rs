use crate::Track;
use mpris::{PlayerFinder, PlaybackStatus};

pub fn current_track() -> Option<Track> {

    let finder =
        PlayerFinder::new()
        .ok()?;


    let player =
        finder
        .find_active()
        .ok()?;


    let metadata =
        player
        .get_metadata()
        .ok()?;


    let position =
        player
        .get_position()
        .ok()?
        .as_millis()
        as u64;


    let duration =
        metadata
        .length()?
        .as_millis()
        as u64;



    let title =
        metadata
        .title()
        .unwrap_or("Unknown")
        .to_string();


    let artist =
        metadata
        .artists()
        .and_then(|a| a.first())
        .unwrap_or(&"Unknown".to_string())
        .to_string();



    let playing =
        player
        .get_playback_status()
        .ok()?
        ==
        PlaybackStatus::Playing;



    Some(Track {

        title,

        artist,

        duration_ms: duration,

        position_ms: position,

        playing,
    })
}