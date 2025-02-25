use std::process::exit;

#[derive(Clone, Debug)]
pub struct DiagPrinter {
    verbose: bool,
    use_colors: bool,
    file_path: String
}

#[derive(PartialEq)]
pub enum DiagType {
    Ice,
    Error,
    Warning,
    Note,
    Info,
    Debug,
}

const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

impl DiagPrinter {
    pub fn new(verbose: bool, use_colors: bool, file_path: String) -> Self {
        Self { verbose, use_colors, file_path }
    }
    pub fn print_info(&self){
        if self.verbose {
            println!("Verbose: {}", self.verbose);
            println!("Using colors: {}", self.use_colors);
            println!("Path: {}", self.file_path);
        }
    }
    fn get_color(&self, diag_type: &DiagType) -> String {
        if self.use_colors {
            match diag_type {
                DiagType::Note | DiagType::Info => {
                    return BOLD.to_owned() + CYAN;
                }
                DiagType::Debug => {
                    return BOLD.to_owned() + GREEN;
                }
                DiagType::Warning => {
                    return BOLD.to_owned() + MAGENTA;
                }
                DiagType::Ice | DiagType::Error => {
                    return BOLD.to_owned() + RED;
                }
            }
        }
        RESET.to_owned()
    }
    fn get_name(&self, diag_type: &DiagType) -> String {
        match diag_type {
            DiagType::Note => {
                "Note".to_string()
            }
            DiagType::Info => {
                "Info".to_string()
            }
            DiagType::Debug => {
                "Debug".to_string()
            }
            DiagType::Warning => {
                "Warning".to_string()
            }
            DiagType::Ice => {
                "Ice".to_string()
            }
            DiagType::Error => {
                "Error".to_string()
            }
        }
    }
    pub fn print_formatted(&self, diag_type: DiagType, format: String) {
        if diag_type == DiagType::Debug && !self.verbose {
            return;
        }
        println!("{}{}: {}{}", self.get_color(&diag_type), self.get_name(&diag_type), RESET, format);
        if diag_type == DiagType::Ice || diag_type == DiagType::Error {
            exit(1);
        }
    }
}