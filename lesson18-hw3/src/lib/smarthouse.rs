use std::{collections::HashMap, fmt};

use crate::devices::*;

//Комната
#[derive(Debug, Default)]
pub struct Room<R> {
    pub devices: HashMap<String, SmartDevice<R>>,
}

impl<R> fmt::Display for Room<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.devices.len();
        write!(f, "Room[devices[")?;

        for (i, (name, dev)) in self.devices.iter().enumerate() {
            write!(f, "Device[name:{name},{dev}]")?;
            if i < size {
                write!(f, ", ")?;
            }
        }
        write!(f, "]]")?;
        Ok(())
    }
}

impl<R> Room<R> {
    pub fn new(devices: HashMap<String, SmartDevice<R>>) -> Self {
        Room { devices }
    }

    pub fn devices_count(&self) -> usize {
        self.devices.len()
    }

    pub fn get_device(&self, name: &str) -> Option<&SmartDevice<R>> {
        self.devices.get(name)
    }

    pub fn get_device_mut(&mut self, name: &str) -> Option<&mut SmartDevice<R>> {
        self.devices.get_mut(name)
    }

    //returns an old device if it presents
    pub fn add_device<S>(&mut self, name: S, device: SmartDevice<R>) -> Option<SmartDevice<R>>
    where
        S: Into<String>,
    {
        self.devices.insert(name.into(), device)
    }

    pub fn remove_device(&mut self, name: &str) -> Option<SmartDevice<R>> {
        self.devices.remove(name)
    }
}

//Дом
#[derive(Debug, Default)]
pub struct SmartHouse<R> {
    pub name: String,
    pub rooms: HashMap<String, Room<R>>,
}

impl<R> fmt::Display for SmartHouse<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rooms_count = self.rooms.len();
        write!(f, "SmartHome[name:{}, rooms[", self.name)?;
        for (idx, (room_name, room)) in self.rooms.iter().enumerate() {
            write!(f, "Room[name:{room_name},{room}]")?;
            if idx < rooms_count {
                write!(f, ",")?;
            }
        }
        write!(f, "]]")?;
        Ok(())
    }
}

impl<R> SmartHouse<R> {
    pub fn new<S>(name: S, rooms: HashMap<String, Room<R>>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            rooms,
        }
    }

    pub fn add_room<S>(&mut self, room_name: S, room: Room<R>) -> Option<Room<R>>
    where
        S: Into<String>,
    {
        self.rooms.insert(room_name.into(), room)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room<R>> {
        self.rooms.remove(room_name)
    }

    pub fn rooms_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn empty(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room<R>> {
        self.rooms.get(room_name)
    }

    pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room<R>> {
        self.rooms.get_mut(room_name)
    }

    pub fn get_device_mut<'a, 'b>(
        &'a mut self,
        room: &'b str,
        device: &'b str,
    ) -> Result<&'a mut SmartDevice<R>, DeviceNotFound<'b>> {
        self.get_room_mut(room)
            .and_then(|room| room.get_device_mut(device))
            .ok_or(DeviceNotFound { room, device })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceNotFound<'a> {
    room: &'a str,
    device: &'a str,
}

impl<'a> fmt::Display for DeviceNotFound<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Device not found for room:{}, device:{}",
            self.room, self.device
        )
    }
}

impl<'a> std::error::Error for DeviceNotFound<'a> {}

#[cfg(test)]
mod tests {
    use rand::rngs::ThreadRng;

    use crate::devices::{Socket, Thermometer};

    use super::*;

    #[test]
    fn room_const_should_work() {
        let rnd = rand::rng();
        let r1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 2.0).unwrap().into(),
            ),
            (
                "therm1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        assert_eq!(r1.devices.len(), 2);
    }

    #[test]
    fn room_get_device_should_work() {
        let rnd = rand::rng();
        let socket = Socket::new(123.to_string(), 2.1).unwrap();
        let thermometer = Thermometer::new(345.to_string(), rnd.clone());
        let mut r1 = Room::new(HashMap::from([
            ("s1".to_owned(), socket.into()),
            ("t1".to_owned(), thermometer.into()),
        ]));

        let opt_ref = r1.get_device("s1");
        assert_eq!(opt_ref.map(|d| d.get_model()), Some("123"));

        let opt_ref: Option<&mut SmartDevice<ThreadRng>> = r1.get_device_mut("s1");
        assert_eq!(opt_ref.as_ref().map(|d| d.get_model()), Some("123"));

        match opt_ref {
            Some(SmartDevice::Socket(socket)) => {
                socket.on();
                assert!(socket.is_on);
            }
            _ => panic!("Something wrong with your code, it should return a Socket instance"),
        }
    }

    #[test]
    fn room_display_should_work() {
        let rnd = rand::rng();
        let r1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 2.5).unwrap().into(),
            ),
            (
                "therm1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let display_str = format!("{:}", r1);

        assert!(display_str.starts_with("Room["));
        assert!(display_str.ends_with("]"));
        assert!(display_str.contains("Socket"));
        assert!(display_str.contains("Thermometer"));
    }

    #[test]
    fn smart_house_constr_should_work() {
        let rnd = rand::rng();

        let room1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 0.1).unwrap().into(),
            ),
            (
                "therm1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let room2: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 2.5).unwrap().into(),
            ),
            (
                "socket2".to_owned(),
                Socket::new("Europe", 3.1).unwrap().into(),
            ),
            (
                "therm1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let smart_house = SmartHouse::new(
            "The House That Jack Built",
            HashMap::from([
                ("Molly's chamber".to_owned(), room1),
                ("Jack's room".to_owned(), room2),
            ]),
        );

        assert_eq!(smart_house.rooms.len(), 2)
    }

    #[test]
    fn smart_house_get_room_should_work() {
        let rnd = rand::rng();

        let room1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 2.5).unwrap().into(),
            ),
            (
                "therm1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let room2: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            (
                "socket1".to_owned(),
                Socket::new("Europe", 0.5).unwrap().into(),
            ),
            (
                "socket2".to_owned(),
                Socket::new("Europe", 0.5).unwrap().into(),
            ),
            (
                "t1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let smart_house = SmartHouse::new(
            "The House That Jack Built",
            HashMap::from([
                ("Molly's chamber".to_owned(), room1),
                ("Jack's room".to_owned(), room2),
            ]),
        );

        let room_opt = smart_house.get_room("Molly's chamber");
        assert_eq!(
            room_opt
                .as_ref()
                .and_then(|r| r.get_device("socket1").map(|d| d.get_model())),
            Some("Europe")
        );

        let room_opt = smart_house.get_room("Jack's room");
        assert_eq!(
            room_opt
                .as_ref()
                .and_then(|r| r.get_device("t1").map(|d| d.get_model())),
            Some("Celsius")
        );
    }

    #[test]
    fn smart_house_get_device_by_room_and_name_should_work() {
        let rnd = rand::rng();

        let room1: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            ("s1".to_owned(), Socket::new("Europe", 1.2).unwrap().into()),
            (
                "t1".to_owned(),
                Thermometer::new("Celsius", rnd.clone()).into(),
            ),
        ]));

        let room2: Room<ThreadRng> = Room::new(HashMap::<String, SmartDevice<ThreadRng>>::from([
            ("s1".to_owned(), Socket::new("Europe", 0.1).unwrap().into()),
            ("s2".to_owned(), Socket::new("Europe", 0.5).unwrap().into()),
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

        let maybe_device = smart_house.get_device_mut("Molly's chamber", "s1");
        assert!(maybe_device.is_ok());
        assert_eq!(maybe_device.unwrap().get_model(), "Europe");

        let maybe_device = smart_house.get_device_mut("Jack's room", "foo-bar");
        assert!(maybe_device.is_err());
        assert_eq!(
            maybe_device.err().unwrap(),
            DeviceNotFound {
                room: "Jack's room",
                device: "foo-bar"
            }
        );
    }
}
