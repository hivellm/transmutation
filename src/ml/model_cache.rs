use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

/// ONNX model session caching for performance optimization
///
/// This module provides lazy loading and caching of ONNX Runtime sessions
/// to avoid reloading models for every conversion.
//
use crate::ml::{LayoutModel, TableStructureModel};

/// Global model cache (lazy initialized)
static MODEL_CACHE: Lazy<Mutex<ModelCache>> = Lazy::new(|| Mutex::new(ModelCache::new()));

/// Cache for loaded ML models
pub struct ModelCache {
    layout_model: Option<Arc<Mutex<LayoutModel>>>,
    table_model: Option<Arc<Mutex<TableStructureModel>>>,
    layout_model_path: Option<PathBuf>,
    table_model_path: Option<PathBuf>,
}

impl ModelCache {
    /// Create new empty cache
    fn new() -> Self {
        Self {
            layout_model: None,
            table_model: None,
            layout_model_path: None,
            table_model_path: None,
        }
    }

    /// Get or load layout model
    ///
    /// Uses cached model if available, otherwise loads from disk.
    /// Returns None if model file doesn't exist (graceful fallback).
    fn get_or_load_layout_model(&mut self, model_path: PathBuf) -> Option<Arc<Mutex<LayoutModel>>> {
        // Check if already cached and path matches
        if let Some(ref cached_path) = self.layout_model_path {
            if *cached_path == model_path {
                if let Some(ref model) = self.layout_model {
                    eprintln!("📦 Using cached LayoutModel");
                    return Some(Arc::clone(model));
                }
            }
        }

        // Load new model
        eprintln!("🔄 Loading LayoutModel from {}", model_path.display());
        match LayoutModel::new(&model_path) {
            Ok(model) => {
                let arc_model = Arc::new(Mutex::new(model));
                self.layout_model = Some(Arc::clone(&arc_model));
                self.layout_model_path = Some(model_path);
                eprintln!("✅ LayoutModel loaded and cached");
                Some(arc_model)
            }
            Err(e) => {
                eprintln!("❌ Failed to load LayoutModel: {}", e);
                None
            }
        }
    }

    /// Get or load table structure model
    fn get_or_load_table_model(
        &mut self,
        model_path: PathBuf,
    ) -> Option<Arc<Mutex<TableStructureModel>>> {
        // Check if already cached and path matches
        if let Some(ref cached_path) = self.table_model_path {
            if *cached_path == model_path {
                if let Some(ref model) = self.table_model {
                    eprintln!("📦 Using cached TableStructureModel");
                    return Some(Arc::clone(model));
                }
            }
        }

        // Load new model
        eprintln!(
            "🔄 Loading TableStructureModel from {}",
            model_path.display()
        );
        match TableStructureModel::new(&model_path, 1.0) {
            Ok(model) => {
                let arc_model = Arc::new(Mutex::new(model));
                self.table_model = Some(Arc::clone(&arc_model));
                self.table_model_path = Some(model_path);
                eprintln!("✅ TableStructureModel loaded and cached");
                Some(arc_model)
            }
            Err(e) => {
                eprintln!("❌ Failed to load TableStructureModel: {}", e);
                None
            }
        }
    }

    /// Clear all cached models (free memory)
    fn clear(&mut self) {
        self.layout_model = None;
        self.table_model = None;
        self.layout_model_path = None;
        self.table_model_path = None;
        eprintln!("🗑️  Model cache cleared");
    }
}

/// Get cached layout model or load from path
pub fn get_layout_model(model_path: PathBuf) -> Option<Arc<Mutex<LayoutModel>>> {
    MODEL_CACHE
        .lock()
        .ok()?
        .get_or_load_layout_model(model_path)
}

/// Get cached table structure model or load from path
pub fn get_table_model(model_path: PathBuf) -> Option<Arc<Mutex<TableStructureModel>>> {
    MODEL_CACHE.lock().ok()?.get_or_load_table_model(model_path)
}

/// Clear all cached models (useful for testing or memory management)
pub fn clear_model_cache() {
    if let Ok(mut cache) = MODEL_CACHE.lock() {
        cache.clear();
    }
}

/// Check if layout model is currently cached
pub fn has_cached_layout_model() -> bool {
    MODEL_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.layout_model.as_ref().map(|_| true))
        .unwrap_or(false)
}

/// Check if table model is currently cached
pub fn has_cached_table_model() -> bool {
    MODEL_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.table_model.as_ref().map(|_| true))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache = ModelCache::new();
        assert!(cache.layout_model.is_none());
        assert!(cache.table_model.is_none());
    }

    #[test]
    fn test_cache_clear() {
        clear_model_cache();
        assert!(!has_cached_layout_model());
        assert!(!has_cached_table_model());
    }
}
