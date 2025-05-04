use std::collections::HashMap;

use lesson8_hw2::{
    report::print_report,
    smarthome::{Room, SmartDevice, SmartHouse, Socket, Thermometer},
};
use rand::{self, rngs::ThreadRng};

fn main() {
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

    print_report(&mut smart_house);

    let socket = smart_house.get_device_mut("Molly's chamber", "s1").unwrap();

    match socket {
       SmartDevice::Socket(s)=> s.on(),
       _ => panic!("Something is wrong with your code, it should be a socket")     
    }

    print_report(&mut smart_house);

}
