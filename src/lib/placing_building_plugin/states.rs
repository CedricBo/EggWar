use bevy::{ecs::resource::Resource, state::state::{ComputedStates, States}};

use crate::buildings::building::BuildingType;

#[derive(Resource, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InPlacing;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum SelectedBuildingToPlace {
    None,
    Selected(BuildingType),
}

impl ComputedStates for InPlacing {
    type SourceStates = SelectedBuildingToPlace;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            SelectedBuildingToPlace::Selected(_) => Some(Self),
            SelectedBuildingToPlace::None => None,
        }
    }
    
    const ALLOW_SAME_STATE_TRANSITIONS: bool = false;
}
