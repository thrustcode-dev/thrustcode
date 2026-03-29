use leptos::prelude::*;

struct Pillar {
    icon:     &'static str,
    name:     &'static str,
    desc:     &'static str,
    metric:   &'static str,
    color:    &'static str,
    bg_color: &'static str,
}

const PILLARS: &[Pillar] = &[
    Pillar {
        icon:     "\u{26A1}",
        name:     "Performance",
        desc:     "Compilation native, zero runtime, abstractions zero cout. Vos systemes tournent au niveau du silicium. 60x plus rapide que Python sur les taches CPU-intensives.",
        metric:   "2x Go \u{2014} 60x Python \u{2014} latence <15ms @ 1k rps",
        color:    "var(--apple-blue-l)",
        bg_color: "rgba(232,105,42,0.1)",
    },
    Pillar {
        icon:     "\u{1F512}",
        name:     "Securite",
        desc:     "Le borrow checker elimine a la compilation 70% des CVE critiques. Recommande par NSA et CISA depuis 2024. Android est passe de 76% a 24% de bugs memoire.",
        metric:   "-70% surface vulnerabilites \u{2014} conformite NIS2 / DORA",
        color:    "var(--apple-purple)",
        bg_color: "rgba(191,90,242,0.1)",
    },
    Pillar {
        icon:     "\u{1F33F}",
        name:     "Green IT",
        desc:     "-50% vs Java, -98% vs Python en consommation energetique. Moins de serveurs, moins de CO\u{2082}, moins de couts cloud. Donnees mesurees, exportables en CSRD.",
        metric:   "-98% vs Python \u{2014} -50% vs Java \u{2014} CSRD-ready",
        color:    "var(--apple-green)",
        bg_color: "rgba(48,209,88,0.1)",
    },
];

#[component]
pub fn Pillars() -> impl IntoView {
    view! {
        <section id="pillars" style="padding:6rem 2.5rem; background:var(--bg1);">
            <div style="max-width:1100px; margin:0 auto;">
                <div class="reveal" style="font-size:0.72rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.12em; text-transform:uppercase; margin-bottom:0.75rem;">
                    "Nos piliers"
                </div>
                <h2 class="reveal" style="font-size:clamp(2rem,4vw,3.2rem); font-weight:600; letter-spacing:-0.03em; line-height:1.1; margin:0 0 3.5rem;">
                    "Trois raisons de choisir "
                    <span class="gradient-text-accent">"Rust."</span>
                </h2>

                <div class="reveal grid grid-cols-1 md:grid-cols-3 overflow-hidden"
                     style="gap:1px; background:var(--glass-border); border:1px solid var(--glass-border); border-radius:var(--r-xl);">
                    {PILLARS.iter().map(|p| view! {
                        <div style=format!("background:var(--glass-bg); backdrop-filter:blur(var(--glass-blur)); padding:2.5rem 2.5rem 2rem; position:relative; overflow:hidden; transition:background 0.2s;")
                             class="group hover:bg-[var(--glass-bg-hover)]">

                            // Accent line top on hover
                            <div style=format!("position:absolute; top:0; left:0; right:0; height:1px; background:{}; opacity:0; transition:opacity 0.2s;", p.color)
                                 class="group-hover:opacity-100"/>

                            <div style=format!("width:44px; height:44px; border-radius:12px; display:flex; align-items:center; justify-content:center; margin-bottom:1.5rem; font-size:1.3rem; background:{};", p.bg_color)>
                                {p.icon}
                            </div>

                            <div style="font-size:1.1rem; font-weight:600; letter-spacing:-0.02em; margin-bottom:0.65rem;">
                                {p.name}
                            </div>

                            <p style="font-size:0.88rem; color:var(--t2); line-height:1.65; margin:0 0 1.5rem;">
                                {p.desc}
                            </p>

                            <div style=format!("font-size:0.72rem; color:var(--t3); padding-top:1rem; border-top:1px solid var(--glass-border); letter-spacing:0.03em;")>
                                {p.metric}
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
