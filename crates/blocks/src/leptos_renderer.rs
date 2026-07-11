use leptos::prelude::*;
use rs_dean_ui::{
    Button, ButtonKind, ButtonModel, Card, Cluster, ComponentDemo, Grid, GridItem, HeadingLevel,
    Section, SectionSurface, Stack, TextAlign, TextStyle, TextTone, UiHeading, UiMediaFrame,
    UiText,
};

use crate::{
    BlockAction, BlockInstance, BlockItem, BlockPage, BlockPattern, BlockPlan, plan_block,
};

#[component]
pub fn Block(instance: BlockInstance) -> AnyView {
    let Ok(plan) = plan_block(&instance) else {
        return view! {
            <section
                class="w-full rounded-box border border-danger bg-error-soft p-s text-0 leading-0 text-text-1"
                data-block-state="invalid"
                role="alert"
            >
                "Block validation failed"
            </section>
        }
        .into_any();
    };
    block_plan_view(plan)
}

#[component]
pub fn BlockPageView(page: BlockPage) -> AnyView {
    if crate::validate_block_page(&page).is_err() {
        return view! {
            <section
                class="w-full rounded-box border border-danger bg-error-soft p-s text-0 leading-0 text-text-1"
                data-block-page-state="invalid"
                role="alert"
            >
                "Block page validation failed"
            </section>
        }
        .into_any();
    }

    view! {
        <main class="grid w-full gap-0" data-block-page-version=page.schema_version>
            {page
                .blocks
                .into_iter()
                .map(|instance| view! { <Block instance /> })
                .collect_view()}
        </main>
    }
    .into_any()
}

fn block_plan_view(plan: BlockPlan) -> AnyView {
    let definition = plan.block.definition();
    let surface = plan.section.surface;
    let copy = block_copy_view(&plan, surface);
    let children = if !plan.has_support() {
        vec![copy]
    } else if plan.reverse {
        vec![block_support_view(&plan), copy]
    } else {
        vec![copy, block_support_view(&plan)]
    };
    let section = plan.section;
    let container = plan.container;
    let grid = plan.outer_grid;
    let slug = definition.slug;
    let pattern = plan.pattern.label();
    let layout = plan.layout.label();

    view! {
        <Section spec=section>
            <rs_dean_ui::Container spec=container>
                <article
                    class="w-full"
                    data-block=slug
                    data-block-pattern=pattern
                    data-block-layout=layout
                >
                    <Grid spec=grid>{children}</Grid>
                </article>
            </rs_dean_ui::Container>
        </Section>
    }
    .into_any()
}

fn block_copy_view(plan: &BlockPlan, surface: SectionSurface) -> AnyView {
    let content = plan.content.clone();
    let item = plan.content_item;
    let stack = plan.content_stack;
    let cluster = plan.action_cluster;
    let align = if matches!(
        plan.layout,
        crate::BlockLayoutPreset::Centered | crate::BlockLayoutPreset::CenteredNarrow
    ) {
        TextAlign::Center
    } else {
        TextAlign::Start
    };
    let on_strong_surface = matches!(surface, SectionSurface::Brand | SectionSurface::Contrast);
    let strong_tone = if surface == SectionSurface::Contrast {
        TextTone::NeutralContent
    } else {
        TextTone::OnBrand
    };
    let eyebrow_tone = if on_strong_surface {
        strong_tone
    } else {
        TextTone::Brand
    };
    let copy_tone = if on_strong_surface {
        strong_tone
    } else {
        TextTone::Secondary
    };
    let title_tone = if on_strong_surface {
        strong_tone
    } else {
        TextTone::Primary
    };
    let heading_style = if plan.pattern == BlockPattern::Hero {
        TextStyle::Display
    } else {
        TextStyle::Title
    };

    view! {
        <GridItem spec=item>
            <Stack spec=stack>
                <UiText
                    text=content.eyebrow
                    element=rs_dean_ui::TextElement::Paragraph
                    style=TextStyle::Eyebrow
                    tone=eyebrow_tone
                    align
                />
                <UiHeading
                    text=content.title
                    level=HeadingLevel::Two
                    style=heading_style
                    tone=title_tone
                    align
                />
                <UiText text=content.body style=TextStyle::Lead tone=copy_tone align />
                <Cluster spec=cluster>
                    {content
                        .actions
                        .into_iter()
                        .map(block_action_view)
                        .collect_view()}
                </Cluster>
            </Stack>
        </GridItem>
    }
    .into_any()
}

fn block_action_view(action: BlockAction) -> AnyView {
    let mut model = ButtonModel::new(action.label, action.value)
        .with_variant(action.variant)
        .with_size(action.size)
        .without_icon();
    if let Some(href) = action.href {
        model = model.with_kind(ButtonKind::Link).as_link(href);
    }
    view! { <Button model /> }.into_any()
}

fn block_support_view(plan: &BlockPlan) -> AnyView {
    let item = plan.support_item;
    let component = plan.primary_component;
    let media_label = plan.content.media_label.clone();
    let items = plan.content.items.clone();
    let collection_grid = plan.collection_grid;
    let media = plan.media.is_visible().then(|| {
        let label = format!("{} - {}", plan.media.label(), media_label);
        view! { <UiMediaFrame label ratio=plan.media.ratio() /> }.into_any()
    });
    let primary = plan
        .shows_primary_component()
        .then(|| view! { <ComponentDemo id=component /> }.into_any());

    let body = if plan.shows_collection_items() {
        view! {
            <Grid spec=collection_grid>
                {items.into_iter().map(block_item_view).collect_view()}
            </Grid>
        }
        .into_any()
    } else {
        ().into_any()
    };

    view! {
        <GridItem spec=item>
            <Stack>
                {media}
                {body}
                {primary}
            </Stack>
        </GridItem>
    }
    .into_any()
}

fn block_item_view(item: BlockItem) -> AnyView {
    view! { <Card model=item.card_model() /> }.into_any()
}
