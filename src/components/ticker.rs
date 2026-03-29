use leptos::prelude::*;

const ITEMS: &[&str] = &[
    "Performance extreme",
    "Securite memoire by design",
    "Green IT \u{2014} -98% vs Python",
    "NSA & CISA recommandent Rust",
    "AWS \u{2014} Microsoft \u{2014} Android \u{2014} Linux",
    "Zero-cost abstractions",
    "Borrow checker \u{2014} pas de GC",
];

#[component]
pub fn Ticker() -> impl IntoView {
    // Duplicate items for seamless loop
    let all_items: Vec<&str> = ITEMS.iter().chain(ITEMS.iter()).copied().collect();

    view! {
        <div style="background:rgba(255,255,255,0.03); border-top:1px solid var(--glass-border); border-bottom:1px solid var(--glass-border); padding:0.8rem 0; overflow:hidden; white-space:nowrap;">
            <div class="inline-flex animate-ticker">
                {all_items.iter().map(|item| view! {
                    <span class="inline-flex items-center gap-4"
                          style="font-size:0.75rem; font-weight:400; color:var(--t3); letter-spacing:0.06em; padding:0 2.5rem;">
                        {*item}
                        <span style="width:3px; height:3px; background:var(--apple-blue-l); border-radius:50%; flex-shrink:0;"/>
                    </span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
