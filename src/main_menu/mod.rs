use crate::state::GameState;
use bevy::prelude::*;

pub struct MainMenu;

#[derive(Component)]
struct MenuItems;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}

fn spawn_main_menu(mut command: Commands) {
    command
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MenuItems,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_column: GridPlacement::span(2),
                        grid_row: GridPlacement::span(1),
                        column_gap: Val::Percent(5.0),
                        width: Val::Percent(40.0),
                        height: Val::Percent(60.0),
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "2048",
                        TextStyle {
                            font_size: 128.0,
                            ..default()
                        },
                    ));

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(50.0),
                                height: Val::Percent(25.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn despawn_main_menu(mut command: Commands, bundles: Query<Entity, With<MenuItems>>) {
    command
        .entity(bundles.get_single().unwrap())
        .despawn_recursive();
}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => state.set(GameState::Playing),
            _ => (),
        }
    }
}
