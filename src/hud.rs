use crate::assets::GameAssets;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePepperCountEvent>()
            .add_startup_system(create_hud)
            .add_system(update_pepper_count);
    }
}

pub struct UpdatePepperCountEvent {
    pub new_value: u32,
}

#[derive(Component)]
pub struct HudCamera;

#[derive(Component)]
pub struct PepperCountText;

fn create_hud(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                padding: Rect::all(Val::Percent(10.0)),
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Peppers: ".to_string(),
                        style: TextStyle {
                            font: assets.fonts.medium.clone_weak(),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: assets.fonts.medium.clone_weak(),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(PepperCountText);
}

fn update_pepper_count(
    mut events: EventReader<UpdatePepperCountEvent>,
    mut query: Query<&mut Text, With<PepperCountText>>,
) {
    for mut text in query.iter_mut() {
        for ev in events.iter() {
            text.sections[1].value = format!("{}", ev.new_value);
        }
    }
}
