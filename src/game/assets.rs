use super::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub segment_texture: Handle<Image>,
    pub beetle_texture: Handle<Image>,
    pub projectile_texture: Handle<Image>,
    pub shroom_texture: Handle<Image>,
    pub shroom_layout: Handle<TextureAtlasLayout>,
    pub spider_texture: Handle<Image>,
    pub explosion_texture: Handle<Image>,
    pub scorpion_texture: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut texture_atlas_layouts = world.get_resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let shroom_layout = texture_atlas_layouts.unwrap().add(layout);
        GameAssets {
            player_texture: asset_server.load("textures/snake.png"),
            segment_texture: asset_server.load("textures/millipede.png"),
            beetle_texture: asset_server.load("textures/beetle.png"),
            projectile_texture: asset_server.load("textures/stinger.png"),
            shroom_texture: asset_server.load("textures/shroom.png"),
            shroom_layout,
            spider_texture: asset_server.load("textures/spider.png"),
            explosion_texture: asset_server.load("textures/explosion.png"),
            scorpion_texture: asset_server.load("textures/scorpion.png"),
        }
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

