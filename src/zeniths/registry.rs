use crate::core::traits::Zenith;
use dashmap::DashMap;
use std::sync::Arc;

pub struct ZenithRegistry {
    zeniths: DashMap<String, Arc<dyn Zenith>>,
    extension_map: DashMap<String, String>, // extension -> zenith_name
}

impl Default for ZenithRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ZenithRegistry {
    pub fn new() -> Self {
        Self {
            zeniths: DashMap::new(),
            extension_map: DashMap::new(),
        }
    }

    pub fn register(&self, zenith: Arc<dyn Zenith>) {
        let name = zenith.name().to_string();
        for ext in zenith.extensions() {
            self.extension_map.insert(ext.to_string(), name.clone());
        }
        self.zeniths.insert(name, zenith);
    }

    pub fn get_by_extension(&self, ext: &str) -> Option<Arc<dyn Zenith>> {
        self.extension_map
            .get(ext)
            .and_then(|name| self.zeniths.get(name.value()).map(|z| z.clone()))
    }

    pub fn list_all(&self) -> Vec<Arc<dyn Zenith>> {
        self.zeniths
            .iter()
            .map(|item| item.value().clone())
            .collect()
    }
}
