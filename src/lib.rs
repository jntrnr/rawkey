#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::RawKey;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;
#[cfg(all(unix, not(target_os = "macos")))]
pub use linux::RawKey;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::RawKey;

#[derive(Clone, Copy)]
pub enum KeyCode {
    Escape,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    LeftShift,
    LeftControl,
    Tab,
    BackSpace,
    Space,
    PageUp,
    PageDown,
    Alt,
}
