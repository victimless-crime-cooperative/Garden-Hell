use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerData {
    pub player_position: Vec3,
    pub player_velocity: Vec3,
    pub distance_from_floor: f32,
    pub floor_normal: Vec3,
    pub speed: f32,
    pub defacto_speed: f32,
    pub kicked_wall: Option<Entity>,
    pub jump_stage: u8,
    pub player_base_speed: f32,
    pub player_current_speed: f32,
    pub player_max_speed: f32,
}

impl PlayerData {
    pub fn new(speed: f32) -> Self {
        PlayerData {
            player_base_speed: speed,
            player_current_speed: speed,
            player_max_speed: speed * 2.0,
            ..default()
        }
    }
}
