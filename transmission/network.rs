use anyhow::Error;

use iroh::Endpoint;
use iroh::protocol::Router;
use iroh_gossip::net;
use iroh_gossip::net::Gossip;
use iroh_gossip::net::GOSSIP_ALPN;

pub struct Network {
    send: fn(message: String) -> Result<(), String>,
    receive: fn() -> Option<String>
}

pub async fn initialize() -> Result<Network, Error> {
    match Endpoint::builder().discovery_n0().bind().await {
        Ok(endpoint) => {
            match Gossip::builder().spawn(endpoint.clone()).await {
                Ok(gossip) => {
                    match Router::builder(endpoint.clone()).accept(GOSSIP_ALPN, gossip.clone()).spawn().await {
                        Ok(router) => {
                            Ok(Network {
                                send: |message| {Err(String::from("Unimplemented"))},
                                receive: || {None}
                            })
                        }
                        Err(error) => Err(error)
                    }
                }
                Err(error) => Err(error)
            }
        }
        Err(error) => Err(error)
    }
}