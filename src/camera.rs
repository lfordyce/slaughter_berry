use crate::player::Player;
use crate::states::Level;
use bevy::prelude::{
    Entity, EventWriter, OrthographicProjection, Query, Transform, Vec2, With, Without,
};
use bevy_parallax::ParallaxMoveEvent;

const ASPECT_RATIO: f32 = 16. / 9.;

struct LevelDims {
    /// Height of the level in pixels
    pub px_hei: i32,

    /// Width of the level in pixels
    pub px_wid: i32,
}

const THRESHOLD: f32 = 0.;
#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            Entity,
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &Level), (Without<OrthographicProjection>, Without<Player>)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (camera, mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, _level_iid) in &level_query {
            let level = LevelDims {
                px_wid: 2416,
                px_hei: 288,
            };

            let level_ratio = level.px_wid as f32 / level.px_hei as f32;
            orthographic_projection.viewport_origin = Vec2::ZERO;
            if level_ratio > ASPECT_RATIO {
                // level is wider than the screen
                let height = (level.px_hei as f32 / 9.).round() * 9.;
                let width = height * ASPECT_RATIO;
                orthographic_projection.scaling_mode =
                    bevy::render::camera::ScalingMode::Fixed { width, height };

                camera_transform.translation.y = -level.px_hei as f32 / 2.;

                // camera_transform.translation.x =
                //     (player_translation.x - level_transform.translation.x - width / 2.)
                //         .clamp(-level.px_wid as f32 / 2., level.px_wid as f32 / 2. - width);

                let delta = player_translation.x - camera_transform.translation.x - (width / 2.);
                if delta.abs() > THRESHOLD {
                    let mut move_by = if delta > 0. {
                        delta - THRESHOLD
                    } else {
                        delta + THRESHOLD
                    };
                    if camera_transform.translation.x + move_by < -level.px_wid as f32 / 2. {
                        // info!("far left condition met");
                        move_by = (-level.px_wid as f32 / 2.) - camera_transform.translation.x;
                    }
                    if camera_transform.translation.x + move_by > level.px_wid as f32 / 2. - width {
                        // info!("far right condition met");
                        move_by =
                            (level.px_wid as f32 / 2.) - camera_transform.translation.x - width;
                    }
                    move_event_writer.send(ParallaxMoveEvent {
                        camera_move_speed: Vec2::new(move_by, 0.),
                        camera,
                    });
                }
            } else {
                // level is taller than the screen
                let width = (level.px_wid as f32 / 16.).round() * 16.;
                let height = width / ASPECT_RATIO;
                orthographic_projection.scaling_mode =
                    bevy::render::camera::ScalingMode::Fixed { width, height };
                camera_transform.translation.y =
                    (player_translation.y - level_transform.translation.y - height / 2.)
                        .clamp(0., level.px_hei as f32 - height);
                camera_transform.translation.x = 0.;
            }

            // camera_transform.translation.x += level_transform.translation.x;
            // camera_transform.translation.y += level_transform.translation.y;
        }
    }
}
