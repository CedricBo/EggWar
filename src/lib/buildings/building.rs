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

    pub fn size_for_type(btype: BuildingType) -> (f32, f32) {
        match btype {
            BuildingType::Grange => (100.0, 100.0),
            BuildingType::Garden => (100.0, 35.0),
            BuildingType::Turret => (20.0, 40.0),
        }
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
            width: size.0,
            height: size.1,
            path: path.into(),
        }
    }
}
