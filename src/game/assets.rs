use super::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub segment_texture: Handle<Image>,
    pub beetle_texture: Handle<Image>,
    pub projectile_texture: Handle<Image>,
    pub shroom_texture: Handle<Image>,
    pub spider_texture: Handle<Image>,
    pub explosion_texture: Handle<Image>,
    pub scorpion_texture: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        GameAssets {
            player_texture: asset_server.load("textures/snake.png"),
            segment_texture: asset_server.load("textures/millipede.png"),
            beetle_texture: asset_server.load("textures/beetle.png"),
            projectile_texture: asset_server.load("textures/stinger.png"),
            shroom_texture: asset_server.load("textures/shroom.png"),
            spider_texture: asset_server.load("textures/spider.png"),
            explosion_texture: asset_server.load("textures/explosion.png"),
            scorpion_texture: asset_server.load("textures/scorpion.png"),
        }
    }
}

pub fn setup_sprite_animation(
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game_assets: Res<GameAssets>,
) {
    let shroom_texture = game_assets.shroom_texture.clone();
}
