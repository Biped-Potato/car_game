use bevy::prelude::*;

use crate::car_suspension::CarPhysics;
#[derive(Component)]
pub struct CameraFollow {
    pub camera_translation_speed: f32,
    pub fake_transform: Transform,
    pub distance_behind: f32,
}
pub fn camera_follow(
    time: Res<Time>,
    mut car_query: Query<(&mut CarPhysics, &mut Transform), Without<CameraFollow>>,
    mut camera_query: Query<(&mut CameraFollow, &mut Transform), Without<CarPhysics>>,
) {
    if let Ok((mut camera_follow, mut camera_transform)) = camera_query.get_single_mut() {
        if let Ok((car_physics, car_transform)) = car_query.get_single_mut() {
            camera_follow.fake_transform.translation = car_transform.translation
                + (Vec3::new(
                    car_physics.car_transform_camera.back().x,
                    0.,
                    car_physics.car_transform_camera.back().z,
                ))
                .normalize()
                    * camera_follow.distance_behind;
            camera_follow
                .fake_transform
                .look_at(car_transform.translation, Vec3::Y);
            camera_follow.fake_transform.translation.y += 2.;

            camera_transform.look_at(car_transform.translation, Vec3::Y);
            camera_transform.translation = Vec3::lerp(
                camera_transform.translation,
                camera_follow.fake_transform.translation,
                camera_follow.camera_translation_speed * time.delta_seconds(),
            );

            //camera_transform.rotation = car_transform.rotation;
            //camera_transform.rotate_x(-10.*0.0174533);
        }
    }
}