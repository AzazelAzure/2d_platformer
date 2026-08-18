use bevy::{ prelude::*,
    camera::ScalingMode,
};

use avian2d::prelude::*;

use crate::resources::PlayerStatus;

mod player;
mod resources;

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


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .init_resource::<PlayerStatus>()
        .add_systems(Startup, setup)
        .add_systems(Update, (floor_collision, player::player_physics, player::player_movement).chain())
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

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Right));
    
    // Player
    commands.spawn((
            player::Player::new(), 
            CollisionLayers::new(
                GameLayer::Player,
                [GameLayer::Floor, GameLayer::Ground, GameLayer::Enemy],
                )));
}

fn floor_collision(
    collision: Query<(Entity, &mut CollidingEntities), With<Floor>>, 
    mut query: Query<&mut LinearVelocity, Without<Floor>>,
    layer_query: Query<&CollisionLayers>,
    mut player_status: ResMut<PlayerStatus>,
    ){
    for (entity, colliding_entities) in &collision{
        if colliding_entities.is_empty(){
            return;
        }
        for e in colliding_entities.iter(){
            if let Ok(layers) = layer_query.get(*e){
                if layers.memberships.has_all(GameLayer::Player){
                    player_status.onground = true;
                    player_status.jumped = false;
                    player_status.falling = false;
                }
            }
            if query.contains(*e)  {
                for mut linear_velocity in &mut query{
                    if linear_velocity.y < 0.0{
                        linear_velocity.y = 0.0;}
               }
            }
        }
    }
}


