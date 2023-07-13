use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

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
            RapierPhysicsPlugin::<NoUserData>::default()
        )
        .add_plugin(
            RapierDebugRenderPlugin::default()
        )
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(update_car_suspension)
        .add_system(camera_follow)
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
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    /*
     * Create the cubes
     */
    let car_size = Vec3::new(5.,3.,7.);

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., 5., 0.)),
        RigidBody::Dynamic,
        Collider::cuboid(car_size.x, car_size.y, car_size.z),
    )).insert(CarPhysics{});
}
pub fn camera_follow(mut car_query: Query<(&mut CarPhysics,&mut Transform),Without<CameraFollow>>,mut camera_query:Query<(&CameraFollow,&mut Transform),Without<CarPhysics>>)
{   
    if let Ok((camera_follow,mut camera_transform)) = camera_query.get_single_mut()
    {
        if let Ok((car_physics,mut car_transform)) = car_query.get_single_mut()
        {
            camera_transform.translation = car_transform.translation +(car_transform.back()+Vec3::new(0.,0.3,0.)).normalize()*camera_follow.distance_behind;
            camera_transform.rotation = car_transform.rotation;
            camera_transform.rotate_x(-10.*0.0174533);
        }
    }
    
}
fn update_car_suspension(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

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