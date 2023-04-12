use async_std::io;
use futures::{prelude::*, select};
use libp2p::{
    core::upgrade,
    gossipsub, identity, mdns, noise,
    swarm::NetworkBehaviour,
    swarm::{SwarmBuilder, SwarmEvent, Swarm},
    tcp, yamux, PeerId, Transport,
};
// use libp2p_quic as quic;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use std::collections::HashSet;
use chrono::Utc;
use crate::{block::Block, block::mine_block};
use crate::blockchain::{Blockchain, new_blockchain};

// pub static KEYS: Lazy = Lazy::new(Keypair::generate_ed25519);
// pub static PEER_ID:  Lazy = Lazy::new(|| PeerId::from(KEYS.public()));
// pub static CHAIN_TOPIC: Lazy = Lazy::new(|| Topic::new("chain"));
// pub static BLOCK_TOPIC: Lazy = Lazy::new(|| Topic::new("block"));
// pub static TRANSACTION_TOPIC: Lazy = Lazy::new(|| Topic::new("transaction"));

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::async_io::Behaviour,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub blocks: Vec<Block>,
    pub receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}


#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut new_blockchain: Blockchain = new_blockchain();
    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());
    info!("Local peer id: {local_peer_id}");

    // Set up an encrypted DNS-enabled TCP Transport over the Mplex protocol.
    let transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseAuthenticated::xx(&id_keys).expect("signing libp2p-noise static keypair"),
        )
        .multiplex(yamux::YamuxConfig::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed();
    // let quic_transport = quic::async_std::Transport::new(quic::Config::new(&id_keys));
    // let transport = OrTransport::new(quic_transport, tcp_transport)
    //     .map(|either_output, _| match either_output {
    //         Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
    //         Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
    //     })
    //     .boxed();

    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()
        .expect("Valid config");

    // build a gossipsub network behaviour
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    )
    .expect("Correct configuration");
    // Create a Gossipsub topic
    let chain_topic = gossipsub::IdentTopic::new("chain");
    let block_topic = gossipsub::IdentTopic::new("block");
    let transaction_topic = gossipsub::IdentTopic::new("transaction");
    // subscribes to our topic
    gossipsub.subscribe(&chain_topic)?;
    gossipsub.subscribe(&block_topic)?;
    gossipsub.subscribe(&transaction_topic)?;

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
        let behaviour = MyBehaviour { gossipsub, mdns };
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
    };

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns
    // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                // if let Err(e) = swarm
                // .behaviour_mut()
                // .gossipsub
                // .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
                //     warn!("Publish error: {e:?}");
                // }
                match line.expect("stdin not to close").as_str() {
                    "ls p" => handle_print_peers(&swarm),
                    cmd if cmd.starts_with("ls c") => handle_print_chain(&new_blockchain),
                    cmd if cmd.starts_with("create b") => handle_create_block(&mut new_blockchain),
                    _ => error!("unknown command"),
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        warn!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => info!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}

pub fn get_list_peers(swarm: &Swarm<MyBehaviour>) -> Vec<String> {
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().map(|p| p.to_string()).collect()
}

pub fn handle_print_peers(swarm: &Swarm<MyBehaviour>) {
    let peers = get_list_peers(swarm);
    peers.iter().for_each(|p| info!("{}", p));
}

pub fn handle_print_chain(chain: &Blockchain) {
    info!("Local Blockchain:");
    let pretty_json =
        serde_json::to_string_pretty(&chain.blocks).expect("can jsonify blocks");
    info!("{}", pretty_json);
}

pub fn handle_create_block(chain: &mut Blockchain) {
    let new_block = Block {
            id: u64::try_from(chain.blocks.len()).expect("usize not bigger than u64"),
            timestamp: Utc::now().timestamp(),
            prev_hash: chain.blocks.last().expect("at least one previous block").hash.clone(),
            nonce: 0,
            hash: String::from("0"),
            transactions: vec![]
    };
    let mined_block: Block = mine_block(new_block, chain.difficulty);
    match chain.add_block(mined_block) {
        Result::Ok(_) => info!("Successfully added block!"),
        Result::Err(_) => panic!("Couldn't add block!")
    };
    // if let Some(data) = cmd.strip_prefix("create b") {
    //     let behaviour = swarm.behaviour_mut();
    //     let latest_block = behaviour
    //         .app
    //         .blocks
    //         .last()
    //         .expect("there is at least one block");
    //     let block = Block::new(
    //         latest_block.id + 1,
    //         latest_block.hash.clone(),
    //         data.to_owned(),
    //     );
    //     let json = serde_json::to_string(&block).expect("can jsonify request");
    //     behaviour.app.blocks.push(block);
    //     info!("broadcasting new block");
    //     behaviour
    //         .floodsub
    //         .publish(BLOCK_TOPIC.clone(), json.as_bytes());
    // }
}
