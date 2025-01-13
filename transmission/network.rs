use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{net_protocol::Blobs, util::local_pool::LocalPool, ALPN as BLOBS_ALPN};
use iroh_docs::{protocol::Docs, ALPN as DOCS_ALPN};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};

use crate::errors::TransmissionError;

#[derive(Debug, Clone)]
pub enum NetworkImplementation {
    Isolated,
    PeerToPeer
}

impl From<String> for NetworkImplementation {
    fn from(implementation_string: String) -> NetworkImplementation {
        match implementation_string.as_str() {
            "default" => NetworkImplementation::Isolated,
            "p2p" => NetworkImplementation::PeerToPeer,
            _ => NetworkImplementation::Isolated
        }
    }
}

pub struct Network {
    pub initialize: fn() -> (),
    pub start: fn() -> (),
    pub shutdown: fn() -> ()
}

pub struct IsolatedNetwork {}

impl IsolatedNetwork {
    pub fn new() -> IsolatedNetwork {
        IsolatedNetwork {}
    }
}

pub struct IrohPeerToPeerNetwork {
    endpoint: Endpoint,
    router: Router
}

pub async fn build_network() -> Result<IrohPeerToPeerNetwork, TransmissionError> {
    // create an iroh endpoint that includes the standard discovery mechanisms
    // we've built at number0
    match Endpoint::builder().discovery_n0().bind().await {
        Ok(endpoint) => {
            let builder = Router::builder(endpoint.clone());
            // build the blobs protocol
            let local_pool = LocalPool::default();
            let blobs = Blobs::memory().build(local_pool.handle(), builder.endpoint());
            // build the gossip protocol
            match Gossip::builder().spawn(builder.endpoint().clone()).await {
                Ok(gossip) => {
                    // build the docs protocol
                    match Docs::memory().spawn(&blobs, &gossip).await {
                        Ok(docs) => {
                            //set up router
                            match builder.accept(BLOBS_ALPN, blobs).accept(GOSSIP_ALPN, gossip).accept(DOCS_ALPN, docs).spawn().await {
                                Ok(router) => {
                                    Ok(IrohPeerToPeerNetwork {
                                        endpoint,
                                        router
                                    })
                                }
                                Err(_) => Err(TransmissionError::new("could not build router"))
                            }
                        }
                        Err(_) => Err(TransmissionError::new("could not build docs protocol"))
                    }
                }
                Err(_) => Err(TransmissionError::new("could not build gossip protocol"))
            }
        }
        Err(_) => Err(TransmissionError::new("could not bind endpoint"))
    }
}