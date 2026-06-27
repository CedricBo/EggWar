use bevy::{
    ecs::{bundle::Bundle, component::Component, system::EntityCommands},
    math::Vec2,
};

use crate::{
    buildings::garden::GardenComponent,
    core::components::{Blockable, Size},
};

#[derive(Component)]
pub struct BuildingComponent {
    building_type: BuildingType,
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub enum BuildingType {
    Grange,
    Garden,
    Stand,
}

#[derive(Bundle)]
pub struct Building {
    building_component: BuildingComponent,
    blockable: Blockable,
    size: Size,
}

impl BuildingComponent {
    pub fn building_type(&self) -> BuildingType {
        self.building_type
    }

    pub fn size_for_type(btype: BuildingType) -> Vec2 {
        match btype {
            BuildingType::Grange => (100.0, 100.0),
            BuildingType::Garden => (150.0, 35.0),
            BuildingType::Stand => (100.0, 60.0),
        }
        .into()
    }

    pub fn path_for_type(btype: BuildingType) -> &'static str {
        match btype {
            BuildingType::Grange => "./grange.png",
            BuildingType::Garden => "./garden.png",
            BuildingType::Stand => "./stand.png",
        }
    }

    pub fn spawn_type(btype: BuildingType, entity_command: &mut EntityCommands) {
        match btype {
            BuildingType::Grange => {}
            BuildingType::Garden => {
                entity_command.insert(GardenComponent);
            }
            BuildingType::Stand => {}
        };

        let building = BuildingComponent::from(btype);
        let size = Self::size_for_type(btype.clone());

        entity_command.insert((Size(size), Blockable, building));
    }
}

impl From<BuildingType> for BuildingComponent {
    fn from(value: BuildingType) -> Self {
        Self {
            building_type: value,
        }
    }
}
