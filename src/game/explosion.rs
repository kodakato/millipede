use super::*;
use std::time::Duration;

#[derive(Component)]
pub struct Explosion(pub Timer);


impl Explosion {
    pub fn spawn(location_transform: &Transform, commands: &mut Commands, game_assets: &Res<GameAssets>) {
        let explosion_texture = &game_assets.explosion_texture;

        commands.spawn((
            Explosion(Timer::new(Duration::from_secs_f32(EXPLOSION_DURATION), TimerMode::Once)),
            SpriteBundle {
                texture: explosion_texture.clone(),
                ..default()
            },
        ));
    }
}

pub fn despawn_explosions(
    mut explosion_query: Query<(Entity, &mut Explosion)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut explosion_timer) in explosion_query.iter_mut() {
        explosion_timer
            .0
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        if explosion_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
