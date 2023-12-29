use bevy::{
    prelude::*,
    sprite::{Material2d, Mesh2dHandle},
};

//TODO I want this in CharacterBundle
#[derive(Component, Debug)]
pub struct CharacterData {
    pub radius: f32,
    // pub level: u32,
}

//TODO is it possible to add more fields to this?
/// A component bundle for entities with a [`Mesh2dHandle`] and a [`Material2d`].
#[derive(Bundle)]
pub struct CharacterBundle<M: Material2d> {
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    // Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    // Indication of whether an entity is visible in any view.
    pub view_visibility: ViewVisibility,
}

impl<M: Material2d> Default for CharacterBundle<M> {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}
