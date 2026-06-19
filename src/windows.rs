use crate::Track;

use std::time::{SystemTime, UNIX_EPOCH};

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};


fn winrt_datetime_to_unix_ms(
    time: windows::Foundation::DateTime
) -> u64 {

    let windows_ticks =
        time.UniversalTime as u64;

    let unix_ticks =
        windows_ticks - 116444736000000000;

    unix_ticks / 10_000
}

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



    let playback =
        session
        .GetPlaybackInfo()
        .ok()?;



    let playing =
        playback
        .PlaybackStatus()
        .ok()?
        ==
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;



    let base_position =
        timeline
        .Position()
        .ok()?
        .Duration
        as u64 / 10_000;



    let mut position = base_position;



    if playing {

        let last_updated =
            timeline
            .LastUpdatedTime()
            .ok()?;


        let last_updated_ms =
            winrt_datetime_to_unix_ms(last_updated);



        let now_ms =
            SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_millis()
            as u64;



        position += now_ms - last_updated_ms;
    }



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
        position_ms: position.min(duration),
        playing,
    })
}