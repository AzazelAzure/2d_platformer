use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Player;

impl Player{
   pub fn new()-> (
        Player,
        Sprite,
        RigidBody,
        Collider,
        CollisionEventsEnabled,
        CollidingEntities,
        Transform,)
    {(
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
    )} 
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

pub fn player_accel(
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

