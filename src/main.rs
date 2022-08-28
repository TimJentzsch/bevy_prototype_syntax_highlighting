use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color as SyntectColor, Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

const CODE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/code.rs"));

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

/// Apply syntax highlighting to the code
fn highlight_code(code: &str) -> Vec<Vec<(SyntectStyle, &str)>> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let rust = syntax_set.find_syntax_by_extension("rs").unwrap();

    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];

    let mut regions = Vec::new();
    let mut highlighter = HighlightLines::new(rust, theme);

    for line in LinesWithEndings::from(code) {
        let mut line_regions = Vec::new();

        for region in highlighter.highlight_line(line, &syntax_set).unwrap() {
            line_regions.push(region);
        }

        regions.push(line_regions);
    }

    regions
}

fn syntect_color_to_bevy_color(color: SyntectColor) -> Color {
    Color::rgba_u8(color.r, color.g, color.b, color.a)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Fonts
    let font_regular: Handle<Font> = asset_server.load("fonts/fira_mono/FiraMono-Regular.ttf");
    let font_size = 20.0;

    // Colors
    let background_color = Color::hex("121212").unwrap().into();

    // The highlighted lines of a Bevy UI code file
    // The first 3 lines contain licensing information, we can skip them for the UI
    let highlighted_lines = highlight_code(CODE);
    let highlighted_lines = highlighted_lines.iter().skip(3);

    // Root with background color
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                overflow: Overflow::Hidden,
                ..default()
            },
            color: background_color,
            ..default()
        })
        .with_children(|parent| {
            for line in highlighted_lines {
                let mut sections = Vec::new();

                // Convert the highlighted lines into text sections
                for (style, text) in line {
                    sections.push(TextSection::new(
                        *text,
                        TextStyle {
                            color: syntect_color_to_bevy_color(style.foreground),
                            font: font_regular.clone(),
                            font_size,
                        },
                    ))
                }

                // Wrapper for each line
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            // TODO: Find a better way to handle height
                            // For some reason, each line takes up a lot of space otherwise
                            size: Size::new(Val::Percent(100.0), Val::Px(font_size * 1.2)),
                            // If we don't specify this, the lines overlap in a weird way once there are too many
                            flex_shrink: 0.0,
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // The actual line text
                        parent.spawn_bundle(TextBundle::from_sections(sections));
                    });
            }
        });
}
