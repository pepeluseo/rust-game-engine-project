mod messages;
mod networking;

use messages::{NetworkCommand, SpriteData};
use my_game_engine::ffi;
use my_game_engine::{on_key_press, spawn_sprite};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn normalize_x(x: f32) -> f32 {
    // Endpoint x is roughly pixel-based. Convert 0..800 to -1.0..1.0.
    (x / 400.0) - 1.0
}

fn normalize_y(y: f32) -> f32 {
    // Endpoint y is roughly pixel-based. Convert 0..600 to 1.0..-1.0.
    1.0 - (y / 300.0)
}

fn create_sprite_from_data(sprite_data: &SpriteData) -> *mut ffi::Sprite {
    let x = normalize_x(sprite_data.x);
    let y = normalize_y(sprite_data.y);

    let width = sprite_data.width.max(40);
    let height = sprite_data.height.max(40);

    println!(
        "Spawning sprite => raw: x={}, y={}, w={}, h={}, rgb=({}, {}, {}), normalized: x={:.2}, y={:.2}",
        sprite_data.x,
        sprite_data.y,
        sprite_data.width,
        sprite_data.height,
        sprite_data.r,
        sprite_data.g,
        sprite_data.b,
        x,
        y
    );

    spawn_sprite!(
        x,
        y,
        width,
        height,
        sprite_data.r,
        sprite_data.g,
        sprite_data.b
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rust Test Game...");
    println!("Press SPACE inside the game window to fetch a random sprite.");
    println!("Close the window to quit.");

    let (command_sender, command_receiver) = mpsc::channel::<NetworkCommand>();
    let (sprite_sender, sprite_receiver) = mpsc::channel::<SpriteData>();

    let networking_handle = thread::spawn(move || {
        networking::run_networking_thread(command_receiver, sprite_sender);
    });

    ffi::create_game_window("Rust Test Game - Random Sprites", 800, 600)?;

    // Initial visible sprite so we know the game is rendering.
    let initial_sprite = spawn_sprite!(0.0, 0.0, 100, 100, 255, 0, 0);
    let mut sprites: Vec<*mut ffi::Sprite> = vec![initial_sprite];

    let mut already_requested = false;

    while !ffi::window_should_close() {
        let window = ffi::get_window();

        on_key_press!(window, ffi::GLFW_KEY_SPACE, {
            if !already_requested {
                println!("SPACE pressed. Sending fetch request to networking thread...");

                if let Err(error) = command_sender.send(NetworkCommand::FetchSprite) {
                    eprintln!("Failed to send fetch command: {error}");
                }

                already_requested = true;
            }
        });

        if let Ok(sprite_data) = sprite_receiver.try_recv() {
            println!("Sprite data received from networking thread.");
            let sprite = create_sprite_from_data(&sprite_data);
            sprites.push(sprite);
            already_requested = false;
        }

        ffi::clear_screen();

        for sprite in &sprites {
            ffi::render_sprite(*sprite);
        }

        ffi::update_game_window();
        thread::sleep(Duration::from_millis(16));
    }

    println!("Window closed. Stopping networking thread...");
    let _ = command_sender.send(NetworkCommand::Quit);
    let _ = networking_handle.join();

    println!("Game exited successfully.");
    Ok(())
}
