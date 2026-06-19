use bevy::ecs::message::Message;

#[derive(Message)]
pub struct OnInPlacingStart;

#[derive(Message)]
pub struct DisableBuildingToPlace;