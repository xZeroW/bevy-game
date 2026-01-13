use bevy::prelude::Component;

#[derive(Component)]
pub struct MoveSpeed(pub u32);

impl Default for MoveSpeed {
    fn default() -> Self {
        MoveSpeed(150)
    }
}
