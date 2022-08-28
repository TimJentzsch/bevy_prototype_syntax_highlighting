use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy::winit::WinitSettings;

fn main() {
    App::new()
        // Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Fonts
    let font_regular: Handle<Font> = asset_server.load("fonts/fira_mono/FiraMono-Regular.ttf");
    let font_size = 20.0;

    // Colors
    let background_color = Color::hex("121212").unwrap().into();

    // Root with background color
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            color: background_color,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Text Example",
                    TextStyle {
                        font: font_regular.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    // TODO: Find a better way to handle height
                    size: Size::new(Val::Percent(100.0), Val::Px(font_size)),
                    ..default()
                }),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    "Text2 Example2",
                    TextStyle {
                        font: font_regular.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(font_size)),
                    ..default()
                }),
            );
        });
}
