use rand::{seq::SliceRandom, thread_rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Body {
    pub species: Species,
    pub body_type: BodyType,
}

impl Body {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let species = *(&ALL_SPECIES).choose(&mut rng).unwrap();
        let body_type = *(&ALL_BODY_TYPES).choose(&mut rng).unwrap();
        Self { species, body_type }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum Species {
    Rat = 0,
    Axolotl = 1,
    Gecko = 2,
    Turtle = 3,
    Squirrel = 4,
    Fungome = 5,
}

/// Data representing per-species generic data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllSpecies<SpeciesMeta> {
    pub rat: SpeciesMeta,
    pub axolotl: SpeciesMeta,
    pub gecko: SpeciesMeta,
    pub turtle: SpeciesMeta,
    pub squirrel: SpeciesMeta,
    pub fungome: SpeciesMeta,
}

impl<SpeciesMeta> core::ops::Index<Species> for AllSpecies<SpeciesMeta> {
    type Output = SpeciesMeta;

    fn index(&self, index: Species) -> &Self::Output {
        match index {
            Species::Rat => &self.rat,
            Species::Axolotl => &self.axolotl,
            Species::Gecko => &self.gecko,
            Species::Turtle => &self.turtle,
            Species::Squirrel => &self.squirrel,
            Species::Fungome => &self.fungome,
        }
    }
}

pub const ALL_SPECIES: [Species; 6] = [
    Species::Rat,
    Species::Axolotl,
    Species::Gecko,
    Species::Turtle,
    Species::Squirrel,
    Species::Fungome,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum BodyType {
    Female = 0,
    Male = 1,
}
pub const ALL_BODY_TYPES: [BodyType; 2] = [BodyType::Female, BodyType::Male];
