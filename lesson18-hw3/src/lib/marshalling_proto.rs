use bincode::{de::read::Reader, enc::write::Writer};
use serde::{de::DeserializeOwned, Serialize};

pub fn marshall<T: Serialize>(v: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::serde::encode_to_vec(v, bincode::config::legacy())
}

pub fn marshall_to_writer<T: Serialize, U: Writer>(
    writer: U,
    v: &T,
) -> Result<(), bincode::error::EncodeError> {
    bincode::serde::encode_into_writer(v, writer, bincode::config::legacy())
}

pub fn unmarshall<T: DeserializeOwned>(encdoed: &[u8]) -> Result<T, bincode::error::DecodeError> {
    bincode::serde::decode_from_slice(encdoed, bincode::config::legacy()).map(|(value, _)| value)
}

pub fn unmarshall_from_reader<T: DeserializeOwned, R: Reader>(
    reader: R,
) -> Result<T, bincode::error::DecodeError> {
    bincode::serde::decode_from_reader(reader, bincode::config::legacy())
}

mod socket_proto {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum Command {
        SetOn,
        SetOff,
        GetPow,
        Quit,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum Response {
        Pow(f32),
        Ack,
    }
}

mod therm_proto {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum Command {
        Set(f32),
        Quit,
    }
}

#[cfg(test)]
mod tests {
    use super::socket_proto;
    use super::therm_proto;
    use super::*;
    use std::fmt::Debug;

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
    fn test_command_therm_marshalling() {
        test_base(therm_proto::Command::Set(1.2));
        test_base(therm_proto::Command::Quit);
    }

    #[test]
    fn test_command_socket_marshalling() {
        test_base(socket_proto::Command::GetPow);
        test_base(socket_proto::Command::SetOff);
        test_base(socket_proto::Command::SetOn);
        test_base(socket_proto::Command::Quit);
    }

    #[test]
    fn test_response_socket_marshalling() {
        test_base(socket_proto::Response::Ack);
        test_base(socket_proto::Response::Pow(42.0));
    }
}
