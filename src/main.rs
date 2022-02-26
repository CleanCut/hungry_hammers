use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use hungry_hammers::{marble::spawn_marble, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 720.0,
            height: 720.0,
            title: "Hungry Hammers".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup)
        .add_system(movement)
        .run();
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

    // Hammer
    let rigid_body = RigidBodyBundle {
        position: (
            Vec2::new(scale(600.0), scale(600.0)),
            std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4,
        )
            .into(),
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
        .insert(ColliderDebugRender::with_id(0))
        .insert(ColliderPositionSync::Discrete)
        .insert(Hammer);
}

#[derive(Component)]
struct Hammer;

fn movement(
    mut hammer_pos_components: Query<&mut RigidBodyPositionComponent, With<Hammer>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let mut mouse_location = None;
    for event in cursor_moved_events.iter() {
        mouse_location = Some(event.position.to_world_coords());
    }
    if let Some(location) = mouse_location {
        for mut hammer_pos_component in hammer_pos_components.iter_mut() {
            hammer_pos_component.next_position.translation =
                Vec2::new(scale(location.x), scale(location.y)).into();
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
