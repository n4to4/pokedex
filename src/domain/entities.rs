use std::convert::TryFrom;

pub struct Pokemon {
    pub number: PokemonNumber,
    name: PokemonName,
    types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PokemonNumber(u16);

#[derive(Debug, Clone)]
pub struct PokemonName(String);

#[derive(Debug, Clone)]
pub struct PokemonTypes(Vec<PokemonType>);

#[derive(Debug, Clone, Copy)]
enum PokemonType {
    Electric,
    Fire,
}

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> u16 {
        n.0
    }
}

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.into_iter() {
                let poketype = PokemonType::try_from(t)?;
                pts.push(poketype);
            }
            Ok(Self(pts))
        }
    }
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(()),
        }
    }
}
