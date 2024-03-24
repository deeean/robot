use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use crate::geometry::Point;

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[napi]
pub async fn mouse_move(x: i32, y: i32) -> Result<bool> {
    match tokio::spawn(async move {
        mouse_move_inner(x, y);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn mouse_click(button: MouseButton, x: i32, y: i32) -> Result<bool> {
    match tokio::spawn(async move {
        let (down, up) = match button {
            MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
            MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
            MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
        };

        mouse_move_inner(x, y);
        mouse_event(down, x, y, 0, 0);
        mouse_event(up, x, y, 0, 0);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn get_mouse_pos() -> Result<Point> {
    match tokio::spawn(async move {
        get_mouse_pos_inner()
    }).await {
        Ok(pos) => Ok(pos),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

fn get_mouse_pos_inner() -> Point {
    let mut pos = windows::Win32::Foundation::POINT { x: 0, y: 0 };
    unsafe {
        // @todo: Handle return value
        let _ = GetCursorPos(&mut pos);
    }

    Point::new(pos.x as u32, pos.y as u32)
}

fn mouse_event(dw_flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dw_data: i32, dw_extra_info: usize) {
    unsafe {
        let x = dx * 65536 / GetSystemMetrics(SM_CXSCREEN);
        let y = dy * 65536 / GetSystemMetrics(SM_CYSCREEN);
        windows::Win32::UI::Input::KeyboardAndMouse::mouse_event(dw_flags, x, y, dw_data, dw_extra_info);
    }
}

fn mouse_move_inner(x: i32, y: i32) {
    mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
}