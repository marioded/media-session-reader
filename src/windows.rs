use crate::{Cover, RepeatMode, Track};

use std::time::{SystemTime, UNIX_EPOCH};

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

use windows::Storage::Streams::DataReader;

fn winrt_datetime_to_ms(time: windows::Foundation::DateTime) -> u64 {
    let ticks = time.UniversalTime as u64;

    if ticks < 116444736000000000 {
        return 0;
    }

    (ticks - 116444736000000000) / 10_000
}

fn get_cover(session: &GlobalSystemMediaTransportControlsSession) -> Option<Cover> {
    let media = session.TryGetMediaPropertiesAsync().ok()?.join().ok()?;

    let thumbnail = media.Thumbnail().ok()?.OpenReadAsync().ok()?.join().ok()?;

    let size = thumbnail.Size().ok()? as usize;

    if size == 0 || size > 10_000_000 {
        return None;
    }

    let reader = DataReader::CreateDataReader(&thumbnail).ok()?;

    reader.LoadAsync(size as u32).ok()?.join().ok()?;

    let mut data = vec![0u8; size];

    reader.ReadBytes(&mut data).ok()?;

    Some(Cover {
        data,

        mime: "image/jpeg".to_string(),
    })
}

pub fn current_track() -> Option<Track> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .ok()?
        .join()
        .ok()?;

    let session = manager.GetCurrentSession().ok()?;

    let media = session.TryGetMediaPropertiesAsync().ok()?.join().ok()?;

    let timeline = session.GetTimelineProperties().ok()?;

    let base_position = timeline.Position().ok()?.Duration as u64 / 10_000;

    let updated = timeline.LastUpdatedTime().ok()?;

    let updated_ms = winrt_datetime_to_ms(updated);

    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_millis() as u64;

    let position = base_position + now_ms.saturating_sub(updated_ms);

    let start = timeline.StartTime().ok()?.Duration as u64;

    let end = timeline.EndTime().ok()?.Duration as u64;

    let duration = (end - start) / 10_000;

    let playback = session.GetPlaybackInfo().ok()?;

    let controls = playback.Controls().ok()?;

    let playing = playback.PlaybackStatus().ok()?
        == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

    let shuffle = playback.IsShuffleActive().ok().and_then(|x| x.Value().ok());

    let repeat = playback
        .AutoRepeatMode()
        .ok()
        .and_then(|x| x.Value().ok())
        .map(|mode| match mode {
            windows::Media::MediaPlaybackAutoRepeatMode::None => RepeatMode::None,

            windows::Media::MediaPlaybackAutoRepeatMode::Track => RepeatMode::Track,

            windows::Media::MediaPlaybackAutoRepeatMode::List => RepeatMode::Playlist,

            _ => RepeatMode::None,
        });

    let playback_rate = playback.PlaybackRate().ok().and_then(|x| x.Value().ok());

    let title = media.Title().ok()?.to_string();

    let artist = media.Artist().ok()?.to_string();

    let album = media.AlbumTitle().ok().map(|x| x.to_string());

    let album_artist = media.AlbumArtist().ok().map(|x| x.to_string());

    let track_number = media.TrackNumber().ok().map(|x| x as u32);

    let genre = media
        .Genres()
        .ok()
        .and_then(|x| x.GetAt(0).ok())
        .map(|x| x.to_string());

    let cover = std::thread::spawn({
        let session = session.clone();

        move || get_cover(&session)
    })
    .join()
    .ok()
    .flatten();

    let position_ms = position.min(duration);

    Some(Track {
        title,
        artist,
        album,
        album_artist,
        cover,
        duration_ms: duration,
        position_ms,
        playing,
        playback_rate,
        shuffle,
        repeat,
        can_next: controls.IsNextEnabled().unwrap_or(false),
        can_previous: controls.IsPreviousEnabled().unwrap_or(false),
        track_number,
        genre,
    })
}
