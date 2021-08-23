#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn prevent_sleep() {
    windows::system::prevent_sleep();
}

#[cfg(target_os = "windows")]
pub fn default_sleep() {
    windows::system::default_sleep();
}