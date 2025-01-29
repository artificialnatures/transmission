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
    ALPN as BLOBS_ALPN
};
use iroh_docs::{
    protocol::Docs,
    AuthorId,
    DocTicket,
    ALPN as DOCS_ALPN
};
use iroh_docs::rpc::client::docs::{
    Entry, 
    LiveEvent, 
    ShareMode
};
use quic_rpc::transport::flume::FlumeConnector;
pub(crate) type BlobsClient = iroh_blobs::rpc::client::blobs::Client<
    FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>,
>;
pub(crate) type DocsClient = iroh_docs::rpc::client::docs::Client<
    FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>,
>;
pub(crate) type IrohDocument = iroh_docs::rpc::client::docs::Doc<
    FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>,
>;
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

    pub async fn generate_invite(self) -> Result<String, TransmissionError> {
        match self {
            Self::Isolated => Err(TransmissionError::new("no invite available")),
            Self::PeerToPeer(iroh_network) => {
                match iroh_network.local_document.share(ShareMode::Write, Default::default()).await {
                    Ok(ticket) => Ok(ticket.to_string()),
                    Err(error) => Err(TransmissionError::new(&error.to_string()))
                }
            }
        }
    }

    pub async fn disconnect(self) -> () {
        match self {
            Self::Isolated => (),
            Self::PeerToPeer(iroh_network) => {
                let _ = iroh_network.router.shutdown().await;
                ()
            }
        }
    }
}

pub struct IrohPeerToPeerNetwork {
    endpoint: Endpoint,
    router: Router,
    gossip: Gossip,
    blobs_client: BlobsClient,
    docs_client: DocsClient,
    author_id: AuthorId,
    local_document: IrohDocument
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
                                        let blobs_client = blobs.client().clone();
                                        let docs_client = docs.client().clone();
                                        match docs_client.authors().create().await {
                                            Ok(author_id) => {
                                                match docs_client.create().await {
                                                    Ok(local_document) => {
                                                        Ok(IrohPeerToPeerNetwork {
                                                            endpoint,
                                                            router,
                                                            gossip,
                                                            blobs_client,
                                                            docs_client,
                                                            author_id,
                                                            local_document
                                                        })
                                                    }
                                                    Err(_) => Err(TransmissionError::new("could not create document"))
                                                }
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