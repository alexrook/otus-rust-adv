use wincode::config::DefaultConfig;
use wincode::io::Reader;
use wincode::io::Writer;
use wincode::SchemaRead;
use wincode::SchemaReadOwned;
use wincode::SchemaWrite;
use wincode::WriteResult;

pub fn marshall<T: SchemaWrite<DefaultConfig, Src = T>>(
    writer: impl Writer,
    v: &T,
) -> WriteResult<()> {
    wincode::serialize_into(writer, v)
}

pub fn unmarshall<'a, T>(reader: impl Reader<'a>) -> Result<T, wincode::ReadError>
where
    T: SchemaReadOwned<DefaultConfig, Dst = T>,
{
    wincode::deserialize_from(reader)
}

mod socket_proto {
    use wincode::{SchemaRead, SchemaWrite};

    #[derive(SchemaWrite, SchemaRead)]
    pub enum Command {
        SetOn,
        SetOff,
        GetPow,
        Quit,
    }

    pub enum Response {
        Pow(f32),
        Ack,
    }
}

mod therm_proto {
    use wincode::{SchemaRead, SchemaWrite};

    #[derive(SchemaWrite, SchemaRead, PartialEq, Debug)]
    pub enum Command {
        Set(f32),
        Quit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn test_base<T>(message: T)
    where
        T: SchemaWrite<DefaultConfig, Src = T>
            + std::cmp::PartialEq
            + std::fmt::Debug
            + for<'a> wincode::SchemaRead<'a, DefaultConfig, Dst = T>,
    {
        let buf = Vec::new();
        let encoded = marshall(buf, &message).unwrap();
        let actual: Result<T, _> = unmarshall(buf);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), message);
    }

    #[test]
    fn test_command_therm_marshalling() {
        test_base(therm_proto::Command::Set(1.2));
        test_base(therm_proto::Command::Quit);
    }

    // #[test]
    // fn test_command_socket_marshalling() {
    //     test_base(socket_proto::Command::GetPow);
    //     test_base(socket_proto::Command::SetOff);
    //     test_base(socket_proto::Command::SetOn);
    //     test_base(socket_proto::Command::Quit);
    // }

    // #[test]
    // fn test_response_socket_marshalling() {
    //     test_base(socket_proto::Response::Ack);
    //     test_base(socket_proto::Response::Pow(42.0));
    // }
}
