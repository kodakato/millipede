use super::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub segment_texture: Handle<Image>,
    pub segment_layout: Handle<TextureAtlasLayout>,
    pub beetle_texture: Handle<Image>,
    pub projectile_texture: Handle<Image>,
    pub shroom_texture: Handle<Image>,
    pub shroom_layout: Handle<TextureAtlasLayout>,
    pub spider_texture: Handle<Image>,
    pub spider_layout: Handle<TextureAtlasLayout>,
    pub explosion_texture: Handle<Image>,
    pub explosion_layout: Handle<TextureAtlasLayout>,
    pub scorpion_texture: Handle<Image>,
    pub scorpion_layout: Handle<TextureAtlasLayout>,
    pub font: Handle<Font>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        // Load the textures
        let player_texture = asset_server.load("textures/snake.png");
        let segment_texture = asset_server.load("textures/segments.png");
        let beetle_texture = asset_server.load("textures/beetle.png");
        let projectile_texture = asset_server.load("textures/stinger.png");
        let shroom_texture = asset_server.load("textures/shrooms.png");
        let spider_texture = asset_server.load("textures/spider.png");
        let explosion_texture = asset_server.load("textures/explosions.png");
        let scorpion_texture = asset_server.load("textures/scorpions.png");
        let font = asset_server.load("fonts/font.ttf");

        // Define the layout
        let mut texture_atlas_layouts = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let shroom_layout = texture_atlas_layouts.add(layout);

        // Explosion
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 3, 1, None, None);
        let explosion_layout = texture_atlas_layouts.add(layout);

        // Spider
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 3, 1, None, None);
        let spider_layout = texture_atlas_layouts.add(layout);

        // Segments
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 3, 1, None, None);
        let segment_layout = texture_atlas_layouts.add(layout);

        // Scorpion
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 2, 1, None, None);
        let scorpion_layout = texture_atlas_layouts.add(layout);

        // Construct the GameAssets instance
        GameAssets {
            player_texture,
            segment_texture,
            segment_layout,
            beetle_texture,
            projectile_texture,
            shroom_texture,
            shroom_layout,
            spider_texture,
            spider_layout,
            explosion_texture,
            explosion_layout,
            scorpion_texture,
            scorpion_layout,
            font,
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Animation {
    pub frames: usize,
    pub current_frame: usize,
    pub timer: Timer,
}

impl Animation {
    pub fn new(frames: usize, frame_duration: f32) -> Self {
        Animation {
            frames,
            current_frame: 0,
            timer: Timer::from_seconds(frame_duration, TimerMode::Once),
        }
    }
}
