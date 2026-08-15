use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
};

const FLOOR_THICKNESS: f32 = 10.0;
const FLOOR_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const PLAYER_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const PLAYER_SPEED: f32 = 400.0;

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Floor;

impl Floor{
    fn new() -> (Floor, Sprite, Transform){
        (
            Floor,
            Sprite::from_color(FLOOR_COLOR, Vec2::new(500.0, 10.0)),
            Transform{
                translation: Vec3::new(50.0, 10.0, 0.0),
                ..default()
            },
        )
    }
}


#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    ){
    
    // Camera
    commands.spawn(Camera2d);

    // Floor
    commands.spawn(Floor::new());

    // Player
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::new(50.0, 50.0)),
        Transform{
            translation: Vec3::new(51.0, 61.0, 0.0),
            ..default()
            },
        Player
    ));
}

fn move_player(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut player_transform: Single<&mut Transform, With<Player>>,
        time: Res<Time>,
    ){
    
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;
    
    if keyboard_input.pressed(KeyCode::ArrowLeft){
        x_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight){
        x_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp){
        y_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown){
        y_direction -= 1.0;
    }

    let new_x_pos = 
        player_transform.translation.x + x_direction * PLAYER_SPEED * time.delta_secs();
    
    let new_y_pos =
        player_transform.translation.y + y_direction * PLAYER_SPEED * time.delta_secs();

    player_transform.translation.x = new_x_pos;
    player_transform.translation.y = new_y_pos;
}
