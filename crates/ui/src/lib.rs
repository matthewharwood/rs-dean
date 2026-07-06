pub mod catalog;
pub mod kit;
pub mod spec;
pub mod theme;

#[cfg(feature = "leptos")]
mod components;

pub use catalog::{
    ComponentDefinition, FrameworkMode, SHADCN_COMPONENT_COUNT, SHADCN_COMPONENTS,
    UiComponentCategory, UiComponentId, UiStateModel,
};
#[cfg(feature = "leptos")]
pub use components::{
    HealthCard, ShadcnComponentGallery, ShadcnComponentPreview, ThemeCycleButton, ThemeScope,
};
pub use kit::{
    ComponentImplementation, ImplementationMaturity, LayoutContract, RenderContract, StateContract,
    component_implementation, implementation_issue_marker, implemented_components,
};
#[cfg(feature = "bevy")]
pub use spec::bevy_adapter::{BevyUiPrimitive, bevy_primitives_for_component};
pub use spec::{
    UiBlock, UiBlockRole, UiBlockTone, UiComponentSpec, component_spec, detail_for_part,
    role_for_part, tone_for_category, tone_for_role,
};
pub use theme::{ActiveTheme, Oklch, Theme, ThemeChoice, ThemeId, Tone, scale};
