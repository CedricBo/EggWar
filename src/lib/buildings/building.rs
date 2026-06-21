use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Building {
    size: Vec2,
    path: String,
    building_type: BuildingType
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub enum BuildingType {
    Grange,
    Garden,
    Turret,
}

impl Building {
    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn building_type(&self) -> BuildingType 
    {
        self.building_type
    }

    pub fn asset_path(&self) -> &str {
        &self.path
    }

    pub fn size_for_type(btype: BuildingType) -> Vec2 {
        match btype {
            BuildingType::Grange => (100.0, 100.0),
            BuildingType::Garden => (100.0, 35.0),
            BuildingType::Turret => (20.0, 40.0),
        }.into()
    }

    pub fn path_for_type(btype: BuildingType) -> &'static str {
        match btype {
            BuildingType::Grange => "./grange.png",
            BuildingType::Garden => "./garden.png",
            BuildingType::Turret => "./turret.png",
        }
    }
}

impl From<BuildingType> for Building {
    fn from(value: BuildingType) -> Self {
        let size = Self::size_for_type(value.clone());
        let path = Self::path_for_type(value.clone());

        Self {
            size,
            path: path.into(),
            building_type: value
        }
    }
}
