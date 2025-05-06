use std::collections::HashMap;

use lesson8_hw2::{
    hashmap,
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

    let socket: &mut Socket<ThreadRng> = smart_house
        .get_device_mut("Molly's chamber", "s1")
        .expect("Something is wrong with your code, the socket should be in house") //разворачиваем Result<SmartDevice,...>
        .try_into()
        .expect("Something is wrong with your code, it should be a socket"); //разворачиваем TryFrom - Result<SmartDevice,...>

    socket.on(); //включаем розетку

    print_report(&mut smart_house);

    //macro edition
    let room1: Room<ThreadRng> = Room::new(hashmap!(
            "ms1".to_owned() => Socket::new("Type A", rnd.clone()).into(),
           "mt1".to_owned() =>  Thermometer::new("Farengate", rnd.clone()).into()));

    let room2: Room<ThreadRng> = Room::new(hashmap!(
            "js1".to_owned() => Socket::new("Type A", rnd.clone()).into(),
            "js2".to_owned() => Socket::new("Type A", rnd.clone()).into(),
           "jt1".to_owned() =>  Thermometer::new("Farengate", rnd.clone()).into()));

    let mut smart_house: SmartHouse<ThreadRng> = SmartHouse::new(
        "The House That Jack Built",
        hashmap!(
            "Molly's chamber".to_owned() => room1,
            "Jack's room".to_owned() => room2
        ),
    );

    let room3: Room<ThreadRng> = Room::new(hashmap!(
        "ms1".to_owned() => Socket::new("Type A", rnd.clone()).into(),
        "mt1".to_owned() =>  Thermometer::new("Farengate", rnd.clone()).into()));

    smart_house.add_room("Maiden's room", room3); //динамически добавляем команту
    print_report(&mut smart_house);
    smart_house.remove_room("Maiden's room");
    println!("-----after delete room---");
    print_report(&mut smart_house);

    let mollys_room: &mut Room<ThreadRng> = smart_house
        .get_room_mut("Molly's chamber")
        .expect("Something wrong with your code, Molly's chamber expected");

        let mut socket = Socket::new("Europe", rnd.clone());
    socket.on();
    mollys_room.add_device("ms2", socket.into());
    println!("-----after add Europe type socket to Molly's room---");
    print_report(&mut smart_house);
}
