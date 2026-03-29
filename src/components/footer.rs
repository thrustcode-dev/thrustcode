use leptos::prelude::*;

const SERVICES_LINKS: &[(&str, &str)] = &[
    ("#services", "Performance"),
    ("#services", "Securite"),
    ("#services", "Green IT"),
    ("#services", "Embedded"),
    ("#services", "Refactoring"),
];

const RESOURCES_LINKS: &[(&str, &str)] = &[
    ("#", "Blog"),
    ("#", "Academy"),
    ("#", "Case studies"),
    ("#magnets", "The Rust Report"),
];

const COMPANY_LINKS: &[(&str, &str)] = &[
    ("#", "A propos"),
    ("#", "Manifeste"),
    ("#contact", "Contact"),
    ("#", "GitHub"),
];

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12"
                 style="background:var(--bg0); border-top:1px solid var(--glass-border); padding:3rem 2.5rem;">

                // ── Logo & description ──
                <div>
                    <a href="#" style="display:inline-flex; align-items:center; gap:7px; font-size:1rem; font-weight:600; letter-spacing:-0.02em; color:var(--t1); text-decoration:none; margin-bottom:0.75rem;">
                        <div style="width:22px; height:22px; background:var(--apple-blue); border-radius:6px; display:flex; align-items:center; justify-content:center;">
                            <svg viewBox="0 0 12 12" style="width:12px; height:12px; fill:white;">
                                <path d="M6 1L11 4v4L6 11 1 8V4z"/>
                            </svg>
                        </div>
                        "thrustcode"
                    </a>
                    <p style="font-size:0.83rem; color:var(--t3); line-height:1.6; max-width:240px; margin:0;">
                        "ESN specialisee Rust. Performance extreme, securite memoire, Green IT. Basee a Paris, active a l'international."
                    </p>
                </div>

                // ── Services ──
                <div>
                    <h5 style="font-size:0.72rem; font-weight:500; color:var(--t3); letter-spacing:0.1em; text-transform:uppercase; margin:0 0 0.85rem;">
                        "Services"
                    </h5>
                    <ul style="list-style:none; margin:0; padding:0; display:flex; flex-direction:column; gap:0.5rem;">
                        {SERVICES_LINKS.iter().map(|(href, label)| view! {
                            <li>
                                <a href=*href style="font-size:0.85rem; color:var(--t2); text-decoration:none; transition:color 0.2s;"
                                   class="hover:text-white">{*label}</a>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>

                // ── Resources ──
                <div>
                    <h5 style="font-size:0.72rem; font-weight:500; color:var(--t3); letter-spacing:0.1em; text-transform:uppercase; margin:0 0 0.85rem;">
                        "Ressources"
                    </h5>
                    <ul style="list-style:none; margin:0; padding:0; display:flex; flex-direction:column; gap:0.5rem;">
                        {RESOURCES_LINKS.iter().map(|(href, label)| view! {
                            <li>
                                <a href=*href style="font-size:0.85rem; color:var(--t2); text-decoration:none; transition:color 0.2s;"
                                   class="hover:text-white">{*label}</a>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>

                // ── Company ──
                <div>
                    <h5 style="font-size:0.72rem; font-weight:500; color:var(--t3); letter-spacing:0.1em; text-transform:uppercase; margin:0 0 0.85rem;">
                        "Entreprise"
                    </h5>
                    <ul style="list-style:none; margin:0; padding:0; display:flex; flex-direction:column; gap:0.5rem;">
                        {COMPANY_LINKS.iter().map(|(href, label)| view! {
                            <li>
                                <a href=*href style="font-size:0.85rem; color:var(--t2); text-decoration:none; transition:color 0.2s;"
                                   class="hover:text-white">{*label}</a>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </div>

            // ── Bottom bar ──
            <div class="flex flex-col sm:flex-row justify-between items-center gap-2"
                 style="background:var(--bg0); border-top:1px solid var(--glass-border); padding:1.25rem 2.5rem;">
                <span style="font-size:0.72rem; color:var(--t3); letter-spacing:0.04em;">
                    "\u{00A9} 2025 Thrustcode SAS \u{2014} thrustcode.dev"
                </span>
                <span style="font-size:0.72rem; color:var(--t3); letter-spacing:0.04em;">
                    "Built with "
                    <span style="color:var(--apple-blue-l);">"Rust"</span>
                    " \u{2014} Powered by "
                    <span style="color:var(--apple-blue-l);">"Claude"</span>
                </span>
            </div>
        </footer>
    }
}
