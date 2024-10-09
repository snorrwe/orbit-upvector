use std::f32::consts::PI;

use bevy::{color::palettes::css::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .insert_resource(TestParams::default())
        .insert_resource(Cartesian::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                draw_example_collection,
                rotate_phi,
                rotate_camera,
                update_text,
                update_cartesian_vectors,
            ),
        )
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

#[derive(Debug, Resource, Default)]
struct Cartesian {
    pub pos: Vec3,
    pub up: Vec3,
}

#[derive(Component)]
struct VectorText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let style = TextStyle {
        font_size: 32.0,
        ..Default::default()
    };
    commands
        .spawn(
            // Here we are able to call the `From` method instead of creating a new `TextSection`.
            // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
            TextBundle::from_sections([
                TextSection::new("Polar Pos: ", style.clone()),
                TextSection::new("-----", style.clone()),
                TextSection::new("\nCart. Pos: ", style.clone()),
                TextSection::new("-----", style.clone()),
                TextSection::new("\nUp: ", style.clone()),
                TextSection::new("-----", style.clone()),
                TextSection::new("\nUp dot Pos: ", style.clone()),
                TextSection::new("-----", style.clone()),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            }),
        )
        .insert(VectorText);
}

fn update_text(
    mut query: Query<&mut Text, With<VectorText>>,
    params: Res<TestParams>,
    vec: Res<Cartesian>,
) {
    for mut t in query.iter_mut() {
        t.sections[1].value = format!(
            "theta={:.2} phi={:.2} r={:.2}",
            params.theta, params.phi, params.radius
        );
        t.sections[3].value = format!("{:.2}", vec.pos);
        t.sections[5].value = format!("{:.2}", vec.up);
        t.sections[7].value = format!("{:.2}", vec.up.dot(vec.pos).abs());
    }
}

fn update_cartesian_vectors(mut cart: ResMut<Cartesian>, params: Res<TestParams>) {
    let TestParams { phi, theta, radius } = *params;
    let (sphi, cphi) = phi.sin_cos();
    let (stheta, ctheta) = theta.sin_cos();
    cart.pos = Vec3::new(
        radius * stheta * cphi,
        radius * stheta * sphi,
        radius * ctheta,
    );

    cart.up = Vec3::new(
        -sphi * stheta.powi(2) * cphi,
        stheta.powi(2) * cphi.powi(2) + ctheta.powi(2),
        -sphi * stheta * ctheta,
    )
    .normalize();
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    cart: Res<Cartesian>,
    params: Res<TestParams>,
) {
    my_gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, params.radius, RED)
        .resolution(64);

    let Cartesian { pos, up } = *cart;

    gizmos.line(pos, Vec3::ZERO, YELLOW);
    gizmos.arrow(pos, pos + up, BLUE);
}

fn rotate_phi(mut p: ResMut<TestParams>, time: Res<Time>) {
    p.theta = p.theta + PI / 4.0 * time.delta_seconds();
    p.phi = p.phi + PI / 8.0 * time.delta_seconds();
}
