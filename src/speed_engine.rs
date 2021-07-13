use crate::errors::SpeedTestError;
use serde::Serialize;
use speedtest_rs::speedtest;

pub struct SpeedEngine {}

use log::info;

#[derive(Serialize, Default, Debug)]
pub struct SpeedResult {
    pub download_band_width: u32,
    pub upload_band_width: u32,
}

impl SpeedEngine {
    pub fn test(&self) -> Result<SpeedResult, SpeedTestError> {
        info!("searching server");
        let mut config = speedtest::get_configuration()?;
        let server_list = speedtest::get_server_list_with_config(&config)?;
        let mut server_list_sorted = server_list.servers_sorted_by_distance(&config);
        server_list_sorted.truncate(3);

        let latency_test_result =
            speedtest::get_best_server_based_on_latency(&server_list_sorted[..])?;

        info!(
            "Choose server hosted by {} ({}){}: {}.{} ms",
            latency_test_result.server.sponsor,
            latency_test_result.server.name,
            latency_test_result
                .server
                .distance
                .map_or("".to_string(), |d| format!(" [{:.2} km]", d)),
            latency_test_result.latency.as_millis(),
            latency_test_result.latency.as_micros() % 1000,
        );

        let best_server = latency_test_result.server;

        fn no_print() {}

        info!("starting test");

        let inner_download_measurement =
            speedtest::test_download_with_progress_and_config(best_server, no_print, &mut config)?;

        let inner_upload_measurement =
            speedtest::test_upload_with_progress_and_config(best_server, no_print, &config)?;

        let result = SpeedResult {
            download_band_width: inner_download_measurement.kbps(),
            upload_band_width: inner_upload_measurement.kbps(),
        };

        Result::Ok(result)
    }
}
