use std::fmt::{self, Display};

use rand::prelude::ThreadRng;
use rand::Rng;

type EntityId = u128;
pub trait WithId {
    fn get_id(&self) -> EntityId;
}

//Термометр
#[derive(Debug, Clone)]
pub struct Thermometer<R: Rng> {
    id: EntityId,
    rnd: R,
}

impl<R: Rng> WithId for Thermometer<R> {
    fn get_id(&self) -> EntityId {
        self.id
    }
}

impl<R: Rng> fmt::Display for Thermometer<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer[id:{}]", self.get_id())
    }
}

impl<R: Rng> Thermometer<R> {
    pub fn new(id: EntityId, rnd: R) -> Self {
        Thermometer { id, rnd }
    }

    pub fn themperature(&mut self) -> f32 {
        self.rnd.random::<f32>() * 100_f32
    }
}

//Розетка
#[derive(Debug, Clone)]
pub struct Socket<R: Rng> {
    id: EntityId,
    pub is_on: bool,
    rnd: R,
}

impl<R: Rng> WithId for Socket<R> {
    fn get_id(&self) -> EntityId {
        self.id
    }
}

impl<R: Rng> fmt::Display for Socket<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Socket[id:{}, is_on:{}]", self.get_id(), self.is_on)
    }
}

impl<R: Rng> Socket<R> {
    pub fn new(id: EntityId, rnd: R) -> Self {
        Socket {
            id,
            is_on: false,
            rnd,
        }
    }

    pub fn on(&mut self) {
        self.is_on = true
    }

    pub fn off(&mut self) {
        self.is_on = false
    }

    pub fn power(&mut self) -> f32 {
        if self.is_on {
            self.rnd.random::<f32>() + 0.1
        } else {
            0.0
        }
    }
}

pub enum SmartDevice<R: Rng> {
    Thermometer(Thermometer<R>),
    Socket(Socket<R>),
}

impl<R: Rng> WithId for SmartDevice<R> {
    fn get_id(&self) -> EntityId {
        match self {
            Self::Thermometer(t) => t.get_id(),
            Self::Socket(s) => s.get_id(),
        }
    }
}

impl<R: Rng> fmt::Display for SmartDevice<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Socket(socket) => socket.fmt(f),
            Self::Thermometer(thermometer) => thermometer.fmt(f),
        }
    }
}

impl<R: Rng> From<Socket<R>> for SmartDevice<R> {
    fn from(value: Socket<R>) -> Self {
        Self::Socket(value)
    }
}

impl<R: Rng> From<Thermometer<R>> for SmartDevice<R> {
    fn from(value: Thermometer<R>) -> Self {
        Self::Thermometer(value)
    }
}

pub trait AnyRoom<R: Rng>: WithId + fmt::Display {
    fn get_device(&self, index: usize) -> &SmartDevice<R>;
    fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice<R>;
}
//Комната
pub struct Room<const S: usize, R: Rng> {
    id: EntityId,
    devices: [SmartDevice<R>; S],
}

impl<const S: usize, R: Rng> WithId for Room<S, R> {
    fn get_id(&self) -> EntityId {
        self.id
    }
}

impl<const S: usize, R: Rng> fmt::Display for Room<S, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Room[id:{}, devices[", self.get_id())?;

        for (i, dev) in self.devices.iter().enumerate() {
            dev.fmt(f)?;
            if i < (S - 1) {
                write!(f, ",")?;
            }
        }
        write!(f, "]]")?;
        Ok(())
    }
}

impl<const S: usize, R: Rng> Room<S, R> {
    pub fn new(id: EntityId, devices: [SmartDevice<R>; S]) -> Self {
        Room { id, devices }
    }
}

impl<const S: usize, R: Rng> AnyRoom<R> for Room<S, R> {
    fn get_device(&self, index: usize) -> &SmartDevice<R> {
        match self.devices.get(index) {
            Some(dev) => dev,
            None => panic!("The device index is out of range"),
        }
    }

    fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice<R> {
        match self.devices.get_mut(index) {
            Some(dev) => dev,
            None => panic!("The device index is out of range"),
        }
    }
}

//Дом
pub struct SmartHouse<R: Rng> {
    id: EntityId,
    rooms: Vec<Box<dyn AnyRoom<R>>>,
}

impl<R: Rng> WithId for SmartHouse<R> {
    fn get_id(&self) -> EntityId {
        self.id
    }
}

impl<R: Rng> Display for SmartHouse<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rooms_count = self.rooms.len();
        write!(f, "SmartHome[id:{}, rooms[", self.get_id())?;
        for (idx, room) in self.rooms.iter().enumerate() {
            room.fmt(f)?;
            if idx + 1 < rooms_count {
                write!(f, ",")?;
            }
        }
        write!(f, "]]")?;
        Ok(())
    }
}
impl<R: Rng> SmartHouse<R> {
    pub fn new(id: EntityId, rooms: Vec<Box<dyn AnyRoom<R>>>) -> Self {
        Self { id, rooms }
    }

    pub fn get_room(&self, room_index: usize) -> &Box<dyn AnyRoom<R>> {
        match self.rooms.get(room_index) {
            Some(room) => room,
            None => panic!("The room index is out of range"),
        }
    }

    pub fn get_room_mut(&mut self, room_index: usize) -> &mut Box<dyn AnyRoom<R>> {
        match self.rooms.get_mut(room_index) {
            Some(room) => room,
            None => panic!("The room index is out of range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermometer_should_work() {
        let mut t1 = Thermometer::new(1234, rand::rng());
        let th1 = t1.themperature();
        assert!(size_of_val(&th1) > 0); //just in case
    }

    #[test]
    fn socket_should_work() {
        let mut s1 = Socket::new(1234, rand::rng());

        s1.on();
        assert!(s1.is_on);
        let power = s1.power();
        assert!(power > 0.0);

        s1.off();
        assert!(!s1.is_on);
        let power = s1.power();
        assert_eq!(power, 0.0); //bcs it's off
    }

    #[test]
    fn room_constr_should_work() {
        let rnd = rand::rng();
        let r1 = Room::new(
            123,
            [
                Socket::new(123, rnd.clone()).into(),
                Thermometer::new(345, rnd.clone()).into(),
            ],
        );

        assert_eq!(r1.devices.len(), 2);
    }

    #[test]
    fn room_get_device_should_work() {
        let rnd = rand::rng();
        let socket = Socket::new(123, rnd.clone());
        let thermometer = Thermometer::new(345, rnd.clone()).into();
        let mut r1 = Room::new(123, [socket.into(), thermometer]);

        let s_ref = r1.get_device(0);
        assert_eq!(s_ref.get_id(), 123);

        let s_ref = r1.get_device_mut(0);
        assert_eq!(s_ref.get_id(), 123);

        match s_ref {
            SmartDevice::Socket(socket) => {
                socket.on();
                assert!(socket.power() > 0.0);
            }
            _ => panic!("Something wrong with your code it should return a Socket instance"),
        }
    }

    #[test]
    fn room_display_should_work() {
        let rnd = rand::rng();
        let r1 = Room::new(
            123,
            [
                Socket::new(123, rnd.clone()).into(),
                Thermometer::new(345, rnd.clone()).into(),
            ],
        );

        let display_str = format!("{:}", r1);

        assert!(display_str.starts_with("Room["));
        assert!(display_str.ends_with("]"));
        assert!(display_str.contains("Socket"));
        assert!(display_str.contains("Thermometer"));
    }

    #[test]
    fn smart_house_constr_should_work() {
        let rnd = rand::rng();

        let mut rooms: Vec<Box<dyn AnyRoom<_>>> = Vec::new();

        let room1 = Room::new(
            123,
            [
                Socket::new(123, rnd.clone()).into(),
                Thermometer::new(345, rnd.clone()).into(),
            ],
        );

        rooms.push(Box::new(room1));

        let room2 = Room::new(
            123,
            [
                Socket::new(123, rnd.clone()).into(),
                Socket::new(345, rnd.clone()).into(),
                Thermometer::new(678, rnd.clone()).into(),
            ],
        );

        rooms.push(Box::new(room2));

        let smart_house = SmartHouse::new(123, rooms);

        assert_eq!(smart_house.rooms.len(), 2)
    }

    #[test]
    fn smart_house_get_room_should_work() {
        let rnd = rand::rng();

        let mut rooms: Vec<Box<dyn AnyRoom<_>>> = Vec::new();

        let room1 = Room::new(
            123,
            [
                Socket::new(123, rnd.clone()).into(),
                Thermometer::new(345, rnd.clone()).into(),
            ],
        );

        rooms.push(Box::new(room1));

        let room2 = Room::new(
            345,
            [
                Socket::new(123, rnd.clone()).into(),
                Socket::new(345, rnd.clone()).into(),
                Thermometer::new(678, rnd.clone()).into(),
            ],
        );

        rooms.push(Box::new(room2));

        let mut smart_house = SmartHouse::new(123, rooms);

        let room_ref = smart_house.get_room(1);
        assert_eq!(room_ref.get_id(), 345);

        let room_ref = smart_house.get_room_mut(0);
        assert_eq!(room_ref.get_id(), 123);
    }
}
