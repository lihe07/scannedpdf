// All units are px

/// Page size, default is A4
#[derive(PartialEq)]
pub enum PageSize {
    A4,
    Custom(u32, u32),
    /// Same size as image
    Image,
}

impl PageSize {
    pub fn width(&self) -> u32 {
        match self {
            PageSize::A4 => 595,
            PageSize::Custom(width, _) => *width,
            PageSize::Image => 0,
        }
    }
    pub fn height(&self) -> u32 {
        match self {
            PageSize::A4 => 842,
            PageSize::Custom(_, height) => *height,
            PageSize::Image => 0,
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}

/// Page margin, default is None
pub enum Margin {
    None,
    Vertical(u32),
    Horizontal(u32),
    Custom(u32, u32),
}

impl Margin {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Margin::None => (0, 0),
            Margin::Vertical(margin) => (0, *margin),
            Margin::Horizontal(margin) => (*margin, 0),
            Margin::Custom(width, height) => (*width, *height),
        }
    }
    pub fn new(x: u32, y: u32) -> Self {
        Margin::Custom(x, y)
    }
    pub fn vertical(margin: u32) -> Self {
        Margin::Vertical(margin)
    }
    pub fn horizontal(margin: u32) -> Self {
        Margin::Horizontal(margin)
    }
}

/// Alignment of the image
pub enum Alignment {
    Center,
    Start,
    End,
    Custom(u32),
}

/// Config of a single page
pub struct PageConfig {
    pub(crate) size: PageSize,
    pub(crate) margin: Margin,
    pub(crate) vertical_alignment: Alignment,
    pub(crate) horizontal_alignment: Alignment,
    pub(crate) quality: u8,
}

impl Default for PageConfig {
    fn default() -> Self {
        // Default size: A4
        PageConfig {
            size: PageSize::A4,
            margin: Margin::None,
            vertical_alignment: Alignment::Center,
            horizontal_alignment: Alignment::Center,
            quality: 60,
        }
    }
}

impl PageConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, size: PageSize) -> Self {
        self.size = size;
        self
    }
    pub fn vertical_alignment(mut self, alignment: Alignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
    pub fn horizontal_alignment(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = quality;
        self
    }
}
