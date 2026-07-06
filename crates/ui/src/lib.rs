use leptos::prelude::*;

const HEALTH_CARD: &str = "rounded-lg border border-slate-200 bg-white p-4 shadow-lg";
const HEALTH_CARD_EYEBROW: &str = "m-0 text-xs font-bold uppercase tracking-widest text-sky-700";
const HEALTH_CARD_TITLE: &str = "mb-0 mt-2 text-xl font-bold text-slate-950";
const HEALTH_CARD_BODY: &str = "mb-0 mt-3 text-sm leading-6 text-slate-600";

#[component]
pub fn HealthCard(title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <section class=HEALTH_CARD>
            <p class=HEALTH_CARD_EYEBROW>"rs-dean"</p>
            <h2 class=HEALTH_CARD_TITLE>{title}</h2>
            <p class=HEALTH_CARD_BODY>{body}</p>
        </section>
    }
}
