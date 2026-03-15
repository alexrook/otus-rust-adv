use std::fmt;

use rand::Rng;

pub trait WithModel {
    fn get_model(&self) -> &str;
}

//Термометр
#[derive(Debug, Clone)]
pub struct Thermometer<R> {
    model: String,
    rnd: R,
}

impl<R> WithModel for Thermometer<R> {
    fn get_model(&self) -> &str {
        &self.model
    }
}

impl<R> fmt::Display for Thermometer<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer[model:{}]", self.get_model())
    }
}

impl<R> Thermometer<R> {
    pub fn new<S>(model: S, rnd: R) -> Self
    where
        S: Into<String>,
    {
        Thermometer {
            model: model.into(),
            rnd,
        }
    }

    pub fn themperature(&mut self) -> f32
    where
        R: Rng,
    {
        self.rnd.random::<f32>() * 100_f32
    }
}

//Розетка
#[derive(Debug, Clone)]
pub struct Socket {
    model: String,
    pub is_on: bool,
    pow: f32,
}

impl WithModel for Socket {
    fn get_model(&self) -> &str {
        &self.model
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Socket[model:{},pow:{} is_on:{}]",
            self.get_model(),
            self.pow,
            self.is_on,
        )
    }
}

impl Socket {
    pub fn new<S>(model: S, pow: f32) -> Result<Self, &'static str>
    where
        S: Into<String>,
    {
        if pow > 0_f32 {
            Ok(Socket {
                model: model.into(),
                is_on: false,
                pow,
            })
        } else {
            Err("The Socket power should be >0")
        }
    }

    pub fn on(&mut self) {
        self.is_on = true
    }

    pub fn off(&mut self) {
        self.is_on = false
    }

    pub fn get_pow(&self) -> f32 {
        self.pow
    }
}

#[derive(Debug)]
pub enum SmartDevice<R> {
    Thermometer(Thermometer<R>),
    Socket(Socket),
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

impl<R> From<Socket> for SmartDevice<R> {
    fn from(value: Socket) -> Self {
        Self::Socket(value)
    }
}

impl<R> From<Thermometer<R>> for SmartDevice<R> {
    fn from(value: Thermometer<R>) -> Self {
        Self::Thermometer(value)
    }
}

#[derive(Debug)]
pub struct DeviceConvError(pub &'static str);

impl<'a, R> TryFrom<&'a mut SmartDevice<R>> for &'a mut Socket {
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

impl<'a, R> TryFrom<&'a mut SmartDevice<R>> for &'a mut Thermometer<R> {
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
        let mut t1 = Thermometer::new("Farengate", rand::rng());
        let th1 = t1.themperature();
        assert!(size_of_val(&th1) > 0); //just in case
    }

    #[test]
    fn socket_should_work() {
        let mut s1 = Socket::new("Type A", 1.2_f32).unwrap();

        s1.on();
        assert!(s1.is_on);

        s1.off();
        assert!(!s1.is_on);
    }
}
