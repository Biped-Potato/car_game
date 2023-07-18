use bevy::{prelude::*, window::PrimaryWindow};

use crate::timer_text::TimerText;

pub fn initialize_fps_text(_commands: Commands, _asset_server: Res<AssetServer>) {
}
pub fn initialize_dialogue(asset_server: Res<AssetServer>,
mut commands: Commands,
primary_query: Query<&Window, With<PrimaryWindow>>)
{
    let Ok(primary) = primary_query.get_single() else
    {
        return;
    };
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Percent(0.),
                        Val::Percent(100.),
                        Val::Px(0.),
                        Val::Px(30.),
                    ),
                    ..default()
                },
                background_color: Color::rgba(0.15, 0.15, 0.15, 1.).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: "Timer: ".to_string(),
                                style: TextStyle {
                                    font:asset_server.load("lato.regular.ttf"),
                                    font_size: 30.0,
                                    color: Color::Rgba {
                                        red: 0.,
                                        green: 0.7215686275,
                                        blue: 0.,
                                        alpha: 1.0,
                                    },
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font:asset_server.load("lato.regular.ttf"),
                                    font_size: 30.0,
                                    color: Color::Rgba {
                                        red: 0.,
                                        green: 0.7215686275,
                                        blue: 0.,
                                        alpha: 1.0,
                                    },
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(TimerText{value : 0.});
                
            });
    });
}