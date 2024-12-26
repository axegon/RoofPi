mod services;
mod utils;
use crate::services::roofpi_service::RoofPiService;

fn main() {
    let mut service = RoofPiService::new();
    service.run();
}
