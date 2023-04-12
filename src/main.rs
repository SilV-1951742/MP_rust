use blockchain::block::encode_hex;
use blockchain::blockchain::{Blockchain, new_blockchain};
use blockchain::p2p_nw;
// use blockchain::transaction::Transaction;
// // use blockchain::merkle_tree;
// use chrono::Utc;
use log::{info, error};
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::nid::Nid;
use openssl::bn::BigNumContext;


fn main() {
    pretty_env_logger::init();

    info!("In main!");

    let mut pub_priv_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for f in 0..5 {
        let private_contents = std::fs::read_to_string(format!("keys/private_key{}.pem", f))
            .expect("Should have been able to read the file");
    
        let public_contents = std::fs::read_to_string(format!("keys/public_key{}.pem", f))
            .expect("Should have been able to read the file");

        let private_key: Vec<u8> = EcKey::private_key_from_pem(private_contents.as_bytes()).unwrap().private_key().to_vec();
        let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
        let mut ctx = BigNumContext::new().unwrap();
        let conversion = PointConversionForm::COMPRESSED;
        let pub_key: Vec<u8> = EcKey::public_key_from_pem(public_contents.as_bytes()).unwrap().public_key().to_bytes(&group, conversion, &mut ctx).unwrap();

        println!("Public key {}", encode_hex(pub_key.clone()));
        println!("Private key {}", encode_hex(private_key.clone()));
    
        let mut hasher = Sha256::new();
        hasher.update("Hello, world!".as_bytes());
        let mbytes: Vec<u8> =  hasher.finalize().as_slice().to_owned();

        let secp = Secp256k1::new();
        let secret_key: SecretKey = SecretKey::from_slice(&private_key).expect("32 bytes, within curve order");
        let public_key: PublicKey = PublicKey::from_slice(&pub_key).expect("33 or 65 bytes, within curve order");
        // let public_key: PublicKey = PublicKey::from_secret_key(&secp, &secret_key);

        let message: Message = Message::from_slice(mbytes.as_slice()).expect("hash convertible to message.");
        let sig = secp.sign_ecdsa(&Message::from(message), &secret_key);
        match secp.verify_ecdsa(&message, &sig, &public_key) {
            Ok(_) => {
                info!("Verified message!");
            },
            Err(_) => {
                error!("Couldn't verify message!");
            }
        }

        pub_priv_map.insert(encode_hex(pub_key.clone()), encode_hex(private_key.clone()));
    }

    println!("Public key -- Private key");
    for (key, val) in &pub_priv_map {
        println!("{} -- {}", key, val);
    }

    let new_blockchain: Blockchain = new_blockchain();

    match new_blockchain.validate_chain() {
        Result::Ok(l) => info!("Chain is ok! Checked {} blocks", l),
        Result::Err(_) => panic!("Invalid chain!")
    };

    assert_eq!(1, new_blockchain.blocks.len());

    println!("{:?}", new_blockchain);

    let _ = p2p_nw::main();
}
