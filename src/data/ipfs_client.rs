use std::fmt;

use ipfs_embed::core::{BitswapStorage, Error, Result};
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkConfig, NetworkService};
use ipfs_embed::Ipfs;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use libipld::Cid;

use async_std::sync::{Arc, Mutex};
use directories_next::ProjectDirs;

use crate::data::content::ContentItemBlock;

pub type IpfsClientRef = Arc<Mutex<IpfsClient>>;

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: Ipfs<DefaultParams, StorageService<DefaultParams>, NetworkService<DefaultParams>>,
    storage: Arc<StorageService<DefaultParams>>,
    network: Arc<NetworkService<DefaultParams>>,
}

impl IpfsClient {
    pub async fn new() -> Result<IpfsClient, Arc<Error>> {
        let sled_config = match ProjectDirs::from("net", "FuzzrNet", "Fuzzr") {
            Some(project_dirs) => {
                sled::Config::new().path(project_dirs.data_local_dir().to_owned())
            }
            None => sled::Config::new().temporary(true),
        };

        let sweep_interval = std::time::Duration::from_millis(10000);
        let cache_size = 1000;

        let storage = Arc::new(StorageService::open(
            &sled_config,
            cache_size,
            Some(sweep_interval),
        )?);

        let bitswap_storage = BitswapStorage::new(storage.clone());

        let net_config = NetworkConfig::new();
        let network = Arc::new(NetworkService::new(net_config, bitswap_storage).await?);

        let ipfs = Ipfs::new(Arc::clone(&storage), Arc::clone(&network));

        Ok(IpfsClient {
            ipfs,
            storage,
            network,
        })
    }

    pub async fn add(&self, block: &ContentItemBlock) -> Result<Cid, Arc<Error>> {
        let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake3_256, block)?;
        self.ipfs.insert(&ipld_block).await?;
        let cid = *ipld_block.cid();

        Ok(cid)
    }

    pub async fn get(&self, cid: &Cid) -> Result<ContentItemBlock, Arc<Error>> {
        let block = self.ipfs.get(cid).await?;
        let content_item = block.decode::<DagCborCodec, ContentItemBlock>()?;

        Ok(content_item)
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}
