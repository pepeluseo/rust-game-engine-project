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
