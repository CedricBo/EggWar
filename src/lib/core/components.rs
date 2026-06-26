use bevy::{ecs::component::Component, math::{UVec2, Vec2}, transform::components::Transform};

#[derive(Component, Default)]
pub struct Size(pub Vec2);

impl Into<Vec2> for &Size {
    fn into(self) -> Vec2 {
        self.0
    }
}

impl From<Vec2> for Size {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

#[derive(Component)]
#[require(Transform, Size)]
pub struct Blockable;

#[derive(Component, Clone)]
pub struct AtlasRange
{
    pub first: usize,
    pub last: usize,
}