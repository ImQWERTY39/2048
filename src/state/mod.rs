use bevy::ecs::schedule::States;

#[derive(Clone, Default, PartialEq, Eq, Hash, States, Debug)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
}
