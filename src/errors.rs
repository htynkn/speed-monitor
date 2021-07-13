#[derive(Debug)]
pub enum SpeedTestError {
    Unknown(),
    SpeedTest(speedtest_rs::error::Error),
}

impl From<speedtest_rs::error::Error> for SpeedTestError {
    fn from(err: speedtest_rs::error::Error) -> SpeedTestError {
        SpeedTestError::SpeedTest(err)
    }
}
