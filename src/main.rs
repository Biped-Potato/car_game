use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
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
        transform: Transform::from_xyz(-9.0, 9.0, 9.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    }).insert(CameraFollow{distance_behind : 10.});
}
const CAR_SIZE : Vec3 = Vec3::new(0.5,0.3,0.7);
pub fn setup_physics(mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>,
                    mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(0.,10.,0.),
            point_light: PointLight {
                intensity: 1000.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::WHITE,
                radius : 1000000000.,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        RigidBody::Fixed,
        PbrBundle {
            transform: Transform::from_xyz(0.0, -ground_height, 0.0).with_scale(Vec3::new(ground_size, ground_height, ground_size)),
            mesh: meshes.add(Mesh::from(shape::Cube {
                ..default()
            })),

            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        },
        Collider::cuboid(1.,1.,1.),
    ));

    let car_size = CAR_SIZE;

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0., 1., 0.).with_scale(car_size*2.),
            mesh: meshes.add(Mesh::from(shape::Cube {
                ..default()
            })),

            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        },
        //TransformBundle::from(Transform::from_xyz(0., 5., 0.)),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ))
    .insert(CarPhysics{})
    .insert(Velocity {
        ..default()
    })
    .insert(ExternalImpulse {
        impulse: Vec3::new(0., 0., 0.),
        torque_impulse: Vec3::new(0., 0., 0.),
    })
    .insert(ExternalForce{
        force : Vec3::new(0.,0.,0.),
        torque : Vec3::new(0.,0.,0.),
    })
    .insert(GravityScale(1.))
    .insert(Damping { linear_damping: 1., angular_damping: 3. });
}

pub fn camera_follow(mut car_query: Query<(&mut CarPhysics,&mut Transform),Without<CameraFollow>>,mut camera_query:Query<(&CameraFollow,&mut Transform),Without<CarPhysics>>)
{   
    if let Ok((camera_follow,mut camera_transform)) = camera_query.get_single_mut()
    {
        if let Ok((car_physics,mut car_transform)) = car_query.get_single_mut()
        {
            /*
            let mut ref_transform = car_transform.clone();
            let mut euler = ref_transform.rotation.to_euler(EulerRot::XYZ);
            euler.0 = 0.;
            euler.2 = 0.;
            ref_transform.rotation = Quat::from_euler(EulerRot::XYZ,euler.0,euler.1,euler.2);

            camera_transform.translation = car_transform.translation +(ref_transform.back()).normalize()*camera_follow.distance_behind;
            camera_transform.look_at(car_transform.translation,Vec3::Y);
            camera_transform.translation.y +=20.;
            
            
            */
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
            let new_impluse =  ExternalImpulse::at_point(Vec3::new(0.,2.,0.),car_transform.translation,car_transform.translation);
            impulse.impulse =new_impluse.impulse;
            //impulse.impulse = Vec3::new(0.,0.01,0.);
            impulse.torque_impulse = Vec3::new(
                1.,
                0.,
                0.
            );
        }
        
    }
}
fn update_car_suspension(
    time : Res<Time>,
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut car_query: Query<(&mut CarPhysics,&mut ExternalForce,&mut Velocity,&mut Transform),Without<CameraFollow>>
) {
    if let Ok((car_physics,mut force,mut velocity,mut car_transform)) = car_query.get_single_mut()
    {
        let f_r_d = car_transform.translation+(car_transform.down()*CAR_SIZE.y+car_transform.forward()*CAR_SIZE.z)+(car_transform.right()*CAR_SIZE.x);
        let f_l_d = car_transform.translation+(car_transform.down()*CAR_SIZE.y+car_transform.forward()*CAR_SIZE.z)+(car_transform.left()*CAR_SIZE.x);
        let b_r_d = car_transform.translation+(car_transform.down()*CAR_SIZE.y+car_transform.back()*CAR_SIZE.z)+(car_transform.right()*CAR_SIZE.x);
        let b_l_d = car_transform.translation+(car_transform.down()*CAR_SIZE.y+car_transform.back()*CAR_SIZE.z)+(car_transform.left()*CAR_SIZE.x);

        let mut wheel_vec : Vec<Vec3> = Vec::new();
        wheel_vec.push(f_r_d);
        wheel_vec.push(f_l_d);
        wheel_vec.push(b_r_d);
        wheel_vec.push(b_l_d);
        
        let max_suspension = 0.4;
        force.force = Vec3::ZERO;
        force.torque = Vec3::ZERO;
        for i in 0..wheel_vec.len()
        {
            let hit = rapier_context.cast_ray(
                wheel_vec[i],
                car_transform.down(),
                max_suspension,
                true,
                QueryFilter::only_fixed(),
            );
            
            if let Some((entity, toi)) = hit {
                let compression = 1.-(toi*car_transform.down().length()/max_suspension);
                let suspension_strength = 5000.;
                let suspension_damping = 800.;

                let add_force = ExternalForce::at_point(
                car_transform.up()*((compression * suspension_strength) - (suspension_damping*(velocity.linvel.y)))*time.delta_seconds()
                ,wheel_vec[i]
                ,car_transform.translation);

                force.force += add_force.force;
                force.torque += add_force.torque;
            }
        }
    }
}