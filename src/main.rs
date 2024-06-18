use bevy::prelude::*;

mod main_menu;
use main_menu::MainMenu;

mod game;
use game::GameScreen;

mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("2048"),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.82, 0.70, 0.54)))
        .add_systems(Startup, spawn_camera)
        .add_plugins(MainMenu)
        .add_plugins(GameScreen)
        .run();
}

fn spawn_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}
