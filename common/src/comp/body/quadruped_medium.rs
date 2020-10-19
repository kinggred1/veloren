use crate::{make_case_elim, make_proj_elim};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

make_proj_elim!(
    body,
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Body {
        pub species: Species,
        pub body_type: BodyType,
    }
);

impl Body {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let species = *(&ALL_SPECIES).choose(&mut rng).unwrap();
        Self::random_with(&mut rng, &species)
    }

    #[inline]
    pub fn random_with(rng: &mut impl rand::Rng, &species: &Species) -> Self {
        let body_type = *(&ALL_BODY_TYPES).choose(rng).unwrap();
        Self { species, body_type }
    }
}

impl From<Body> for super::Body {
    fn from(body: Body) -> Self { super::Body::QuadrupedMedium(body) }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum Species {
    Grolgar = 0,
    Saber = 1,
    Tiger = 2,
    Tuskram = 3,
    Lion = 6,
    Tarasque = 7,
    Wolf = 8,
    Frostfang = 9,
    Mouflon = 10,
    Catoblepas = 11,
    Bonerattler = 12,
    Deer = 13,
    Hirdrasil = 14,
    Roshwalr = 15,
}

/// Data representing per-species generic data.
///
/// NOTE: Deliberately don't (yet?) implement serialize.
#[derive(Clone, Debug, Deserialize)]
pub struct AllSpecies<SpeciesMeta> {
    pub grolgar: SpeciesMeta,
    pub saber: SpeciesMeta,
    pub tiger: SpeciesMeta,
    pub tuskram: SpeciesMeta,
    pub lion: SpeciesMeta,
    pub tarasque: SpeciesMeta,
    pub wolf: SpeciesMeta,
    pub frostfang: SpeciesMeta,
    pub mouflon: SpeciesMeta,
    pub catoblepas: SpeciesMeta,
    pub bonerattler: SpeciesMeta,
    pub deer: SpeciesMeta,
    pub hirdrasil: SpeciesMeta,
    pub roshwalr: SpeciesMeta,
}

impl<'a, SpeciesMeta> core::ops::Index<&'a Species> for AllSpecies<SpeciesMeta> {
    type Output = SpeciesMeta;

    #[inline]
    fn index(&self, &index: &'a Species) -> &Self::Output {
        match index {
            Species::Grolgar => &self.grolgar,
            Species::Saber => &self.saber,
            Species::Tiger => &self.tiger,
            Species::Tuskram => &self.tuskram,
            Species::Lion => &self.lion,
            Species::Tarasque => &self.tarasque,
            Species::Wolf => &self.wolf,
            Species::Frostfang => &self.frostfang,
            Species::Mouflon => &self.mouflon,
            Species::Catoblepas => &self.catoblepas,
            Species::Bonerattler => &self.bonerattler,
            Species::Deer => &self.deer,
            Species::Hirdrasil => &self.hirdrasil,
            Species::Roshwalr => &self.roshwalr,
        }
    }
}

pub const ALL_SPECIES: [Species; 14] = [
    Species::Grolgar,
    Species::Saber,
    Species::Tiger,
    Species::Tuskram,
    Species::Lion,
    Species::Tarasque,
    Species::Wolf,
    Species::Frostfang,
    Species::Mouflon,
    Species::Catoblepas,
    Species::Bonerattler,
    Species::Deer,
    Species::Hirdrasil,
    Species::Roshwalr,
];

impl<'a, SpeciesMeta: 'a> IntoIterator for &'a AllSpecies<SpeciesMeta> {
    type IntoIter = std::iter::Copied<std::slice::Iter<'static, Self::Item>>;
    type Item = Species;

    fn into_iter(self) -> Self::IntoIter { ALL_SPECIES.iter().copied() }
}

make_case_elim!(
    body_type,
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum BodyType {
        Female = 0,
        Male = 1,
    }
);

pub const ALL_BODY_TYPES: [BodyType; 2] = [BodyType::Female, BodyType::Male];
