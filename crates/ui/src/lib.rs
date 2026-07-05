use leptos::prelude::*;

#[component]
pub fn HealthCard(title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <section class="health-card">
            <p class="health-card__eyebrow">"rs-dean"</p>
            <h2 class="health-card__title">{title}</h2>
            <p class="health-card__body">{body}</p>
        </section>
    }
}
