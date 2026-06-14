use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
