// Custom ANSI colors and styling system
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    
    // Colors
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    // Bright colors
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
    
    // Background colors
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
}

// Custom styling functions
pub fn style_text(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, colors::RESET)
}

pub fn bold_text(text: &str) -> String {
    format!("{}{}{}", colors::BOLD, text, colors::RESET)
}

pub fn dim_text(text: &str) -> String {
    format!("{}{}{}", colors::DIM, text, colors::RESET)
}

pub fn bright_text(text: &str, color: &str) -> String {
    format!("{}{}{}{}", colors::BOLD, color, text, colors::RESET)
}

// Color presets based on opensync.dev style
pub fn primary(text: &str) -> String {
    bright_text(text, colors::BRIGHT_CYAN)
}

pub fn secondary(text: &str) -> String {
    bright_text(text, colors::BRIGHT_MAGENTA)
}

pub fn success(text: &str) -> String {
    bright_text(text, colors::BRIGHT_GREEN)
}

pub fn warning(text: &str) -> String {
    bright_text(text, colors::BRIGHT_YELLOW)
}

pub fn error(text: &str) -> String {
    bright_text(text, colors::BRIGHT_RED)
}

pub fn info(text: &str) -> String {
    bright_text(text, colors::BRIGHT_BLUE)
}

pub fn muted(text: &str) -> String {
    dim_text(text)
}

pub fn accent(text: &str) -> String {
    style_text(text, colors::BRIGHT_CYAN)
}

pub fn border(text: &str) -> String {
    style_text(text, colors::BRIGHT_CYAN)
}

pub fn highlight(text: &str) -> String {
    bright_text(text, colors::BRIGHT_YELLOW)
}

// Unicode box drawing characters for modern UI
pub const BOX_H: &str = "─";
pub const BOX_V: &str = "│";
pub const BOX_TL: &str = "╭";
pub const BOX_TR: &str = "╮";
pub const BOX_BL: &str = "╰";
pub const BOX_BR: &str = "╯";
pub const BOX_T: &str = "┬";
pub const BOX_B: &str = "┴";
pub const BOX_L: &str = "├";
pub const BOX_R: &str = "┤";
pub const BOX_CROSS: &str = "┼";

// Progress bar characters
pub const PROGRESS_FULL: &str = "█";
pub const PROGRESS_PARTIAL: &str = "▓";
pub const PROGRESS_EMPTY: &str = "░";
pub const SPINNER_CHARS: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];