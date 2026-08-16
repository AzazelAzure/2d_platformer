use bevy::{
    prelude::*,
    camera::ScalingMode,
};

use avian2d::prelude::*;

mod player;

// Floor Constants
const FLOOR_THICKNESS: f32 = 30.0;
const FLOOR_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

// Player Constants
const PLAYER_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const PLAYER_WALK_SPEED: f32 = 400.0;
const PLAYER_RUN_SPEED: f32 = 600.0;

// Physics Constants
const GRAVITY: f32 = -200.0;

// Screen Constants
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

// Wall Constants
const LEFT_WALL: f32 = -1.0 * (SCREEN_WIDTH/2.0);
const RIGHT_WALL: f32 = SCREEN_WIDTH/2.0;
const TOP_WALL: f32 = SCREEN_HEIGHT/2.0;
const BOTTOM_WALL: f32 = -1.0 * (SCREEN_HEIGHT/2.0);
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Player,  // Layer 1
    Enemy,   // Layer 2
    Ground,  // Layer 3
    Floor,   // Layer 4
}


#[derive(Component)]
#[require(Sprite, Transform)]
struct Wall;

enum WallLocation{
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation{
    fn position(&self) -> Vec2 {
        match self{
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
        }
    }
    fn size(&self) -> Vec2{
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        match self{
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)

            },
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall{
    fn new(location: WallLocation) -> (Wall, Sprite, Transform){(
        Wall,
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform{
            translation: location.position().extend(0.0),
            scale: location.size().extend(1.0),
            ..default()
        },
    )}
}

#[derive(Component)]
#[require(Sprite, Transform)]
struct Floor;

impl Floor{
    fn new() -> (Floor, Sprite, Transform, CollisionLayers){
        (
            Floor,
            Sprite::from_color(FLOOR_COLOR, Vec2::new(500.0, FLOOR_THICKNESS)),
            Transform{
                translation: Vec3::new(50.0, 10.0, 0.0),
                ..default()
            },
            CollisionLayers::new(
                GameLayer::Ground,
                [GameLayer::Player, GameLayer::Enemy]),
        )
    }
}


#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_accel, floor_collision).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    ){
    
    // Camera
    commands.spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection{
                scaling_mode: ScalingMode::AutoMin{
                    min_width: SCREEN_WIDTH,
                    min_height: SCREEN_HEIGHT},
                ..OrthographicProjection::default_2d()
            }),
        ));

    // Floor
    commands.spawn((
        Floor::new(),
        RigidBody::Static,
        Collider::rectangle(500.0, FLOOR_THICKNESS),
        CollisionEventsEnabled,
        CollidingEntities::default(),
        ));

    // Player
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::new(50.0, 50.0)),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 50.0),
        CollisionEventsEnabled,
        CollidingEntities::default(),
        CollisionLayers::new(
            GameLayer::Player,
            [GameLayer::Ground, GameLayer::Enemy, GameLayer::Floor],
            ),
        Transform{
            translation: Vec3::new(51.0, 55.0, 0.0),
            ..default()
            },
        Player
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Right));
}

fn player_accel(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
    time: Res<Time>,
    ){
    let delta_secs = time.delta_secs();
    let mut moving: bool= false;
    for mut linear_velocity in &mut query{
        for key in keyboard_input.get_pressed(){
            match key{
                KeyCode::ArrowLeft => {linear_velocity.x -= 50.0 * delta_secs; moving = true},
                KeyCode::ArrowRight => {linear_velocity.x += 50.0 * delta_secs; moving = true},
                KeyCode::ArrowDown =>  {linear_velocity.y -= 50.0 * delta_secs; moving = true},
                KeyCode::ArrowUp =>  {linear_velocity.y += 250.0 * delta_secs; moving = true},
                _ =>{}
            }
        }    
    linear_velocity.y += GRAVITY * delta_secs;
    if !moving{
        if linear_velocity.x > 0.0 {linear_velocity.x -= 100.0 * delta_secs};
        if linear_velocity.x < 0.0 {linear_velocity.x += 100.0 * delta_secs};
        }
    }
}

fn floor_collision(collision: Query<(Entity, &CollidingEntities), With<Floor>>, 
    mut query: Query<&mut LinearVelocity>
    ){
    
    for (entity, colliding_entities) in &collision{
        if colliding_entities.is_empty(){
            return;
        }
        if query.contains(entity)  {
            for mut linear_velocity in &mut query{
                if linear_velocity.y < 0.0{
                    linear_velocity.y = 0.0;
                }
            }
        }
    }
}


