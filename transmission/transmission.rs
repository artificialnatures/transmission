use crate::errors::TransmissionError;
use crate::network::{Network, NetworkImplementation};
use crate::renderer::{Renderer, RendererImplementation};

pub struct TransmissionConfiguration {
    pub renderer_implementation: RendererImplementation,
    pub network_implementation: NetworkImplementation
}

pub struct Transmission {
    pub renderer: Renderer,
    pub network: Network
}

impl Transmission {
    pub async fn new(config: TransmissionConfiguration) -> Result<Transmission, TransmissionError> {
        let renderer = Renderer::new(config.renderer_implementation);
        match Network::new(config.network_implementation).await {
            Ok(network) => {
                Ok(
                    Transmission {
                        renderer,
                        network
                    }
                )
            }
            Err(error) => Err(error)
        }
    }
}