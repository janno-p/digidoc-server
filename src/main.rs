use digidoc_server::configuration::get_configuration;
use digidoc_server::digidoc::LibDigiDoc;
use digidoc_server::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let _digi_doc = LibDigiDoc::load();

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
