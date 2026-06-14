use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Building {
    width: f32,
    height: f32,
    path: String,
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub enum BuildingType {
    Grange,
    Garden,
    Turret,
}

impl Building {
    pub fn grange() -> Self {
        Self {
            width: 100.0,
            height: 100.0,
            path: "./grange.png".into(),
        }
    }

    pub fn garden() -> Self {
        Self {
            width: 100.0,
            height: 35.0,
            path: "./garden.png".into(),
        }
    }

    pub fn turret() -> Self {
        Self {
            width: 20.0,
            height: 40.0,
            path: "./turret.png".into(),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn asset_path(&self) -> &str {
        &self.path
    }
}

impl From<BuildingType> for Building {
    fn from(value: BuildingType) -> Self {
        match value {
            BuildingType::Grange => Building::grange(),
            BuildingType::Garden => Building::garden(),
            BuildingType::Turret => Building::turret(),
        }
    }
}
