//! Theme definitions for the TUI.

use ratatui::style::Color;

/// Theme configuration for UI colors
#[derive(Debug, Clone)]
pub struct Theme {
   pub alias_color: Color,
   pub background: Color,
   pub border: Color,
   pub foreground: Color,
   pub function_color: Color,
   pub highlight: Color,
   pub mode_normal_color: Color,
   pub name: String,
   pub primary: Color,
   pub secondary: Color,
   pub selection: Color,
}

impl Default for Theme {
   fn default() -> Self {
      Self::default_theme()
   }
}

impl Theme {
   // ==========================================================================
   // HELPERS
   // ==========================================================================
   pub fn available_themes() -> Vec<String> {
      vec![
         "catppuccin".to_string(),
         "default".to_string(),
         "dracula".to_string(),
         "gruvbox".to_string(),
         "nord".to_string(),
         "shades_of_purple".to_string(),
         "solarized".to_string(),
         "tokyonight".to_string(),
      ]
   }

   pub fn from_name(name: &str) -> Option<Self> {
      match name.to_lowercase().as_str() {
         "catppuccin" => Some(Self::catppuccin_mocha()),
         "default" => Some(Self::default_theme()),
         "dracula" => Some(Self::dracula()),
         "gruvbox" => Some(Self::gruvbox()),
         "nord" => Some(Self::nord()),
         "shades_of_purple" => Some(Self::shades_of_purple()),
         "solarized" => Some(Self::solarized()),
         "tokyonight" => Some(Self::tokyo_night_storm()),
         _ => None,
      }
   }

   // ==========================================================================
   // ACTUAL THEMES
   // ==========================================================================
   pub fn catppuccin_mocha() -> Self {
      Self {
         alias_color: Color::Rgb(245, 194, 231),
         background: Color::Rgb(30, 30, 46),
         border: Color::Rgb(49, 50, 68),
         foreground: Color::Rgb(205, 214, 244),
         function_color: Color::Rgb(137, 180, 250),
         highlight: Color::Rgb(166, 227, 161),
         mode_normal_color: Color::Rgb(166, 227, 161),
         name: "catppuccin".to_string(),
         primary: Color::Rgb(137, 180, 250),
         secondary: Color::Rgb(245, 194, 231),
         selection: Color::Rgb(49, 50, 68),
      }
   }

   /// the original "alf" color scheme
   pub fn default_theme() -> Self {
      Self {
         alias_color: Color::Rgb(253, 90, 30),
         background: Color::Rgb(17, 17, 17),
         border: Color::Rgb(100, 100, 100),
         foreground: Color::Rgb(220, 220, 220),
         function_color: Color::Rgb(0, 199, 255),
         highlight: Color::Rgb(255, 200, 100),
         mode_normal_color: Color::Rgb(144, 238, 144),
         name: "default".to_string(),
         selection: Color::Rgb(23, 148, 129),
         // Unused fields (kept for future expansion)
         primary: Color::Reset,
         secondary: Color::Reset,
      }
   }

   pub fn dracula() -> Self {
      Self {
         alias_color: Color::Rgb(255, 121, 198),
         background: Color::Rgb(40, 42, 54),
         border: Color::Rgb(68, 71, 90),
         foreground: Color::Rgb(248, 248, 242),
         function_color: Color::Rgb(189, 147, 249),
         highlight: Color::Rgb(80, 250, 123),
         mode_normal_color: Color::Rgb(80, 250, 123),
         name: "dracula".to_string(),
         primary: Color::Rgb(189, 147, 249),
         secondary: Color::Rgb(255, 121, 198),
         selection: Color::Rgb(68, 71, 90),
      }
   }

   pub fn gruvbox() -> Self {
      Self {
         alias_color: Color::Rgb(251, 184, 108),
         background: Color::Rgb(40, 40, 40),
         border: Color::Rgb(146, 131, 116),
         foreground: Color::Rgb(235, 219, 178),
         function_color: Color::Rgb(142, 192, 124),
         highlight: Color::Rgb(250, 189, 47),
         mode_normal_color: Color::Rgb(142, 192, 124),
         name: "gruvbox".to_string(),
         primary: Color::Rgb(251, 184, 108),
         secondary: Color::Rgb(184, 187, 38),
         selection: Color::Rgb(60, 56, 54),
      }
   }

   pub fn nord() -> Self {
      Self {
         alias_color: Color::Rgb(191, 144, 0),
         background: Color::Rgb(46, 52, 64),
         border: Color::Rgb(76, 86, 106),
         foreground: Color::Rgb(236, 239, 244),
         function_color: Color::Rgb(136, 192, 208),
         highlight: Color::Rgb(163, 190, 140),
         mode_normal_color: Color::Rgb(163, 190, 140),
         name: "nord".to_string(),
         primary: Color::Rgb(136, 192, 208),
         secondary: Color::Rgb(129, 161, 193),
         selection: Color::Rgb(59, 66, 82),
      }
   }

   pub fn shades_of_purple() -> Self {
      Self {
         alias_color: Color::Rgb(255, 0, 144),
         background: Color::Rgb(46, 22, 69),
         border: Color::Rgb(85, 60, 111),
         foreground: Color::Rgb(255, 255, 255),
         function_color: Color::Rgb(165, 91, 237),
         highlight: Color::Rgb(128, 203, 196),
         mode_normal_color: Color::Rgb(128, 203, 196),
         name: "shades_of_purple".to_string(),
         primary: Color::Rgb(165, 91, 237),
         secondary: Color::Rgb(255, 0, 144),
         selection: Color::Rgb(75, 42, 107),
      }
   }

   pub fn solarized() -> Self {
      Self {
         alias_color: Color::Rgb(181, 137, 0),
         background: Color::Rgb(0, 43, 54),
         border: Color::Rgb(7, 54, 66),
         foreground: Color::Rgb(131, 148, 150),
         function_color: Color::Rgb(38, 139, 210),
         highlight: Color::Rgb(181, 137, 0),
         mode_normal_color: Color::Rgb(42, 161, 152),
         name: "solarized".to_string(),
         primary: Color::Rgb(38, 139, 210),
         secondary: Color::Rgb(42, 161, 152),
         selection: Color::Rgb(7, 54, 66),
      }
   }

   pub fn tokyo_night_storm() -> Self {
      Self {
         alias_color: Color::Rgb(187, 154, 247),
         background: Color::Rgb(36, 40, 59),
         border: Color::Rgb(52, 59, 88),
         foreground: Color::Rgb(169, 177, 214),
         function_color: Color::Rgb(122, 162, 247),
         highlight: Color::Rgb(158, 206, 106),
         mode_normal_color: Color::Rgb(158, 206, 106),
         name: "tokyonight".to_string(),
         primary: Color::Rgb(122, 162, 247),
         secondary: Color::Rgb(187, 154, 247),
         selection: Color::Rgb(52, 59, 88),
      }
   }
}
