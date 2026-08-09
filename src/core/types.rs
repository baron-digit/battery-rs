#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Voltage(f32);

impl Voltage {
    pub const fn from_volts(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_volts(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Current(f32);

impl Current {
    pub const fn from_amperes(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_amperes(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f32);

impl Temperature {
    pub const fn from_celsius(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_celsius(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Capacity(f32);

impl Capacity {
    pub fn from_ampere_hours(value: f32) -> Option<Self> {
        (value > 0.0).then_some(Self(value))
    }

    pub const fn as_ampere_hours(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Charge(f32);

impl Charge {
    pub const fn from_ampere_hours(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_ampere_hours(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Resistance(f32);

impl Resistance {
    pub const fn from_ohms(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_ohms(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Power(f32);

impl Power {
    pub const fn from_watts(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_watts(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy(f32);

impl Energy {
    pub const fn from_watt_hours(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_watt_hours(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StateOfCharge(f32);

impl StateOfCharge {
    pub fn from_fraction(value: f32) -> Option<Self> {
        (0.0..=1.0).contains(&value).then_some(Self(value))
    }

    pub const fn as_fraction(self) -> f32 {
        self.0
    }

    pub const fn as_percent(self) -> f32 {
        self.0 * 100.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StateOfHealth(f32);

impl StateOfHealth {
    pub fn from_fraction(value: f32) -> Option<Self> {
        (0.0..=1.0).contains(&value).then_some(Self(value))
    }

    pub const fn as_fraction(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Timestamp {
    pub const fn from_micros(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_micros(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeDelta(f32);

impl TimeDelta {
    pub const fn from_seconds(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_seconds(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellIndex(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleIndex(pub u16);
