pub struct PageConfig {
    width: f64,
    height: f64,
    dpi: f64
}

impl Default for PageConfig {
    fn default() -> Self {
        // Default size: A4
        PageConfig {
            width,
            dpi: 300.0
        }
    }
}

pub struct Config {
    default_page_config: PageConfig,
    memory: bool
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_page_config: PageConfig::default(),
            memory: false
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn memory(mut self, memory: bool) -> Self {

        self
    }

    pub fn resize(mut self, resize: bool) -> Self {

        self
    }
}