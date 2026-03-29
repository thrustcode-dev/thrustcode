use leptos::prelude::*;

struct Testimonial {
    quote:        &'static str,
    name:         &'static str,
    role:         &'static str,
    initials:     &'static str,
    gradient:     &'static str,
    metric:       &'static str,
    metric_label: &'static str,
}

const TESTIMONIALS: &[Testimonial] = &[
    Testimonial {
        quote:        "Thrustcode a reduit notre latence p99 de 2,8s a 47ms en 6 semaines. La methode est rigoureuse, les resultats sont auditables. C'est exactement ce qu'on cherchait.",
        name:         "Jean-Marc Lefebvre",
        role:         "CTO \u{2014} Fintech scale-up \u{2014} Paris",
        initials:     "JM",
        gradient:     "linear-gradient(135deg,var(--apple-blue),var(--apple-purple))",
        metric:       "-98%",
        metric_label: "latence",
    },
    Testimonial {
        quote:        "La migration C++ \u{2192} Rust a elimine toute une classe de vulnerabilites memoire. Le rapport CISA etait exactement ce dont nous avions besoin pour notre audit NIS2.",
        name:         "Sophie Bernard",
        role:         "RSSI \u{2014} Editeur logiciel \u{2014} Lyon",
        initials:     "SB",
        gradient:     "linear-gradient(135deg,var(--apple-purple),var(--apple-teal))",
        metric:       "-70%",
        metric_label: "surface CVE",
    },
    Testimonial {
        quote:        "On est passe de 18 a 6 instances EC2. La facture AWS a baisse de 61% le mois suivant. L'equipe Thrustcode a livre dans les delais, avec une doc complete.",
        name:         "Alexis Renard",
        role:         "VP Engineering \u{2014} SaaS B2B \u{2014} Bordeaux",
        initials:     "AR",
        gradient:     "linear-gradient(135deg,var(--apple-green),var(--apple-teal))",
        metric:       "-61%",
        metric_label: "cout cloud",
    },
];

#[component]
pub fn Testimonials() -> impl IntoView {
    view! {
        <section id="testimonials" style="padding:6rem 2.5rem; background:var(--bg0); position:relative;">
            // Mesh orb
            <div style="position:absolute; inset:0; overflow:hidden; pointer-events:none; z-index:0;">
                <div class="animate-float" style="position:absolute; width:600px; height:400px; border-radius:50%; filter:blur(80px); opacity:0.35; top:0; right:-100px; background:radial-gradient(circle,rgba(191,90,242,0.06),transparent 70%);"/>
            </div>

            <div style="max-width:1100px; margin:0 auto; position:relative; z-index:10;">
                <div class="reveal" style="font-size:0.72rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.12em; text-transform:uppercase; margin-bottom:0.75rem;">
                    "Temoignages"
                </div>
                <h2 class="reveal" style="font-size:clamp(2rem,4vw,3.2rem); font-weight:600; letter-spacing:-0.03em; line-height:1.1; margin:0;">
                    "Ce que disent nos "
                    <span class="gradient-text-accent">"clients."</span>
                </h2>
                <p class="reveal" style="font-size:1rem; color:var(--t2); max-width:480px; line-height:1.65; margin:0.85rem 0 3.5rem;">
                    "Des equipes tech qui ont franchi le pas et mesurent les resultats."
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                    {TESTIMONIALS.iter().map(|t| view! {
                        <div class="reveal"
                             style="background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-lg); padding:2rem; backdrop-filter:blur(var(--glass-blur)); display:flex; flex-direction:column; gap:1.5rem; transition:all 0.2s;"
                             class="hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1">

                            <p style="font-size:0.9rem; color:var(--t2); line-height:1.7; font-style:italic; margin:0; padding-left:1rem; border-left:2px solid var(--apple-blue-l);">
                                {format!("\u{201C}{}\u{201D}", t.quote)}
                            </p>

                            <div class="flex items-center gap-[0.85rem]">
                                <div style=format!("width:38px; height:38px; border-radius:50%; display:flex; align-items:center; justify-content:center; font-size:0.75rem; font-weight:600; color:white; flex-shrink:0; background:{};", t.gradient)>
                                    {t.initials}
                                </div>
                                <div>
                                    <div style="font-size:0.88rem; font-weight:600; letter-spacing:-0.01em;">{t.name}</div>
                                    <div style="font-size:0.72rem; color:var(--t3); margin-top:2px;">{t.role}</div>
                                </div>
                                <div style="margin-left:auto; text-align:center; flex-shrink:0;">
                                    <div class="gradient-text-accent" style="font-size:1.3rem; font-weight:600; letter-spacing:-0.03em;">
                                        {t.metric}
                                    </div>
                                    <div style="font-size:0.62rem; color:var(--t3); letter-spacing:0.06em; text-transform:uppercase;">
                                        {t.metric_label}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
