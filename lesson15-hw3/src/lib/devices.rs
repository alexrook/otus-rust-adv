use std::fmt::{self};

use rand::Rng;

pub trait WithModel {
    fn get_model(&self) -> &str;
}

//Термометр
#[derive(Debug, Clone)]
pub struct Thermometer {
    model: String,
    temperature: f32,
}

impl WithModel for Thermometer {
    fn get_model(&self) -> &str {
        &self.model
    }
}

impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer[model:{}]", self.get_model())
    }
}

impl Thermometer {
    pub fn new<S>(model: S) -> Self
    where
        S: Into<String>,
    {
        Thermometer {
            model: model.into(),
            temperature: 0_f32,
        }
    }

    pub fn themperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, new_temperature: f32) {
        self.temperature = new_temperature
    }
}

//Розетка
#[derive(Debug, Clone)]
pub struct Socket<R> {
    model: String,
    pub is_on: bool,
    rnd: R,
}

impl<R> WithModel for Socket<R> {
    fn get_model(&self) -> &str {
        &self.model
    }
}

impl<R> fmt::Display for Socket<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Socket[model:{}, is_on:{}]",
            self.get_model(),
            self.is_on,
        )
    }
}

impl<R> Socket<R> {
    pub fn new<S>(model: S, rnd: R) -> Self
    where
        S: Into<String>,
    {
        Socket {
            model: model.into(),
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

    pub fn power(&mut self) -> f32
    where
        R: Rng,
    {
        if self.is_on {
            self.rnd.random::<f32>() + 0.1
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub enum SmartDevice<R> {
    Thermometer(Thermometer),
    Socket(Socket<R>),
}

impl<R> WithModel for SmartDevice<R> {
    fn get_model(&self) -> &str {
        match self {
            Self::Thermometer(t) => t.get_model(),
            Self::Socket(s) => s.get_model(),
        }
    }
}

impl<R> fmt::Display for SmartDevice<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Socket(socket) => socket.fmt(f),
            Self::Thermometer(thermometer) => thermometer.fmt(f),
        }
    }
}

impl<R> From<Socket<R>> for SmartDevice<R> {
    fn from(value: Socket<R>) -> Self {
        Self::Socket(value)
    }
}

impl<R> From<Thermometer> for SmartDevice<R> {
    fn from(value: Thermometer) -> Self {
        Self::Thermometer(value)
    }
}

#[derive(Debug)]
pub struct DeviceConvError(pub &'static str);

impl<'a, R> TryFrom<&'a mut SmartDevice<R>> for &'a mut Socket<R> {
    type Error = DeviceConvError;
    fn try_from(value: &'a mut SmartDevice<R>) -> Result<Self, Self::Error> {
        match value {
            SmartDevice::Socket(s) => Ok(s),
            SmartDevice::Thermometer(_) => Err(DeviceConvError(
                "could not convert SmartDevice<Thermometer> to Socket",
            )),
        }
    }
}

impl<'a, R> TryFrom<&'a mut SmartDevice<R>> for &'a mut Thermometer {
    type Error = DeviceConvError;
    fn try_from(value: &'a mut SmartDevice<R>) -> Result<Self, Self::Error> {
        match value {
            SmartDevice::Thermometer(t) => Ok(t),
            SmartDevice::Socket(_) => Err(DeviceConvError(
                "could not convert SmartDevice<Socket> to Thr",
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn thermometer_should_work() {
        let mut instance = Thermometer::new("Farengate");
        let th1 = instance.themperature();
        assert!(th1 == 0_f32); 

        instance.set_temperature(42_f32);
        assert!(instance.temperature == 42_f32);

    }

    #[test]
    fn socket_should_work() {
        let mut s1 = Socket::new("Type A", rand::rng());

        s1.on();
        assert!(s1.is_on);
        let power = s1.power();
        assert!(power > 0.0);

        s1.off();
        assert!(!s1.is_on);
        let power = s1.power();
        assert_eq!(power, 0.0); //bcs it's off
    }
    
}
