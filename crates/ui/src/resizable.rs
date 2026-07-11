use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResizableDensity {
    Standard,
    Dense,
}

impl ResizableDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResizableOrientation {
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResizablePart {
    PanelGroup,
    Panel,
    Handle,
}

impl ResizablePart {
    pub const ALL: &'static [Self] = &[Self::PanelGroup, Self::Panel, Self::Handle];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PanelGroup => "ResizablePanelGroup",
            Self::Panel => "ResizablePanel",
            Self::Handle => "ResizableHandle",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ResizablePanel {
    #[garde(length(min = 1, max = 96))]
    pub title: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub size_percent: u8,
    #[garde(skip)]
    pub min_percent: u8,
    #[garde(skip)]
    pub max_percent: u8,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ResizableModel {
    #[garde(skip)]
    pub density: ResizableDensity,
    #[garde(skip)]
    pub orientation: ResizableOrientation,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 2, max = 6), dive, custom(resizable_panels_are_valid))]
    pub panels: Vec<ResizablePanel>,
    #[garde(custom(resizable_active_panel_references_panel(&self.panels)))]
    pub active_panel: Option<String>,
    #[garde(custom(resizable_handle_references_gap(&self.panels)))]
    pub resizing_handle: Option<usize>,
    #[garde(custom(validate_optional_resizable_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResizableState {
    active_panel: Option<String>,
    resizing_handle: Option<usize>,
    panel_sizes: Vec<u8>,
    default_panel_sizes: Vec<u8>,
    panel_bounds: Vec<(u8, u8)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResizableIntent {
    FocusPanel(String),
    BlurPanel,
    StartResize(usize),
    Resize {
        handle_index: usize,
        leading_percent: u8,
    },
    EndResize,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResizableChange {
    Focused(String),
    Blurred,
    ResizeStarted(usize),
    Resized {
        handle_index: usize,
        panel_sizes: Vec<u8>,
    },
    ResizeEnded,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResizableRenderNode {
    pub part: ResizablePart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ResizableDensity,
    pub orientation: ResizableOrientation,
    pub percent: u8,
    pub min_percent: u8,
    pub max_percent: u8,
    pub selected: bool,
    pub resizing: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizableLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub header_gap: f32,
    pub title_font_size: f32,
    pub status_font_size: f32,
    pub group_min_height: f32,
    pub panel_padding: f32,
    pub panel_gap: f32,
    pub emphasized_panel_padding: f32,
    pub emphasized_panel_gap: f32,
    pub panel_title_font_size: f32,
    pub disabled_panel_title_font_size: f32,
    pub detail_font_size: f32,
    pub meta_font_size: f32,
    pub handle_margin_top: f32,
    pub handle_padding: f32,
    pub handle_gap: f32,
    pub emphasized_handle_margin_top: f32,
    pub emphasized_handle_padding: f32,
    pub emphasized_handle_gap: f32,
    pub range_control_height: f32,
    pub range_track_height: f32,
    pub range_thumb_size: f32,
    pub line_height: f32,
}

impl ResizablePanel {
    pub fn new(title: impl Into<String>, value: impl Into<String>, size_percent: u8) -> Self {
        let title = title.into();
        Self {
            detail: format!("{title} panel"),
            title,
            value: value.into(),
            size_percent,
            min_percent: 15,
            max_percent: 85,
            disabled: false,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn with_bounds(mut self, min_percent: u8, max_percent: u8) -> Self {
        self.min_percent = min_percent;
        self.max_percent = max_percent;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl ResizableModel {
    pub fn new(panels: Vec<ResizablePanel>) -> Self {
        Self {
            density: ResizableDensity::Standard,
            orientation: ResizableOrientation::Horizontal,
            label: "Resizable workspace".to_owned(),
            panels,
            active_panel: None,
            resizing_handle: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ResizableDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_orientation(mut self, orientation: ResizableOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_active_panel(mut self, value: impl Into<String>) -> Self {
        self.active_panel = Some(value.into());
        self
    }

    pub fn without_active_panel(mut self) -> Self {
        self.active_panel = None;
        self
    }

    pub const fn with_resizing_handle(mut self, handle_index: usize) -> Self {
        self.resizing_handle = Some(handle_index);
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn without_error(mut self) -> Self {
        self.error = None;
        self
    }

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn state(&self) -> ResizableState {
        ResizableState::from_model(self)
    }
}

impl ResizableState {
    pub fn new(
        active_panel: Option<String>,
        resizing_handle: Option<usize>,
        panel_sizes: Vec<u8>,
        panel_bounds: Vec<(u8, u8)>,
    ) -> Self {
        Self {
            active_panel,
            resizing_handle,
            default_panel_sizes: panel_sizes.clone(),
            panel_sizes,
            panel_bounds,
        }
    }

    pub fn from_model(model: &ResizableModel) -> Self {
        Self::new(
            model.active_panel.clone(),
            model.resizing_handle,
            model
                .panels
                .iter()
                .map(|panel| panel.size_percent)
                .collect(),
            model
                .panels
                .iter()
                .map(|panel| (panel.min_percent, panel.max_percent))
                .collect(),
        )
    }

    pub fn active_panel(&self) -> Option<&str> {
        self.active_panel.as_deref()
    }

    pub const fn resizing_handle(&self) -> Option<usize> {
        self.resizing_handle
    }

    pub fn panel_sizes(&self) -> &[u8] {
        &self.panel_sizes
    }

    pub fn panel_size(&self, index: usize) -> Option<u8> {
        self.panel_sizes.get(index).copied()
    }

    pub fn is_active_panel(&self, value: &str) -> bool {
        self.active_panel.as_deref() == Some(value)
    }

    pub fn is_resizing_handle(&self, index: usize) -> bool {
        self.resizing_handle == Some(index)
    }

    pub fn handle_range(&self, handle_index: usize) -> Option<(u8, u8)> {
        if handle_index + 1 >= self.panel_sizes.len()
            || self.panel_bounds.len() != self.panel_sizes.len()
        {
            return None;
        }

        let pair_total = u16::from(self.panel_sizes[handle_index])
            + u16::from(self.panel_sizes[handle_index + 1]);
        let (leading_min, leading_max) = self.panel_bounds[handle_index];
        let (trailing_min, trailing_max) = self.panel_bounds[handle_index + 1];
        let min = u16::from(leading_min).max(pair_total.saturating_sub(u16::from(trailing_max)));
        let max = u16::from(leading_max).min(pair_total.saturating_sub(u16::from(trailing_min)));

        if min > max {
            None
        } else {
            Some((min as u8, max as u8))
        }
    }

    pub fn apply(&mut self, intent: ResizableIntent) -> ResizableChange {
        match intent {
            ResizableIntent::FocusPanel(value) => self.focus_panel(value),
            ResizableIntent::BlurPanel => self.blur_panel(),
            ResizableIntent::StartResize(index) => self.start_resize(index),
            ResizableIntent::Resize {
                handle_index,
                leading_percent,
            } => self.resize(handle_index, leading_percent),
            ResizableIntent::EndResize => self.end_resize(),
            ResizableIntent::Reset => self.reset(),
        }
    }

    fn focus_panel(&mut self, value: String) -> ResizableChange {
        if value.is_empty() || self.active_panel.as_ref() == Some(&value) {
            return ResizableChange::Unchanged;
        }
        self.active_panel = Some(value.clone());
        ResizableChange::Focused(value)
    }

    fn blur_panel(&mut self) -> ResizableChange {
        if self.active_panel.take().is_some() {
            ResizableChange::Blurred
        } else {
            ResizableChange::Unchanged
        }
    }

    fn start_resize(&mut self, index: usize) -> ResizableChange {
        if self.handle_range(index).is_none() || self.resizing_handle == Some(index) {
            return ResizableChange::Unchanged;
        }
        self.resizing_handle = Some(index);
        ResizableChange::ResizeStarted(index)
    }

    fn resize(&mut self, handle_index: usize, leading_percent: u8) -> ResizableChange {
        let Some((min, max)) = self.handle_range(handle_index) else {
            return ResizableChange::Unchanged;
        };
        let pair_total = u16::from(self.panel_sizes[handle_index])
            + u16::from(self.panel_sizes[handle_index + 1]);
        let next_leading = leading_percent.clamp(min, max);
        let next_trailing = (pair_total - u16::from(next_leading)) as u8;

        if self.panel_sizes[handle_index] == next_leading
            && self.panel_sizes[handle_index + 1] == next_trailing
            && self.resizing_handle == Some(handle_index)
        {
            return ResizableChange::Unchanged;
        }

        self.panel_sizes[handle_index] = next_leading;
        self.panel_sizes[handle_index + 1] = next_trailing;
        self.resizing_handle = Some(handle_index);
        ResizableChange::Resized {
            handle_index,
            panel_sizes: self.panel_sizes.clone(),
        }
    }

    fn end_resize(&mut self) -> ResizableChange {
        if self.resizing_handle.take().is_some() {
            ResizableChange::ResizeEnded
        } else {
            ResizableChange::Unchanged
        }
    }

    fn reset(&mut self) -> ResizableChange {
        if self.active_panel.is_none()
            && self.resizing_handle.is_none()
            && self.panel_sizes == self.default_panel_sizes
        {
            return ResizableChange::Unchanged;
        }
        self.active_panel = None;
        self.resizing_handle = None;
        self.panel_sizes = self.default_panel_sizes.clone();
        ResizableChange::Reset
    }
}

pub fn validate_resizable_model(model: &ResizableModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn resizable_layout_metrics(
    model: &ResizableModel,
    inline_size: f32,
) -> ResizableLayoutMetrics {
    let dense = model.density == ResizableDensity::Dense;
    let dense_root = dense && model.error.is_none() && !model.disabled;
    ResizableLayoutMetrics {
        max_width: scale::container::CONTENT,
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        header_gap: scale::space::xs2(inline_size),
        title_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        status_font_size: scale::font_size::f00(inline_size),
        group_min_height: match (model.density, model.orientation) {
            (ResizableDensity::Standard, ResizableOrientation::Horizontal) => {
                scale::space::xl2(inline_size)
            }
            (ResizableDensity::Standard, ResizableOrientation::Vertical) => {
                scale::space::xl4(inline_size)
            }
            (ResizableDensity::Dense, ResizableOrientation::Horizontal) => {
                scale::space::xl(inline_size)
            }
            (ResizableDensity::Dense, ResizableOrientation::Vertical) => {
                scale::space::xl3(inline_size)
            }
        },
        panel_padding: if dense {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        panel_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        emphasized_panel_padding: scale::space::s(inline_size),
        emphasized_panel_gap: scale::space::xs2(inline_size),
        panel_title_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        disabled_panel_title_font_size: scale::font_size::f0(inline_size),
        detail_font_size: scale::font_size::f00(inline_size),
        meta_font_size: scale::font_size::f00(inline_size),
        handle_margin_top: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        handle_padding: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        handle_gap: scale::space::xs3(inline_size),
        emphasized_handle_margin_top: scale::space::xs(inline_size),
        emphasized_handle_padding: scale::space::xs2(inline_size),
        emphasized_handle_gap: scale::space::xs3(inline_size),
        range_control_height: scale::space::s(inline_size),
        range_track_height: scale::space::xs3(inline_size),
        range_thumb_size: scale::space::xs(inline_size),
        line_height: scale::line_height::LH0,
    }
}

pub const fn resizable_panel_uses_emphasized_metrics(
    active: bool,
    disabled: bool,
    invalid: bool,
) -> bool {
    active || disabled || invalid
}

pub const fn resizable_handle_uses_emphasized_metrics(
    resizing: bool,
    disabled: bool,
    invalid: bool,
) -> bool {
    resizing || disabled || invalid
}

pub fn resizable_render_nodes(
    model: &ResizableModel,
    state: &ResizableState,
) -> Vec<ResizableRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let mut nodes = Vec::with_capacity(model.panels.len().saturating_mul(2));
    nodes.push(ResizableRenderNode {
        part: ResizablePart::PanelGroup,
        index: 0,
        value: resizable_sizes_label(state.panel_sizes()),
        label: model.label.clone(),
        detail: model.error.clone().unwrap_or_else(|| {
            format!(
                "{} panels, {}",
                model.panels.len(),
                model.orientation.label()
            )
        }),
        density: model.density,
        orientation: model.orientation,
        percent: 100,
        min_percent: 0,
        max_percent: 100,
        selected: state.active_panel().is_some() || state.resizing_handle().is_some(),
        resizing: state.resizing_handle().is_some(),
        visible: true,
        actionable: false,
        invalid,
        loading: model.loading,
        disabled: model.disabled,
    });

    for (index, panel) in model.panels.iter().enumerate() {
        let panel_disabled = blocked || panel.disabled;
        nodes.push(ResizableRenderNode {
            part: ResizablePart::Panel,
            index,
            value: panel.value.clone(),
            label: panel.title.clone(),
            detail: panel.detail.clone(),
            density: model.density,
            orientation: model.orientation,
            percent: state.panel_size(index).unwrap_or(panel.size_percent),
            min_percent: panel.min_percent,
            max_percent: panel.max_percent,
            selected: state.is_active_panel(&panel.value),
            resizing: false,
            visible: true,
            actionable: !panel_disabled,
            invalid,
            loading: model.loading,
            disabled: panel_disabled,
        });

        if let Some(next_panel) = model.panels.get(index + 1) {
            let handle_disabled = blocked || panel.disabled || next_panel.disabled;
            let (min_percent, max_percent) = state
                .handle_range(index)
                .unwrap_or((panel.min_percent, panel.max_percent));
            nodes.push(ResizableRenderNode {
                part: ResizablePart::Handle,
                index,
                value: format!("handle-{index}"),
                label: format!("Resize {} and {}", panel.title, next_panel.title),
                detail: format!(
                    "{} before {}, {} after",
                    state.panel_size(index).unwrap_or(panel.size_percent),
                    next_panel.title,
                    state
                        .panel_size(index + 1)
                        .unwrap_or(next_panel.size_percent)
                ),
                density: model.density,
                orientation: model.orientation,
                percent: state.panel_size(index).unwrap_or(panel.size_percent),
                min_percent,
                max_percent,
                selected: state.is_resizing_handle(index),
                resizing: state.is_resizing_handle(index),
                visible: true,
                actionable: !handle_disabled,
                invalid,
                loading: model.loading,
                disabled: handle_disabled,
            });
        }
    }

    nodes
}

pub fn default_resizable_model() -> ResizableModel {
    ResizableModel::new(default_resizable_panels()).with_label("Workspace")
}

pub fn default_resizable_panels() -> Vec<ResizablePanel> {
    vec![
        ResizablePanel::new("Navigation", "navigation", 36)
            .with_detail("Persistent workspace navigation.")
            .with_bounds(20, 70),
        ResizablePanel::new("Preview", "preview", 64)
            .with_detail("Primary content preview panel.")
            .with_bounds(30, 80),
    ]
}

pub fn resizable_sizes_label(sizes: &[u8]) -> String {
    sizes
        .iter()
        .map(|size| format!("{size}%"))
        .collect::<Vec<_>>()
        .join(" / ")
}

pub fn resizable_panel_flex_style(percent: u8) -> String {
    format!("flex: 0 1 {percent}%;")
}

fn resizable_panels_are_valid(panels: &Vec<ResizablePanel>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(panels.len());
    let mut total_size = 0u16;
    for panel in panels {
        if !values.insert(panel.value.as_str()) {
            return Err(garde::Error::new("resizable panel values must be unique"));
        }
        if !(5..=95).contains(&panel.min_percent) || !(5..=95).contains(&panel.max_percent) {
            return Err(garde::Error::new(
                "resizable panel min and max percentages must be between 5 and 95",
            ));
        }
        if panel.min_percent > panel.max_percent {
            return Err(garde::Error::new(
                "resizable panel min percentage cannot exceed max percentage",
            ));
        }
        if panel.size_percent < panel.min_percent || panel.size_percent > panel.max_percent {
            return Err(garde::Error::new(
                "resizable panel size must fall within its min and max bounds",
            ));
        }
        total_size += u16::from(panel.size_percent);
    }
    if total_size != 100 {
        return Err(garde::Error::new(
            "resizable panel sizes must add up to 100 percent",
        ));
    }
    Ok(())
}

fn resizable_active_panel_references_panel<'a>(
    panels: &'a [ResizablePanel],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !panels
                .iter()
                .any(|panel| panel.value == *value && !panel.disabled)
        {
            return Err(garde::Error::new(
                "active resizable panel must reference an enabled panel",
            ));
        }
        Ok(())
    }
}

fn resizable_handle_references_gap<'a>(
    panels: &'a [ResizablePanel],
) -> impl FnOnce(&Option<usize>, &()) -> garde::Result + 'a {
    move |handle, _context| {
        if let Some(handle) = handle
            && (*handle + 1 >= panels.len()
                || panels[*handle].disabled
                || panels[*handle + 1].disabled)
        {
            return Err(garde::Error::new(
                "resizing handle must reference a gap between enabled panels",
            ));
        }
        Ok(())
    }
}

fn validate_optional_resizable_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "resizable error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_resizable_model(&default_resizable_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_panels() {
        let model = ResizableModel::new(Vec::new());
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_panel_values() {
        let model = ResizableModel::new(vec![
            ResizablePanel::new("Left", "panel", 50),
            ResizablePanel::new("Right", "panel", 50),
        ]);
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_sizes_that_do_not_sum_to_one_hundred() {
        let model = ResizableModel::new(vec![
            ResizablePanel::new("Left", "left", 40),
            ResizablePanel::new("Right", "right", 40),
        ]);
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_size_outside_panel_bounds() {
        let model = ResizableModel::new(vec![
            ResizablePanel::new("Left", "left", 20).with_bounds(30, 80),
            ResizablePanel::new("Right", "right", 80).with_bounds(20, 80),
        ]);
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_active_panel() {
        let model = default_resizable_model().with_active_panel("missing");
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_resizing_handle() {
        let model = default_resizable_model().with_resizing_handle(5);
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_resizable_model().with_error("");
        assert!(validate_resizable_model(&model).is_err());
    }

    #[test]
    fn state_resizes_clamps_and_resets_panels() {
        let model = default_resizable_model();
        let mut state = model.state();
        assert_eq!(
            state.apply(ResizableIntent::StartResize(0)),
            ResizableChange::ResizeStarted(0)
        );
        assert_eq!(
            state.apply(ResizableIntent::Resize {
                handle_index: 0,
                leading_percent: 90,
            }),
            ResizableChange::Resized {
                handle_index: 0,
                panel_sizes: vec![70, 30],
            }
        );
        assert_eq!(state.panel_sizes(), &[70, 30]);
        assert_eq!(
            state.apply(ResizableIntent::EndResize),
            ResizableChange::ResizeEnded
        );
        assert_eq!(state.apply(ResizableIntent::Reset), ResizableChange::Reset);
        assert_eq!(state.panel_sizes(), &[36, 64]);
    }

    #[test]
    fn state_focuses_and_blurs_panel() {
        let model = default_resizable_model();
        let mut state = model.state();
        assert_eq!(
            state.apply(ResizableIntent::FocusPanel("preview".to_owned())),
            ResizableChange::Focused("preview".to_owned())
        );
        assert!(state.is_active_panel("preview"));
        assert_eq!(
            state.apply(ResizableIntent::BlurPanel),
            ResizableChange::Blurred
        );
        assert!(state.active_panel().is_none());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_resizable_model();
        let nodes = resizable_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ResizablePart::PanelGroup)
        );
        for part in ResizablePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn loading_disables_handles() {
        let model = default_resizable_model().loading();
        let nodes = resizable_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == ResizablePart::Handle)
                .all(|node| node.disabled && !node.actionable)
        );
    }

    #[test]
    fn layout_metrics_follow_density_orientation_and_token_scales() {
        let standard = resizable_layout_metrics(&default_resizable_model(), 1_000.0);
        let dense_vertical = resizable_layout_metrics(
            &default_resizable_model()
                .with_density(ResizableDensity::Dense)
                .with_orientation(ResizableOrientation::Vertical),
            1_000.0,
        );

        assert!(dense_vertical.root_padding < standard.root_padding);
        assert!(dense_vertical.panel_padding < standard.panel_padding);
        assert!(dense_vertical.group_min_height > standard.group_min_height);
        assert!(standard.range_control_height >= standard.range_thumb_size);
        assert!(standard.range_thumb_size > standard.range_track_height);
        assert_eq!(standard.line_height, scale::line_height::LH0);
    }

    #[test]
    fn emphasized_metrics_cover_css_state_precedence() {
        assert!(resizable_panel_uses_emphasized_metrics(true, false, false));
        assert!(resizable_panel_uses_emphasized_metrics(false, true, false));
        assert!(resizable_panel_uses_emphasized_metrics(false, false, true));
        assert!(!resizable_panel_uses_emphasized_metrics(
            false, false, false
        ));
        assert!(resizable_handle_uses_emphasized_metrics(true, false, false));
        assert!(!resizable_handle_uses_emphasized_metrics(
            false, false, false
        ));
    }

    #[test]
    fn sizes_label_is_stable() {
        assert_eq!(resizable_sizes_label(&[25, 50, 25]), "25% / 50% / 25%");
    }
}
