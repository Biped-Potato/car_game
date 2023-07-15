use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{car_suspension::CarPhysics, car_camera::CameraFollow};
#[derive(Component)]
pub struct CarController {
    pub rotated_last_frame: bool,
    pub rotate_speed: f32,
    pub speed: f32,
    pub center_of_mass_altered: bool,
    pub rotate_to_rotation: Quat,
    pub slerp_speed: f32,
    pub car_linear_damping : f32,
}
pub fn car_controls(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut car_query: Query<
        (
            Entity,
            &mut Damping,
            &mut CarController,
            &mut CarPhysics,
            &mut ExternalForce,
            &mut ExternalImpulse,
            &mut Transform,
        ),
    >,
    mut transform_query: Query<&mut Transform,Without<CarPhysics>>
) {
    if let Ok((
        entity,
        mut damping,
        mut car_controller,
        mut car_physics,
        mut force,
        mut impulse,
        car_transform,
    )) = car_query.get_single_mut()
    {
        if keys.just_pressed(KeyCode::Space) {
            let _rng = rand::thread_rng();
            let new_impluse = ExternalImpulse::at_point(
                Vec3::new(0., 2., 0.),
                car_transform.translation,
                car_transform.translation,
            );
            impulse.impulse = new_impluse.impulse;
            //impulse.impulse = Vec3::new(0.,0.01,0.);
            impulse.torque_impulse = Vec3::new(1., 0., 0.);
        }
        if car_controller.center_of_mass_altered == false {
            commands
                .entity(entity)
                .insert(AdditionalMassProperties::MassProperties(MassProperties {
                    mass : 1.,
                    local_center_of_mass: Vec3::new(0., -0.5, 0.),
                    ..default()
                }));

            car_controller.center_of_mass_altered = true;
        }
        let mut num_on_ground = 0;
        for i in 0..car_physics.wheel_infos.len()
        {
            if car_physics.wheel_infos[i].hit == true
            {
                num_on_ground+=1;
                
            }
        }
        if num_on_ground>1 
        {
            damping.linear_damping = car_controller.car_linear_damping;
            if keys.pressed(KeyCode::W) {
                force.force += car_transform.forward() * car_controller.speed * time.delta_seconds();
            }
            if keys.just_pressed(KeyCode::W) {
                force.torque += car_transform.left() * 300.;
            }
            if keys.pressed(KeyCode::S) {
                force.force -= car_transform.forward() * car_controller.speed * time.delta_seconds();
            }
            if keys.just_pressed(KeyCode::S) {
                force.torque -= car_transform.left() * 300.;
            }
            car_controller.rotate_to_rotation = car_transform.rotation;

            car_physics.car_transform_camera.rotation = Quat::slerp(
                car_physics.car_transform_camera.rotation,
                car_controller.rotate_to_rotation,
                car_controller.slerp_speed * time.delta_seconds(),
            );
            if keys.pressed(KeyCode::A) {
                force.torque += car_transform.up() * time.delta_seconds() * car_controller.rotate_speed;
            }
            if keys.pressed(KeyCode::D) {
                force.torque -= car_transform.up() * time.delta_seconds() * car_controller.rotate_speed;
            }
        }
        else {
            damping.linear_damping = 0.;
        }
        let mut one_pressed = false;
        if keys.pressed(KeyCode::A) {
            one_pressed = true;
            let mut fake_transform = car_transform.clone();
            fake_transform.rotate_y(1.);
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[0].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,fake_transform.rotation,car_physics.wheels_animation_speed*time.delta_seconds());
            }
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[1].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,fake_transform.rotation,car_physics.wheels_animation_speed*time.delta_seconds());
            }
        }
        else if keys.pressed(KeyCode::D)
        {
            let mut fake_transform = car_transform.clone();
            fake_transform.rotate_y(-1.);
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[0].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,fake_transform.rotation,car_physics.wheels_animation_speed*time.delta_seconds());
            }
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[1].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,fake_transform.rotation,car_physics.wheels_animation_speed*time.delta_seconds());
            }
        }
        else {
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[0].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,car_transform.rotation,car_physics.wheels_stationary_animation_speed*time.delta_seconds());
            }
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[1].entity)
            {
                wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,car_transform.rotation,car_physics.wheels_stationary_animation_speed*time.delta_seconds());
            }
        }
        car_physics.car_transform_camera.translation = car_transform.translation;
        
    }
}
