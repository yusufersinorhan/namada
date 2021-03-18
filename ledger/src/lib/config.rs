//! Node and client configuration settings

use std::path::PathBuf;

use config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Node {
    home: PathBuf,
    tendermint_path: PathBuf,
    db_path: PathBuf,
    libp2p_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Tendermint {
    pub host: String,
    pub port: String,
    pub network: String,
}
#[derive(Debug, Deserialize)]
pub struct Gossip {
    pub host: String,
    pub port: String,
    pub peers: Vec<String>,
    pub topics: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub node: Node,
    pub tendermint: Tendermint,
    pub p2p: Gossip,
}

impl Config {
    pub fn new(home: String) -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();

        s.set_default("node.home", home.to_string())?;
        s.set_default("node.db_path", "db")?;
        s.set_default("node.libp2p_path", "libp2p")?;
        s.set_default("node.tendermint_path", "tendermint")?;

        s.set_default("tendermint.host", "127.0.0.1")?;
        s.set_default("tendermint.port", 26658)?;
        s.set_default("tendermint.network", "mainnet")?;

        s.set_default("p2p.host", "127.0.0.1")?;
        s.set_default("p2p.port", 20201)?;
        s.set_default("p2p.peers", Vec::<String>::new())?;
        s.set_default("p2p.topics", Vec::<String>::new())?;

        s.merge(
            config::File::with_name(&format!("{}/{}", home, "settings.toml"))
                .required(false),
        )?;

        s.try_into()
    }

    pub fn tendermint_home_dir(&self) -> PathBuf {
        self.node.home.join(self.node.tendermint_path.clone())
    }

    pub fn gossip_home_dir(&self) -> PathBuf {
        self.node.home.join(self.node.libp2p_path.clone())
    }

    pub fn db_home_dir(&self) -> PathBuf {
        self.node.home.join(self.node.db_path.clone())
    }
}