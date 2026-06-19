use crate::Track;

use mpris::{
    PlayerFinder,
    PlaybackStatus
};


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
        .ok()?;


    let status =
        player
        .get_playback_status()
        .ok()?;



    Some(Track {

        title:
            metadata
            .title()
            .unwrap_or("")
            .to_string(),


        artist:
            metadata
            .artists()
            .and_then(|x| x.first())
            .unwrap_or(&"")
            .to_string(),


        duration_ms:
            metadata
            .length()
            .map(|x| x.as_millis() as u64)
            .unwrap_or(0),


        position_ms:
            position
            .as_millis() as u64,


        playing:
            status == PlaybackStatus::Playing,
    })
}