use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_marble(commands: &mut Commands, asset_server: &AssetServer, x: f32, y: f32) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(scale(x), scale(y)).into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }
        .into(),
        forces: RigidBodyForces {
            ..Default::default()
        }
        .into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(scale(16.0)).into(),
        material: ColliderMaterial {
            restitution: 1.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/marble.png"),
            transform: Transform::from_scale(Vec3::splat(0.50))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)), // x and y are overwritten by the RigidBody position
            ..Default::default()
        })
        .insert_bundle(collider)
        .insert_bundle(rigid_body)
        .insert(ColliderPositionSync::Discrete);
}
