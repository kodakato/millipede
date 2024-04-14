use super::*;


#[derive(Component)]
pub struct Spider;

impl Spider {
    pub fn spawn(
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        window_query: &Query<&Window, With<PrimaryWindow>>, 
    ) {
        let window = window_query.get_single().unwrap();
        let spider_texture = asset_server.load("spider.png");
        commands.spawn((
            SpriteBundle {
                texture: spider_texture,
                transform: Transform::from_xyz(window.width() - 100.0, 100.0, 0.0),
                ..default()
            },
            Spider,
        ));
    }

}

pub fn spawn_spider(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    Spider::spawn(&mut commands, &asset_server, &window_query)
}

