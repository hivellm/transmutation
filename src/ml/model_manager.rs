/// Model download and caching management
use crate::error::{Result, TransmutationError};
use std::path::{Path, PathBuf};
use std::fs;
use std::env;

/// Known model names
pub const LAYOUT_MODEL_NAME: &str = "layout_model.onnx";
pub const TABLE_STRUCTURE_MODEL_NAME: &str = "table_structure_model.onnx";

/// Manages ML model downloads and caching
pub struct ModelManager {
    cache_dir: PathBuf,
    /// Search paths for models (in priority order)
    search_paths: Vec<PathBuf>,
}

impl ModelManager {
    /// Create new model manager with default cache directory
    pub fn new() -> Result<Self> {
        let cache_dir = Self::default_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;
        
        // Build search paths in priority order
        let search_paths = Self::build_search_paths()?;
        
        Ok(Self { cache_dir, search_paths })
    }
    
    /// Get default cache directory: ~/.cache/transmutation_models/
    fn default_cache_dir() -> Result<PathBuf> {
        if let Some(cache_dir) = dirs::cache_dir() {
            return Ok(cache_dir.join("transmutation_models"));
        }
        
        // Fallback to home directory
        let home = dirs::home_dir()
            .ok_or_else(|| TransmutationError::IoError(
                std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
            ))?;
        
        Ok(home.join(".cache").join("transmutation_models"))
    }
    
    /// Build list of paths to search for models
    fn build_search_paths() -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        
        // 1. Environment variable (highest priority)
        if let Ok(env_path) = env::var("TRANSMUTATION_MODELS_DIR") {
            paths.push(PathBuf::from(env_path));
        }
        
        // 2. Project models/ directory (for development)
        if let Ok(current_dir) = env::current_dir() {
            paths.push(current_dir.join("models"));
            paths.push(current_dir.join("transmutation").join("models"));
        }
        
        // 3. Executable directory (for deployment)
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                paths.push(exe_dir.join("models"));
            }
        }
        
        // 4. System cache (lowest priority)
        paths.push(Self::default_cache_dir()?);
        
        Ok(paths)
    }
    
    /// Load model or download if not available
    /// Returns None if model is not available (graceful fallback)
    pub fn load_or_download(&self, model_name: &str) -> Option<PathBuf> {
        // Try all search paths
        for search_path in &self.search_paths {
            let model_path = search_path.join(model_name);
            if model_path.exists() {
                eprintln!("✅ Found {} at {}", model_name, model_path.display());
                return Some(model_path);
            }
        }
        
        eprintln!("⚠️  Model {} not found in any search path", model_name);
        eprintln!("   Searched:");
        for path in &self.search_paths {
            eprintln!("     - {}", path.display());
        }
        eprintln!("   To export models, run: python scripts/export_onnx_models.py");
        
        None
    }
    
    /// Get path for a specific model (legacy method)
    pub fn get_model_path(&self, model_name: &str) -> PathBuf {
        self.cache_dir.join(model_name)
    }
    
    /// Check if model exists in any search path
    pub fn has_model(&self, model_name: &str) -> bool {
        self.search_paths.iter().any(|path| path.join(model_name).exists())
    }
    
    /// Check if layout model is available
    pub fn has_layout_model(&self) -> bool {
        self.has_model(LAYOUT_MODEL_NAME)
    }
    
    /// Check if table structure model is available
    pub fn has_table_model(&self) -> bool {
        self.has_model(TABLE_STRUCTURE_MODEL_NAME)
    }
    
    /// Get paths for all required models
    /// Returns None if any required model is missing
    pub fn get_all_models(&self) -> Option<ModelPaths> {
        let layout_model = self.load_or_download(LAYOUT_MODEL_NAME)?;
        let table_model = self.load_or_download(TABLE_STRUCTURE_MODEL_NAME);
        
        Some(ModelPaths {
            layout_model,
            table_model,
        })
    }
    
    /// Download model from HuggingFace (placeholder for future implementation)
    pub async fn download_model(&self, _model_name: &str, _repo_id: &str) -> Result<PathBuf> {
        // TODO: Implement actual download from HuggingFace
        // For now, return error indicating manual download needed
        Err(TransmutationError::UnsupportedFormat(
            "Automatic model download not yet implemented. Please manually place ONNX models in models/ directory".to_string()
        ))
    }
    
    /// Verify model checksum (placeholder)
    pub fn verify_model(&self, _model_path: &Path) -> Result<()> {
        // TODO: Implement checksum verification
        Ok(())
    }
}

/// Paths to all ML models
pub struct ModelPaths {
    pub layout_model: PathBuf,
    pub table_model: Option<PathBuf>,
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ModelManager")
    }
}

