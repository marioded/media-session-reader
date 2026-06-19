use crate::Track;

use windows::Media::Control::
    GlobalSystemMediaTransportControlsSessionManager;


pub fn current_track() -> Option<Track> {

    let manager =
        GlobalSystemMediaTransportControlsSessionManager::
        RequestAsync()
        .ok()?
        .get()
        .ok()?;


    let session =
        manager
        .GetCurrentSession()
        .ok()?;


    let media =
        session
        .TryGetMediaPropertiesAsync()
        .ok()?
        .get()
        .ok()?;


    let timeline =
        session
        .GetTimelineProperties()
        .ok()?;


    let position =
        timeline
        .Position()
        .ok()?
        .Duration
        as u64 / 10_000;


    let end =
        timeline
        .EndTime()
        .ok()?
        .Duration
        as u64;


    let start =
        timeline
        .StartTime()
        .ok()?
        .Duration
        as u64;


    let duration =
        (end - start) / 10_000;


    let title =
        media
        .Title()
        .ok()?
        .to_string();


    let artist =
        media
        .Artist()
        .ok()?
        .to_string();



    Some(Track {

        title,

        artist,

        duration_ms: duration,

        position_ms: position,

        playing: true,
    })
}