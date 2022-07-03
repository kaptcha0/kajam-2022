use crate::assets::GameAssets;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePepperCountEvent>()
            .add_event::<UpdateHealthPointsEvent>()
            .add_startup_system(create_hud)
            .add_system(update_pepper_count)
            .add_system(update_health_points);
    }
}

pub struct UpdatePepperCountEvent(pub u32);
pub struct UpdateHealthPointsEvent(pub f32);

#[derive(Component)]
struct HudCamera;

#[derive(Component)]
struct PepperCount;

#[derive(Component)]
struct HealthPoints;

fn create_hud(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Relative,
                margin: Rect::all(Val::Px(10.0)),
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
        .insert(PepperCount);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Relative,
                margin: Rect::all(Val::Px(10.0)),
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Health: ".to_string(),
                        style: TextStyle {
                            font: assets.fonts.medium.clone_weak(),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "500".to_string(),
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
        .insert(HealthPoints);
}

fn update_pepper_count(
    mut events: EventReader<UpdatePepperCountEvent>,
    mut query: Query<&mut Text, With<PepperCount>>,
) {
    for mut text in query.iter_mut() {
        for ev in events.iter() {
            text.sections[1].value = format!("{}", ev.0);
        }
    }
}

fn update_health_points(
    mut events: EventReader<UpdateHealthPointsEvent>,
    mut query: Query<&mut Text, With<HealthPoints>>,
) {
    for mut text in query.iter_mut() {
        for ev in events.iter() {
            text.sections[1].value = format!("{:.0}", ev.0);
        }
    }
}
