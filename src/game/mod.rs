use crate::state::GameState;
use bevy::prelude::*;

pub struct GameScreen;

use rand::Rng;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Number {
    Empty,
    Contains(u32),
}

#[derive(Resource)]
struct Game {
    board: Vec<Vec<Number>>,
}

#[derive(Component)]
struct Block(usize, usize);

impl Game {
    fn new() -> Self {
        Self {
            board: vec![vec![Number::Empty; 4]; 4],
        }
    }

    fn init(&mut self) {
        let mut random = rand::thread_rng();

        let (row1, col1) = self.empty_place().unwrap();
        let (row2, col2) = self.empty_place().unwrap();

        let number1 = if random.gen_ratio(4, 5) { 2 } else { 4 };
        let number2 = if random.gen_ratio(4, 5) { 2 } else { 4 };

        self.board[row1][col1] = Number::Contains(number1);
        self.board[row2][col2] = Number::Contains(number2);
    }

    fn up(&mut self) {
        let mut random = rand::thread_rng();

        for row in 1..4 {
            for col in 0..4 {
                let current = match self.board[row][col] {
                    Number::Empty => continue,
                    Number::Contains(i) => i,
                };

                let mut current_row = (row - 1) as isize;

                while current_row >= 0 {
                    match self.board[current_row as usize][col] {
                        Number::Empty => current_row -= 1,
                        Number::Contains(k) => {
                            self.board[row][col] = Number::Empty;

                            if current == k {
                                self.board[current_row as usize][col] =
                                    Number::Contains(current * 2);
                            } else {
                                self.board[(current_row + 1) as usize][col] =
                                    Number::Contains(current);
                            }

                            break;
                        }
                    }
                }

                if current_row == -1 {
                    self.board[row][col] = Number::Empty;
                    self.board[(current_row + 1) as usize][col] = Number::Contains(current);
                }
            }
        }

        let (row, col) = match self.empty_place() {
            Some(i) => i,
            None => return,
        };

        let number = if random.gen_ratio(4, 5) { 2 } else { 4 };
        self.board[row][col] = Number::Contains(number);
    }

    fn down(&mut self) {
        let mut random = rand::thread_rng();

        for row in (0..3).rev() {
            for col in 0..4 {
                let current = match self.board[row][col] {
                    Number::Empty => continue,
                    Number::Contains(i) => i,
                };

                let mut current_row = (row + 1) as isize;

                while current_row <= 3 {
                    match self.board[current_row as usize][col] {
                        Number::Empty => current_row += 1,
                        Number::Contains(k) => {
                            self.board[row][col] = Number::Empty;

                            if current == k {
                                self.board[current_row as usize][col] =
                                    Number::Contains(current * 2);
                            } else {
                                self.board[(current_row - 1) as usize][col] =
                                    Number::Contains(current);
                            }

                            break;
                        }
                    }
                }

                if current_row == 4 {
                    self.board[row][col] = Number::Empty;
                    self.board[(current_row - 1) as usize][col] = Number::Contains(current);
                }
            }
        }

        let (row, col) = match self.empty_place() {
            Some(i) => i,
            None => return,
        };

        let number = if random.gen_ratio(4, 5) { 2 } else { 4 };
        self.board[row][col] = Number::Contains(number);
    }

    fn left(&mut self) {
        let mut random = rand::thread_rng();

        for row in 0..4 {
            for col in 1..4 {
                let current = match self.board[row][col] {
                    Number::Empty => continue,
                    Number::Contains(i) => i,
                };

                let mut current_col = (col - 1) as isize;

                while current_col >= 0 {
                    match self.board[row][current_col as usize] {
                        Number::Empty => current_col -= 1,
                        Number::Contains(k) => {
                            self.board[row][col] = Number::Empty;

                            if current == k {
                                self.board[row][current_col as usize] =
                                    Number::Contains(current * 2);
                            } else {
                                self.board[row][(current_col + 1) as usize] =
                                    Number::Contains(current);
                            }

                            break;
                        }
                    }
                }

                if current_col == -1 {
                    self.board[row][col] = Number::Empty;
                    self.board[row][(current_col + 1) as usize] = Number::Contains(current);
                }
            }
        }

        let (row, col) = match self.empty_place() {
            Some(i) => i,
            None => return,
        };

        let number = if random.gen_ratio(4, 5) { 2 } else { 4 };
        self.board[row][col] = Number::Contains(number);
    }

    fn right(&mut self) {
        let mut random = rand::thread_rng();

        for row in 0..4 {
            for col in (0..3).rev() {
                let current = match self.board[row][col] {
                    Number::Empty => continue,
                    Number::Contains(i) => i,
                };

                let mut current_col = (col + 1) as isize;

                while current_col <= 3 {
                    match self.board[row][current_col as usize] {
                        Number::Empty => current_col += 1,
                        Number::Contains(k) => {
                            self.board[row][col] = Number::Empty;

                            if current == k {
                                self.board[row][current_col as usize] =
                                    Number::Contains(current * 2);
                            } else {
                                self.board[row][(current_col - 1) as usize] =
                                    Number::Contains(current);
                            }

                            break;
                        }
                    }
                }

                if current_col == 4 {
                    self.board[row][col] = Number::Empty;
                    self.board[row][(current_col - 1) as usize] = Number::Contains(current);
                }
            }
        }

        let (row, col) = match self.empty_place() {
            Some(i) => i,
            None => return,
        };

        let number = if random.gen_ratio(4, 5) { 2 } else { 4 };
        self.board[row][col] = Number::Contains(number);
    }

    fn reset(&mut self) {
        self.board = vec![vec![Number::Empty; 4]; 4];
        self.init();
    }

    fn empty_place(&self) -> Option<(usize, usize)> {
        let mut random = rand::thread_rng();
        let mut has_empty = false;

        'main_loop: for i in &self.board {
            for j in i {
                if *j == Number::Empty {
                    has_empty = true;
                    break 'main_loop;
                }
            }
        }

        if !has_empty {
            return None;
        }

        loop {
            let row = random.gen_range(0..=3);
            let col = random.gen_range(0..=3);

            if self.board[row][col] == Number::Empty {
                return Some((row, col));
            }
        }
    }
}

impl Plugin for GameScreen {
    fn build(&self, app: &mut App) {
        let mut board = Game::new();
        board.init();

        app.insert_resource(board)
            .add_systems(OnEnter(GameState::Playing), spawn_entities)
            .add_systems(
                Update,
                detect_key_press.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_entities(mut command: Commands, game: Res<Game>) {
    command
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                row_gap: Val::Px(25.0),
                column_gap: Val::Px(25.0),
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                padding: UiRect::all(Val::Px(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|mut parent| {
            for i in 0..4 {
                for j in 0..4 {
                    let number_str = match game.board[i][j] {
                        Number::Empty => None,
                        Number::Contains(i) => Some(i.to_string()),
                    };

                    create_box(&mut parent, i, j, number_str);
                }
            }
        });
}

fn create_box(builder: &mut ChildBuilder, pos_x: usize, pos_y: usize, number_str: Option<String>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        number_str.unwrap_or_default(),
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                Block(pos_x, pos_y),
            ));
        });
}

fn detect_key_press(
    keys: Res<Input<KeyCode>>,
    mut blocks: Query<(&Block, &mut Text)>,
    mut game: ResMut<Game>,
) {
    if keys.any_just_released([KeyCode::Up, KeyCode::W]) {
        game.up();
        update_board(&mut blocks, &game);
    }

    if keys.any_just_released([KeyCode::Down, KeyCode::S]) {
        game.down();
        update_board(&mut blocks, &game);
    }

    if keys.any_just_released([KeyCode::Left, KeyCode::A]) {
        game.left();
        update_board(&mut blocks, &game);
    }

    if keys.any_just_released([KeyCode::Right, KeyCode::D]) {
        game.right();
        update_board(&mut blocks, &game);
    }

    if keys.just_released(KeyCode::R) {
        game.reset();
        update_board(&mut blocks, &game);
    }
}

fn update_board(blocks: &mut Query<(&Block, &mut Text)>, game: &ResMut<Game>) {
    for (block_pos, mut text) in blocks.iter_mut() {
        text.sections[0].value = match game.board[block_pos.0][block_pos.1] {
            Number::Empty => String::new(),
            Number::Contains(i) => i.to_string(),
        }
    }
}
