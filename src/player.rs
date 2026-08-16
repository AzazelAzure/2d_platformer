use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;

impl Player{
    fn new()-> (
        Player,
        Sprite,
        RigidBody,
        Collider,
        CollisionEventsEnabled,
        CollidingEntities,
        Transform,){(
        
        Player,
        Sprite::from_color(COLOR, PLAYER_SIZE),
        RigidBody::Kinematic,
        Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
        CollisionEventsEnabled,
        CollidingEntities::default(),
        Transform{
            translation: Vec3::new(PLAYER_WIDTH, PLAYER_HEIGHT, 0.0),
            ..default()
        },

        )
    }
    
}
// Player Constants
const COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const WALK_SPEED: f32 = 400.0;
const RUN_SPEED: f32 = 600.0;
const GRAVITY: f32 = -200.0;
const JUMP_HEIGHT: f32 = 250.0;
const PLAYER_HEIGHT: f32 = 50.0;
const PLAYER_WIDTH: f32 = 50.0;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);



