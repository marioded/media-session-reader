mod types;

pub use types::Track;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub fn current_track() -> Option<Track> {

    #[cfg(target_os = "windows")]
    {
        return windows::current_track();
    }

    #[cfg(target_os = "linux")]
    {
        return linux::current_track();
    }

    #[allow(unreachable_code)]
    None
}