use std::ffi::{CString, NulError};
use std::os::raw::{c_char, c_float, c_int, c_void};

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

pub type GLFWwindow = c_void;

#[repr(C)]
#[derive(Debug)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
    pub x: c_float,
    pub y: c_float,
}

extern "C" {
    #[link_name = "create_game_window"]
    fn c_create_game_window(title: *const c_char, width: c_int, height: c_int);

    #[link_name = "create_sprite"]
    fn c_create_sprite(
        x: c_float,
        y: c_float,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    #[link_name = "render_sprite"]
    fn c_render_sprite(sprite: *mut Sprite);

    #[link_name = "update_sprite_position"]
    fn c_update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);

    #[link_name = "update_game_window"]
    fn c_update_game_window();

    #[link_name = "clear_screen"]
    fn c_clear_screen();

    #[link_name = "window_should_close"]
    fn c_window_should_close() -> c_int;

    #[link_name = "get_key"]
    fn c_get_key(window: *mut GLFWwindow, key: c_int) -> c_int;

    #[link_name = "get_window"]
    fn c_get_window() -> *mut GLFWwindow;
}

pub fn create_game_window(title: &str, width: i32, height: i32) -> Result<(), NulError> {
    let c_title = CString::new(title)?;

    unsafe {
        c_create_game_window(c_title.as_ptr(), width as c_int, height as c_int);
    }

    Ok(())
}

pub fn create_sprite(
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    r: i32,
    g: i32,
    b: i32,
) -> *mut Sprite {
    unsafe {
        c_create_sprite(
            x as c_float,
            y as c_float,
            width as c_int,
            height as c_int,
            r as c_int,
            g as c_int,
            b as c_int,
        )
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn render_sprite(sprite: *mut Sprite) {
    if sprite.is_null() {
        return;
    }

    unsafe {
        c_render_sprite(sprite);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn update_sprite_position(sprite: *mut Sprite, x: f32, y: f32) {
    if sprite.is_null() {
        return;
    }

    unsafe {
        c_update_sprite_position(sprite, x as c_float, y as c_float);
    }
}

pub fn update_game_window() {
    unsafe {
        c_update_game_window();
    }
}

pub fn clear_screen() {
    unsafe {
        c_clear_screen();
    }
}

pub fn window_should_close() -> bool {
    unsafe { c_window_should_close() != 0 }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int {
    if window.is_null() {
        return 0;
    }

    unsafe { c_get_key(window, key) }
}

pub fn get_window() -> *mut GLFWwindow {
    unsafe { c_get_window() }
}
