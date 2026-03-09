use std::{
    cell::RefCell,
    fmt::{self, Display, Error, Write},
};

use rand::Rng;

use crate::smarthome::{Room, SmartDevice, SmartHouse, Socket, Thermometer, WithModel};

pub trait Report {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub struct ReportWrapper<'a>(RefCell<&'a mut dyn Report>);

impl<'a> Display for ReportWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow_mut().mk_report(f)
    }
}

pub fn print_report(v: &mut dyn Report) {
    println!("{}", ReportWrapper(RefCell::new(v)))
}

pub fn mk_report(v: &mut dyn Report) -> Result<String, Error> {
    let mut ret = String::default();
    write!(ret, "{}", ReportWrapper(RefCell::new(v)))?;
    Ok(ret)
}

impl<R: Rng> Report for Thermometer<R> {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.themperature();
        write!(
            f,
            "Thermometer[model:{}, themperature:{}]",
            self.get_model(),
            t
        )
    }
}

impl<R: Rng> Report for Socket<R> {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = self.power();
        write!(f, "Socket[model:{}, power:{}]", self.get_model(), p)
    }
}

impl<R: Rng> Report for SmartDevice<R> {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Report for SmartDevice[")?;
        match self {
            Self::Thermometer(t) => t.mk_report(f)?,
            Self::Socket(s) => s.mk_report(f)?,
        };
        write!(f, "]")
    }
}

impl<R: Rng> Report for Room<R> {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Report for Room[")?;
        write!(f, "it containts {} devices:", self.devices_count())?;
        for (n, device) in &mut self.devices {
            write!(f, ", {n}:")?;
            device.mk_report(f)?;
        }
        write!(f, "]")
    }
}

impl<R: Rng> Report for SmartHouse<R> {
    fn mk_report(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Report for SmartHouse[")?;
        write!(f, "it containts {} rooms:", self.rooms_count())?;
        for (name, room) in &mut self.rooms {
            write!(f, ", {name}:")?;
            room.mk_report(f)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::rngs::ThreadRng;

    use crate::smarthome::*;

    use super::*;

    #[test]
    fn report_for_device_should_work() {
        let device = Thermometer::new("Farengate", rand::rng());

        let report = mk_report(&mut SmartDevice::from(device));

        assert!(report.is_ok());

        assert!(report.unwrap().contains("Farengate"));
    }

    #[test]
    fn report_for_smarthome_should_work() {
        let rnd = rand::rng();

        let room1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            ("s1".to_owned(), Socket::new("Europe", rnd.clone()).into()),
            (
                "t1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let room2: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            ("s1".to_owned(), Socket::new("Europe", rnd.clone()).into()),
            ("s2".to_owned(), Socket::new("Europe", rnd.clone()).into()),
            (
                "t1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let mut smart_house = SmartHouse::new(
            "The House That Jack Built",
            HashMap::from([
                ("Molly's chamber".to_owned(), room1),
                ("Jack's room".to_owned(), room2),
            ]),
        );

        let report = mk_report(&mut smart_house);
        assert!(report.is_ok());
        assert!(report.unwrap().contains("SmartHouse"));
    }
}
