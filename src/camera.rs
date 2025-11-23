use crate::{kcc::CharacterControllerState, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        sync_camera_transform.after(TransformEasingSystems::UpdateEasingTick),
    );
}

#[derive(Component, Clone, Copy)]
#[relationship(relationship_target = CharacterControllerCamera)]
pub struct CharacterControllerCameraOf(pub Entity);

#[derive(Component, Clone, Copy)]
#[relationship_target(relationship = CharacterControllerCameraOf)]
pub struct CharacterControllerCamera(Entity);

impl CharacterControllerCamera {
    pub fn get(self) -> Entity {
        self.0
    }
}

pub(crate) fn sync_camera_transform(
    mut cameras: Query<
        (&mut Transform, &CharacterControllerCameraOf),
        (Without<CharacterControllerState>,),
    >,
    kccs: Query<(&Transform, &CharacterController, &CharacterControllerState)>,
) {
    // TODO: DIY TransformHelper to use current global transform.
    // Can't use GlobalTransform directly: outdated -> jitter
    // Can't use TransformHelper directly: access conflict with &mut Transform
    for (mut camera_transform, camera_of) in cameras.iter_mut() {
        if let Ok((kcc_transform, cfg, state)) = kccs.get(camera_of.0) {
            let height = state
                // changing the collider does not change the transform, so to get the correct position for the feet,
                // we need to use the collider we spawned with.
                .standing_collider
                .aabb(Vec3::default(), Rotation::default())
                .size()
                .y;
            let view_height = if state.crouching {
                cfg.crouch_view_height
            } else {
                cfg.standing_view_height
            };
            camera_transform.translation =
                kcc_transform.translation + Vec3::Y * (-height / 2.0 + view_height);
        }
    }
}
