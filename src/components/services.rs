use leptos::prelude::*;

struct Service {
    category:       &'static str,
    category_color: &'static str,
    name:           &'static str,
    is_new:         bool,
    description:    &'static str,
    kpi:            &'static str,
}

const SERVICES: &[Service] = &[
    Service { category: "Performance", category_color: "var(--apple-blue-l)", name: "Migration de goulots", is_new: false,
        description: "Identification et reecriture des chemins critiques (Python/Java/Go \u{2192} Rust). ROI immediat sur CPU et latence.",
        kpi: "-40 a -80% CPU \u{2014} x2 a x10 debit" },
    Service { category: "Performance", category_color: "var(--apple-blue-l)", name: "Refactoring de code vers Rust", is_new: true,
        description: "Modernisation progressive d'un codebase existant par modules. Methode Strangler Fig, zero downtime, tests continus.",
        kpi: "-40 a -70% dette technique \u{2014} couverture tests \u{2265} 80%" },
    Service { category: "Performance", category_color: "var(--apple-blue-l)", name: "Microservices haute performance", is_new: false,
        description: "Services Rust natifs (Actix-web, Axum, Tokio) optimises pour le throughput et la latence p99 sous charge massive.",
        kpi: "60k rps \u{2014} p99 <15ms \u{2014} cold start <30ms" },
    Service { category: "Securite", category_color: "var(--apple-purple)", name: "Audit securite memoire", is_new: false,
        description: "Analyse selon les 20 criteres CISA/NSA 2025. Rapport CVE potentiels, unsafe blocks, surface d'attaque.",
        kpi: "100% codebase \u{2014} livraison <10j \u{2014} NIS2 / DORA" },
    Service { category: "Securite", category_color: "var(--apple-purple)", name: "Migration C/C++ \u{2192} Rust", is_new: false,
        description: "Reecriture securisee de composants critiques exposes reseau. Elimine les classes entieres de CVE memoire.",
        kpi: "-70% surface vulnerabilites \u{2014} conformite NIS2" },
    Service { category: "Green IT", category_color: "var(--apple-green)", name: "Audit energetique numerique", is_new: false,
        description: "Mesure reelle de la consommation de vos workloads (RAPL, Scaphandre). Baseline CO\u{2082} par service, par API, par batch.",
        kpi: "Mesure milliseconde \u{2014} rapport CSRD-ready" },
    Service { category: "Green IT", category_color: "var(--apple-green)", name: "Migration green", is_new: false,
        description: "Remplacement des workloads les plus energivores par des equivalents Rust. Economies mesurees et exportables.",
        kpi: "-50% vs Java \u{2014} -98% vs Python (source AWS)" },
    Service { category: "Transversal", category_color: "var(--apple-teal)", name: "Embedded & firmware Rust", is_new: false,
        description: "Firmware sur microcontroleurs (STM32, ESP32, RISC-V) en Rust bare-metal. Fiabilite et securite sans RTOS.",
        kpi: "Zero unsafe non justifie \u{2014} industrie \u{2014} medical" },
    Service { category: "Transversal", category_color: "var(--apple-teal)", name: "Formation Rust equipes", is_new: false,
        description: "Parcours de montee en competences sur mesure : 3 niveaux (decouverte, intermediaire, expert). Intra ou inter-entreprise.",
        kpi: "3 jours \u{2192} autonomie basique \u{2014} 10j \u{2192} prod-ready" },
];

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <section id="services" style="padding:6rem 2.5rem; background:var(--bg1);">
            <div style="max-width:1100px; margin:0 auto;">
                <div class="reveal" style="font-size:0.72rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.12em; text-transform:uppercase; margin-bottom:0.75rem;">
                    "Nos services"
                </div>
                <h2 class="reveal" style="font-size:clamp(2rem,4vw,3.2rem); font-weight:600; letter-spacing:-0.03em; line-height:1.1; margin:0 0 3.5rem;">
                    "Ce que nous "
                    <span class="gradient-text-accent">"construisons."</span>
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                    {SERVICES.iter().map(|s| view! {
                        <div class="reveal group"
                             style="background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-lg); padding:1.75rem; backdrop-filter:blur(var(--glass-blur)); transition:all 0.2s; position:relative; overflow:hidden;"
                             class="hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1">

                            // Gradient glow on hover
                            <div style="position:absolute; inset:0; background:radial-gradient(circle at 0% 0%,rgba(204,75,32,0.08),transparent 60%); opacity:0; transition:opacity 0.2s; pointer-events:none;"
                                 class="group-hover:opacity-100"/>

                            <div style=format!("font-size:0.7rem; font-weight:500; letter-spacing:0.1em; text-transform:uppercase; margin-bottom:0.6rem; color:{}; position:relative; z-index:1;", s.category_color)>
                                {s.category}
                            </div>

                            <div style="font-size:1rem; font-weight:600; letter-spacing:-0.02em; margin-bottom:0.5rem; line-height:1.3; position:relative; z-index:1;">
                                {s.name}
                                {if s.is_new { view! {
                                    <span style="display:inline-block; font-size:0.65rem; font-weight:500; background:rgba(232,105,42,0.12); color:var(--apple-blue-l); border:1px solid rgba(232,105,42,0.25); border-radius:var(--r-full); padding:2px 8px; margin-left:7px; letter-spacing:0.04em; vertical-align:middle;">
                                        "Nouveau"
                                    </span>
                                }.into_any() } else { view! { <span/> }.into_any() }}
                            </div>

                            <p style="font-size:0.83rem; color:var(--t2); line-height:1.6; margin:0 0 1rem; position:relative; z-index:1;">
                                {s.description}
                            </p>

                            <div style="font-size:0.72rem; color:var(--t3); padding-top:0.85rem; border-top:1px solid var(--glass-border); letter-spacing:0.02em; position:relative; z-index:1;">
                                {s.kpi}
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
