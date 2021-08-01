use reqwest::Response;
use serde_json::Value;
use std::alloc::Global;
use std::str::Bytes;

/// Structure that holds the JSON response of the home API
pub struct HomeData {
    home_data_response: Value,
}

impl HomeData {
    /// Retrieve data from the home API
    pub fn request_home_data() -> HomeData {
        let resp =
            reqwest::blocking::get("https://cd-static.bamgrid.com/dp-117731241344/home.json")
                .unwrap();

        dbg!(&resp);

        HomeData {
            home_data_response: resp.json::<serde_json::Value>().unwrap(),
        }
    }

    /// Retrieve an image from the home API
    pub fn image(&self) -> Vec<u8> {
        let mut resp =
            reqwest::blocking::get("https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/9F9C4A480357CD8D21E2C675B146D40782B92F570660B028AC7FA149E21B88D2/scale?format=jpeg&quality=90&scalingAlgorithm=lanczos3&width=500")
                .unwrap();
        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf);
        buf
    }
}
