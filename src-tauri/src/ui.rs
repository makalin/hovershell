use crate::{
    config::Config,
    error::{HoverShellError, Result},
};
use log::{error, info};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct UIManager {
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
    layout_config: LayoutConfig,
    animation_config: AnimationConfig,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub effects: ThemeEffects,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub border: String,
    pub shadow: String,
}

#[derive(Debug, Clone)]
pub struct ThemeFonts {
    pub family: String,
    pub size: u16,
    pub weight: String,
    pub line_height: f32,
}

#[derive(Debug, Clone)]
pub struct ThemeEffects {
    pub blur: u8,
    pub opacity: f32,
    pub shadow: bool,
    pub border_radius: u16,
    pub animations: bool,
}

#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub position: String,
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub padding: u16,
    pub margin: u16,
}

#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration: u32,
    pub easing: String,
    pub fade_in: bool,
    pub fade_out: bool,
    pub slide_in: bool,
    pub slide_out: bool,
}

impl UIManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing UI manager");
        
        Ok(Self {
            themes: HashMap::new(),
            current_theme: None,
            layout_config: LayoutConfig::default(),
            animation_config: AnimationConfig::default(),
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down UI manager");
        self.themes.clear();
        self.current_theme = None;
        Ok(())
    }

    pub async fn initialize(&mut self, config: &Config) -> Result<()> {
        // Load built-in themes
        self.load_builtin_themes().await?;
        
        // Load custom themes from config directory
        self.load_custom_themes().await?;
        
        // Apply theme from config
        self.apply_theme(&config.ui.theme).await?;
        
        // Update layout config from config
        self.update_layout_config(&config.ui).await?;
        
        info!("UI manager initialized");
        Ok(())
    }

    async fn load_builtin_themes(&mut self) -> Result<()> {
        // Tokyo Night theme
        let tokyo_night = Theme {
            name: "tokyo-night".to_string(),
            display_name: "Tokyo Night".to_string(),
            description: "Dark theme inspired by Tokyo's night sky".to_string(),
            colors: ThemeColors {
                background: "#1a1b26".to_string(),
                foreground: "#a9b1d6".to_string(),
                primary: "#7aa2f7".to_string(),
                secondary: "#9ece6a".to_string(),
                accent: "#ff9e64".to_string(),
                success: "#9ece6a".to_string(),
                warning: "#e0af68".to_string(),
                error: "#f7768e".to_string(),
                border: "#565f89".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 18,
                opacity: 0.92,
                shadow: true,
                border_radius: 8,
                animations: true,
            },
        };
        self.themes.insert("tokyo-night".to_string(), tokyo_night);

        // Dracula theme
        let dracula = Theme {
            name: "dracula".to_string(),
            display_name: "Dracula".to_string(),
            description: "Dark theme with vibrant colors".to_string(),
            colors: ThemeColors {
                background: "#282a36".to_string(),
                foreground: "#f8f8f2".to_string(),
                primary: "#bd93f9".to_string(),
                secondary: "#50fa7b".to_string(),
                accent: "#ff79c6".to_string(),
                success: "#50fa7b".to_string(),
                warning: "#f1fa8c".to_string(),
                error: "#ff5555".to_string(),
                border: "#6272a4".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 15,
                opacity: 0.95,
                shadow: true,
                border_radius: 6,
                animations: true,
            },
        };
        self.themes.insert("dracula".to_string(), dracula);

        // Light theme
        let light = Theme {
            name: "light".to_string(),
            display_name: "Light".to_string(),
            description: "Clean light theme".to_string(),
            colors: ThemeColors {
                background: "#ffffff".to_string(),
                foreground: "#333333".to_string(),
                primary: "#007acc".to_string(),
                secondary: "#28a745".to_string(),
                accent: "#ff6b35".to_string(),
                success: "#28a745".to_string(),
                warning: "#ffc107".to_string(),
                error: "#dc3545".to_string(),
                border: "#e0e0e0".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 0,
                opacity: 1.0,
                shadow: true,
                border_radius: 4,
                animations: true,
            },
        };
        self.themes.insert("light".to_string(), light);

        // Monokai theme
        let monokai = Theme {
            name: "monokai".to_string(),
            display_name: "Monokai".to_string(),
            description: "Classic Monokai color scheme".to_string(),
            colors: ThemeColors {
                background: "#272822".to_string(),
                foreground: "#f8f8f2".to_string(),
                primary: "#f92672".to_string(),
                secondary: "#a6e22e".to_string(),
                accent: "#fd971f".to_string(),
                success: "#a6e22e".to_string(),
                warning: "#e6db74".to_string(),
                error: "#f92672".to_string(),
                border: "#49483e".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 15,
                opacity: 0.95,
                shadow: true,
                border_radius: 6,
                animations: true,
            },
        };
        self.themes.insert("monokai".to_string(), monokai);

        // Nord theme
        let nord = Theme {
            name: "nord".to_string(),
            display_name: "Nord".to_string(),
            description: "Arctic-inspired color palette".to_string(),
            colors: ThemeColors {
                background: "#2e3440".to_string(),
                foreground: "#d8dee9".to_string(),
                primary: "#88c0d0".to_string(),
                secondary: "#a3be8c".to_string(),
                accent: "#ebcb8b".to_string(),
                success: "#a3be8c".to_string(),
                warning: "#ebcb8b".to_string(),
                error: "#bf616a".to_string(),
                border: "#4c566a".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 18,
                opacity: 0.92,
                shadow: true,
                border_radius: 8,
                animations: true,
            },
        };
        self.themes.insert("nord".to_string(), nord);

        // Gruvbox theme
        let gruvbox = Theme {
            name: "gruvbox".to_string(),
            display_name: "Gruvbox".to_string(),
            description: "Retro groove color scheme".to_string(),
            colors: ThemeColors {
                background: "#282828".to_string(),
                foreground: "#ebdbb2".to_string(),
                primary: "#fe8019".to_string(),
                secondary: "#b8bb26".to_string(),
                accent: "#fabd2f".to_string(),
                success: "#b8bb26".to_string(),
                warning: "#fabd2f".to_string(),
                error: "#fb4934".to_string(),
                border: "#504945".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 16,
                opacity: 0.94,
                shadow: true,
                border_radius: 7,
                animations: true,
            },
        };
        self.themes.insert("gruvbox".to_string(), gruvbox);

        // One Dark theme
        let one_dark = Theme {
            name: "one-dark".to_string(),
            display_name: "One Dark".to_string(),
            description: "Atom's One Dark theme".to_string(),
            colors: ThemeColors {
                background: "#282c34".to_string(),
                foreground: "#abb2bf".to_string(),
                primary: "#61afef".to_string(),
                secondary: "#98c379".to_string(),
                accent: "#e06c75".to_string(),
                success: "#98c379".to_string(),
                warning: "#e5c07b".to_string(),
                error: "#e06c75".to_string(),
                border: "#3e4451".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 17,
                opacity: 0.93,
                shadow: true,
                border_radius: 8,
                animations: true,
            },
        };
        self.themes.insert("one-dark".to_string(), one_dark);

        // Solarized Dark theme
        let solarized_dark = Theme {
            name: "solarized-dark".to_string(),
            display_name: "Solarized Dark".to_string(),
            description: "Solarized dark color scheme".to_string(),
            colors: ThemeColors {
                background: "#002b36".to_string(),
                foreground: "#839496".to_string(),
                primary: "#268bd2".to_string(),
                secondary: "#859900".to_string(),
                accent: "#b58900".to_string(),
                success: "#859900".to_string(),
                warning: "#b58900".to_string(),
                error: "#dc322f".to_string(),
                border: "#073642".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 14,
                opacity: 0.96,
                shadow: true,
                border_radius: 5,
                animations: true,
            },
        };
        self.themes.insert("solarized-dark".to_string(), solarized_dark);

        // Solarized Light theme
        let solarized_light = Theme {
            name: "solarized-light".to_string(),
            display_name: "Solarized Light".to_string(),
            description: "Solarized light color scheme".to_string(),
            colors: ThemeColors {
                background: "#fdf6e3".to_string(),
                foreground: "#657b83".to_string(),
                primary: "#268bd2".to_string(),
                secondary: "#859900".to_string(),
                accent: "#b58900".to_string(),
                success: "#859900".to_string(),
                warning: "#b58900".to_string(),
                error: "#dc322f".to_string(),
                border: "#eee8d5".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 8,
                opacity: 0.98,
                shadow: true,
                border_radius: 4,
                animations: true,
            },
        };
        self.themes.insert("solarized-light".to_string(), solarized_light);

        // Catppuccin Mocha theme
        let catppuccin_mocha = Theme {
            name: "catppuccin-mocha".to_string(),
            display_name: "Catppuccin Mocha".to_string(),
            description: "Soothing pastel theme".to_string(),
            colors: ThemeColors {
                background: "#1e1e2e".to_string(),
                foreground: "#cdd6f4".to_string(),
                primary: "#89b4fa".to_string(),
                secondary: "#a6e3a1".to_string(),
                accent: "#f9e2af".to_string(),
                success: "#a6e3a1".to_string(),
                warning: "#f9e2af".to_string(),
                error: "#f38ba8".to_string(),
                border: "#313244".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 20,
                opacity: 0.90,
                shadow: true,
                border_radius: 10,
                animations: true,
            },
        };
        self.themes.insert("catppuccin-mocha".to_string(), catppuccin_mocha);

        // Catppuccin Latte theme
        let catppuccin_latte = Theme {
            name: "catppuccin-latte".to_string(),
            display_name: "Catppuccin Latte".to_string(),
            description: "Light pastel theme".to_string(),
            colors: ThemeColors {
                background: "#eff1f5".to_string(),
                foreground: "#4c4f69".to_string(),
                primary: "#1e66f5".to_string(),
                secondary: "#40a02b".to_string(),
                accent: "#df8e1d".to_string(),
                success: "#40a02b".to_string(),
                warning: "#df8e1d".to_string(),
                error: "#d20f39".to_string(),
                border: "#ccd0da".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 12,
                opacity: 0.97,
                shadow: true,
                border_radius: 6,
                animations: true,
            },
        };
        self.themes.insert("catppuccin-latte".to_string(), catppuccin_latte);

        // Material Dark theme
        let material_dark = Theme {
            name: "material-dark".to_string(),
            display_name: "Material Dark".to_string(),
            description: "Google Material Design dark theme".to_string(),
            colors: ThemeColors {
                background: "#212121".to_string(),
                foreground: "#ffffff".to_string(),
                primary: "#bb86fc".to_string(),
                secondary: "#03dac6".to_string(),
                accent: "#ff6e6e".to_string(),
                success: "#03dac6".to_string(),
                warning: "#ffb74d".to_string(),
                error: "#cf6679".to_string(),
                border: "#424242".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 16,
                opacity: 0.94,
                shadow: true,
                border_radius: 8,
                animations: true,
            },
        };
        self.themes.insert("material-dark".to_string(), material_dark);

        // GitHub Dark theme
        let github_dark = Theme {
            name: "github-dark".to_string(),
            display_name: "GitHub Dark".to_string(),
            description: "GitHub's dark theme".to_string(),
            colors: ThemeColors {
                background: "#0d1117".to_string(),
                foreground: "#e6edf3".to_string(),
                primary: "#58a6ff".to_string(),
                secondary: "#3fb950".to_string(),
                accent: "#f85149".to_string(),
                success: "#3fb950".to_string(),
                warning: "#d29922".to_string(),
                error: "#f85149".to_string(),
                border: "#30363d".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 18,
                opacity: 0.92,
                shadow: true,
                border_radius: 8,
                animations: true,
            },
        };
        self.themes.insert("github-dark".to_string(), github_dark);

        // GitHub Light theme
        let github_light = Theme {
            name: "github-light".to_string(),
            display_name: "GitHub Light".to_string(),
            description: "GitHub's light theme".to_string(),
            colors: ThemeColors {
                background: "#ffffff".to_string(),
                foreground: "#24292f".to_string(),
                primary: "#0969da".to_string(),
                secondary: "#1a7f37".to_string(),
                accent: "#d1242f".to_string(),
                success: "#1a7f37".to_string(),
                warning: "#9a6700".to_string(),
                error: "#d1242f".to_string(),
                border: "#d0d7de".to_string(),
                shadow: "#000000".to_string(),
            },
            fonts: ThemeFonts {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14,
                weight: "normal".to_string(),
                line_height: 1.4,
            },
            effects: ThemeEffects {
                blur: 10,
                opacity: 0.98,
                shadow: true,
                border_radius: 6,
                animations: true,
            },
        };
        self.themes.insert("github-light".to_string(), github_light);

        info!("Loaded {} built-in themes", self.themes.len());
        Ok(())
    }

    async fn load_custom_themes(&mut self) -> Result<()> {
        // TODO: Load custom themes from ~/.hovershell/themes/
        let themes_dir = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".hovershell")
            .join("themes");

        if themes_dir.exists() {
            // TODO: Implement custom theme loading
            info!("Custom themes directory found: {:?}", themes_dir);
        }

        Ok(())
    }

    pub async fn apply_theme(&mut self, theme_name: &str) -> Result<()> {
        if let Some(theme) = self.themes.get(theme_name) {
            self.current_theme = Some(theme_name.to_string());
            info!("Applied theme: {}", theme_name);
            
            // TODO: Emit theme change event to frontend
            // This would involve sending the theme data to the frontend
        } else {
            return Err(HoverShellError::UI(format!("Theme not found: {}", theme_name)));
        }
        
        Ok(())
    }

    pub async fn get_theme(&self, theme_name: &str) -> Option<&Theme> {
        self.themes.get(theme_name)
    }

    pub async fn get_current_theme(&self) -> Option<&Theme> {
        self.current_theme.as_ref().and_then(|name| self.themes.get(name))
    }

    pub async fn get_theme_list(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    pub async fn create_theme(&mut self, theme: Theme) -> Result<()> {
        self.themes.insert(theme.name.clone(), theme);
        info!("Created theme: {}", self.themes.len());
        Ok(())
    }

    pub async fn delete_theme(&mut self, theme_name: &str) -> Result<()> {
        if self.themes.remove(theme_name).is_some() {
            info!("Deleted theme: {}", theme_name);
            Ok(())
        } else {
            Err(HoverShellError::UI(format!("Theme not found: {}", theme_name)))
        }
    }

    pub async fn export_theme(&self, theme_name: &str) -> Result<Value> {
        if let Some(theme) = self.themes.get(theme_name) {
            Ok(serde_json::to_value(theme)?)
        } else {
            Err(HoverShellError::UI(format!("Theme not found: {}", theme_name)))
        }
    }

    pub async fn import_theme(&mut self, theme_data: Value) -> Result<()> {
        let theme: Theme = serde_json::from_value(theme_data)?;
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }

    async fn update_layout_config(&mut self, ui_config: &crate::config::UIConfig) {
        self.layout_config.position = ui_config.position.clone();
        self.layout_config.height = self.parse_size(&ui_config.height).unwrap_or(600);
        self.layout_config.padding = ui_config.padding;
        self.layout_config.margin = 16; // Default margin
        
        // Update effects from theme
        if let Some(theme) = self.get_current_theme().await {
            self.layout_config.padding = ui_config.padding;
        }
    }

    fn parse_size(&self, size_str: &str) -> Option<u32> {
        if size_str.ends_with("vh") {
            let value: f32 = size_str.trim_end_matches("vh").parse().ok()?;
            Some((value * 600.0) as u32) // Assume 600px viewport height
        } else if size_str.ends_with("px") {
            size_str.trim_end_matches("px").parse().ok()
        } else {
            size_str.parse().ok()
        }
    }

    pub async fn get_layout_config(&self) -> &LayoutConfig {
        &self.layout_config
    }

    pub async fn update_layout_config_value(&mut self, key: &str, value: Value) -> Result<()> {
        match key {
            "position" => {
                if let Some(pos) = value.as_str() {
                    self.layout_config.position = pos.to_string();
                }
            }
            "width" => {
                if let Some(w) = value.as_u64() {
                    self.layout_config.width = w as u32;
                }
            }
            "height" => {
                if let Some(h) = value.as_u64() {
                    self.layout_config.height = h as u32;
                }
            }
            "padding" => {
                if let Some(p) = value.as_u64() {
                    self.layout_config.padding = p as u16;
                }
            }
            _ => {
                return Err(HoverShellError::UI(format!("Unknown layout config key: {}", key)));
            }
        }
        Ok(())
    }

    pub async fn get_animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    pub async fn update_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }

    pub async fn generate_css(&self) -> Result<String> {
        let theme = self.get_current_theme().await
            .ok_or_else(|| HoverShellError::UI("No theme applied".to_string()))?;
        
        let css = format!(
            r#"
:root {{
    --bg-color: {};
    --fg-color: {};
    --primary-color: {};
    --secondary-color: {};
    --accent-color: {};
    --success-color: {};
    --warning-color: {};
    --error-color: {};
    --border-color: {};
    --shadow-color: {};
    --font-family: {};
    --font-size: {}px;
    --line-height: {};
    --blur: {}px;
    --opacity: {};
    --border-radius: {}px;
    --padding: {}px;
    --margin: {}px;
}}

body {{
    background-color: var(--bg-color);
    color: var(--fg-color);
    font-family: var(--font-family);
    font-size: var(--font-size);
    line-height: var(--line-height);
    padding: var(--padding);
    margin: var(--margin);
    border-radius: var(--border-radius);
    backdrop-filter: blur(var(--blur));
    opacity: var(--opacity);
}}
"#,
            theme.colors.background,
            theme.colors.foreground,
            theme.colors.primary,
            theme.colors.secondary,
            theme.colors.accent,
            theme.colors.success,
            theme.colors.warning,
            theme.colors.error,
            theme.colors.border,
            theme.colors.shadow,
            theme.fonts.family,
            theme.fonts.size,
            theme.fonts.line_height,
            theme.effects.blur,
            theme.effects.opacity,
            theme.effects.border_radius,
            self.layout_config.padding,
            self.layout_config.margin,
        );
        
        Ok(css)
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            position: "top".to_string(),
            width: 800,
            height: 600,
            min_width: 400,
            min_height: 300,
            max_width: None,
            max_height: None,
            padding: 16,
            margin: 16,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 300,
            easing: "ease-out".to_string(),
            fade_in: true,
            fade_out: true,
            slide_in: true,
            slide_out: true,
        }
    }
}