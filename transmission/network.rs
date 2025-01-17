use iroh::{
    protocol::Router, 
    Endpoint
};
use iroh_gossip::{
    net::Gossip, 
    ALPN as GOSSIP_ALPN
};
use iroh_blobs::{
    net_protocol::Blobs, 
    util::local_pool::LocalPool, 
    ALPN as BLOBS_ALPN,
    store::mem::Store as BlobStore
};
use iroh_docs::{
    protocol::Docs, AuthorId, ALPN as DOCS_ALPN
};

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

pub enum Network {
    Isolated,
    PeerToPeer(IrohPeerToPeerNetwork)
}

impl Network {
    pub async fn new(implementation: NetworkImplementation) -> Result<Network, TransmissionError> {
        match implementation {
            NetworkImplementation::Isolated => Ok(Network::Isolated),
            NetworkImplementation::PeerToPeer => {
                match IrohPeerToPeerNetwork::new().await {
                    Ok(network) => Ok(Network::PeerToPeer(network)),
                    Err(error) => Err(error)
                }
            }
        }
    }
}

pub struct IrohPeerToPeerNetwork {
    endpoint: Endpoint,
    router: Router,
    gossip: Gossip,
    blobs: Blobs<BlobStore>,
    docs: Docs<BlobStore>,
    author_id: AuthorId
}

impl IrohPeerToPeerNetwork {
    pub async fn new() -> Result<IrohPeerToPeerNetwork, TransmissionError> {
        match Endpoint::builder().discovery_n0().bind().await {
            Ok(endpoint) => {
                let builder = Router::builder(endpoint.clone());
                let local_pool = LocalPool::default();
                let blobs = Blobs::memory().build(local_pool.handle(), builder.endpoint());
                match Gossip::builder().spawn(builder.endpoint().clone()).await {
                    Ok(gossip) => {
                        match Docs::memory().spawn(&blobs, &gossip).await {
                            Ok(docs) => {
                                match builder
                                    .accept(BLOBS_ALPN, blobs.clone())
                                    .accept(GOSSIP_ALPN, gossip.clone())
                                    .accept(DOCS_ALPN, docs.clone()).spawn().await {
                                    Ok(router) => {
                                        match docs.client().authors().create().await {
                                            Ok(author_id) => {
                                                Ok(IrohPeerToPeerNetwork {
                                                    endpoint,
                                                    router,
                                                    gossip,
                                                    blobs,
                                                    docs,
                                                    author_id
                                                })
                                            }
                                            Err(_) => Err(TransmissionError::new("could not create author id"))
                                        }
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
}