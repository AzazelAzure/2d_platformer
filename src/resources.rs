use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerStatus{
    pub jumped: bool,
    pub falling: bool,
    pub moving: bool,
    pub onground: bool,
}
