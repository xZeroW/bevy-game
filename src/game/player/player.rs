use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (crate::game::player::controls::controls,
                crate::game::player::controls::sync_position_transform).chain());
    }
}
