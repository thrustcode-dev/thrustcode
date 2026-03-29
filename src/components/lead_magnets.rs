use leptos::prelude::*;

struct Magnet {
    magnet_type: &'static str,
    color:       &'static str,
    title:       &'static str,
    description: &'static str,
    cta:         &'static str,
}

const MAGNETS: &[Magnet] = &[
    Magnet {
        magnet_type: "Outil interactif", color: "var(--apple-blue-l)",
        title: "Calculateur ROI cloud",
        description: "Saisissez votre stack, votre trafic mensuel et votre facture cloud. L'outil calcule les economies potentielles avec Rust (CPU, RAM, \u{20AC}/mois). Resultat envoye par email.",
        cta: "Obtenir mon estimation",
    },
    Magnet {
        magnet_type: "Simulateur CO\u{2082}", color: "var(--apple-green)",
        title: "Empreinte carbone numerique",
        description: "Calcule les tonnes de CO\u{2082} economisees par an en migrant vers Rust. Export PDF avec donnees auditables pour vos rapports CSRD.",
        cta: "Simuler mon impact",
    },
    Magnet {
        magnet_type: "Rapport PDF", color: "var(--apple-purple)",
        title: "Audit securite memoire offert",
        description: "Checklist de 20 verifications basee sur les guidelines CISA/NSA juin 2025. Format PDF brande, genere automatiquement apres soumission.",
        cta: "Telecharger l'audit",
    },
    Magnet {
        magnet_type: "Newsletter", color: "var(--apple-orange)",
        title: "The Rust Report",
        description: "Veille technique et reglementaire Rust bi-mensuelle : benchmarks, nouvelles crates, actualites NSA/CISA, cas d'usage industrie.",
        cta: "S'abonner gratuitement",
    },
    Magnet {
        magnet_type: "Mini-cours", color: "var(--apple-teal)",
        title: "5 jours pour comprendre Rust",
        description: "Email drip de 5 lecons non-techniques pour decideurs DSI/CTO : pourquoi Rust, ce que ca change pour votre infra, les chiffres, les entreprises qui l'ont adopte.",
        cta: "Demarrer la formation",
    },
    Magnet {
        magnet_type: "Audit live", color: "var(--apple-blue-l)",
        title: "Demo 30 min offerte",
        description: "Un expert Thrustcode analyse en direct votre architecture et identifie 3 quick wins Rust concrets. Sans engagement. Reponse <4h ouvrees.",
        cta: "Reserver ma demo",
    },
];

#[component]
pub fn LeadMagnets() -> impl IntoView {
    view! {
        <section id="magnets" style="padding:6rem 2.5rem; background:var(--bg0); position:relative;">
            // Mesh orb
            <div style="position:absolute; inset:0; overflow:hidden; pointer-events:none; z-index:0;">
                <div class="animate-float" style="position:absolute; width:500px; height:500px; border-radius:50%; filter:blur(80px); opacity:0.35; top:0; left:-100px; background:radial-gradient(circle,rgba(232,105,42,0.08),transparent 70%);"/>
            </div>

            <div style="max-width:1100px; margin:0 auto; position:relative; z-index:10;">
                <div class="reveal" style="font-size:0.72rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.12em; text-transform:uppercase; margin-bottom:0.75rem;">
                    "Ressources gratuites"
                </div>
                <h2 class="reveal" style="font-size:clamp(2rem,4vw,3.2rem); font-weight:600; letter-spacing:-0.03em; line-height:1.1; margin:0;">
                    "Commencez par vous "
                    <span class="gradient-text-accent">"convaincre."</span>
                </h2>
                <p class="reveal" style="font-size:1rem; color:var(--t2); max-width:480px; line-height:1.65; margin:0.85rem 0 3.5rem;">
                    "Six outils concrets pour mesurer, avant toute conversation commerciale, ce que Rust peut apporter a votre stack."
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                    {MAGNETS.iter().map(|m| view! {
                        <div class="reveal group"
                             style="background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-lg); padding:1.75rem; backdrop-filter:blur(var(--glass-blur)); display:flex; flex-direction:column; transition:all 0.2s;"
                             class="hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1">

                            <div style=format!("font-size:0.7rem; font-weight:500; letter-spacing:0.1em; text-transform:uppercase; margin-bottom:0.75rem; display:flex; align-items:center; gap:6px; color:{};", m.color)>
                                <div style=format!("width:5px; height:5px; border-radius:50%; flex-shrink:0; background:{};", m.color)/>
                                {m.magnet_type}
                            </div>

                            <div style="font-size:1rem; font-weight:600; letter-spacing:-0.02em; margin-bottom:0.5rem; line-height:1.3;">
                                {m.title}
                            </div>

                            <p style="font-size:0.83rem; color:var(--t2); line-height:1.6; flex:1; margin:0;">
                                {m.description}
                            </p>

                            <div style="font-size:0.8rem; color:var(--apple-blue-l); margin-top:1.25rem; padding-top:0.85rem; border-top:1px solid var(--glass-border); display:flex; align-items:center; gap:5px; transition:gap 0.2s; cursor:pointer;"
                                 class="group-hover:gap-2">
                                {m.cta}" \u{2192}"
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
