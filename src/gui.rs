#[macro_use] extern crate log;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn main() -> iced::Result {
    windows::gui::main()
}