pub mod bootstap;
pub mod config;
pub mod db;
pub mod extractor;
pub mod response;

pub mod proto {
    tonic::include_proto!("herax.service");
}
