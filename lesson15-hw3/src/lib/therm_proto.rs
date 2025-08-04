use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Set(f32),
    Get,
    Quit,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Temperature(f32),
    Ack,
}

pub fn marshall<T: Serialize>(v: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::serde::encode_to_vec(v, bincode::config::legacy())
}

pub fn unmarshall<T: DeserializeOwned>(encdoed: &[u8]) -> Result<T, bincode::error::DecodeError> {
    bincode::serde::decode_from_slice(encdoed, bincode::config::legacy()).map(|(value, _)| value)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    fn test_base<T>(message: T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let encoded: Vec<u8> = marshall(&message).unwrap();
        let actual: Result<T, _> = unmarshall(&encoded);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), message);
    }

    #[test]
    fn test_command_marshalling() {
        test_base(Command::Set(42.0));
        test_base(Command::Get);
        test_base(Command::Quit);
    }

    #[test]
    fn test_response_marshalling() {
        test_base(Response::Temperature(42.0));
        test_base(Response::Ack);
    }
}
