use bevy::prelude::*;

use crate::game::player::component::Player;
use crate::game::common::components::characters::health::Health;

#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub damage: f32,
}

pub struct PlayerEventsPlugin;

impl Plugin for PlayerEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(|evt: On<PlayerDamagedEvent>, health_query: Query<&mut Health, With<Player>>| on_player_damaged(evt.event(), health_query));
    }
}

fn on_player_damaged(event: &PlayerDamagedEvent, mut health_query: Query<&mut Health, With<Player>>) {
    let mut health = match health_query.single_mut() {
        Ok(h) => h,
        Err(_) => return,
    };

    health.take_damage(event.damage);
    // info!("Player took {} damage (health {}/{})", event.damage, health.current, health.max);
}
