use lesson3_hw1::smarthome::{AnyRoom, Room, SmartDevice, SmartHouse, Socket, Thermometer};
use rand::Rng;

fn main() {
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

    println!("{:}", smart_house);

    let room1 = smart_house.get_room_mut(0);
    device_on(room1.get_device_mut(0));

    let room2 = smart_house.get_room_mut(1);
    device_on(room2.get_device_mut(0));
    device_on(room2.get_device_mut(1));

    println!("{:}", smart_house);
}

fn device_on<R: Rng>(device: &mut SmartDevice<R>) {
    match device {
        SmartDevice::Socket(socket) => socket.on(),
        _ => panic!("Something wrong with your code, it should return Socket"),
    }
}
