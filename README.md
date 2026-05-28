# Building a Game Engine with Rust from the Ground Up

This repository serves as the starting point for building a simple yet functional game engine using Rust. The project is designed to help you apply the concepts and skills you've learned throughout the Rust course. By completing this project, you'll gain hands-on experience with Rust's advanced features, including multi-threading, macros, and interoperability with C code.

## Project Overview

This project implements a simple game engine from scratch using Rust and a C OpenGL/GLFW wrapper library.

The project demonstrates:

- Rust and C interoperability through FFI.
- A Cargo `build.rs` script to compile and link a C library.
- Safe Rust wrapper functions around C functions.
- Declarative macros to simplify game engine operations.
- Visual tests for windowing, rendering, input handling, screen clearing, and sprite movement.
- A simple game using multithreading, channels, HTTP requests, and JSON deserialization.

The project contains two main Rust crates:

```text
my_game_engine
rust_test_game
```

---

## Project Structure

```text
starter/
├── Makefile
├── README.md
├── c_output/
├── c_test_game/
├── opengl_wrapper_lib/
│   ├── opengl_wrapper_lib.c
│   └── opengl_wrapper_lib.h
├── my_game_engine/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── opengl_wrapper_lib/
│   │   ├── opengl_wrapper_lib.c
│   │   └── opengl_wrapper_lib.h
│   └── src/
│       ├── ffi.rs
│       ├── lib.rs
│       └── macros.rs
└── rust_test_game/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        ├── messages.rs
        └── networking.rs
```

---

## Requirements

This project was developed and tested in a Linux/WSL environment.

Required tools and libraries:

- Rust
- Cargo
- GCC
- Make
- GLFW
- OpenGL development libraries

## Getting Started

When working on this project, you can choose between utilizing Udacity's VM environment or setting it all up on your local machine. If you prefer to use the Udacity VM, you can skip the local environment prerequisite steps and jump straight to the project instructions.

### Local environment prerequisites

While this project has no specific dependencies on any system, it was built on a Unix-based machine. So, if you're on Windows, I'd recommend using the Windows Subsystem for Linux (WSL), so all instructions here directly apply to your system.  

For this project, you'll need to have Rust installed in your machine. If you haven't installed Rust yet, you can do so with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Also, because we are dealing with C code in this project, you'll need to have a C compiler installed on your machine. You can install the `build-essential` package, which includes the GNU C Compiler (GCC) and other necessary tools:

```bash
sudo apt update
sudo apt install build-essential
```

Finally, you'll need to have GLFW installed in your machine. GLFW is a C library that will be the foundation of our game engine. You can install it with:

```bash
sudo apt install libglfw3 libglfw3-dev
```

### Running the Test C Game

To start with your project, clone this repository to your local machine:

```bash
git clone https://github.com/udacity/intro-to-rust-starter.git
# or, git clone git@github.com:udacity/intro-to-rust-starter.git
```

To ensure you are set up correctly, you can run the test C game that comes with this project. You can build and run the test game with:

```bash
cd intro-to-rust-starter/starter
make run-c
```

You should see the following pop-up window:

![readme_image_assets/img.png](readme_image_assets/img.png)

### Starting the Rust Game Engine

With this done, we are ready to start creating our game engine project. Inside the `starter` folder, create a new Rust library project called `my_game_engine` (or whatever name you prefer), and then refer to the project instructions in the Udacity website to start building your game engine.

> Note: If you choose another name for your engine, you must update the references to `my_game_engine` in the `starter` folder's `Makefile` to reflect the new name.

## Description

The Rust game engine includes a `build.rs` script:

```text
my_game_engine/build.rs
```

The build script compiles:

```text
opengl_wrapper_lib/opengl_wrapper_lib.c
```

and links the required graphics libraries.

---

## `my_game_engine/Cargo.toml`

```toml
[package]
name = "my_game_engine"
version = "0.1.0"
edition = "2021"

[dependencies]

[build-dependencies]
cc = "1"
```

---

## `my_game_engine/build.rs`

```rust
fn main() {
    println!("cargo:rerun-if-changed=opengl_wrapper_lib/opengl_wrapper_lib.c");
    println!("cargo:rerun-if-changed=opengl_wrapper_lib/opengl_wrapper_lib.h");

    cc::Build::new()
        .file("opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("opengl_wrapper_lib")
        .compile("openglwrapper");

    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
}
```

---

## Validation

```bash
cd my_game_engine
cargo build
```

Expected result:

```text
Finished `dev` profile
```

---

# Task 2: FFI Bindings

## Description

FFI bindings are implemented in:

```text
my_game_engine/src/ffi.rs
```

The Rust FFI module binds to the C functions declared in:

```text
opengl_wrapper_lib/opengl_wrapper_lib.h
```

Implemented bindings include:

- `create_game_window`
- `create_sprite`
- `render_sprite`
- `update_sprite_position`
- `update_game_window`
- `clear_screen`
- `window_should_close`
- `get_key`
- `get_window`

The C-compatible `Sprite` struct is represented in Rust with:

```rust
#[repr(C)]
```

This ensures the Rust struct layout is compatible with the C struct layout.

---

## C Header: `opengl_wrapper_lib/opengl_wrapper_lib.h`

```c
#ifndef OPENGL_WRAPPER_LIB_H
#define OPENGL_WRAPPER_LIB_H

#include <GLFW/glfw3.h>

// Structure to represent a sprite
typedef struct {
    int width;
    int height;
    int color[3]; // RGB color
    float x, y; // Position
} Sprite;

// Function to create a game window
void create_game_window(const char *title, int width, int height);

// Function to create a sprite
Sprite* create_sprite(float x, float y, int width, int height, int r, int g, int b);

// Function to render a sprite
void render_sprite(Sprite *sprite);

// Function to update a sprite position
void update_sprite_position(Sprite *sprite, float x, float y);

// Function to update the game window
void update_game_window();

// Function to clear the screen
void clear_screen();

// Function to check if the window should close
int window_should_close();

// Function to get key state
int get_key(GLFWwindow* window, int key);

// Function to get the window pointer
GLFWwindow* get_window();

#endif // OPENGL_WRAPPER_LIB_H
```

---

## `my_game_engine/src/ffi.rs`

```rust
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
```

---

## Validation

```bash
cd my_game_engine
cargo build
```

Expected result:

```text
Finished `dev` profile
```

---

# Task 3: FFI Visual Tests

## Description

Visual tests are implemented in:

```text
my_game_engine/src/lib.rs
```

Implemented tests:

- `test_simple_game_loop`
- `test_sprite_rendering`
- `test_screen_clearing`
- `test_key_presses`
- `test_sprite_position_update`

Run tests one by one because each test opens a graphical window.

---

## `my_game_engine/src/lib.rs`

```rust
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

        let sprite = spawn_sprite!(-0.5, 0.0, 120, 120, 255, 0, 0);

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

        assert!(!red_sprite.is_null(), "Red sprite pointer should not be null");
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
```

---

## Run Visual Tests

```bash
cd my_game_engine

cargo test test_simple_game_loop -- --nocapture
cargo test test_sprite_rendering -- --nocapture
cargo test test_screen_clearing -- --nocapture
cargo test test_key_presses -- --nocapture
cargo test test_sprite_position_update -- --nocapture
cargo test test_start_window_and_game_loop_macro -- --nocapture
```

---

## `test_key_presses` Controls

For `test_key_presses`, press:

```text
SPACE
LEFT
RIGHT
UP
DOWN
```

Expected result:

```text
test tests::test_key_presses ... ok
```

---

# Task 4: Declarative Macros

## Description

Declarative macros are implemented in:

```text
my_game_engine/src/macros.rs
```

Implemented macros:

- `spawn_sprite!`
- `on_key_press!`
- `tick!`
- `move_sprite!`
- `start_window_and_game_loop!`

---

## `my_game_engine/src/macros.rs`

```rust
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
```

---

## Macro Examples

### `spawn_sprite!`

Creates and renders a sprite.

```rust
let sprite = spawn_sprite!(0.0, 0.0, 100, 100, 255, 0, 0);
```

---

### `on_key_press!`

Runs an action when a key is pressed.

```rust
on_key_press!(window, ffi::GLFW_KEY_SPACE, {
    println!("Space pressed");
});
```

---

### `tick!`

Updates the game window and sleeps for a short duration.

```rust
tick!();
```

Or with custom milliseconds:

```rust
tick!(32);
```

---

### `move_sprite!`

Updates a sprite position and renders it.

```rust
move_sprite!(sprite, x, y);
```

With screen clearing:

```rust
move_sprite!(sprite, x, y, clear_screen = true);
```

---

### `start_window_and_game_loop!`

Creates a window and starts a reusable game loop.

```rust
start_window_and_game_loop!(
    title: "My Game",
    width: 800,
    height: 600,
    on_start: {
        // setup
    },
    on_update: {
        // per-frame logic
    },
    on_exit: {
        // cleanup
    }
);
```

---

# Task 5: Tests Updated to Use Macros

## Description

The tests were updated to use the macros:

- `test_sprite_rendering` uses `spawn_sprite!`
- `test_key_presses` uses `on_key_press!`
- `test_sprite_position_update` uses `move_sprite!`
- `test_start_window_and_game_loop_macro` validates `start_window_and_game_loop!`

---

## Run Macro Validation Test

```bash
cd my_game_engine
cargo test test_start_window_and_game_loop_macro -- --nocapture
```

Expected result:

```text
test tests::test_start_window_and_game_loop_macro ... ok
```

---

# Task 6: Simple Game

## Description

The required simple game is implemented in:

```text
rust_test_game/
```

The game demonstrates:

- A game loop.
- Keyboard input with `on_key_press!`.
- HTTP requests using `reqwest`.
- JSON deserialization using `serde`.
- A secondary networking thread.
- Message passing using `std::sync::mpsc`.
- Non-blocking receive using `try_recv`.
- Graceful shutdown using a `Quit` command and `join`.

The game fetches sprite data from:

[Random Sprite Data API](https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler)

---

## `rust_test_game/Cargo.toml`

```toml
[package]
name = "rust_test_game"
version = "0.1.0"
edition = "2021"

[dependencies]
my_game_engine = { path = "../my_game_engine" }
reqwest = { version = "0.12", default-features = false, features = ["blocking", "json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
```

---

## `rust_test_game/src/messages.rs`

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteData {
    pub b: i32,
    pub g: i32,
    pub height: i32,
    pub r: i32,
    pub width: i32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub enum NetworkCommand {
    FetchSprite,
    Quit,
}
```

---

## `rust_test_game/src/networking.rs`

```rust
use crate::messages::{NetworkCommand, SpriteData};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};

const SPRITE_ENDPOINT: &str =
    "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

pub fn fetch_sprite_data() -> Result<SpriteData, Box<dyn Error + Send + Sync>> {
    let sprite_data = reqwest::blocking::get(SPRITE_ENDPOINT)?.json::<SpriteData>()?;
    Ok(sprite_data)
}

pub fn run_networking_thread(
    command_receiver: Receiver<NetworkCommand>,
    sprite_sender: Sender<SpriteData>,
) {
    while let Ok(command) = command_receiver.recv() {
        match command {
            NetworkCommand::FetchSprite => match fetch_sprite_data() {
                Ok(sprite_data) => {
                    if sprite_sender.send(sprite_data).is_err() {
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Failed to fetch sprite data: {error}");
                }
            },
            NetworkCommand::Quit => {
                break;
            }
        }
    }
}
```

---

## `rust_test_game/src/main.rs`

```rust
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
```

---

## Run the Game

```bash
cd rust_test_game
cargo run
```

---

## How to Play

1. A window opens with an initial red sprite.
2. Click inside the game window.
3. Press `SPACE`.
4. The networking thread fetches random sprite data.
5. A new sprite appears when the response is received.
6. Close the window to quit.

---

## Expected Terminal Output

```text
Starting Rust Test Game...
Press SPACE inside the game window to fetch a random sprite.
Close the window to quit.
SPACE pressed. Sending fetch request to networking thread...
Sprite data received from networking thread.
Spawning sprite => raw: x=393, y=104, w=44, h=44, rgb=(83, 4, 71), normalized: x=-0.02, y=0.65
Window closed. Stopping networking thread...
Game exited successfully.
```

---

# Validation Commands

## Validate the Game Engine

```bash
cd my_game_engine
cargo fmt
cargo build
cargo clippy -- -D warnings
```

---

## Run Visual Engine Tests

```bash
cargo test test_simple_game_loop -- --nocapture
cargo test test_sprite_rendering -- --nocapture
cargo test test_screen_clearing -- --nocapture
cargo test test_key_presses -- --nocapture
cargo test test_sprite_position_update -- --nocapture
cargo test test_start_window_and_game_loop_macro -- --nocapture
```

---

## Validate the Simple Game

```bash
cd ../rust_test_game
cargo fmt
cargo build
cargo clippy -- -D warnings
cargo run
```

---

# WSL Graphics Notes

When running inside WSL, graphical warnings related to Mesa, EGL, ZINK, or libdecor may appear.

Examples:

```text
libEGL warning
MESA: error: ZINK
Failed to load plugin 'libdecor-gtk.so'
```

These warnings do not block the project if the game window opens and sprites render correctly.

---

# Rust Concepts Demonstrated

This project demonstrates:

- Cargo library and binary crates.
- Cargo build scripts.
- C interoperability with FFI.
- `#[repr(C)]` structs.
- Safe Rust wrappers around unsafe C calls.
- Declarative macros with `macro_rules!`.
- Visual tests.
- Multithreading with `std::thread`.
- Message passing with `std::sync::mpsc`.
- Non-blocking channel receive with `try_recv`.
- HTTP requests with `reqwest`.
- JSON deserialization with `serde`.
- Modular Rust code using `mod`.
- Error handling with `Result`.

---

# Final Project Status

```text
✅ C test game runs successfully
✅ Rust library game engine created
✅ build.rs compiles the C library
✅ FFI bindings implemented
✅ Rust wrapper functions implemented
✅ #[repr(C)] Sprite struct implemented
✅ Required visual tests implemented
✅ Declarative macros implemented
✅ Tests updated to use macros
✅ Simple game created
✅ HTTP request handled on secondary thread
✅ Message passing implemented
✅ Code formatted with cargo fmt
✅ Code checked with cargo clippy
```

## License

[License](LICENSE.txt)
