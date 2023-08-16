use bevy::prelude::*;

#[bevy_main]
fn main() {
    println!("Starting launcher: Mobile");
    game::app(true).run();
}
