use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::{Class, Syncable};
#[allow(unused_imports)]
use crate::primitive::Variant;
use crate::Result;

#[derive(Debug, Clone, PartialEq, NetworkList, NetworkMap, Default)]
pub struct CertManager {
    #[network(rename = "sslKey", variant = "ByteArray")]
    pub ssl_key: String,
    #[network(rename = "sslCert", variant = "ByteArray")]
    pub ssl_cert: String,
}

impl CertManager {
    pub fn set_ssl_cert(&mut self, cert: String) -> Result<()> {
        #[cfg(feature = "server")]
        self.send_sync("setSslCert", vec![Variant::ByteArray(cert.clone())])?;

        self.ssl_cert = cert;

        Ok(())
    }

    pub fn set_ssl_key(&mut self, key: String) -> Result<()> {
        #[cfg(feature = "server")]
        self.send_sync("setSslKey", vec![Variant::ByteArray(key.clone())])?;

        self.ssl_key = key;
        
        Ok(())
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for CertManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "setSslCert" => self.set_ssl_cert(get_param!(msg)),
            "setSslKey" => self.set_ssl_key(get_param!(msg)),
            _ => Ok(()),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for CertManager {}

impl Syncable for CertManager {
    const CLASS: Class = Class::CertManager;
}
