#[cfg(feature = "bevy")]
use bevy::color::Alpha;

pub mod scale {
    const fn fluid_value(
        minimum: f32,
        intercept: f32,
        cqi_coefficient: f32,
        maximum: f32,
        inline_size: f32,
    ) -> f32 {
        let preferred = intercept + cqi_coefficient * inline_size / 100.0;
        if preferred < minimum {
            minimum
        } else if preferred > maximum {
            maximum
        } else {
            preferred
        }
    }

    pub fn estimate_text_block_height(
        text: &str,
        width: f32,
        font_size: f32,
        line_height: f32,
        glyph_ratio: f32,
    ) -> f32 {
        let glyph_width = font_size * glyph_ratio;
        let characters_per_line = (width / glyph_width.max(1.0)).floor().max(1.0) as usize;
        let lines = text
            .split_whitespace()
            .fold((1_usize, 0_usize), |(lines, used), word| {
                let length = word.chars().count();
                if used == 0 {
                    (lines, length)
                } else if used + 1 + length <= characters_per_line {
                    (lines, used + 1 + length)
                } else {
                    (lines + 1, length)
                }
            })
            .0;
        font_size * line_height * lines as f32
    }

    pub fn estimate_precise_text_block_height(
        text: &str,
        width: f32,
        font_size: f32,
        line_height: f32,
        letter_spacing_em: f32,
    ) -> f32 {
        let line_width = width.max(1.0);
        let space_width = estimate_inline_text_width(" ", font_size, letter_spacing_em);
        let (lines, _) =
            text.split_whitespace()
                .fold((1_usize, 0.0_f32), |(mut lines, mut used), word| {
                    let word_width = estimate_inline_text_width(word, font_size, letter_spacing_em);
                    if used > 0.0 && used + space_width + word_width > line_width {
                        lines += 1;
                        used = word_width;
                    } else if used > 0.0 {
                        used += space_width + word_width;
                    } else {
                        used = word_width;
                    }
                    while used > line_width {
                        lines += 1;
                        used -= line_width;
                    }
                    (lines, used)
                });
        font_size * line_height * lines as f32
    }

    pub fn estimate_inline_text_width(text: &str, font_size: f32, letter_spacing_em: f32) -> f32 {
        text.chars()
            .map(|character| inline_glyph_width_em(character) + letter_spacing_em.max(0.0))
            .sum::<f32>()
            * font_size
    }

    const fn inline_glyph_width_em(character: char) -> f32 {
        match character {
            ' ' => 0.32,
            'I' => 0.32,
            'J' | 'L' | 'T' => 0.58,
            'F' => 0.58,
            'E' => 0.61,
            'S' => 0.64,
            'A' | 'B' | 'Y' => 0.69,
            'R' | 'V' => 0.7,
            'C' | 'D' => 0.73,
            'M' | 'W' => 0.86,
            'i' | 'j' | 'l' | 't' => 0.29,
            'f' | 'r' => 0.38,
            'm' | 'w' => 0.82,
            'o' | 'q' => 0.61,
            'd' | 'g' | 'p' => 0.62,
            '0'..='9' => 0.58,
            '-' | '_' => 0.42,
            '.' | ',' | ':' | ';' | '!' | '|' => 0.28,
            _ if character.is_ascii_uppercase() => 0.66,
            _ if character.is_ascii_lowercase() => 0.55,
            _ => 0.62,
        }
    }

    pub mod opacity {
        pub const DISABLED: f32 = 0.45;
    }

    pub mod font_size {
        use super::fluid_value;

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

        pub const fn f00(inline_size: f32) -> f32 {
            fluid_value(11.1104, 10.6288, 0.1508, F00, inline_size)
        }

        pub const fn f0(inline_size: f32) -> f32 {
            fluid_value(13.3328, 12.5712, 0.2381, F0, inline_size)
        }

        pub const fn f1(inline_size: f32) -> f32 {
            fluid_value(16.0, 14.8576, 0.3571, F1, inline_size)
        }

        pub const fn f2(inline_size: f32) -> f32 {
            fluid_value(19.2, 17.5424, 0.5179, F2, inline_size)
        }

        pub const fn f3(inline_size: f32) -> f32 {
            fluid_value(23.04, 20.6944, 0.733, F3, inline_size)
        }

        pub const fn f4(inline_size: f32) -> f32 {
            fluid_value(27.648, 24.3872, 1.0192, F4, inline_size)
        }

        pub const fn f5(inline_size: f32) -> f32 {
            fluid_value(33.1776, 28.7104, 1.3974, F5, inline_size)
        }

        pub const fn f6(inline_size: f32) -> f32 {
            fluid_value(39.8128, 33.7504, 1.8948, F6, inline_size)
        }

        pub const fn f7(inline_size: f32) -> f32 {
            fluid_value(47.776, 39.6272, 2.5463, F7, inline_size)
        }

        pub const fn f8(inline_size: f32) -> f32 {
            fluid_value(57.3312, 46.464, 3.3961, F8, inline_size)
        }
    }

    pub mod space {
        use super::fluid_value;

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
        pub const FIELD: f32 = 40.0;
        pub const SELECTOR: f32 = 4.0;

        pub const fn xs3(inline_size: f32) -> f32 {
            fluid_value(4.0, 3.7136, 0.0893, XS3, inline_size)
        }

        pub const fn xs2(inline_size: f32) -> f32 {
            fluid_value(8.0, 7.4288, 0.1786, XS2, inline_size)
        }

        pub const fn xs(inline_size: f32) -> f32 {
            fluid_value(12.0, 11.1424, 0.2679, XS, inline_size)
        }

        pub const fn s(inline_size: f32) -> f32 {
            fluid_value(16.0, 14.8576, 0.3571, S, inline_size)
        }

        pub const fn m(inline_size: f32) -> f32 {
            fluid_value(24.0, 22.2864, 0.5357, M, inline_size)
        }

        pub const fn l(inline_size: f32) -> f32 {
            fluid_value(32.0, 29.7136, 0.7143, L, inline_size)
        }

        pub const fn xl(inline_size: f32) -> f32 {
            fluid_value(48.0, 44.5696, 1.0714, XL, inline_size)
        }

        pub const fn xl2(inline_size: f32) -> f32 {
            fluid_value(64.0, 59.4288, 1.4286, XL2, inline_size)
        }

        pub const fn xl3(inline_size: f32) -> f32 {
            fluid_value(96.0, 89.1424, 2.1429, XL3, inline_size)
        }

        pub const fn xl4(inline_size: f32) -> f32 {
            fluid_value(128.0, 118.8544, 2.8571, XL4, inline_size)
        }
    }

    pub mod container {
        pub const CONTROL: f32 = 448.0;
        pub const NARROW: f32 = 768.0;
        pub const READING: f32 = 896.0;
        pub const CONTENT: f32 = 1_024.0;
        pub const WIDE: f32 = 1_280.0;
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

    pub mod letter_spacing {
        pub const LABEL: f32 = 0.08;
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

    pub const fn with_opacity(self, opacity: f32) -> Self {
        Self {
            color: self.color,
            alpha: self.alpha * opacity,
        }
    }

    #[cfg(feature = "bevy")]
    pub fn to_bevy(self) -> bevy::prelude::Color {
        self.color.to_bevy().with_alpha(self.alpha)
    }

    #[cfg(feature = "bevy")]
    pub fn to_bevy_on(self, background: Self) -> bevy::prelude::Color {
        let foreground = self.color.to_bevy().to_srgba();
        let background_color = background.color.to_bevy().to_srgba();
        let foreground_alpha = self.alpha.clamp(0.0, 1.0);
        let background_alpha = background.alpha.clamp(0.0, 1.0);
        let output_alpha = foreground_alpha + background_alpha * (1.0 - foreground_alpha);
        if output_alpha <= f32::EPSILON {
            return bevy::prelude::Color::NONE;
        }
        let composite = |foreground_channel: f32, background_channel: f32| {
            (foreground_channel * foreground_alpha
                + background_channel * background_alpha * (1.0 - foreground_alpha))
                / output_alpha
        };
        bevy::prelude::Color::srgba(
            composite(foreground.red, background_color.red),
            composite(foreground.green, background_color.green),
            composite(foreground.blue, background_color.blue),
            output_alpha,
        )
    }

    #[cfg(feature = "bevy")]
    pub fn to_bevy_css_alpha_on(self, background: Self) -> bevy::prelude::Color {
        let alpha = self.alpha.clamp(0.0, 1.0);
        if alpha <= f32::EPSILON {
            return bevy::prelude::Color::NONE;
        }
        if alpha >= 1.0 {
            return self.to_bevy();
        }

        let foreground = self.color.to_bevy().to_srgba();
        let background_srgba = background.color.to_bevy().to_srgba();
        let target = bevy::prelude::Color::srgba(
            foreground.red * alpha + background_srgba.red * (1.0 - alpha),
            foreground.green * alpha + background_srgba.green * (1.0 - alpha),
            foreground.blue * alpha + background_srgba.blue * (1.0 - alpha),
            1.0,
        )
        .to_linear();
        let background_linear = background.color.to_bevy().to_linear();
        let source = |target_channel: f32, background_channel: f32| {
            ((target_channel - background_channel * (1.0 - alpha)) / alpha).clamp(0.0, 1.0)
        };

        bevy::prelude::Color::linear_rgba(
            source(target.red, background_linear.red),
            source(target.green, background_linear.green),
            source(target.blue, background_linear.blue),
            alpha,
        )
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

    pub const fn shadow_1(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.08)
    }

    pub const fn shadow_2(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.1)
    }

    pub const fn shadow_3(&self) -> Tone {
        Tone::with_alpha(self.base_content, 0.12)
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
            "--default-border-width",
            "--spacing-3xs",
            "--container-reading",
            "--text-0",
            "--shadow-2",
        ] {
            assert!(THEME_CSS.contains(token), "missing Tailwind token {token}");
        }
    }

    #[test]
    fn css_theme_utilities_remain_runtime_variables() {
        assert!(THEME_CSS.contains("@theme {"));
        assert!(!THEME_CSS.contains("@theme inline"));
        assert!(THEME_CSS.contains("--default-border-width: var(--border);"));
    }

    #[test]
    fn scoped_themes_rebind_derived_semantic_tokens() {
        for token in [
            "--color-surface-elevated",
            "--color-surface-overlay",
            "--color-text-1",
            "--color-border-subtle",
            "--color-brand",
            "--color-primary-soft",
            "--color-focus-ring",
            "--shadow-3",
        ] {
            assert!(
                THEME_CSS.matches(token).count() >= 2,
                "scoped themes do not rebind {token}"
            );
        }
    }

    #[test]
    fn inline_text_width_estimator_distinguishes_glyph_shapes_and_tracking() {
        let wide = scale::estimate_inline_text_width("ON", 12.0, 0.08);
        let narrow = scale::estimate_inline_text_width("II", 12.0, 0.08);
        let tracked = scale::estimate_inline_text_width("ON", 12.0, 0.16);

        assert!(wide > narrow);
        assert!(tracked > wide);
    }

    #[test]
    fn precise_text_height_uses_per_glyph_line_widths() {
        let one_line = scale::estimate_precise_text_block_height(
            "The DOM renderer owns selected state locally.",
            320.0,
            12.8,
            1.5,
            0.0,
        );
        let wrapped = scale::estimate_precise_text_block_height(
            "Trigger, panel, focus, and selected states all resolve through shared Tailwind tokens.",
            320.0,
            16.0,
            1.5,
            0.0,
        );
        assert_eq!(one_line, 12.8 * 1.5);
        assert!(wrapped > 16.0 * 1.5);
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
        assert_eq!(theme.shadow_1(), Tone::with_alpha(theme.base_content, 0.08));
        assert_eq!(theme.shadow_2(), Tone::with_alpha(theme.base_content, 0.1));
        assert_eq!(theme.shadow_3(), Tone::with_alpha(theme.base_content, 0.12));
    }

    #[test]
    fn tone_opacity_multiplies_existing_alpha() {
        let theme = ThemeId::Dark.palette();
        let tone = theme.primary_soft();
        let faded = tone.with_opacity(scale::opacity::DISABLED);

        assert_eq!(faded.color, tone.color);
        assert!((faded.alpha - tone.alpha * scale::opacity::DISABLED).abs() < f32::EPSILON);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn translucent_tones_can_resolve_over_an_opaque_surface() {
        let theme = ThemeId::Dark.palette();
        let resolved = theme
            .primary_soft()
            .to_bevy_on(theme.surface_2())
            .to_srgba();

        assert!((resolved.alpha - 1.0).abs() < f32::EPSILON);
        assert_ne!(resolved, theme.primary_soft().to_bevy().to_srgba());

        let css_surface = theme
            .primary_soft()
            .to_bevy_on(theme.surface_1())
            .to_srgba();
        let expected = [35.0 / 255.0, 40.0 / 255.0, 62.0 / 255.0];
        for (actual, expected) in [css_surface.red, css_surface.green, css_surface.blue]
            .into_iter()
            .zip(expected)
        {
            assert!(
                (actual - expected).abs() < 0.015,
                "expected {expected:.4}, got {actual:.4} from {css_surface:?}"
            );
        }
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn css_alpha_adapter_preserves_transparency_and_display_space_compositing() {
        let theme = ThemeId::Dark.palette();
        let overlay = theme.surface_overlay();
        let source = overlay.to_bevy_css_alpha_on(theme.surface_1());
        let source_linear = source.to_linear();
        let background_linear = theme.surface_1().to_bevy().to_linear();
        let alpha = source_linear.alpha;
        let blended = bevy::prelude::Color::linear_rgba(
            source_linear.red * alpha + background_linear.red * (1.0 - alpha),
            source_linear.green * alpha + background_linear.green * (1.0 - alpha),
            source_linear.blue * alpha + background_linear.blue * (1.0 - alpha),
            1.0,
        )
        .to_srgba();
        let expected = overlay.to_bevy_on(theme.surface_1()).to_srgba();

        assert!((alpha - overlay.alpha).abs() < f32::EPSILON);
        for (actual, expected) in [blended.red, blended.green, blended.blue].into_iter().zip([
            expected.red,
            expected.green,
            expected.blue,
        ]) {
            assert!((actual - expected).abs() < 0.001);
        }
    }
}
