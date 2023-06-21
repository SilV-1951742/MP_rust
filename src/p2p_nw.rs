// use async_std::io;
// use futures::{prelude::*, select};
// use libp2p::{
//     core::upgrade,
//     gossipsub, identity,
//     mdns,
//     noise,
//     swarm::NetworkBehaviour,
//     swarm::{SwarmBuilder, SwarmEvent, Swarm},
//     tcp,
//     yamux, PeerId, Transport,
// };

// use async_std::io;
// use async_trait::async_trait;
// use either::Either;
// use futures::channel::{mpsc, oneshot};
use futures::prelude::*;
// use std::collections::{hash_map, HashMap, HashSet};
// use std::iter;
use log::info;
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmEvent, SwarmBuilder};
use libp2p::{identity, PeerId, ping, Multiaddr};
use std::error::Error;


pub async fn run() -> Result<(), Box<dyn Error>> {
    let local_key: identity::Keypair = identity::Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_key.public());
    info!("Local peer id: {local_peer_id:?}");

    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = PingBehaviour::default();
    
    let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id)
        .build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        info!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}

/// Our network behaviour.
/// pings can be observed.
#[derive(NetworkBehaviour, Default)]
struct PingBehaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
