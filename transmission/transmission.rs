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
    pub fn new(config: TransmissionConfiguration) -> Transmission {
        let mut renderer =
            match config.renderer_implementation {
                RendererImplementation::Default | RendererImplementation::Cli => {
                    Renderer {
                        initialize: || {},
                        start: || {}
                    }
                },
                RendererImplementation::Bevy => {
                    Renderer {
                        initialize: || {},
                        start: || {}
                    }
                }
            };
        let network =
            match config.network_implementation {
                NetworkImplementation::Isolated => {
                    Network {
                        initialize: || {},
                        start: || {},
                        shutdown: || {}
                    }
                },
                NetworkImplementation::PeerToPeer => {
                    Network {
                        initialize: || {},
                        start: || {},
                        shutdown: || {}
                    }
                }
            };
        Transmission {
            renderer,
            network
        }
    }
}