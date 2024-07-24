#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;


#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub use windows as platform;

#[cfg(target_os = "macos")]
pub use macos as platform;

pub enum Mouseclick {
    LEFT,
    RIGHT,
    MIDDLE
}

pub mod mouse_position;