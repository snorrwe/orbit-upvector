use std::f32::consts::PI;

use bevy::{color::palettes::css::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .insert_resource(TestParams::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_example_collection, rotate_phi, rotate_camera))
        .run();
}

// We can create our own gizmo config group!
#[derive(Reflect, GizmoConfigGroup, Default, Clone, Copy)]
struct MyRoundGizmos {}

impl Default for TestParams {
    fn default() -> Self {
        Self {
            phi: 1.0 * PI,
            theta: PI / 2.0,
            radius: 2.0,
        }
    }
}

#[derive(Debug, Resource)]
struct TestParams {
    pub phi: f32,
    pub theta: f32,
    pub radius: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    params: Res<TestParams>,
) {
    let TestParams { phi, theta, radius } = *params;
    my_gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, radius, RED)
        .resolution(64);

    let (sphi, cphi) = phi.sin_cos();
    let (stheta, ctheta) = theta.sin_cos();

    let pos = Vec3::new(
        radius * stheta * cphi,
        radius * stheta * sphi,
        radius * ctheta,
    );

    let up = Vec3::new(
        -sphi * stheta.powi(2) * cphi,
        stheta.powi(2) * cphi.powi(2) + ctheta.powi(2),
        -sphi * stheta * ctheta,
    )
    .normalize();

    gizmos.line(pos, Vec3::ZERO, YELLOW);
    gizmos.arrow(pos, pos + up, BLUE);

    dbg!(pos.dot(up), up.length());
}

fn rotate_phi(mut p: ResMut<TestParams>, time: Res<Time>) {
    p.theta = p.theta + PI / 4.0 * time.delta_seconds();
    p.phi = p.phi + PI / 8.0 * time.delta_seconds();
}
