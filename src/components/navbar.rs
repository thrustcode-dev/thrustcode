use leptos::prelude::*;

const NAV_LINKS: &[(&str, &str)] = &[
    ("#pillars",  "Piliers"),
    ("#process",  "Processus"),
    ("#services", "Services"),
    ("#magnets",  "Ressources"),
    ("#contact",  "Contact"),
];

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <nav class="fixed top-0 left-0 right-0 z-[200] flex items-center justify-between px-6 md:px-10 h-[52px] border-b"
             style="background: rgba(0,0,0,0.70); backdrop-filter: blur(20px) saturate(180%); border-color: var(--glass-border);">

            // ── Logo ──
            <a href="#" style="display:flex; align-items:center; gap:7px; font-size:1rem; font-weight:600; letter-spacing:-0.02em; color:var(--t1); text-decoration:none;">
                <div style="width:22px; height:22px; background:var(--apple-blue); border-radius:6px; display:flex; align-items:center; justify-content:center;">
                    <svg viewBox="0 0 12 12" style="width:12px; height:12px; fill:white;">
                        <path d="M6 1L11 4v4L6 11 1 8V4z"/>
                    </svg>
                </div>
                "thrustcode"
            </a>

            // ── Desktop links ──
            <ul class="hidden md:flex gap-0 list-none m-0 p-0">
                {NAV_LINKS.iter().map(|(href, label)| view! {
                    <li>
                        <a href=*href
                           style="font-size:0.8rem; font-weight:400; color:var(--t2); padding:0 0.85rem; letter-spacing:-0.01em; text-decoration:none; transition:color 0.2s;"
                           class="hover:text-white">
                            {*label}
                        </a>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>

            // ── CTA + burger ──
            <div class="flex items-center gap-4">
                <a href="#contact"
                   style="background:var(--apple-blue); color:white; border:none; padding:0.45rem 1.1rem; border-radius:var(--r-full); font-weight:600; font-size:0.82rem; letter-spacing:-0.01em; text-decoration:none; transition:all 0.2s; cursor:pointer;"
                   class="hover:opacity-90 hover:scale-[1.03]">
                    "Audit gratuit"
                </a>

                <button
                    class="md:hidden"
                    style="background:transparent; border:none; color:var(--t1); cursor:pointer; font-size:1.3rem; padding:0; line-height:1;"
                    on:click=move |_| set_is_open.update(|v| *v = !*v)>
                    {move || if is_open.get() { "✕" } else { "☰" }}
                </button>
            </div>

            // ── Mobile menu ──
            <Show when=move || is_open.get()>
                <div class="absolute top-[52px] left-0 right-0 md:hidden"
                     style="background:rgba(0,0,0,0.95); backdrop-filter:blur(20px); border-bottom:1px solid var(--glass-border);">
                    <ul class="flex flex-col py-4 list-none m-0 p-0">
                        {NAV_LINKS.iter().map(|(href, label)| {
                            let set_is_open = set_is_open;
                            view! {
                                <li>
                                    <a href=*href
                                       on:click=move |_| set_is_open.set(false)
                                       style="display:block; padding:0.75rem 1.5rem; color:var(--t2); text-decoration:none; transition:all 0.2s;"
                                       class="hover:text-white">
                                        {*label}
                                    </a>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </Show>
        </nav>
    }
}
