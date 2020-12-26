/// The Gyroscope operating mode which determines power consumption, speed, etc...
#[allow(dead_code)]
#[derive(Debug)]
pub enum GyroscopePowerMode {
    LowPower,
    NormalPower,
    HighPerformance,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum GyroscopeScale {
    Scale125Dps,
    Scale250Dps,
    Scale500Dps,
    Scale1000Dps,
    Scale2000Dps
}
