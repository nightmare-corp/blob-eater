use bevy::prelude::*;
//TODO font
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct LevelText;

fn setup(mut commands: Commands /* , asset_server: Res<AssetServer> */) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                    ..default()
                }
            } else {
                // "default_font" feature is unavailable, load a font to use instead.
                TextStyle {
                    // font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            }),
        ]),
        LevelText,
    ));

    #[cfg(feature = "default_font")]
    commands.spawn(
        // Here we are able to call the `From` method instead of creating a new `TextSection`.
        // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
        TextBundle::from("From an &str into a TextBundle with the default font!").with_style(
            Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
        ),
    );

    #[cfg(not(feature = "default_font"))]
    commands.spawn(
        TextBundle::from_section(
            "Default font disabled",
            TextStyle {
                // font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
    );
}

// fn text_update_system(
//     // diagnostics: Res<DiagnosticsStore>,
//     mut query: Query<&mut Text, With<LevelText>>,
// ) {
//     let x = 100;
//     for mut text in &mut query {
//         text.sections[1].value = format!("{x:.2}");
//     }
// }
