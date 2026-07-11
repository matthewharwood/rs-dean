use std::collections::HashSet;

use garde::Validate;
use rs_dean_ui::{ButtonSize, ButtonVariant, CardAction, CardModel, CardVariant, UiComponentId};
use serde::{Deserialize, Serialize};

use crate::{BlockDefinition, BlockId, BlockInteraction, BlockLayoutPreset, BlockPattern};

pub const BLOCK_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub variant: ButtonVariant,
    #[garde(skip)]
    pub size: ButtonSize,
    #[garde(custom(validate_optional_href))]
    pub href: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockItem {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 480))]
    pub body: String,
    #[garde(length(min = 1, max = 96))]
    pub meta: String,
    #[garde(skip)]
    pub selected: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockContent {
    #[garde(length(min = 1, max = 80))]
    pub eyebrow: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub body: String,
    #[garde(length(min = 1, max = 160))]
    pub media_label: String,
    #[garde(length(max = 3), dive)]
    pub actions: Vec<BlockAction>,
    #[garde(length(max = 12), dive)]
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockInstance {
    #[garde(length(min = 1, max = 128))]
    pub key: String,
    #[garde(dive)]
    pub block: BlockId,
    #[garde(dive)]
    pub content: BlockContent,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockPage {
    #[garde(custom(validate_schema_version))]
    pub schema_version: u16,
    #[garde(length(min = 1, max = 128), dive, custom(validate_unique_block_keys))]
    pub blocks: Vec<BlockInstance>,
}

impl BlockAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            href: None,
        }
    }

    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

impl BlockItem {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        meta: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            body: body.into(),
            meta: meta.into(),
            selected: false,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub fn card_model(&self) -> CardModel {
        let variant = if self.selected {
            CardVariant::Elevated
        } else {
            CardVariant::Outline
        };
        CardModel::new(
            self.title.clone(),
            self.meta.clone(),
            self.body.clone(),
            "Shared block item",
        )
        .with_variant(variant)
        .with_action(CardAction::new("Open", self.value.clone()))
    }
}

impl BlockContent {
    pub fn new(
        eyebrow: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        let title = title.into();
        Self {
            eyebrow: eyebrow.into(),
            media_label: title.clone(),
            title,
            body: body.into(),
            actions: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn with_media_label(mut self, media_label: impl Into<String>) -> Self {
        self.media_label = media_label.into();
        self
    }

    pub fn with_actions(mut self, actions: Vec<BlockAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_items(mut self, items: Vec<BlockItem>) -> Self {
        self.items = items;
        self
    }
}

impl BlockInstance {
    pub fn new(key: impl Into<String>, block: BlockId, content: BlockContent) -> Self {
        Self {
            key: key.into(),
            block,
            content,
        }
    }

    pub fn fixture(definition: &BlockDefinition) -> Self {
        let family = definition.family_definition();
        let body = format!(
            "{} This fixture is original rs-dean content rendered from one validated block contract.",
            family.pattern.summary()
        );
        let items = (1..=fixture_item_count(definition))
            .map(|index| {
                let item = BlockItem::new(
                    format!("{}-item-{index}", definition.slug),
                    format!("{} item {index}", family.name),
                    "Shared UI components keep content, behavior, and theme semantics aligned.",
                    format!("0{index}"),
                );
                if index == 1 { item.selected() } else { item }
            })
            .collect();

        Self::new(
            format!("{}-fixture", definition.slug),
            definition.id,
            BlockContent::new(family.name, definition.name, body)
                .with_media_label(format!("{} visual fixture", definition.name))
                .with_actions(fixture_actions(definition))
                .with_items(items),
        )
    }

    pub fn definition(&self) -> &'static BlockDefinition {
        self.block.definition()
    }
}

fn fixture_actions(definition: &BlockDefinition) -> Vec<BlockAction> {
    let action_count = match definition.interaction {
        BlockInteraction::Static => 0,
        BlockInteraction::Overlay if definition.name.contains("single action") => 1,
        BlockInteraction::Actions
        | BlockInteraction::Form
        | BlockInteraction::Toggle
        | BlockInteraction::Filter
        | BlockInteraction::Search
        | BlockInteraction::Menu
        | BlockInteraction::Overlay
        | BlockInteraction::Pagination
        | BlockInteraction::Tabs => 2,
    };
    (0..action_count)
        .map(|index| {
            let label = if index == 0 {
                "Primary action"
            } else {
                "Secondary action"
            };
            let action =
                BlockAction::new(label, format!("{}-action-{}", definition.slug, index + 1));
            if index == 0 {
                action
            } else {
                action.with_variant(ButtonVariant::Outline)
            }
        })
        .collect()
}

fn fixture_item_count(definition: &BlockDefinition) -> usize {
    let family = definition.family_definition();
    let supports_items = matches!(
        family.pattern,
        BlockPattern::Feature
            | BlockPattern::Bento
            | BlockPattern::Pricing
            | BlockPattern::Metrics
            | BlockPattern::Testimonial
            | BlockPattern::Collection
            | BlockPattern::Page
    ) || (family.pattern == BlockPattern::Data
        && matches!(
            family.primary_component,
            UiComponentId::Card | UiComponentId::Item
        ));
    if !supports_items {
        return 0;
    }

    for (word, count) in [("single", 1), ("two", 2), ("three", 3), ("four", 4)] {
        if definition
            .name
            .to_ascii_lowercase()
            .split(|character: char| !character.is_ascii_alphanumeric())
            .any(|part| part == word)
        {
            return count;
        }
    }

    match definition.resolved_layout() {
        BlockLayoutPreset::Split | BlockLayoutPreset::SplitReverse => 2,
        BlockLayoutPreset::ThreeColumn | BlockLayoutPreset::Bento => 3,
        BlockLayoutPreset::FourColumn => 4,
        BlockLayoutPreset::Stack
        | BlockLayoutPreset::Full
        | BlockLayoutPreset::Centered
        | BlockLayoutPreset::CenteredNarrow
        | BlockLayoutPreset::SidebarStart
        | BlockLayoutPreset::SidebarEnd
        | BlockLayoutPreset::TwelveColumn
        | BlockLayoutPreset::Overlay
        | BlockLayoutPreset::Shell
        | BlockLayoutPreset::Page => 4,
    }
}

impl BlockPage {
    pub fn new(blocks: Vec<BlockInstance>) -> Self {
        Self {
            schema_version: BLOCK_SCHEMA_VERSION,
            blocks,
        }
    }

    pub fn fixtures(ids: impl IntoIterator<Item = BlockId>) -> Self {
        Self::new(
            ids.into_iter()
                .map(|id| BlockInstance::fixture(id.definition()))
                .collect(),
        )
    }
}

pub fn validate_block_instance(instance: &BlockInstance) -> Result<(), garde::Report> {
    instance.validate()
}

pub fn validate_block_page(page: &BlockPage) -> Result<(), garde::Report> {
    page.validate()
}

fn validate_optional_href(value: &Option<String>, _context: &()) -> garde::Result {
    let Some(href) = value else {
        return Ok(());
    };
    let normalized = href.trim().to_ascii_lowercase();
    if !(1..=512).contains(&href.chars().count()) {
        Err(garde::Error::new(
            "block action href must be 1 to 512 characters",
        ))
    } else if normalized.starts_with("javascript:") || normalized.starts_with("data:") {
        Err(garde::Error::new(
            "block action href must not use an executable URL scheme",
        ))
    } else {
        Ok(())
    }
}

fn validate_schema_version(value: &u16, _context: &()) -> garde::Result {
    if *value == BLOCK_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(garde::Error::new(format!(
            "block schema version must be {BLOCK_SCHEMA_VERSION}"
        )))
    }
}

fn validate_unique_block_keys(values: &[BlockInstance], _context: &()) -> garde::Result {
    let mut keys = HashSet::with_capacity(values.len());
    for value in values {
        if !keys.insert(value.key.as_str()) {
            return Err(garde::Error::new(format!(
                "block page contains duplicate key `{}`",
                value.key
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BLOCK_COUNT, BLOCKS};

    #[test]
    fn every_catalog_entry_builds_a_valid_fixture() {
        for definition in BLOCKS {
            validate_block_instance(&BlockInstance::fixture(&definition))
                .expect("catalog fixture should validate");
        }
    }

    #[test]
    fn page_rejects_duplicate_instance_keys() {
        let instance = BlockInstance::fixture(&BLOCKS[0]);
        let page = BlockPage::new(vec![instance.clone(), instance]);
        assert!(validate_block_page(&page).is_err());
    }

    #[test]
    fn block_id_rejects_values_outside_the_catalog() {
        let invalid = BlockId((BLOCK_COUNT + 1) as u16);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn block_pages_round_trip_through_the_serialized_authoring_boundary() {
        let page = BlockPage::fixtures([BLOCKS[0].id, BLOCKS[BLOCKS.len() - 1].id]);
        let json = serde_json::to_string(&page).expect("block page should serialize");
        let decoded: BlockPage =
            serde_json::from_str(&json).expect("block page should deserialize");

        validate_block_page(&decoded).expect("round-tripped block page should validate");
        assert_eq!(decoded, page);
    }

    #[test]
    fn block_content_allows_patterns_without_actions_or_repeated_items() {
        let content = BlockContent::new("Context", "Static copy", "No actions are required.");
        let instance = BlockInstance::new("static-copy", BLOCKS[0].id, content);

        validate_block_instance(&instance).expect("optional collections should validate empty");
    }

    #[test]
    fn block_actions_reject_executable_url_schemes() {
        let instance = BlockInstance::new(
            "unsafe-link",
            BLOCKS[0].id,
            BlockContent::new(
                "Context",
                "Unsafe link",
                "The edge rejects executable URLs.",
            )
            .with_actions(vec![
                BlockAction::new("Run", "unsafe").with_href("javascript:alert(1)"),
            ]),
        );

        assert!(validate_block_instance(&instance).is_err());
    }
}
