mod create_pokemon;
pub(crate) mod entities;

use crate::repositories::pokemon::InMemoryRepository;
use crate::repositories::pokemon::{Insert, Repository};
use entities::*;
use std::convert::TryFrom;

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

impl From<Insert> for Response {
    fn from(i: Insert) -> Self {
        match i {
            Insert::Ok(number) => Response::Ok(number.into()),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        }
    }
}

fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => repo.insert(number, name, types).into(),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let mut repo = InMemoryRepository::new();
        let res = execute(&mut repo, req);

        assert_eq!(res, Response::Ok(number));
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let mut repo = InMemoryRepository::new();

        let res = execute(&mut repo, req);
        assert_eq!(res, Response::BadRequest);
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();

        let mut repo = InMemoryRepository::new();
        repo.insert(number, name, types);

        let req = Request {
            number: number.into(),
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };
        let res = execute(&mut repo, req);

        assert_eq!(res, Response::Conflict);
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repo = InMemoryRepository::new().with_error();
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(&mut repo, req);

        assert_eq!(res, Response::Error);
    }
}
