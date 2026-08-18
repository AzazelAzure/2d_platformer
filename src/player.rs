use bevy::prelude::*;
use avian2d::prelude::*;
use crate::resources::PlayerStatus as status;

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
const GRAVITY: f32 = -1000.0;
const JUMP_HEIGHT: f32 = 600.0;
const PLAYER_HEIGHT: f32 = 50.0;
const PLAYER_WIDTH: f32 = 50.0;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
const TERMINAL_VELOCITY: f32 = -1500.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
    time: Res<Time>,
    mut status: ResMut<status>,
    ){
    let delta_secs = time.delta_secs();
    for mut linear_velocity in &mut query{
        // Move Left
        if keyboard_input.pressed(KeyCode::ArrowLeft){
            if linear_velocity.x > RUN_SPEED * -1.0 {linear_velocity.x -= 100.0 * delta_secs.powf(0.6)}
            else{linear_velocity.x = RUN_SPEED * -1.0}
            status.moving = true;
        }

        // Move right
        if keyboard_input.pressed(KeyCode::ArrowRight){
            if linear_velocity.x < RUN_SPEED {linear_velocity.x += 100.0 * delta_secs.powf(0.6)}
            else {linear_velocity.x = RUN_SPEED}
            status.moving = true;
        }
        
        if keyboard_input.just_released(KeyCode::ArrowLeft) || 
            keyboard_input.just_released(KeyCode::ArrowRight){
           status.moving = false;
        }

        // Jump
        if keyboard_input.pressed(KeyCode::ArrowUp){
            if !status.jumped && !status.falling{
                linear_velocity.y += 200.0 * delta_secs.powf(0.4);
                status.onground = false;
            }
        }
        if keyboard_input.just_released(KeyCode::ArrowUp){
            status.jumped = true;
            if !status.falling {linear_velocity.y = 200.0;}
        }
    }
}    


pub fn player_physics(
            mut query: Query<&mut LinearVelocity, With<Player>>,
            time: Res<Time>,
            mut status: ResMut<status>,
    ){
    let delta_secs = time.delta_secs();
    for mut linear_velocity in &mut query{
        // Player specific velocity handlers
        if linear_velocity.y >= JUMP_HEIGHT {linear_velocity.y = 300.0; status.falling = true;}
        if linear_velocity.y > TERMINAL_VELOCITY && !status.onground{
            if linear_velocity.y >= 0.0{linear_velocity.y += GRAVITY * delta_secs.powf(0.8)}
            else {linear_velocity.y += GRAVITY * delta_secs}}
        if !status.moving{
            if linear_velocity.x > 0.0 {linear_velocity.x -= 100.0 * delta_secs};
            if linear_velocity.x < 0.0 {linear_velocity.x += 100.0 * delta_secs};
        }    
        
    }
}


