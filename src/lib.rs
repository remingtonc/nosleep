#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn prevent_sleep() {
    windows::prevent_sleep();
}