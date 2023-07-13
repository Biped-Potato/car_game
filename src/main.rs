use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins(
            DefaultPlugins
        )
        .add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(10.)
        )
        .add_plugin(
            RapierDebugRenderPlugin::default()
        )
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(update_car_suspension)
        .add_system(camera_follow)
        .add_system(car_controls)
        .run();
}
#[derive(Component)]
pub struct CarPhysics
{

}
#[derive(Component)]
pub struct CameraFollow
{
    pub distance_behind : f32,
}
fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 30.0, 100.0)
            .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
        ..Default::default()
    }).insert(CameraFollow{distance_behind : 100.});
}

pub fn setup_physics(mut commands: Commands) {

    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    let car_size = Vec3::new(5.,3.,7.);

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., 5., 0.)),
        RigidBody::Dynamic,
        Collider::cuboid(car_size.x, car_size.y, car_size.z),
    ))
    .insert(CarPhysics{})
    .insert(ExternalImpulse {
        impulse: Vec3::new(0., 0., 0.),
        torque_impulse: Vec3::new(0., 0., 0.),
    })
    .insert(GravityScale(10.));
}
pub fn camera_follow(mut car_query: Query<(&mut CarPhysics,&mut Transform),Without<CameraFollow>>,mut camera_query:Query<(&CameraFollow,&mut Transform),Without<CarPhysics>>)
{   
    if let Ok((camera_follow,mut camera_transform)) = camera_query.get_single_mut()
    {
        if let Ok((car_physics,mut car_transform)) = car_query.get_single_mut()
        {
            camera_transform.translation = car_transform.translation +(car_transform.back()+Vec3::new(0.,0.3,0.)).normalize()*camera_follow.distance_behind;
            camera_transform.look_at(car_transform.translation,Vec3::Y);
            camera_transform.translation.y +=13.;
            //camera_transform.rotation = car_transform.rotation;
            //camera_transform.rotate_x(-10.*0.0174533);
        }
    }
    
}
pub fn neg_or_pos(rng : &mut ThreadRng) -> i32
{
    if rng.gen_range(0..2) == 1
    {
        return 1;
    }
    return -1;
}
pub fn car_controls(keys: Res<Input<KeyCode>>,mut car_query: Query<(&mut CarPhysics,&mut ExternalImpulse,&mut Transform),Without<CameraFollow>>)
{
    if let Ok((car_physics,mut impulse,mut car_transform)) = car_query.get_single_mut()
    {
        if keys.just_pressed(KeyCode::Space)
        {
            let mut rng = rand::thread_rng();
            impulse.impulse = ExternalImpulse::at_point(Vec3::new(0.,0.01,0.),car_transform.translation,car_transform.translation).impulse;
            //impulse.impulse = Vec3::new(0.,0.01,0.);
            impulse.torque_impulse = Vec3::new(
                rng.gen_range(1.0..3.0)*neg_or_pos(&mut rng) as f32,
                rng.gen_range(1.0..3.0)*neg_or_pos(&mut rng) as f32,
                rng.gen_range(1.0..3.0)*neg_or_pos(&mut rng) as f32
            );
        }
        
    }
}
fn update_car_suspension(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mut car_query: Query<(&mut CarPhysics,&mut ExternalImpulse,&mut Transform),Without<CameraFollow>>
) {
    if let Ok((car_physics,mut impulse,mut car_transform)) = car_query.get_single_mut()
    {
        let window = windows.single();

        let Some(cursor_position) = window.cursor_position() else { return; };

        // We will color in read the colliders hovered by the mouse.
        for (camera, camera_transform) in &cameras {

            // Then cast the ray.
            let hit = rapier_context.cast_ray(
                ray.origin,
                ray.direction,
                f32::MAX,
                true,
                QueryFilter::only_dynamic(),
            );

            if let Some((entity, _toi)) = hit {
                // Color in blue the entity we just hit.
                // Because of the query filter, only colliders attached to a dynamic body
                // will get an event.
                let color = Color::BLUE;
                commands.entity(entity).insert(ColliderDebugColor(color));
            }
        }
    }
}