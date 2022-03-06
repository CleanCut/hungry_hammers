use std::time::Duration;

#[cfg(target_family = "wasm")]
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_prototype_lyon::{
    plugin::ShapePlugin,
    prelude::{DrawMode, GeometryBuilder, PathBuilder, StrokeMode},
};
use bevy_rapier2d::prelude::*;
use hungry_hammers::{marble::spawn_marble, prelude::*};

struct DebugInitialized(bool);

fn main() {
    let mut app = App::new();
    #[cfg(target_family = "wasm")]
    app.insert_resource(AssetServerSettings {
        asset_folder: "static/hungry_hammers/assets".to_string(),
    });
    app.insert_resource(WindowDescriptor {
        width: 720.0,
        height: 720.0,
        title: "Hungry Hammers".into(),
        ..Default::default()
    })
    .insert_resource(DebugInitialized(false))
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(ShapePlugin)
    .add_startup_system(setup.label("setup"))
    .add_system(setup_debug_colliders)
    .add_system(movement);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut integration_parameters: ResMut<IntegrationParameters>,
) {
    // configure physics
    rapier_config.scale = PHYSICS_SCALE;
    rapier_config.gravity = Vec2::ZERO.into();
    integration_parameters.max_ccd_substeps = 3;

    // configure camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Arena circle colliders
    let segments = 32;
    let radius = 328.0;
    for i in 0..segments {
        let inner = std::f64::consts::TAU / segments as f64 * i as f64;
        let mut inner_x = (inner.cos() * radius) as f32;
        let mut inner_y = (inner.sin() * radius) as f32;
        // Clamp near-zero values to zero, ie -0.0000000000000044087286 => 0.0
        if (inner_x > -0.000001) && (inner_x < 0.000001) {
            inner_x = 0.0;
        }
        if (inner_y > -0.000001) && (inner_y < 0.000001) {
            inner_y = 0.0;
        }
        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(scale(32.0), scale(5.0)).into(),
            position: ColliderPosition(
                (
                    Vec2::new(scale(inner_x), scale(inner_y)),
                    (inner + std::f64::consts::FRAC_PI_2) as f32,
                )
                    .into(),
            )
            .into(),
            ..Default::default()
        };
        commands
            .spawn_bundle(collider)
            //.insert(ColliderDebugRender::with_id(i))
            .insert(ColliderPositionSync::Discrete);
    }
    // Arena image
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/arena.png"),
        transform: Transform::from_scale(Vec3::splat(0.50)),
        ..Default::default()
    });

    // Marbles
    let rows = 5;
    let cols = 5;
    let start = Vec2::new(-150.0, 150.0);
    let step = 75.0;
    for row in 0..cols {
        let offset = if row % 2 == 0 { 0.0 } else { step * 0.5 };
        for col in 0..rows {
            spawn_marble(
                &mut commands,
                &asset_server,
                start.x + offset + (col as f32) * step,
                start.y - ((row as f32) * step),
            );
        }
    }

    // Hammers
    use std::f32::consts::*;
    spawn_hammer(&mut commands, Vec2::new(-300.0, -300.0), FRAC_PI_4, 0);
    spawn_hammer(
        &mut commands,
        Vec2::new(-300.0, 300.0),
        FRAC_PI_2 + FRAC_PI_4,
        1,
    );
    spawn_hammer(&mut commands, Vec2::new(300.0, 300.0), PI + FRAC_PI_4, 2);
    spawn_hammer(
        &mut commands,
        Vec2::new(300.0, -300.0),
        PI + FRAC_PI_2 + FRAC_PI_4,
        3,
    );
}

pub fn spawn_hammer(commands: &mut Commands, start: Vec2, angle: f32, id: usize) {
    let end = start * 0.6;
    let rigid_body = RigidBodyBundle {
        position: (Vec2::new(scale(start.x), scale(start.y)), angle).into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }
        .into(),
        body_type: RigidBodyType::KinematicPositionBased.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(scale(128.0), scale(48.0)).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert_bundle(rigid_body)
        //.insert(ColliderDebugRender::with_id(id))
        .insert(ColliderPositionSync::Discrete)
        .insert(Hammer::new(id, start, end));
}

#[derive(Component)]
struct Hammer {
    id: usize,
    start: Vec2,
    end: Vec2,
    forward_timer: Timer,
    back_timer: Timer,
    forward: bool,
}

impl Hammer {
    fn new(id: usize, start: Vec2, end: Vec2) -> Self {
        let forward_timer = Timer::from_seconds(0.125, false);
        let mut back_timer = Timer::from_seconds(0.5, false);
        back_timer.tick(Duration::from_secs_f32(10.0));

        Hammer {
            id,
            start,
            end,
            forward_timer,
            back_timer,
            forward: false,
        }
    }
}

#[derive(Component)]
struct DebugForHammer(usize);

fn setup_debug_collider(
    commands: &mut Commands,
    shape: &ColliderShapeComponent,
    position: &ColliderPositionComponent,
    id: usize,
) {
    // Add the collider lines, a visual representation of the sprite's collider
    // he[0] is half the length of one dimension, he[1] is half the length of the other
    let he: Vec2 = unscale_vec2(shape.0.as_cuboid().unwrap().half_extents.into());
    // These are used to rotate the points of the collider cuboid to the right place
    let sin = position.0.rotation.angle().sin();
    let cos = position.0.rotation.angle().cos();
    // These are the (x, y) coordinates of the center of the cuboid
    let v: Vec2 = unscale_vec2(position.0.translation.vector.into());
    // Make a vector of the corners of the cuboid, so we can draw the shape
    let mut points = Vec::<Vec2>::new();
    for (a, b) in [(1., 1.), (1., -1.), (-1., -1.), (-1., 1.)] {
        // compute the four corners. we have to shift a half extent from center and then rotate
        // TODO: Why is the y value ending up -425.0 too low???
        points.push(Vec2::new(
            (v.x + (a * he[0])) * cos - (v.y + (b * he[1])) * sin,
            (v.x + (a * he[0])) * sin + (v.y + (b * he[1])) * cos,
        ));
    }
    let mut path_builder = PathBuilder::new();
    let offset = Vec2::new(0.0, 425.0); // I have no idea why I need this correction, but it works
    path_builder.move_to(points[0] + offset);
    for point in &points[1..] {
        path_builder.line_to(*point + offset);
    }
    path_builder.close(); // draws the line from the last point to the first point
    let line = path_builder.build();
    let transform = Transform::from_xyz(v.x, v.y, 999.9); // put the debug lines on top of everything else
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line.0, // can be changed to `&line` once bevy_prototype_lyon > 0.4 is released
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
            transform,
        ))
        .insert(DebugForHammer(id));
}

fn setup_debug_colliders(
    mut commands: Commands,
    query: Query<(&ColliderShapeComponent, &ColliderPositionComponent, &Hammer)>,
    mut debug_initialized: ResMut<DebugInitialized>,
) {
    if !debug_initialized.0 {
        debug_initialized.0 = true;
        for (shape, position, hammer) in query.iter() {
            setup_debug_collider(&mut commands, shape, position, hammer.id);
        }
    }
}

fn movement(
    mut hammer_pos_components: Query<(&mut RigidBodyPositionComponent, &mut Hammer)>,
    mut hammer_debug: Query<(&mut Transform, &DebugForHammer)>,
    time: Res<Time>,
    mouse: Res<Input<MouseButton>>,
) {
    for (mut hammer_pos_component, mut hammer) in hammer_pos_components.iter_mut() {
        // Process a click
        if hammer.id == 0 && mouse.just_pressed(MouseButton::Left) && !hammer.forward {
            hammer.forward = true;
            let new_duration_secs =
                hammer.forward_timer.duration().as_secs_f32() * hammer.back_timer.percent_left();
            hammer
                .forward_timer
                .set_elapsed(Duration::from_secs_f32(new_duration_secs));
        }
        // Handle already moving hammers
        let new_pos: Vec2;
        if hammer.forward {
            let finished = hammer.forward_timer.tick(time.delta()).just_finished();
            new_pos = hammer
                .start
                .lerp(hammer.end, hammer.forward_timer.percent());
            hammer_pos_component.next_position.translation.vector =
                Vec2::new(scale(new_pos.x), scale(new_pos.y)).into();
            if finished {
                hammer.forward = false;
                hammer.forward_timer.reset();
                hammer.back_timer.reset();
            }
        } else {
            hammer.back_timer.tick(time.delta());
            new_pos = hammer
                .start
                .lerp(hammer.end, hammer.back_timer.percent_left());
            hammer_pos_component.next_position.translation.vector =
                Vec2::new(scale(new_pos.x), scale(new_pos.y)).into();
        }
        for (mut transform, belongs_to) in hammer_debug.iter_mut() {
            if hammer.id == belongs_to.0 {
                transform.translation = new_pos.extend(transform.translation.z);
            }
        }
    }
}

trait ToWorldCoords {
    fn to_world_coords(&self) -> Vec2;
}

impl ToWorldCoords for Vec2 {
    fn to_world_coords(&self) -> Vec2 {
        Vec2::new(self.x - 360.0, self.y - 360.0)
    }
}
