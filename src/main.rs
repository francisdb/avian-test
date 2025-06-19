use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct ParentCube;

fn main() {
    App::new()
        // Enable physics
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, reset_parent_cube_on_r)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        //AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
        ParentCube, // Add marker
    ))
    .with_children(|commands| {
        // add a double sized cube on each side
        commands.spawn((
            Collider::cuboid(2.0, 2.0, 2.0),
            Mesh3d(meshes.add(Cuboid::from_length(2.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 124, 144))),
            Transform::from_xyz(1.5, 0.0, 0.0),
        ));
        commands.spawn((
            Collider::cuboid(2.0, 2.0, 2.0),
            Mesh3d(meshes.add(Cuboid::from_length(2.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(144, 255, 124))),
            Transform::from_xyz(-1.5, 0.0, 0.0),
        ));
    });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn reset_parent_cube_on_r(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, Option<&mut LinearVelocity>, Option<&mut AngularVelocity>), With<ParentCube>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (mut transform, velocity, angular_velocity) in query.iter_mut() {
            // Reset position
            *transform = Transform::from_xyz(0.0, 4.0, 0.0);
            // Reset linear velocity if present
            if let Some(mut v) = velocity {
                v.0 = Vec3::ZERO;
            }
            // Reset angular velocity if present
            if let Some(mut av) = angular_velocity {
                av.0 = Vec3::ZERO;
            }
        }
    }
}
