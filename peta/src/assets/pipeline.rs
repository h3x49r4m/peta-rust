//! Asset processing pipeline for component-based themes

use crate::core::{Error, Result};
use crate::components::ComponentRegistry;
use crate::assets::{CssGenerator, CssConfig, JsGenerator, JsConfig};
use crate::content::rst::{MathCssGenerator, MathJsGenerator};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Asset processing pipeline
pub struct AssetPipeline {
    /// Theme name
    #[allow(dead_code)]
    theme_name: String,
    /// Theme directory
    theme_dir: PathBuf,
    /// Output directory
    output_dir: PathBuf,
    /// Component registry
    component_registry: ComponentRegistry,
    /// Configuration
    config: AssetConfig,
    /// Asset cache
    #[allow(dead_code)]
    asset_cache: HashMap<String, ProcessedAsset>,
}

/// Asset processing configuration
#[derive(Debug, Clone)]
pub struct AssetConfig {
    /// Minify CSS
    pub minify_css: bool,
    /// Minify JS
    pub minify_js: bool,
    /// Optimize images
    pub optimize_images: bool,
    /// Image quality (0-100)
    pub image_quality: u8,
    /// Generate source maps
    pub generate_sourcemaps: bool,
    /// Bundle assets
    pub bundle_assets: bool,
    /// Cache busting
    pub cache_busting: bool,
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            minify_css: true,
            minify_js: true,
            optimize_images: true,
            image_quality: 85,
            generate_sourcemaps: false,
            bundle_assets: true,
            cache_busting: true,
        }
    }
}

/// Processed asset
#[derive(Debug, Clone)]
pub struct ProcessedAsset {
    /// Asset content
    pub content: Vec<u8>,
    /// Content type
    pub content_type: String,
    /// File name
    pub file_name: String,
    /// Source map (if generated)
    pub source_map: Option<String>,
    /// Cache busting hash
    pub cache_bust: Option<String>,
}

impl AssetPipeline {
    /// Create a new asset pipeline
    pub fn new(theme_name: &str, output_dir: &Path) -> Self {
        let theme_dir = Path::new("themes").join(theme_name);
        
        Self {
            theme_name: theme_name.to_string(),
            theme_dir: theme_dir.clone(),
            output_dir: output_dir.to_path_buf(),
            component_registry: ComponentRegistry::new(),
            config: AssetConfig::default(),
            asset_cache: HashMap::new(),
        }
    }
    
    /// Create asset pipeline with custom configuration
    pub fn with_config(theme_name: &str, output_dir: &Path, config: AssetConfig) -> Self {
        let mut pipeline = Self::new(theme_name, output_dir);
        pipeline.config = config;
        pipeline
    }
    
    /// Set component registry
    pub fn set_component_registry(&mut self, registry: ComponentRegistry) {
        self.component_registry = registry;
    }
    
    /// Process all theme assets
    pub fn process_assets(&mut self) -> Result<()> {
        // Create output directories
        self.create_output_directories()?;

        // Generate code block assets (from Rust)
        self.generate_code_block_assets()?;

        // Generate math formula assets (from Rust)
        self.generate_math_assets()?;

        // Generate embedded snippet card assets (from Rust)
        self.generate_embedded_snippet_card_assets()?;

        // Process component assets
        self.process_component_assets()?;

        // Process theme assets
        self.process_theme_assets()?;

        // Generate asset bundles
        if self.config.bundle_assets {
            self.generate_asset_bundles()?;
        }

        // Generate asset manifest
        self.generate_asset_manifest()?;

        Ok(())
    }

    /// Generate code block CSS and JS from Rust generators
    fn generate_code_block_assets(&mut self) -> Result<()> {
        // Generate code block CSS
        let css_config = CssConfig::default();
        let css_generator = CssGenerator::with_config(css_config);
        let css_content = css_generator.generate()?;

        // Write directly to output_dir/css/ (not output_dir/assets/css/)
        let css_output_path = self.output_dir.join("css").join("code-blocks.css");
        fs::create_dir_all(css_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create CSS directory: {}", e)))?;
        fs::write(&css_output_path, css_content)
            .map_err(|e| Error::asset(format!("Failed to write code-blocks.css: {}", e)))?;

        // Generate code block JS
        let js_config = JsConfig::default();
        let js_generator = JsGenerator::with_config(js_config);
        let js_content = js_generator.generate()?;

        // Write directly to output_dir/js/ (not output_dir/assets/js/)
        let js_output_path = self.output_dir.join("js").join("code-blocks.js");
        fs::create_dir_all(js_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create JS directory: {}", e)))?;
        fs::write(&js_output_path, js_content)
            .map_err(|e| Error::asset(format!("Failed to write code-blocks.js: {}", e)))?;

        Ok(())
    }

    /// Generate math formula CSS and JS from Rust generators
    fn generate_math_assets(&mut self) -> Result<()> {
        // Generate math formula CSS
        let math_css_generator = MathCssGenerator::new()?;
        let math_css_content = math_css_generator.generate()?;

        let math_css_output_path = self.output_dir.join("css").join("math-formulas.css");
        fs::create_dir_all(math_css_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create math CSS directory: {}", e)))?;
        fs::write(&math_css_output_path, math_css_content)
            .map_err(|e| Error::asset(format!("Failed to write math-formulas.css: {}", e)))?;

        // Generate math formula JS
        let math_js_generator = MathJsGenerator::new()?;
        let math_js_content = math_js_generator.generate()?;

        let math_js_output_path = self.output_dir.join("js").join("math-formulas.js");
        fs::create_dir_all(math_js_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create math JS directory: {}", e)))?;
        fs::write(&math_js_output_path, math_js_content)
            .map_err(|e| Error::asset(format!("Failed to write math-formulas.js: {}", e)))?;

        Ok(())
    }

    /// Generate embedded snippet card CSS and JS from Rust generators
    fn generate_embedded_snippet_card_assets(&mut self) -> Result<()> {
        // Generate embedded snippet card CSS
        let css_generator = crate::assets::css_generator::EmbeddedSnippetCardCssGenerator::new()?;
        let css_content = css_generator.generate()?;

        let css_output_path = self.output_dir.join("css").join("embedded-snippet-cards.css");
        fs::create_dir_all(css_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create embedded snippet card CSS directory: {}", e)))?;
        fs::write(&css_output_path, css_content)
            .map_err(|e| Error::asset(format!("Failed to write embedded-snippet-cards.css: {}", e)))?;

        // Generate embedded snippet card JS
        let js_generator = crate::assets::js_generator::EmbeddedSnippetCardJsGenerator::new()?;
        let js_content = js_generator.generate()?;

        let js_output_path = self.output_dir.join("js").join("embedded-snippet-cards.js");
        fs::create_dir_all(js_output_path.parent().unwrap())
            .map_err(|e| Error::asset(format!("Failed to create embedded snippet card JS directory: {}", e)))?;
        fs::write(&js_output_path, js_content)
            .map_err(|e| Error::asset(format!("Failed to write embedded-snippet-cards.js: {}", e)))?;

        Ok(())
    }
    
    /// Create output directories
    fn create_output_directories(&self) -> Result<()> {
        let dirs = [
            "css",
            "js",
            "images",
            "fonts",
            "contexts",
            "components",
        ];
        
        for dir in &dirs {
            let output_path = self.output_dir.join("assets").join(dir);
            fs::create_dir_all(&output_path)
                .map_err(|e| Error::asset(format!("Failed to create directory {}: {}", output_path.display(), e)))?;
        }
        
        Ok(())
    }
    
    /// Process component assets
    fn process_component_assets(&mut self) -> Result<()> {
        let enabled_components: Vec<_> = self.component_registry.get_enabled_components().into_iter().cloned().collect();
        
        for component in enabled_components {
            self.process_single_component_assets(&component)?;
        }
        
        Ok(())
    }
    
    /// Process assets for a single component
    fn process_single_component_assets(&mut self, component: &crate::components::Component) -> Result<()> {
        let component_dir = self.theme_dir.join("components").join(&component.name);
        
        if !component_dir.exists() {
            return Ok(());
        }
        
        // Process CSS files
        for style_file in &component.styles {
            let style_path = component_dir.join(style_file);
            if style_path.exists() {
                let processed = self.process_css_file(&style_path)?;
                self.write_processed_asset(&style_path, processed, "css")?;
            }
        }
        
        // Process JavaScript files
        for script_file in &component.scripts {
            let script_path = component_dir.join(script_file);
            if script_path.exists() {
                let processed = self.process_js_file(&script_path)?;
                self.write_processed_asset(&script_path, processed, "js")?;
            }
        }
        
        // Process static data files
        for data_file in &component.static_data {
            let data_path = component_dir.join(data_file);
            if data_path.exists() {
                let processed = self.process_static_data_file(&data_path)?;
                self.write_processed_asset(&data_path, processed, "json")?;
            }
        }
        
        Ok(())
    }
    
    /// Process theme assets
    fn process_theme_assets(&mut self) -> Result<()> {
        let theme_assets_dir = self.theme_dir.join("assets");
        
        if !theme_assets_dir.exists() {
            return Ok(());
        }
        
        // Process CSS files
        self.process_directory_assets(&theme_assets_dir.join("css"), "css")?;
        
        // Process JavaScript files
        self.process_directory_assets(&theme_assets_dir.join("js"), "js")?;
        
        // Process images
        self.process_directory_assets(&theme_assets_dir.join("images"), "images")?;
        
        // Process fonts
        self.process_directory_assets(&theme_assets_dir.join("fonts"), "fonts")?;
        
        Ok(())
    }
    
    /// Process all assets in a directory
    fn process_directory_assets(&mut self, dir: &Path, asset_type: &str) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            match asset_type {
                "css" => {
                    let processed = self.process_css_file(path)?;
                    self.write_processed_asset(path, processed, asset_type)?;
                }
                "js" => {
                    let processed = self.process_js_file(path)?;
                    self.write_processed_asset(path, processed, asset_type)?;
                }
                "images" => {
                    let processed = self.process_image_file(path)?;
                    self.write_processed_asset(path, processed, asset_type)?;
                }
                "fonts" => {
                    let processed = self.process_font_file(path)?;
                    self.write_processed_asset(path, processed, asset_type)?;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Process CSS file
    fn process_css_file(&self, path: &Path) -> Result<ProcessedAsset> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::asset(format!("Failed to read CSS file {}: {}", path.display(), e)))?;
        
        let processed_content = if self.config.minify_css {
            self.minify_css(&content)?
        } else {
            content.into_bytes()
        };
        
        let cache_bust = if self.config.cache_busting {
            Some(self.generate_cache_bust(&processed_content))
        } else {
            None
        };
        
        Ok(ProcessedAsset {
            content: processed_content,
            content_type: "text/css".to_string(),
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("style.css")
                .to_string(),
            source_map: None,
            cache_bust,
        })
    }
    
    /// Process JavaScript file
    fn process_js_file(&self, path: &Path) -> Result<ProcessedAsset> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::asset(format!("Failed to read JS file {}: {}", path.display(), e)))?;
        
        let processed_content = if self.config.minify_js {
            self.minify_js(&content)?
        } else {
            content.into_bytes()
        };
        
        let cache_bust = if self.config.cache_busting {
            Some(self.generate_cache_bust(&processed_content))
        } else {
            None
        };
        
        Ok(ProcessedAsset {
            content: processed_content,
            content_type: "application/javascript".to_string(),
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("script.js")
                .to_string(),
            source_map: None,
            cache_bust,
        })
    }
    
    /// Process image file
    fn process_image_file(&self, path: &Path) -> Result<ProcessedAsset> {
        let content = fs::read(path)
            .map_err(|e| Error::asset(format!("Failed to read image file {}: {}", path.display(), e)))?;
        
        let processed_content = if self.config.optimize_images {
            self.optimize_image(&content, path)?
        } else {
            content
        };
        
        let content_type = self.get_image_content_type(path)?;
        
        Ok(ProcessedAsset {
            content: processed_content,
            content_type,
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("image.png")
                .to_string(),
            source_map: None,
            cache_bust: None,
        })
    }
    
    /// Process font file
    fn process_font_file(&self, path: &Path) -> Result<ProcessedAsset> {
        let content = fs::read(path)
            .map_err(|e| Error::asset(format!("Failed to read font file {}: {}", path.display(), e)))?;
        
        let content_type = self.get_font_content_type(path)?;
        
        Ok(ProcessedAsset {
            content,
            content_type,
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("font.woff2")
                .to_string(),
            source_map: None,
            cache_bust: None,
        })
    }
    
    /// Process static data file
    fn process_static_data_file(&self, path: &Path) -> Result<ProcessedAsset> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::asset(format!("Failed to read data file {}: {}", path.display(), e)))?;
        
        Ok(ProcessedAsset {
            content: content.into_bytes(),
            content_type: "application/json".to_string(),
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("data.json")
                .to_string(),
            source_map: None,
            cache_bust: None,
        })
    }
    
    /// Write processed asset to output directory
    fn write_processed_asset(&self, original_path: &Path, asset: ProcessedAsset, asset_type: &str) -> Result<()> {
        // Determine output path
        let relative_path = original_path.strip_prefix(&self.theme_dir)
            .map_err(|e| Error::asset(format!("Failed to get relative path: {}", e)))?;
        
        let mut output_file_name = asset.file_name.clone();
        if let Some(cache_bust) = &asset.cache_bust {
            let stem = Path::new(&asset.file_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("asset");
            let extension = Path::new(&asset.file_name)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            output_file_name = format!("{}.{}.{}", stem, cache_bust, extension);
        }
        
        let output_path = self.output_dir
            .join("assets")
            .join(asset_type)
            .join(relative_path.parent().unwrap_or_else(|| Path::new("")))
            .join(output_file_name);
        
        // Create parent directories if needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::asset(format!("Failed to create directory {}: {}", parent.display(), e)))?;
        }
        
        // Write file
        fs::write(&output_path, asset.content)
            .map_err(|e| Error::asset(format!("Failed to write asset {}: {}", output_path.display(), e)))?;
        
        // Write source map if available
        if let Some(source_map) = asset.source_map {
            let source_map_path = output_path.with_extension("map");
            fs::write(&source_map_path, source_map)
                .map_err(|e| Error::asset(format!("Failed to write source map {}: {}", source_map_path.display(), e)))?;
        }
        
        Ok(())
    }
    
    /// Generate asset bundles
    fn generate_asset_bundles(&self) -> Result<()> {
        // Generate CSS bundle
        self.generate_css_bundle()?;
        
        // Generate JavaScript bundle
        self.generate_js_bundle()?;
        
        Ok(())
    }
    
    /// Generate CSS bundle
    fn generate_css_bundle(&self) -> Result<()> {
        let mut bundle_content = String::new();
        
        // Collect all CSS files
        let css_dir = self.output_dir.join("assets").join("css");
        
        if css_dir.exists() {
            for entry in fs::read_dir(&css_dir)
                .map_err(|e| Error::asset(format!("Failed to read CSS directory: {}", e)))?
            {
                let entry = entry
                    .map_err(|e| Error::asset(format!("Failed to read directory entry: {}", e)))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map(|s| s == "css").unwrap_or(false) {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| Error::asset(format!("Failed to read CSS file {}: {}", path.display(), e)))?;
                    bundle_content.push_str(&content);
                    bundle_content.push('\n');
                }
            }
        }
        
        // Write bundle
        let bundle_path = self.output_dir.join("assets").join("css").join("bundle.css");
        fs::write(&bundle_path, bundle_content)
            .map_err(|e| Error::asset(format!("Failed to write CSS bundle: {}", e)))?;
        
        Ok(())
    }
    
    /// Generate JavaScript bundle
    fn generate_js_bundle(&self) -> Result<()> {
        let mut bundle_content = String::new();
        
        // Collect all JS files
        let js_dir = self.output_dir.join("assets").join("js");
        
        if js_dir.exists() {
            for entry in fs::read_dir(&js_dir)
                .map_err(|e| Error::asset(format!("Failed to read JS directory: {}", e)))?
            {
                let entry = entry
                    .map_err(|e| Error::asset(format!("Failed to read directory entry: {}", e)))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map(|s| s == "js").unwrap_or(false) {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| Error::asset(format!("Failed to read JS file {}: {}", path.display(), e)))?;
                    bundle_content.push_str(&content);
                    bundle_content.push('\n');
                }
            }
        }
        
        // Write bundle
        let bundle_path = self.output_dir.join("assets").join("js").join("bundle.js");
        fs::write(&bundle_path, bundle_content)
            .map_err(|e| Error::asset(format!("Failed to write JS bundle: {}", e)))?;
        
        Ok(())
    }
    
    /// Generate asset manifest
    fn generate_asset_manifest(&self) -> Result<()> {
        let mut manifest = HashMap::new();
        
        // Add component assets
        let enabled_components = self.component_registry.get_enabled_components();
        for component in enabled_components {
            let mut component_assets = HashMap::new();
            
            for style in &component.styles {
                component_assets.insert("css".to_string(), format!("/assets/css/{}", style));
            }
            
            for script in &component.scripts {
                component_assets.insert("js".to_string(), format!("/assets/js/{}", script));
            }
            
            manifest.insert(component.name.clone(), component_assets);
        }
        
        // Write manifest
        let manifest_path = self.output_dir.join("assets").join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| Error::asset(format!("Failed to serialize manifest: {}", e)))?;
        
        fs::write(&manifest_path, manifest_json)
            .map_err(|e| Error::asset(format!("Failed to write manifest: {}", e)))?;
        
        Ok(())
    }
    
    /// Minify CSS content
    fn minify_css(&self, content: &str) -> Result<Vec<u8>> {
        // Simple CSS minification
        let minified = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("/*") || line.ends_with("*/"))
            .collect::<Vec<_>>()
            .join(" ");
        
        Ok(minified.into_bytes())
    }
    
    /// Minify JavaScript content
    fn minify_js(&self, content: &str) -> Result<Vec<u8>> {
        // Simple JS minification
        let minified = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join(";");
        
        Ok(minified.into_bytes())
    }
    
    /// Optimize image content
    fn optimize_image(&self, content: &[u8], _path: &Path) -> Result<Vec<u8>> {
        // Placeholder for image optimization
        // In a real implementation, you'd use an image processing library
        Ok(content.to_vec())
    }
    
    /// Generate cache busting hash
    fn generate_cache_bust(&self, content: &[u8]) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())[..8].to_string()
    }
    
    /// Get image content type
    fn get_image_content_type(&self, path: &Path) -> Result<String> {
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::asset("Unknown image file".to_string()))?;
        
        match extension.to_lowercase().as_str() {
            "png" => Ok("image/png".to_string()),
            "jpg" | "jpeg" => Ok("image/jpeg".to_string()),
            "gif" => Ok("image/gif".to_string()),
            "svg" => Ok("image/svg+xml".to_string()),
            "webp" => Ok("image/webp".to_string()),
            _ => Ok("application/octet-stream".to_string()),
        }
    }
    
    /// Get font content type
    fn get_font_content_type(&self, path: &Path) -> Result<String> {
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::asset("Unknown font file".to_string()))?;
        
        match extension.to_lowercase().as_str() {
            "woff" => Ok("font/woff".to_string()),
            "woff2" => Ok("font/woff2".to_string()),
            "ttf" => Ok("font/ttf".to_string()),
            "otf" => Ok("font/otf".to_string()),
            "eot" => Ok("application/vnd.ms-fontobject".to_string()),
            "svg" => Ok("image/svg+xml".to_string()),
            _ => Ok("application/octet-stream".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_asset_pipeline_creation() {
        let temp_dir = TempDir::new().unwrap();
        let pipeline = AssetPipeline::new("test", temp_dir.path());
        
        assert_eq!(pipeline.theme_name, "test");
        assert_eq!(pipeline.output_dir, temp_dir.path());
    }
    
    #[test]
    fn test_css_minification() {
        let pipeline = AssetPipeline::new("test", Path::new("/tmp"));
        let css = r#"
            body {
                margin: 0;
                padding: 0;
            }
            
            .container {
                width: 100%;
            }
        "#;
        
        let minified = pipeline.minify_css(css).unwrap();
        let minified_str = String::from_utf8(minified).unwrap();
        
        assert!(!minified_str.contains('\n'));
        assert!(minified_str.contains("margin:0") && minified_str.contains("padding:0"));
    }
    
    #[test]
    fn test_js_minification() {
        let pipeline = AssetPipeline::new("test", Path::new("/tmp"));
        let js = r#"
            function test() {
                console.log("Hello, world!");
            }
            
            test();
        "#;
        
        let minified = pipeline.minify_js(js).unwrap();
        let minified_str = String::from_utf8(minified).unwrap();
        
        assert!(!minified_str.contains('\n'));
        assert!(minified_str.contains("function test") && minified_str.contains("console.log") && minified_str.contains("Hello, world!"));
    }
    
    #[test]
    fn test_cache_bust_generation() {
        let pipeline = AssetPipeline::new("test", Path::new("/tmp"));
        let content = b"test content";
        
        let cache_bust = pipeline.generate_cache_bust(content);
        assert_eq!(cache_bust.len(), 8);
    }
}