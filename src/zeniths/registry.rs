// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::core::traits::Zenith;
use dashmap::DashMap;
use std::sync::Arc;

pub struct ZenithRegistry {
    zeniths: DashMap<String, Arc<dyn Zenith>>,
    extension_map: DashMap<String, Vec<(i32, String, usize)>>, // extension -> Vec<(priority, zenith_name, order)>
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
        let priority = zenith.priority();
        for ext in zenith.extensions() {
            self.extension_map
                .entry(ext.to_string())
                .and_modify(|entries: &mut Vec<(i32, String, usize)>| {
                    entries.retain(|(p, n, _)| !(p == &priority && n != &name));
                    entries.push((priority, name.clone(), entries.len()));
                    entries.sort_by_key(|(p, _, idx)| (std::cmp::Reverse(*p), *idx));
                })
                .or_insert_with(|| vec![(priority, name.clone(), 0)]);
        }
        self.zeniths.insert(name, zenith);
    }

    pub fn get_by_extension(&self, ext: &str) -> Option<Arc<dyn Zenith>> {
        self.extension_map
            .get(ext)
            .and_then(|entries| entries.first().map(|(_, n, _)| n.clone()))
            .and_then(|name| self.zeniths.get(&name).map(|z| z.clone()))
    }

    pub fn list_all(&self) -> Vec<Arc<dyn Zenith>> {
        self.zeniths
            .iter()
            .map(|item| item.value().clone())
            .collect()
    }
}
