use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::car_camera::CameraFollow;
#[derive(Clone)]
pub struct WheelInfo
{
    pub hit : bool,
    pub entity : Entity,
}
#[derive(Component)]
pub struct CarPhysics {
    pub plane : Vec3,
    pub car_size : Vec3,
    pub wheel_infos : Vec<WheelInfo>,
    pub car_transform_camera: Transform,
    pub wheels_animation_speed : f32,
    pub wheels_stationary_animation_speed : f32,
}
pub fn update_car_suspension(
    time: Res<Time>,
    _commands: Commands,
    rapier_context: Res<RapierContext>,
    mut car_query: Query<
        (
            &mut CarPhysics,
            &mut ExternalForce,
            &mut Velocity,
            &mut Transform,
        ),
    >,
    mut transform_query: Query<&mut Transform,Without<CarPhysics>>
) {
    if let Ok((mut car_physics, mut force, velocity, car_transform)) = car_query.get_single_mut() {
        
        let f_r_d = car_transform.translation
            + (car_transform.down() * car_physics.car_size.y + car_transform.forward() * car_physics.car_size.z)
            + (car_transform.right() * car_physics.car_size.x);
        let f_l_d = car_transform.translation
            + (car_transform.down() * car_physics.car_size.y + car_transform.forward() * car_physics.car_size.z)
            + (car_transform.left() * car_physics.car_size.x);
        let b_r_d = car_transform.translation
            + (car_transform.down() * car_physics.car_size.y + car_transform.back() * car_physics.car_size.z)
            + (car_transform.right() * car_physics.car_size.x);
        let b_l_d = car_transform.translation
            + (car_transform.down() * car_physics.car_size.y + car_transform.back() * car_physics.car_size.z)
            + (car_transform.left() * car_physics.car_size.x);

        let mut wheel_vec: Vec<Vec3> = Vec::new();
        wheel_vec.push(f_r_d);
        wheel_vec.push(f_l_d);
        wheel_vec.push(b_r_d);
        wheel_vec.push(b_l_d);

        let max_suspension = 0.2;
        force.force = Vec3::ZERO;
        force.torque = Vec3::ZERO;
        for i in 0..wheel_vec.len() {
            if let Ok(mut wheel_transform) = transform_query.get_mut(car_physics.wheel_infos[i].entity)
            {
                let hit = rapier_context.cast_ray_and_get_normal(
                    wheel_vec[i]+car_transform.up()*0.01,
                    car_transform.down(),
                    max_suspension,
                    true,
                    QueryFilter::only_fixed(),
                );
                if let Some((_entity, ray_intersection)) = hit {
                    car_physics.wheel_infos[i].hit = true;
                    let compression = 1. - (ray_intersection.toi * car_transform.down().length() / max_suspension);
                    let suspension_strength = 15000.;
                    let suspension_damping = 1200.;

                    let add_force = ExternalForce::at_point(
                        car_transform.up()
                            * ((compression * suspension_strength)
                                - (suspension_damping * (velocity.linvel.y)))
                            * time.delta_seconds(),
                        wheel_vec[i],
                        car_transform.translation,
                    );

                    force.force += add_force.force;
                    force.torque += add_force.torque;

                    wheel_transform.translation = ray_intersection.point+car_transform.up()*0.2;
                    if i == 2 || i == 3
                    {
                        wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,car_transform.rotation,car_physics.wheels_stationary_animation_speed*time.delta_seconds());
                    }
                }
                else
                {
                    car_physics.wheel_infos[i].hit = false;
                    
                    wheel_transform.translation = wheel_vec[i]-car_transform.up()*(max_suspension-0.2);
                    if i == 2 || i == 3
                    {
                        wheel_transform.rotation = Quat::slerp(wheel_transform.rotation,car_transform.rotation,car_physics.wheels_stationary_animation_speed*time.delta_seconds());
                    }
                   
                    
                }
            }
        }
    }
}
