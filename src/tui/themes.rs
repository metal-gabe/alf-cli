//! Theme definitions for the TUI.

use ratatui::style::Color;

/// Theme configuration for UI colors
#[derive(Debug, Clone)]
pub struct Theme {
   /// Theme name
   pub name: String,
   /// Background color
   pub background: Color,
   /// Foreground/text color
   pub foreground: Color,
   /// Primary accent color
   pub primary: Color,
   /// Secondary accent color
   pub secondary: Color,
   /// Border color
   pub border: Color,
   /// Selection highlight color
   pub selection: Color,
   /// Text highlight color
   pub highlight: Color,
   /// Alias entry badge color
   pub alias_color: Color,
   /// Function entry badge color
   pub function_color: Color,
   /// Normal mode indicator color
   pub mode_normal_color: Color,
}

impl Default for Theme {
   fn default() -> Self {
      Self::default_theme()
   }
}

impl Theme {
   /// Default theme: the original "alf" color scheme
   pub fn default_theme() -> Self {
      Self {
         name: "default".to_string(),
         background: Color::Rgb(17, 17, 17),
         foreground: Color::Rgb(220, 220, 220),
         alias_color: Color::Rgb(253, 90, 30),
         function_color: Color::Rgb(0, 199, 255),
         selection: Color::Rgb(23, 148, 129),
         border: Color::Rgb(100, 100, 100),
         highlight: Color::Rgb(255, 200, 100),
         mode_normal_color: Color::Rgb(144, 238, 144),
         // Unused fields (kept for future expansion)
         primary: Color::Reset,
         secondary: Color::Reset,
      }
   }

   /// Gruvbox theme
   pub fn gruvbox() -> Self {
      Self {
         name: "gruvbox".to_string(),
         background: Color::Rgb(40, 40, 40),
         foreground: Color::Rgb(235, 219, 178),
         primary: Color::Rgb(251, 184, 108),
         secondary: Color::Rgb(184, 187, 38),
         border: Color::Rgb(146, 131, 116),
         selection: Color::Rgb(60, 56, 54),
         highlight: Color::Rgb(250, 189, 47),
         alias_color: Color::Rgb(251, 184, 108),
         function_color: Color::Rgb(142, 192, 124),
         mode_normal_color: Color::Rgb(142, 192, 124),
      }
   }

   /// Nord theme
   pub fn nord() -> Self {
      Self {
         name: "nord".to_string(),
         background: Color::Rgb(46, 52, 64),
         foreground: Color::Rgb(236, 239, 244),
         primary: Color::Rgb(136, 192, 208),
         secondary: Color::Rgb(129, 161, 193),
         border: Color::Rgb(76, 86, 106),
         selection: Color::Rgb(59, 66, 82),
         highlight: Color::Rgb(163, 190, 140),
         alias_color: Color::Rgb(191, 144, 0),
         function_color: Color::Rgb(136, 192, 208),
         mode_normal_color: Color::Rgb(163, 190, 140),
      }
   }

   /// Dracula theme
   pub fn dracula() -> Self {
      Self {
         name: "dracula".to_string(),
         background: Color::Rgb(40, 42, 54),
         foreground: Color::Rgb(248, 248, 242),
         primary: Color::Rgb(189, 147, 249),
         secondary: Color::Rgb(255, 121, 198),
         border: Color::Rgb(68, 71, 90),
         selection: Color::Rgb(68, 71, 90),
         highlight: Color::Rgb(80, 250, 123),
         alias_color: Color::Rgb(255, 121, 198),
         function_color: Color::Rgb(189, 147, 249),
         mode_normal_color: Color::Rgb(80, 250, 123),
      }
   }

   /// Solarized Dark theme
   pub fn solarized() -> Self {
      Self {
         name: "solarized".to_string(),
         background: Color::Rgb(0, 43, 54),
         foreground: Color::Rgb(131, 148, 150),
         primary: Color::Rgb(38, 139, 210),
         secondary: Color::Rgb(42, 161, 152),
         border: Color::Rgb(7, 54, 66),
         selection: Color::Rgb(7, 54, 66),
         highlight: Color::Rgb(181, 137, 0),
         alias_color: Color::Rgb(181, 137, 0),
         function_color: Color::Rgb(38, 139, 210),
         mode_normal_color: Color::Rgb(42, 161, 152),
      }
   }

   /// Catppuccin Mocha theme
   pub fn catppuccin_mocha() -> Self {
      Self {
         name: "catppuccin".to_string(),
         background: Color::Rgb(30, 30, 46),
         foreground: Color::Rgb(205, 214, 244),
         primary: Color::Rgb(137, 180, 250),
         secondary: Color::Rgb(245, 194, 231),
         border: Color::Rgb(49, 50, 68),
         selection: Color::Rgb(49, 50, 68),
         highlight: Color::Rgb(166, 227, 161),
         alias_color: Color::Rgb(245, 194, 231),
         function_color: Color::Rgb(137, 180, 250),
         mode_normal_color: Color::Rgb(166, 227, 161),
      }
   }

   /// Tokyo Night Storm theme
   pub fn tokyo_night_storm() -> Self {
      Self {
         name: "tokyonight".to_string(),
         background: Color::Rgb(36, 40, 59),
         foreground: Color::Rgb(169, 177, 214),
         primary: Color::Rgb(122, 162, 247),
         secondary: Color::Rgb(187, 154, 247),
         border: Color::Rgb(52, 59, 88),
         selection: Color::Rgb(52, 59, 88),
         highlight: Color::Rgb(158, 206, 106),
         alias_color: Color::Rgb(187, 154, 247),
         function_color: Color::Rgb(122, 162, 247),
         mode_normal_color: Color::Rgb(158, 206, 106),
      }
   }

   /// Shades of Purple theme
   pub fn shades_of_purple() -> Self {
      Self {
         name: "shades_of_purple".to_string(),
         background: Color::Rgb(46, 22, 69),
         foreground: Color::Rgb(255, 255, 255),
         primary: Color::Rgb(165, 91, 237),
         secondary: Color::Rgb(255, 0, 144),
         border: Color::Rgb(85, 60, 111),
         selection: Color::Rgb(75, 42, 107),
         highlight: Color::Rgb(128, 203, 196),
         alias_color: Color::Rgb(255, 0, 144),
         function_color: Color::Rgb(165, 91, 237),
         mode_normal_color: Color::Rgb(128, 203, 196),
      }
   }

   /// Get theme by name
   pub fn from_name(name: &str) -> Option<Self> {
      match name.to_lowercase().as_str() {
         "default" => Some(Self::default_theme()),
         "gruvbox" => Some(Self::gruvbox()),
         "nord" => Some(Self::nord()),
         "dracula" => Some(Self::dracula()),
         "solarized" => Some(Self::solarized()),
         "catppuccin" => Some(Self::catppuccin_mocha()),
         "tokyonight" => Some(Self::tokyo_night_storm()),
         "shades_of_purple" => Some(Self::shades_of_purple()),
         _ => None,
      }
   }

   /// Get list of available theme names
   pub fn available_themes() -> Vec<String> {
      vec![
         "default".to_string(),
         "gruvbox".to_string(),
         "nord".to_string(),
         "dracula".to_string(),
         "solarized".to_string(),
         "catppuccin".to_string(),
         "tokyonight".to_string(),
         "shades_of_purple".to_string(),
      ]
   }
}
