#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = $crate::ffi::create_sprite($x, $y, $width, $height, $r, $g, $b);
        assert!(!sprite.is_null(), "Sprite pointer should not be null");
        $crate::ffi::render_sprite(sprite);
        sprite
    }};
}

#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $action:block) => {{
        if $crate::ffi::get_key($window, $key) == $crate::ffi::GLFW_PRESS {
            $action
        }
    }};
}

#[macro_export]
macro_rules! tick {
    () => {{
        $crate::ffi::update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }};
    ($millis:expr) => {{
        $crate::ffi::update_game_window();
        std::thread::sleep(std::time::Duration::from_millis($millis));
    }};
}

#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {{
        $crate::ffi::update_sprite_position($sprite, $x, $y);
        $crate::ffi::render_sprite($sprite);
    }};
    ($sprite:expr, $x:expr, $y:expr, clear_screen = true) => {{
        $crate::ffi::clear_screen();
        $crate::ffi::update_sprite_position($sprite, $x, $y);
        $crate::ffi::render_sprite($sprite);
    }};
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    (
        title: $title:expr,
        width: $width:expr,
        height: $height:expr,
        on_start: $on_start:block,
        on_update: $on_update:block,
        on_exit: $on_exit:block
    ) => {{
        $crate::ffi::create_game_window($title, $width, $height)
            .expect("Failed to create game window");

        $on_start

        while !$crate::ffi::window_should_close() {
            $on_update
            $crate::tick!();
        }

        $on_exit
    }};
}
