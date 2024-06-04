use super::*;
use rand::*;

use crate::constants::*;

#[derive(Component, PartialEq)]
pub enum Mushroom {
    Normal,
    Poison,
}

#[derive(Event)]
pub struct SpawnMushroomEvent(pub Transform);

pub fn spawn_shroom(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnMushroomEvent>,
    game_assets: Res<GameAssets>,
    mut shroom_amount: ResMut<ShroomAmount>,
) {
    for event in spawn_event.read() {
        let shroom_texture = &game_assets.shroom_texture;

        commands.spawn((
            Mushroom::Normal,
            Health(MUSHROOM_HEALTH),
            SpriteBundle {
                texture: shroom_texture.clone(),
                transform: event.0,
                ..default()
            },
            Name::from("Mushroom"),
        ));

        // Add to shroom count
        shroom_amount.0 += 1;
    }
}


#[derive(Resource)]
pub struct ShroomAmount(pub u8);

pub fn spawn_shroom_field(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut shroom_amount: ResMut<ShroomAmount>,
    mut spawn_mushroom: EventWriter<SpawnMushroomEvent>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..MUSHROOM_MAX_AMOUNT {
        let x = rand::thread_rng().gen_range(0.0 + SPAWN_MARGIN..window.width() - SPAWN_MARGIN);
        let y = rand::thread_rng().gen_range(TOP_BOUND..window.height() - TOP_UI_HEIGHT);
        spawn_mushroom.send(SpawnMushroomEvent(Transform::from_xyz(x,y,0.0)));
    }
}

pub fn despawn_mushroom(
    mut commands: Commands,
    shroom_q: Query<(Entity, &Health)>,
    mut shroom_amount: ResMut<ShroomAmount>,
) {
    for (shroom_entity, shroom_health) in shroom_q.iter() {
        // Skip those with health
        if shroom_health.0 > 0 {
            continue;
        }
        commands.entity(shroom_entity).despawn();
        shroom_amount.0 -= 1;
    }
}

pub fn despawn_shroom_field(mut commands: Commands, mushroom_query: Query<Entity, With<Mushroom>>) {
    for mushroom_entity in mushroom_query.iter() {
        commands.entity(mushroom_entity).despawn();
    }
}


pub fn update_shroom_color(
    mut shroom_q: Query<(&Mushroom, &mut Sprite, &mut Transform), With<Mushroom>>,
) {
    for (mushroom, mut mushroom_sprite, mut mushroom_transform) in shroom_q.iter_mut() {
        if *mushroom == Mushroom::Poison && mushroom_sprite.color == Color::rgb(1.0, 1.0, 1.0) {
            // Set the color
            mushroom_sprite.color = MUSHROOM_POISON_COLOR;
            mushroom_transform.translation.z = 0.5;
        }
    }
}
