pub mod catalog;
pub mod model;
pub mod plan;

#[cfg(feature = "bevy")]
pub mod bevy_adapter;
#[cfg(feature = "leptos")]
mod leptos_renderer;

#[cfg(feature = "bevy")]
pub use bevy_adapter::{BevyBlockPrimitive, BevyBlockPrimitiveKind, bevy_block_primitives};
pub use catalog::{
    BLOCK_COUNT, BLOCK_FAMILIES, BLOCK_FAMILY_COUNT, BLOCKS, BlockDefinition, BlockFamily,
    BlockFamilyDefinition, BlockId, BlockInteraction, BlockLayoutPreset, BlockMediaKind,
    BlockPattern, BlockSourceKind, BlockSurface, BlockSurfaceChrome, block_by_slug,
};
#[cfg(feature = "leptos")]
pub use leptos_renderer::{Block, BlockPageView};
pub use model::{
    BLOCK_SCHEMA_VERSION, BlockAction, BlockContent, BlockInstance, BlockItem, BlockPage,
    validate_block_instance, validate_block_page,
};
pub use plan::{BlockPlan, plan_block, validate_block_plan};
