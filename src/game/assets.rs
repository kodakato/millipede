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

        // Load the textures
        let player_texture = asset_server.load("textures/snake.png");
        let segment_texture = asset_server.load("textures/millipede.png");
        let beetle_texture = asset_server.load("textures/beetle.png");
        let projectile_texture = asset_server.load("textures/stinger.png");
        let shroom_texture = asset_server.load("textures/shrooms.png");
        let spider_texture = asset_server.load("textures/spider.png");
        let explosion_texture = asset_server.load("textures/explosion.png");
        let scorpion_texture = asset_server.load("textures/scorpion.png");

        // Define the layout
        let mut texture_atlas_layouts = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let shroom_layout = texture_atlas_layouts.add(layout);

        // Construct the GameAssets instance
        GameAssets {
            player_texture,
            segment_texture,
            beetle_texture,
            projectile_texture,
            shroom_texture,
            shroom_layout,
            spider_texture,
            explosion_texture,
            scorpion_texture,
        }
    }
}


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

