#[cfg(feature = "bevy")]
use bevy::color::Alpha;

pub mod scale {
    pub mod font_size {
        pub const F0000: f32 = 8.19;
        pub const F000: f32 = 10.24;
        pub const F00: f32 = 12.8;
        pub const F0: f32 = 16.0;
        pub const F1: f32 = 20.0;
        pub const F2: f32 = 25.0;
        pub const F3: f32 = 31.25;
        pub const F4: f32 = 39.06;
        pub const F5: f32 = 48.83;
        pub const F6: f32 = 61.04;
        pub const F7: f32 = 76.29;
        pub const F8: f32 = 95.37;
    }

    pub mod space {
        pub const XS3: f32 = 5.0;
        pub const XS2: f32 = 10.0;
        pub const XS: f32 = 15.0;
        pub const S: f32 = 20.0;
        pub const M: f32 = 30.0;
        pub const L: f32 = 40.0;
        pub const XL: f32 = 60.0;
        pub const XL2: f32 = 80.0;
        pub const XL3: f32 = 120.0;
        pub const XL4: f32 = 160.0;
    }

    pub mod radius {
        pub const XS3: f32 = 3.0;
        pub const XS2: f32 = 4.0;
        pub const XS: f32 = 6.0;
        pub const S: f32 = 8.0;
        pub const M: f32 = 12.0;
        pub const L: f32 = 16.0;
        pub const XL: f32 = 24.0;
        pub const XL2: f32 = 32.0;
        pub const PILL: f32 = 9999.0;
    }

    pub mod weight {
        pub const W1: u16 = 100;
        pub const W2: u16 = 200;
        pub const W3: u16 = 300;
        pub const W4: u16 = 400;
        pub const W5: u16 = 500;
        pub const W6: u16 = 600;
        pub const W7: u16 = 700;
        pub const W8: u16 = 800;
        pub const W9: u16 = 900;
    }

    pub mod line_height {
        pub const LH00: f32 = 1.4;
        pub const LH0: f32 = 1.5;
        pub const LH1: f32 = 1.5;
        pub const LH2: f32 = 1.4;
        pub const LH3: f32 = 1.3;
        pub const LH4: f32 = 1.15;
        pub const LH5: f32 = 1.1;
        pub const LH6: f32 = 1.05;
        pub const LH7: f32 = 1.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl Oklch {
    pub const fn new(lightness: f32, chroma: f32, hue: f32) -> Self {
        Self {
            lightness,
            chroma,
            hue,
        }
    }

    #[cfg(feature = "bevy")]
    pub fn to_bevy(self) -> bevy::prelude::Color {
        bevy::prelude::Color::oklch(self.lightness, self.chroma, self.hue)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tone {
    pub color: Oklch,
    pub alpha: f32,
}

impl Tone {
    pub const fn opaque(color: Oklch) -> Self {
        Self { color, alpha: 1.0 }
    }

    pub const fn with_alpha(color: Oklch, alpha: f32) -> Self {
        Self { color, alpha }
    }

    #[cfg(feature = "bevy")]
    pub fn to_bevy(self) -> bevy::prelude::Color {
        self.color.to_bevy().with_alpha(self.alpha)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub base_100: Oklch,
    pub base_200: Oklch,
    pub base_300: Oklch,
    pub base_content: Oklch,
    pub primary: Oklch,
    pub primary_content: Oklch,
    pub secondary: Oklch,
    pub secondary_content: Oklch,
    pub accent: Oklch,
    pub accent_content: Oklch,
    pub neutral: Oklch,
    pub neutral_content: Oklch,
    pub info: Oklch,
    pub info_content: Oklch,
    pub success: Oklch,
    pub success_content: Oklch,
    pub warning: Oklch,
    pub warning_content: Oklch,
    pub error: Oklch,
    pub error_content: Oklch,
    pub border: f32,
    pub depth: f32,
    pub radius_selector: f32,
    pub radius_field: f32,
    pub radius_box: f32,
}

impl Theme {
    pub const fn surface_1(&self) -> Tone {
        Tone::opaque(self.base_100)
    }

    pub const fn surface_2(&self) -> Tone {
        Tone::opaque(self.base_200)
    }

    pub const fn surface_3(&self) -> Tone {
        Tone::opaque(self.base_300)
    }

    pub const fn surface_elevated(&self) -> Tone {
        Tone::opaque(self.base_100)
    }

    pub const fn surface_sunken(&self) -> Tone {
        Tone::opaque(self.base_200)
    }

    pub const fn surface_overlay(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.4)
    }

    pub const fn text_1(&self) -> Tone {
        Tone::opaque(self.base_content)
    }

    pub const fn text_2(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.78)
    }

    pub const fn text_muted(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.55)
    }

    pub const fn text_disabled(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.38)
    }

    pub const fn text_on_brand(&self) -> Tone {
        Tone::opaque(self.primary_content)
    }

    pub const fn link(&self) -> Tone {
        Tone::opaque(self.primary)
    }

    pub const fn link_visited(&self) -> Tone {
        Tone::opaque(self.secondary)
    }

    pub const fn border_faint(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.06)
    }

    pub const fn border_subtle(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.12)
    }

    pub const fn border_muted(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.18)
    }

    pub const fn border_strong(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.32)
    }

    pub const fn border_bold(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.55)
    }

    pub const fn brand(&self) -> Tone {
        Tone::opaque(self.primary)
    }

    pub const fn brand_text(&self) -> Tone {
        Tone::opaque(self.primary_content)
    }

    pub const fn danger(&self) -> Tone {
        Tone::opaque(self.error)
    }

    pub const fn primary_soft(&self) -> Tone {
        Tone::with_alpha(self.primary, 0.12)
    }

    pub const fn secondary_soft(&self) -> Tone {
        Tone::with_alpha(self.secondary, 0.12)
    }

    pub const fn accent_soft(&self) -> Tone {
        Tone::with_alpha(self.accent, 0.12)
    }

    pub const fn success_soft(&self) -> Tone {
        Tone::with_alpha(self.success, 0.15)
    }

    pub const fn warning_soft(&self) -> Tone {
        Tone::with_alpha(self.warning, 0.15)
    }

    pub const fn error_soft(&self) -> Tone {
        Tone::with_alpha(self.error, 0.15)
    }

    pub const fn info_soft(&self) -> Tone {
        Tone::with_alpha(self.info, 0.15)
    }

    pub const fn focus_ring(&self) -> Tone {
        Tone::with_alpha(self.primary, 0.7)
    }

    pub const fn hover_tint(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.06)
    }

    pub const fn press_tint(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.1)
    }

    pub const fn selected_tint(&self) -> Tone {
        Tone::with_alpha(self.primary, 0.14)
    }

    #[cfg(feature = "bevy")]
    pub fn interactions(&self, rest: Tone) -> (bevy::prelude::Color, bevy::prelude::Color) {
        if rest.alpha <= f32::EPSILON {
            (self.hover_tint().to_bevy(), self.press_tint().to_bevy())
        } else {
            (
                composite(rest.to_bevy(), self.hover_tint().to_bevy()),
                composite(rest.to_bevy(), self.press_tint().to_bevy()),
            )
        }
    }
}

#[cfg(feature = "bevy")]
fn composite(base: bevy::prelude::Color, over: bevy::prelude::Color) -> bevy::prelude::Color {
    let base = base.to_srgba();
    let over = over.to_srgba();
    let alpha = over.alpha;
    bevy::prelude::Color::srgba(
        base.red * (1.0 - alpha) + over.red * alpha,
        base.green * (1.0 - alpha) + over.green * alpha,
        base.blue * (1.0 - alpha) + over.blue * alpha,
        base.alpha.max(alpha),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeId {
    Light,
    Dark,
    Catppuccin,
    Synthwave,
    Cyberpunk,
    Forest,
    Lofi,
    Dracula,
    Luxury,
}

impl ThemeId {
    pub const ALL: [ThemeId; 9] = [
        ThemeId::Light,
        ThemeId::Dark,
        ThemeId::Catppuccin,
        ThemeId::Synthwave,
        ThemeId::Cyberpunk,
        ThemeId::Forest,
        ThemeId::Lofi,
        ThemeId::Dracula,
        ThemeId::Luxury,
    ];

    pub const fn slug(self) -> &'static str {
        match self {
            ThemeId::Light => "light",
            ThemeId::Dark => "dark",
            ThemeId::Catppuccin => "catppuccin",
            ThemeId::Synthwave => "synthwave",
            ThemeId::Cyberpunk => "cyberpunk",
            ThemeId::Forest => "forest",
            ThemeId::Lofi => "lofi",
            ThemeId::Dracula => "dracula",
            ThemeId::Luxury => "luxury",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            ThemeId::Light => "Light",
            ThemeId::Dark => "Dark",
            ThemeId::Catppuccin => "Catppuccin",
            ThemeId::Synthwave => "Synthwave",
            ThemeId::Cyberpunk => "Cyberpunk",
            ThemeId::Forest => "Forest",
            ThemeId::Lofi => "Lo-Fi",
            ThemeId::Dracula => "Dracula",
            ThemeId::Luxury => "Luxury",
        }
    }

    pub const fn palette(self) -> Theme {
        match self {
            ThemeId::Light => LIGHT,
            ThemeId::Dark => DARK,
            ThemeId::Catppuccin => CATPPUCCIN,
            ThemeId::Synthwave => SYNTHWAVE,
            ThemeId::Cyberpunk => CYBERPUNK,
            ThemeId::Forest => FOREST,
            ThemeId::Lofi => LOFI,
            ThemeId::Dracula => DRACULA,
            ThemeId::Luxury => LUXURY,
        }
    }

    pub fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|theme| *theme == self)
            .expect("invariant: every ThemeId appears in ThemeId::ALL");
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeChoice {
    #[default]
    Auto,
    Explicit(ThemeId),
}

impl ThemeChoice {
    pub const ALL: [ThemeChoice; 10] = [
        ThemeChoice::Auto,
        ThemeChoice::Explicit(ThemeId::Light),
        ThemeChoice::Explicit(ThemeId::Dark),
        ThemeChoice::Explicit(ThemeId::Catppuccin),
        ThemeChoice::Explicit(ThemeId::Synthwave),
        ThemeChoice::Explicit(ThemeId::Cyberpunk),
        ThemeChoice::Explicit(ThemeId::Forest),
        ThemeChoice::Explicit(ThemeId::Lofi),
        ThemeChoice::Explicit(ThemeId::Dracula),
        ThemeChoice::Explicit(ThemeId::Luxury),
    ];

    pub const fn label(self) -> &'static str {
        match self {
            ThemeChoice::Auto => "Auto",
            ThemeChoice::Explicit(theme) => theme.label(),
        }
    }

    pub const fn data_theme(self) -> Option<&'static str> {
        match self {
            ThemeChoice::Auto => None,
            ThemeChoice::Explicit(theme) => Some(theme.slug()),
        }
    }

    pub const fn theme_id(self) -> Option<ThemeId> {
        match self {
            ThemeChoice::Auto => None,
            ThemeChoice::Explicit(theme) => Some(theme),
        }
    }

    pub fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|choice| *choice == self)
            .expect("invariant: every ThemeChoice appears in ThemeChoice::ALL");
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
}

#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveTheme(pub ThemeId);

impl ActiveTheme {
    pub const fn palette(self) -> Theme {
        self.0.palette()
    }
}

impl Default for ActiveTheme {
    fn default() -> Self {
        Self(ThemeId::Dark)
    }
}

const fn oklch(lightness: f32, chroma: f32, hue: f32) -> Oklch {
    Oklch::new(lightness, chroma, hue)
}

pub const LIGHT: Theme = Theme {
    base_100: oklch(0.99, 0.003, 270.0),
    base_200: oklch(0.96, 0.005, 270.0),
    base_300: oklch(0.91, 0.008, 270.0),
    base_content: oklch(0.2, 0.015, 270.0),
    primary: oklch(0.52, 0.22, 270.0),
    primary_content: oklch(0.98, 0.012, 270.0),
    secondary: oklch(0.64, 0.21, 350.0),
    secondary_content: oklch(0.98, 0.012, 350.0),
    accent: oklch(0.72, 0.14, 195.0),
    accent_content: oklch(0.2, 0.05, 195.0),
    neutral: oklch(0.22, 0.012, 270.0),
    neutral_content: oklch(0.96, 0.005, 270.0),
    info: oklch(0.65, 0.18, 230.0),
    info_content: oklch(0.98, 0.012, 230.0),
    success: oklch(0.65, 0.17, 155.0),
    success_content: oklch(0.98, 0.012, 155.0),
    warning: oklch(0.78, 0.18, 80.0),
    warning_content: oklch(0.2, 0.08, 60.0),
    error: oklch(0.65, 0.22, 25.0),
    error_content: oklch(0.98, 0.012, 25.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 8.0,
    radius_field: 8.0,
    radius_box: 16.0,
};

pub const DARK: Theme = Theme {
    base_100: oklch(0.22, 0.012, 270.0),
    base_200: oklch(0.18, 0.012, 270.0),
    base_300: oklch(0.14, 0.012, 270.0),
    base_content: oklch(0.94, 0.005, 270.0),
    primary: oklch(0.7, 0.2, 270.0),
    primary_content: oklch(0.15, 0.06, 270.0),
    secondary: oklch(0.7, 0.18, 350.0),
    secondary_content: oklch(0.15, 0.05, 350.0),
    accent: oklch(0.76, 0.14, 195.0),
    accent_content: oklch(0.18, 0.05, 195.0),
    neutral: oklch(0.28, 0.014, 270.0),
    neutral_content: oklch(0.92, 0.005, 270.0),
    info: oklch(0.72, 0.15, 230.0),
    info_content: oklch(0.15, 0.05, 230.0),
    success: oklch(0.72, 0.15, 155.0),
    success_content: oklch(0.15, 0.05, 155.0),
    warning: oklch(0.82, 0.18, 80.0),
    warning_content: oklch(0.18, 0.07, 60.0),
    error: oklch(0.72, 0.2, 25.0),
    error_content: oklch(0.15, 0.07, 25.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 8.0,
    radius_field: 8.0,
    radius_box: 16.0,
};

pub const CATPPUCCIN: Theme = Theme {
    base_100: oklch(0.96, 0.013, 280.0),
    base_200: oklch(0.93, 0.013, 280.0),
    base_300: oklch(0.9, 0.013, 280.0),
    base_content: oklch(0.43, 0.04, 270.0),
    primary: oklch(0.63, 0.18, 15.0),
    primary_content: oklch(0.98, 0.012, 15.0),
    secondary: oklch(0.53, 0.27, 295.0),
    secondary_content: oklch(0.98, 0.012, 295.0),
    accent: oklch(0.58, 0.13, 200.0),
    accent_content: oklch(0.98, 0.012, 200.0),
    neutral: oklch(0.3, 0.04, 270.0),
    neutral_content: oklch(0.96, 0.013, 280.0),
    info: oklch(0.56, 0.22, 260.0),
    info_content: oklch(0.98, 0.012, 260.0),
    success: oklch(0.64, 0.16, 145.0),
    success_content: oklch(0.15, 0.06, 145.0),
    warning: oklch(0.73, 0.13, 75.0),
    warning_content: oklch(0.2, 0.07, 60.0),
    error: oklch(0.56, 0.22, 25.0),
    error_content: oklch(0.98, 0.012, 25.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 8.0,
    radius_field: 8.0,
    radius_box: 16.0,
};

pub const SYNTHWAVE: Theme = Theme {
    base_100: oklch(0.18, 0.04, 290.0),
    base_200: oklch(0.15, 0.04, 290.0),
    base_300: oklch(0.11, 0.04, 290.0),
    base_content: oklch(0.95, 0.04, 295.0),
    primary: oklch(0.72, 0.25, 340.0),
    primary_content: oklch(0.15, 0.08, 340.0),
    secondary: oklch(0.75, 0.18, 200.0),
    secondary_content: oklch(0.15, 0.06, 200.0),
    accent: oklch(0.85, 0.18, 90.0),
    accent_content: oklch(0.15, 0.07, 90.0),
    neutral: oklch(0.25, 0.05, 290.0),
    neutral_content: oklch(0.92, 0.04, 290.0),
    info: oklch(0.75, 0.18, 200.0),
    info_content: oklch(0.15, 0.06, 200.0),
    success: oklch(0.75, 0.2, 155.0),
    success_content: oklch(0.15, 0.07, 155.0),
    warning: oklch(0.82, 0.2, 65.0),
    warning_content: oklch(0.15, 0.08, 60.0),
    error: oklch(0.7, 0.25, 0.0),
    error_content: oklch(0.98, 0.02, 0.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 0.0,
    radius_field: 0.0,
    radius_box: 0.0,
};

pub const CYBERPUNK: Theme = Theme {
    base_100: oklch(0.91, 0.22, 92.0),
    base_200: oklch(0.85, 0.22, 90.0),
    base_300: oklch(0.78, 0.22, 88.0),
    base_content: oklch(0.15, 0.04, 90.0),
    primary: oklch(0.65, 0.3, 0.0),
    primary_content: oklch(0.98, 0.04, 0.0),
    secondary: oklch(0.75, 0.2, 200.0),
    secondary_content: oklch(0.15, 0.06, 200.0),
    accent: oklch(0.15, 0.04, 90.0),
    accent_content: oklch(0.85, 0.22, 90.0),
    neutral: oklch(0.15, 0.04, 90.0),
    neutral_content: oklch(0.95, 0.04, 90.0),
    info: oklch(0.75, 0.2, 200.0),
    info_content: oklch(0.15, 0.07, 200.0),
    success: oklch(0.7, 0.22, 145.0),
    success_content: oklch(0.15, 0.07, 145.0),
    warning: oklch(0.82, 0.22, 65.0),
    warning_content: oklch(0.15, 0.08, 65.0),
    error: oklch(0.6, 0.3, 25.0),
    error_content: oklch(0.98, 0.04, 25.0),
    border: 2.0,
    depth: 0.0,
    radius_selector: 0.0,
    radius_field: 0.0,
    radius_box: 0.0,
};

pub const FOREST: Theme = Theme {
    base_100: oklch(0.18, 0.03, 155.0),
    base_200: oklch(0.14, 0.03, 155.0),
    base_300: oklch(0.1, 0.025, 155.0),
    base_content: oklch(0.9, 0.04, 155.0),
    primary: oklch(0.6, 0.16, 155.0),
    primary_content: oklch(0.15, 0.05, 155.0),
    secondary: oklch(0.5, 0.12, 165.0),
    secondary_content: oklch(0.95, 0.02, 165.0),
    accent: oklch(0.75, 0.13, 75.0),
    accent_content: oklch(0.15, 0.06, 75.0),
    neutral: oklch(0.22, 0.03, 155.0),
    neutral_content: oklch(0.92, 0.04, 155.0),
    info: oklch(0.7, 0.13, 220.0),
    info_content: oklch(0.15, 0.05, 220.0),
    success: oklch(0.7, 0.15, 145.0),
    success_content: oklch(0.15, 0.06, 145.0),
    warning: oklch(0.8, 0.16, 80.0),
    warning_content: oklch(0.15, 0.07, 60.0),
    error: oklch(0.65, 0.2, 25.0),
    error_content: oklch(0.98, 0.012, 25.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 8.0,
    radius_field: 8.0,
    radius_box: 16.0,
};

pub const LOFI: Theme = Theme {
    base_100: oklch(0.98, 0.002, 70.0),
    base_200: oklch(0.94, 0.002, 70.0),
    base_300: oklch(0.88, 0.003, 70.0),
    base_content: oklch(0.15, 0.003, 70.0),
    primary: oklch(0.15, 0.003, 70.0),
    primary_content: oklch(0.98, 0.002, 70.0),
    secondary: oklch(0.35, 0.005, 70.0),
    secondary_content: oklch(0.98, 0.002, 70.0),
    accent: oklch(0.5, 0.005, 70.0),
    accent_content: oklch(0.98, 0.002, 70.0),
    neutral: oklch(0.2, 0.005, 70.0),
    neutral_content: oklch(0.95, 0.002, 70.0),
    info: oklch(0.45, 0.04, 220.0),
    info_content: oklch(0.98, 0.002, 220.0),
    success: oklch(0.45, 0.04, 155.0),
    success_content: oklch(0.98, 0.002, 155.0),
    warning: oklch(0.55, 0.05, 80.0),
    warning_content: oklch(0.98, 0.002, 80.0),
    error: oklch(0.45, 0.06, 25.0),
    error_content: oklch(0.98, 0.002, 25.0),
    border: 1.0,
    depth: 0.0,
    radius_selector: 4.0,
    radius_field: 4.0,
    radius_box: 4.0,
};

pub const DRACULA: Theme = Theme {
    base_100: oklch(0.28, 0.018, 280.0),
    base_200: oklch(0.24, 0.018, 280.0),
    base_300: oklch(0.2, 0.018, 280.0),
    base_content: oklch(0.92, 0.005, 280.0),
    primary: oklch(0.72, 0.13, 290.0),
    primary_content: oklch(0.2, 0.06, 290.0),
    secondary: oklch(0.75, 0.18, 0.0),
    secondary_content: oklch(0.2, 0.08, 0.0),
    accent: oklch(0.82, 0.18, 90.0),
    accent_content: oklch(0.2, 0.07, 90.0),
    neutral: oklch(0.35, 0.018, 280.0),
    neutral_content: oklch(0.92, 0.005, 280.0),
    info: oklch(0.75, 0.15, 200.0),
    info_content: oklch(0.15, 0.06, 200.0),
    success: oklch(0.75, 0.18, 145.0),
    success_content: oklch(0.15, 0.07, 145.0),
    warning: oklch(0.85, 0.18, 80.0),
    warning_content: oklch(0.2, 0.08, 60.0),
    error: oklch(0.7, 0.22, 15.0),
    error_content: oklch(0.98, 0.02, 15.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 8.0,
    radius_field: 8.0,
    radius_box: 16.0,
};

pub const LUXURY: Theme = Theme {
    base_100: oklch(0.08, 0.005, 50.0),
    base_200: oklch(0.05, 0.005, 50.0),
    base_300: oklch(0.03, 0.005, 50.0),
    base_content: oklch(0.88, 0.04, 70.0),
    primary: oklch(0.75, 0.13, 75.0),
    primary_content: oklch(0.1, 0.04, 75.0),
    secondary: oklch(0.85, 0.04, 70.0),
    secondary_content: oklch(0.15, 0.04, 70.0),
    accent: oklch(0.45, 0.15, 25.0),
    accent_content: oklch(0.95, 0.02, 25.0),
    neutral: oklch(0.15, 0.01, 50.0),
    neutral_content: oklch(0.9, 0.04, 70.0),
    info: oklch(0.7, 0.13, 220.0),
    info_content: oklch(0.15, 0.05, 220.0),
    success: oklch(0.7, 0.15, 155.0),
    success_content: oklch(0.15, 0.06, 155.0),
    warning: oklch(0.78, 0.16, 75.0),
    warning_content: oklch(0.15, 0.07, 60.0),
    error: oklch(0.65, 0.2, 20.0),
    error_content: oklch(0.98, 0.02, 20.0),
    border: 1.0,
    depth: 1.0,
    radius_selector: 4.0,
    radius_field: 4.0,
    radius_box: 8.0,
};

#[cfg(test)]
mod tests {
    use super::*;

    const THEME_CSS: &str = include_str!("../styles/theme.css");

    #[test]
    fn every_theme_has_a_css_selector() {
        for theme in ThemeId::ALL {
            assert!(
                THEME_CSS.contains(&format!(r#"[data-theme="{}"]"#, theme.slug())),
                "missing CSS selector for {}",
                theme.slug()
            );
        }
    }

    #[test]
    fn auto_cycle_covers_every_explicit_theme() {
        let mut seen = Vec::new();
        let mut current = ThemeChoice::Auto;
        for _ in 0..ThemeChoice::ALL.len() {
            seen.push(current);
            current = current.next();
        }
        assert_eq!(seen, ThemeChoice::ALL);
        assert_eq!(current, ThemeChoice::Auto);
    }

    #[test]
    fn css_exports_semantic_tailwind_tokens() {
        for token in [
            "--color-surface-1",
            "--color-surface-2",
            "--color-surface-3",
            "--color-text-1",
            "--color-text-2",
            "--color-text-muted",
            "--color-border-subtle",
            "--color-brand",
            "--color-focus-ring",
            "--radius-field",
            "--radius-box",
            "--spacing-3xs",
            "--text-0",
            "--shadow-2",
        ] {
            assert!(THEME_CSS.contains(token), "missing Tailwind token {token}");
        }
    }

    #[test]
    fn semantic_methods_match_primitives() {
        let theme = ThemeId::Catppuccin.palette();
        assert_eq!(theme.surface_1(), Tone::opaque(theme.base_100));
        assert_eq!(theme.brand(), Tone::opaque(theme.primary));
        assert_eq!(
            theme.text_muted(),
            Tone::with_alpha(theme.base_content, 0.55)
        );
    }
}
