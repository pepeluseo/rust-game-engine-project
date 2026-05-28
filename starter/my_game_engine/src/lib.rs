pub mod ffi;
pub mod macros;

#[cfg(test)]
mod tests {
    use super::ffi;
    use crate::move_sprite;
    use crate::on_key_press;
    use crate::start_window_and_game_loop;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_simple_game_loop() {
        ffi::create_game_window("Rust Simple Game Loop Test", 800, 600)
            .expect("Failed to create game window");

        while !ffi::window_should_close() {
            ffi::update_game_window();
            thread::sleep(Duration::from_millis(16));
        }
    }

    #[test]
    fn test_sprite_rendering() {
        ffi::create_game_window("Rust Sprite Rendering Test", 800, 600)
            .expect("Failed to create game window");

        let sprite = ffi::create_sprite(-0.5, 0.0, 120, 120, 255, 0, 0);

        while !ffi::window_should_close() {
            ffi::clear_screen();
            ffi::render_sprite(sprite);
            ffi::update_game_window();
            thread::sleep(Duration::from_millis(16));
        }
    }

    #[test]
    fn test_screen_clearing() {
        ffi::create_game_window("Rust Screen Clearing Test", 800, 600)
            .expect("Failed to create game window");

        let red_sprite = ffi::create_sprite(-0.5, 0.0, 120, 120, 255, 0, 0);
        let green_sprite = ffi::create_sprite(0.5, 0.0, 120, 120, 0, 255, 0);

        assert!(
            !red_sprite.is_null(),
            "Red sprite pointer should not be null"
        );
        assert!(
            !green_sprite.is_null(),
            "Green sprite pointer should not be null"
        );

        let start_time = Instant::now();

        while !ffi::window_should_close() {
            ffi::clear_screen();

            if start_time.elapsed() < Duration::from_secs(3) {
                ffi::render_sprite(red_sprite);
            } else {
                ffi::render_sprite(green_sprite);
            }

            ffi::update_game_window();
            thread::sleep(Duration::from_millis(16));
        }
    }

    #[test]
    fn test_key_presses() {
        ffi::create_game_window("Rust Key Press Test", 800, 600)
            .expect("Failed to create game window");

        let space_sprite = ffi::create_sprite(-0.8, 0.6, 80, 80, 255, 255, 255);
        let left_sprite = ffi::create_sprite(-0.4, 0.3, 80, 80, 255, 0, 0);
        let right_sprite = ffi::create_sprite(0.0, 0.0, 80, 80, 0, 255, 0);
        let up_sprite = ffi::create_sprite(0.4, -0.3, 80, 80, 0, 0, 255);
        let down_sprite = ffi::create_sprite(0.8, -0.6, 80, 80, 255, 255, 0);

        assert!(
            !space_sprite.is_null(),
            "Space sprite pointer should not be null"
        );
        assert!(
            !left_sprite.is_null(),
            "Left sprite pointer should not be null"
        );
        assert!(
            !right_sprite.is_null(),
            "Right sprite pointer should not be null"
        );
        assert!(!up_sprite.is_null(), "Up sprite pointer should not be null");
        assert!(
            !down_sprite.is_null(),
            "Down sprite pointer should not be null"
        );

        let mut space_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;

        while !ffi::window_should_close() {
            let window = ffi::get_window();

            on_key_press!(window, ffi::GLFW_KEY_SPACE, {
                space_pressed = true;
            });

            on_key_press!(window, ffi::GLFW_KEY_LEFT, {
                left_pressed = true;
            });

            on_key_press!(window, ffi::GLFW_KEY_RIGHT, {
                right_pressed = true;
            });

            on_key_press!(window, ffi::GLFW_KEY_UP, {
                up_pressed = true;
            });

            on_key_press!(window, ffi::GLFW_KEY_DOWN, {
                down_pressed = true;
            });

            ffi::clear_screen();

            if space_pressed {
                ffi::render_sprite(space_sprite);
            }

            if left_pressed {
                ffi::render_sprite(left_sprite);
            }

            if right_pressed {
                ffi::render_sprite(right_sprite);
            }

            if up_pressed {
                ffi::render_sprite(up_sprite);
            }

            if down_pressed {
                ffi::render_sprite(down_sprite);
            }

            ffi::update_game_window();
            thread::sleep(Duration::from_millis(16));

            if space_pressed && left_pressed && right_pressed && up_pressed && down_pressed {
                break;
            }
        }

        assert!(space_pressed, "Space key should be detected");
        assert!(left_pressed, "Left key should be detected");
        assert!(right_pressed, "Right key should be detected");
        assert!(up_pressed, "Up key should be detected");
        assert!(down_pressed, "Down key should be detected");
    }

    #[test]
    fn test_sprite_position_update() {
        ffi::create_game_window("Rust Sprite Position Update Test", 800, 600)
            .expect("Failed to create game window");

        let sprite = ffi::create_sprite(-0.9, 0.0, 100, 100, 0, 0, 255);

        assert!(!sprite.is_null(), "Sprite pointer should not be null");

        let mut x = -0.9_f32;
        let y = 0.0_f32;

        while !ffi::window_should_close() {
            move_sprite!(sprite, x, y, clear_screen = true);

            ffi::update_game_window();
            thread::sleep(Duration::from_millis(16));

            x += 0.01;

            if x > 0.9 {
                break;
            }
        }

        assert!(x > 0.9, "Sprite should have moved across the screen");
    }
    #[test]
    fn test_start_window_and_game_loop_macro() {
        let mut frame_count = 0;

        start_window_and_game_loop!(
            title: "Rust Macro Game Loop Test",
            width: 800,
            height: 600,
            on_start: {
                // Game loop setup can go here.
            },
            on_update: {
                frame_count += 1;
                ffi::clear_screen();

                if frame_count > 120 {
                    break;
                }
            },
            on_exit: {
                assert!(frame_count > 0, "Game loop should run at least once");
            }
        );
    }
}
