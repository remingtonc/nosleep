#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn prevent_sleep() {
    windows::prevent_sleep();
}

#[cfg(target_os = "windows")]
pub fn default_sleep() {
    windows::default_sleep();
}