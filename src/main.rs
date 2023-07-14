use std::sync::atomic::AtomicBool;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use vector_operations::move_towards;

pub mod vector_operations;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(
            DefaultPlugins
        )
        .add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default()
        )
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(
            WorldInspectorPlugin::default()
        )
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(update_car_suspension)
        .add_system(camera_follow)
        .add_system(car_controls.after(update_car_suspension))
        .add_system(check_assets_ready)
        .insert_resource(MapStatus{loaded : false})
        .init_resource::<AssetsLoading>()
        .run();
}
#[derive(Component)]
pub struct CarPhysics
{
    pub car_transform_camera : Transform
}
#[derive(Component)]
pub struct CameraFollow
{
    pub camera_translation_speed : f32,
    pub fake_transform : Transform,
    pub distance_behind : f32,
}
fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-90.0, 500.0, 90.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    }).insert(CameraFollow{camera_translation_speed:1000.,fake_transform : Transform::from_xyz(0.,0.,0.),distance_behind : 10.});
}
const CAR_SIZE : Vec3 = Vec3::new(0.5,0.3,0.7);
pub fn setup_physics(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands) 
{
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    let x_shape: Handle<Mesh> = asset_server.load("racetrack.glb#Mesh0/Primitive0");
    loading.0.push(x_shape.clone_untyped());
    
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance : 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        ..default()
    });


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
    .insert(CarPhysics{car_transform_camera : Transform::from_xyz(0.,0.,0.)})
    .insert(CarController{rotate_to_rotation:Quat::IDENTITY,slerp_speed : 5.,rotated_last_frame : false,center_of_mass_altered : false,speed: 20000.,rotate_speed : 1500.})
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
    .insert(Damping { linear_damping: 0.5, angular_damping: 3. })
    .insert(Ccd::enabled());
    
}

pub fn camera_follow(time:Res<Time>,mut car_query: Query<(&mut CarPhysics,&mut Transform),Without<CameraFollow>>,mut camera_query:Query<(&mut CameraFollow,&mut Transform),Without<CarPhysics>>)
{   
    if let Ok((mut camera_follow,mut camera_transform)) = camera_query.get_single_mut()
    {
        if let Ok((car_physics,mut car_transform)) = car_query.get_single_mut()
        {
            

            camera_follow.fake_transform.translation = car_transform.translation +(Vec3::new(car_physics.car_transform_camera.back().x,0.,car_physics.car_transform_camera.back().z)).normalize()*camera_follow.distance_behind;
            camera_follow.fake_transform.look_at(car_transform.translation,Vec3::Y);
            camera_follow.fake_transform.translation.y +=2.;
            
            camera_transform.look_at(car_transform.translation,Vec3::Y);
            camera_transform.translation = Vec3::lerp(camera_transform.translation, camera_follow.fake_transform.translation, camera_follow.camera_translation_speed*time.delta_seconds());
            
            //camera_transform.rotation = car_transform.rotation;
            //camera_transform.rotate_x(-10.*0.0174533);
        }
    }
    
}
#[derive(Resource,Default)]
pub struct AssetsLoading(Vec<HandleUntyped>);

fn check_assets_ready(
    mut map_status : ResMut<MapStatus>,
    commands: Commands,
    server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;
    static SETUP_PHYSICS_CALLED: AtomicBool = AtomicBool::new(false);
    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            if !SETUP_PHYSICS_CALLED.load(std::sync::atomic::Ordering::Relaxed) {
                setup_map(commands, map_status,server, meshes, materials);
                SETUP_PHYSICS_CALLED.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
#[derive(Resource)]
pub struct MapStatus
{
    pub loaded : bool,
}
fn setup_map(mut commands: Commands,
    mut map_status : ResMut<MapStatus>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,)
{
    let mesh_handle: Handle<Mesh> = asset_server.load("racetrack.glb#Mesh0/Primitive0");

    let m = meshes.get(&mesh_handle);
    let mut map_mesh = m.unwrap().clone();
    Mesh::generate_tangents(&mut map_mesh);

    
    let x_shape = Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).unwrap();
    if Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).is_none()
    {
        println!("{}","the mesh failed to load");
    }
    let texture_handle = asset_server.load("sand.png");
    let normal_handle = asset_server.load("normal_map.png");
    let wall_mat = materials.add(StandardMaterial {
        normal_map_texture : Some(normal_handle.clone()),
        base_color: Color::WHITE,
        perceptual_roughness :0.5,
        base_color_texture: Some(texture_handle.clone()),
        cull_mode: None,
        unlit: false,
        ..default()
    });

    //println!("{}",x_shape);
    commands.spawn((
        RigidBody::Fixed,
        PbrBundle {
            
            transform: Transform::from_xyz(0.,0., 0.).with_scale(Vec3::new(1.,1.,1.)),
            mesh : meshes.add(map_mesh), 
            material :  materials.add(Color::rgb(0.2, 0.5, 0.1).into()),
            //scene: asset_server.load("map.glb#Scene0"),
            ..default()
        },

    )).insert(x_shape);
    map_status.loaded = true;
}
pub fn neg_or_pos(rng : &mut ThreadRng) -> i32
{
    if rng.gen_range(0..2) == 1
    {
        return 1;
    }
    return -1;
}
#[derive(Component)]
pub struct CarController
{
    pub rotated_last_frame: bool,
    pub rotate_speed : f32,
    pub speed  : f32,
    pub center_of_mass_altered: bool,
    pub rotate_to_rotation : Quat,
    pub slerp_speed : f32,
}
pub fn car_controls(mut commands:Commands,time:Res<Time>,keys: Res<Input<KeyCode>>,mut car_query: Query<(Entity,&mut CarController,&mut CarPhysics,&mut ExternalForce,&mut ExternalImpulse,&mut Transform),Without<CameraFollow>>)
{
    if let Ok((entity,mut car_controller,mut car_physics,mut force,mut impulse,mut car_transform)) = car_query.get_single_mut()
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
        if car_controller.center_of_mass_altered == false
        {
            commands.entity(entity).insert(AdditionalMassProperties::MassProperties(MassProperties { local_center_of_mass: Vec3::new(0.,-10000.,0.),..default() }));

            car_controller.center_of_mass_altered = true;
        }
        if keys.pressed(KeyCode::W)
        {
            force.force += car_transform.forward()*car_controller.speed*time.delta_seconds();       
        }
        if keys.just_pressed(KeyCode::W)
        {
            force.torque+=car_transform.left()*300.;
        }
        if keys.pressed(KeyCode::S)
        {
            force.force -= car_transform.forward()*car_controller.speed*time.delta_seconds();
        }
        if keys.just_pressed(KeyCode::S)
        {
            force.torque-=car_transform.left()*300.;
        }

        car_controller.rotate_to_rotation =  car_transform.rotation;

        car_physics.car_transform_camera.rotation = Quat::slerp(car_physics.car_transform_camera.rotation,car_controller.rotate_to_rotation,car_controller.slerp_speed*time.delta_seconds());
        car_physics.car_transform_camera.translation = car_transform.translation;
        if keys.pressed(KeyCode::A)
        {
            force.torque+=car_transform.up()*time.delta_seconds()*car_controller.rotate_speed;
        }
        if keys.pressed(KeyCode::D)
        {
            force.torque-=car_transform.up()*time.delta_seconds()*car_controller.rotate_speed;
        }
    }
}
fn update_car_suspension(
    time : Res<Time>,
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut car_query: Query<(&mut CarPhysics,&mut ExternalForce,&mut Velocity,&mut Transform),Without<CameraFollow>>
) {
    if let Ok((
        car_physics,
        mut force,
        mut velocity,
        mut car_transform)) 
        = car_query.get_single_mut()
    {
        let f_r_d = car_transform.translation+
        (car_transform.down()*CAR_SIZE.y+car_transform.forward()*CAR_SIZE.z)+(car_transform.right()*CAR_SIZE.x);
        let f_l_d = car_transform.translation+
        (car_transform.down()*CAR_SIZE.y+car_transform.forward()*CAR_SIZE.z)+(car_transform.left()*CAR_SIZE.x);
        let b_r_d = car_transform.translation+
        (car_transform.down()*CAR_SIZE.y+car_transform.back()*CAR_SIZE.z)+(car_transform.right()*CAR_SIZE.x);
        let b_l_d = car_transform.translation+
        (car_transform.down()*CAR_SIZE.y+car_transform.back()*CAR_SIZE.z)+(car_transform.left()*CAR_SIZE.x);

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
                car_transform.up()*
                ((compression * suspension_strength) - (suspension_damping*(velocity.linvel.y)))
                *time.delta_seconds()
                ,wheel_vec[i]
                ,car_transform.translation);

                force.force += add_force.force;
                force.torque += add_force.torque;
            }
        }
    }
}