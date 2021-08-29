mod create_pokemon;
mod entities;

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
}

fn execute(req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(number.into()),
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

        let res = execute(req);
        assert_eq!(res, Response::Ok(number));
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);
        assert_eq!(res, Response::BadRequest);
    }
}
