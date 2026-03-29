use leptos::prelude::*;

struct Step {
    num:           &'static str,
    tag:           &'static str,
    tag_color:     &'static str,
    tag_text:      &'static str,
    duration:      &'static str,
    title:         &'static str,
    description:   &'static str,
    chips:         &'static [&'static str],
    visual:        StepVisual,
    is_last:       bool,
}

#[derive(Copy, Clone)]
enum StepVisual { Code, Bars, Sprints, Metrics, Outcomes }

const STEPS: &[Step] = &[
    Step {
        num: "01", tag: "Decouverte",   tag_color: "rgba(232,105,42,0.1)", tag_text: "var(--apple-blue-l)",
        duration: "Semaine 1",          title: "Diagnostic & cadrage",
        description: "Nous analysons votre contexte en profondeur : stack actuelle, goulots identifies, contraintes reglementaires, equipe en place. Un atelier de 2 jours avec vos equipes techniques et decideurs.",
        chips: &["Cartographie codebase", "Scoring performance / secu / green", "3 axes prioritaires"],
        visual: StepVisual::Code, is_last: false,
    },
    Step {
        num: "02", tag: "Planification", tag_color: "rgba(191,90,242,0.1)", tag_text: "var(--apple-purple)",
        duration: "Semaine 2",           title: "Roadmap & proposition technique",
        description: "Sur la base du diagnostic, nous construisons un plan de migration priorise par ROI. Chaque module est evalue selon trois criteres : impact performance, reduction risque securite, gain energetique.",
        chips: &["Roadmap 30 / 60 / 90 jours", "ROI estime par module", "Architecture cible Rust"],
        visual: StepVisual::Bars, is_last: false,
    },
    Step {
        num: "03", tag: "Developpement", tag_color: "rgba(48,209,88,0.1)", tag_text: "var(--apple-green)",
        duration: "Semaines 3 \u{2192} N",  title: "Migration incrementale en sprints",
        description: "Nos ingenieurs Rust travaillent module par module, en cohabitation avec votre code existant via FFI. Chaque sprint de 2 semaines livre du code teste, documente et deployable.",
        chips: &["Strangler Fig Pattern", "Sprints de 2 semaines", "Tests non-regression continus", "FFI cohabitation"],
        visual: StepVisual::Sprints, is_last: false,
    },
    Step {
        num: "04", tag: "Validation",    tag_color: "rgba(255,159,10,0.1)", tag_text: "var(--apple-orange)",
        duration: "J-14 avant go-live",  title: "Hardening, tests & mesures",
        description: "Avant chaque mise en production : tests de charge (k6), audit securite (cargo-audit, fuzzing), mesure energetique reelle. Les resultats sont documentes et compares a la baseline initiale.",
        chips: &["Benchmark avant / apres", "cargo-audit + fuzzing", "Mesure CO\u{2082} Scaphandre", "Tests k6 sous charge"],
        visual: StepVisual::Metrics, is_last: false,
    },
    Step {
        num: "05", tag: "Transfert",     tag_color: "rgba(48,209,88,0.1)", tag_text: "var(--apple-green)",
        duration: "Post go-live",        title: "Transfert de competences & suivi",
        description: "Nous ne disparaissons pas apres la livraison. Sessions de pair programming avec vos developpeurs, documentation architecture complete, et suivi de 30 jours post go-live.",
        chips: &["Pair programming sessions", "Documentation complete", "Suivi 30j \u{2014} SLA garanti"],
        visual: StepVisual::Outcomes, is_last: true,
    },
];

fn code_visual() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-[3px] font-mono">
            <span style="font-size:0.72rem; color:var(--t4);">"// Analyse statique en cours..."</span>
            <span style="font-size:0.78rem; color:var(--t2);">
                <span style="color:var(--apple-blue-l);">"cargo"</span>
                " audit --json | jq "
                <span style="color:var(--apple-green);">"'.vulnerabilities'"</span>
            </span>
            <span style="font-size:0.78rem; color:var(--t2);">
                <span style="color:var(--apple-blue-l);">"cargo"</span>
                " clippy "
                <span style="color:var(--apple-green);">"-- -D warnings -W clippy::all"</span>
            </span>
            <span style="font-size:0.78rem; color:var(--t2);">
                <span style="color:var(--apple-blue-l);">"cargo"</span>
                " flamegraph "
                <span style="color:var(--apple-green);">"--bin"</span>
                " api_server"
            </span>
            <span style="font-size:0.72rem; color:var(--apple-green); margin-top:6px; padding-top:8px; border-top:1px solid var(--glass-border);">
                "\u{2713} 147 modules \u{2014} 12 points critiques \u{2014} 3 quick wins identifies"
            </span>
        </div>
    }
}

fn bars_visual() -> impl IntoView {
    let bars = &[
        ("Auth service",  "88%", "var(--apple-green)",  "ROI x4.2"),
        ("Data pipeline", "72%", "var(--apple-blue-l)", "ROI x3.1"),
        ("API gateway",   "55%", "var(--apple-orange)", "ROI x2.4"),
        ("Cache layer",   "38%", "var(--apple-purple)", "ROI x1.8"),
    ];
    view! {
        <div class="flex flex-col gap-[9px]">
            {bars.iter().map(|(label, width, color, value)| view! {
                <div class="flex items-center gap-[10px]">
                    <span style="font-size:0.72rem; color:var(--t2); min-width:88px;">{*label}</span>
                    <div style="flex:1; height:5px; background:rgba(255,255,255,0.06); border-radius:3px; overflow:hidden;">
                        <div style=format!("width:{}; height:100%; border-radius:3px; background:{};", width, color)/>
                    </div>
                    <span style=format!("font-size:0.72rem; font-weight:500; min-width:54px; text-align:right; color:{};", color)>{*value}</span>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

fn sprints_visual() -> impl IntoView {
    let sprints = &[
        ("done", "Sprint 1 \u{2014} Auth service",       "\u{2713} Livre",   "background:rgba(48,209,88,0.1); color:var(--apple-green);"),
        ("done", "Sprint 2 \u{2014} Data pipeline",      "\u{2713} Livre",   "background:rgba(48,209,88,0.1); color:var(--apple-green);"),
        ("live", "Sprint 3 \u{2014} API gateway",        "\u{25CF} En cours","background:rgba(232,105,42,0.1); color:var(--apple-blue-l);"),
        ("wait", "Sprint 4 \u{2014} Cache layer",        "Planifie",         "background:rgba(255,255,255,0.05); color:var(--t3);"),
        ("wait", "Sprint 5 \u{2014} WebSocket service",  "Planifie",         "background:rgba(255,255,255,0.05); color:var(--t3);"),
    ];
    view! {
        <div class="flex flex-col gap-[7px]">
            {sprints.iter().map(|(status, label, badge, badge_style)| {
                let dot_style = match *status {
                    "done" => "background:var(--apple-green);",
                    "live" => "background:var(--apple-blue-l);",
                    _      => "background:rgba(255,255,255,0.15);",
                };
                view! {
                    <div class="flex items-center gap-[9px]" style="font-size:0.78rem; color:var(--t2);">
                        <div style=format!("width:7px; height:7px; border-radius:50%; flex-shrink:0; {}", dot_style)/>
                        <span>{*label}</span>
                        <span class="ml-auto" style=format!("font-size:0.65rem; padding:2px 8px; border-radius:var(--r-full); {}", badge_style)>
                            {*badge}
                        </span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

fn metrics_visual() -> impl IntoView {
    view! {
        <div class="flex gap-6 flex-wrap">
            <div class="flex items-center gap-3">
                <div class="flex flex-col gap-[2px]">
                    <div style="font-size:0.62rem; color:var(--t3); letter-spacing:0.06em;">"AVANT"</div>
                    <div style="font-size:0.95rem; font-weight:600; color:var(--t3); text-decoration:line-through;">"2 840 ms"</div>
                    <div style="font-size:0.62rem; color:var(--t4);">"latence p99"</div>
                </div>
                <div style="color:var(--t3);">"\u{2192}"</div>
                <div class="flex flex-col gap-[2px]">
                    <div style="font-size:0.62rem; color:var(--t3); letter-spacing:0.06em;">"APRES"</div>
                    <div style="font-size:0.95rem; font-weight:600; color:var(--apple-green);">"47 ms"</div>
                    <div style="font-size:0.62rem; color:var(--t4);">"latence p99"</div>
                </div>
            </div>
            <div class="flex items-center gap-3">
                <div class="flex flex-col gap-[2px]">
                    <div style="font-size:0.62rem; color:var(--t3); letter-spacing:0.06em;">"AVANT"</div>
                    <div style="font-size:0.95rem; font-weight:600; color:var(--t3); text-decoration:line-through;">"18 instances"</div>
                    <div style="font-size:0.62rem; color:var(--t4);">"EC2 t3.large"</div>
                </div>
                <div style="color:var(--t3);">"\u{2192}"</div>
                <div class="flex flex-col gap-[2px]">
                    <div style="font-size:0.62rem; color:var(--t3); letter-spacing:0.06em;">"APRES"</div>
                    <div style="font-size:0.95rem; font-weight:600; color:var(--apple-green);">"6 instances"</div>
                    <div style="font-size:0.62rem; color:var(--t4);">"EC2 t3.large"</div>
                </div>
            </div>
        </div>
    }
}

fn outcomes_visual() -> impl IntoView {
    let outcomes = &[
        ("rgba(48,209,88,0.1)",   "var(--apple-green)",  "Equipe autonome sur le code Rust"),
        ("rgba(232,105,42,0.1)",  "var(--apple-blue-l)", "Documentation architecture livree"),
        ("rgba(255,159,10,0.1)",  "var(--apple-orange)", "SLA 30 jours post go-live inclus"),
        ("rgba(191,90,242,0.1)",  "var(--apple-purple)", "Rapport de resultats signe & auditable"),
    ];
    view! {
        <div class="flex flex-col gap-[7px]">
            {outcomes.iter().map(|(bg, color, text)| view! {
                <div class="flex items-center gap-[10px]" style="font-size:0.82rem; color:var(--t2);">
                    <div style=format!("width:22px; height:22px; border-radius:6px; display:flex; align-items:center; justify-content:center; font-size:11px; flex-shrink:0; background:{}; color:{};", bg, color)>
                        "\u{2713}"
                    </div>
                    {*text}
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn Process() -> impl IntoView {
    view! {
        <section id="process" style="padding:7rem 2.5rem; background:var(--bg0); position:relative;">
            // Mesh orb
            <div style="position:absolute; inset:0; overflow:hidden; pointer-events:none; z-index:0;">
                <div class="animate-float" style="position:absolute; width:600px; height:600px; border-radius:50%; filter:blur(80px); opacity:0.35; top:10%; right:-150px; background:radial-gradient(circle,rgba(48,209,88,0.07),transparent 70%);"/>
            </div>

            <div style="max-width:900px; margin:0 auto; position:relative; z-index:10;">
                <div class="reveal" style="font-size:0.72rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.12em; text-transform:uppercase; margin-bottom:0.75rem;">
                    "Comment ca marche"
                </div>
                <h2 class="reveal" style="font-size:clamp(2rem,4vw,3.2rem); font-weight:600; letter-spacing:-0.03em; line-height:1.1; margin:0;">
                    "De votre besoin a la "
                    <span class="gradient-text-accent">"mise en production."</span>
                </h2>
                <p class="reveal" style="font-size:1rem; color:var(--t2); max-width:480px; line-height:1.65; margin:0.85rem 0 4rem;">
                    "Un processus structure en 5 etapes, concu pour minimiser les risques et maximiser la valeur livree a chaque sprint."
                </p>

                <div class="reveal flex flex-col" style="gap:2px;">
                    {STEPS.iter().map(|step| {
                        let visual = match step.visual {
                            StepVisual::Code     => view! { <div>{code_visual()}</div> }.into_any(),
                            StepVisual::Bars     => view! { <div>{bars_visual()}</div> }.into_any(),
                            StepVisual::Sprints  => view! { <div>{sprints_visual()}</div> }.into_any(),
                            StepVisual::Metrics  => view! { <div>{metrics_visual()}</div> }.into_any(),
                            StepVisual::Outcomes => view! { <div>{outcomes_visual()}</div> }.into_any(),
                        };
                        let num_style = if step.is_last {
                            "background:rgba(48,209,88,0.1); border:1px solid rgba(48,209,88,0.3); color:var(--apple-green);"
                        } else {
                            "background:var(--glass-bg); border:1px solid var(--glass-border); color:var(--t2);"
                        };

                        view! {
                            <div style="display:grid; grid-template-columns:56px 1fr; gap:1.5rem; padding-bottom:2.5rem; position:relative;">
                                // Connector line
                                {if !step.is_last { view! {
                                    <div style="position:absolute; left:27px; top:52px; bottom:0; width:1px; background:linear-gradient(to bottom,var(--glass-border-h),transparent);"/>
                                }.into_any() } else { view! { <span/> }.into_any() }}

                                // Number circle
                                <div style="display:flex; flex-direction:column; align-items:center; padding-top:2px;">
                                    <div style=format!("width:44px; height:44px; border-radius:50%; display:flex; align-items:center; justify-content:center; font-size:0.75rem; font-weight:600; flex-shrink:0; {}", num_style)>
                                        {step.num}
                                    </div>
                                </div>

                                // Content
                                <div style="padding-top:0.5rem;">
                                    <div style="margin-bottom:0.6rem;">
                                        <span style=format!("display:inline-block; font-size:0.68rem; font-weight:500; padding:2px 10px; border-radius:var(--r-full); letter-spacing:0.04em; background:{}; color:{};", step.tag_color, step.tag_text)>
                                            {step.tag}
                                        </span>
                                        <span style="font-size:0.72rem; color:var(--t3); margin-left:0.5rem;">
                                            {format!("\u{2014} {}", step.duration)}
                                        </span>
                                    </div>

                                    <h3 style="font-size:1.15rem; font-weight:600; letter-spacing:-0.02em; margin:0 0 0.5rem; line-height:1.2;">
                                        {step.title}
                                    </h3>

                                    <p style="font-size:0.88rem; color:var(--t2); line-height:1.7; max-width:560px; margin:0 0 1rem;">
                                        {step.description}
                                    </p>

                                    <div style="display:flex; flex-wrap:wrap; gap:6px; margin-bottom:1.25rem;">
                                        {step.chips.iter().map(|chip| view! {
                                            <span style="font-size:0.72rem; color:var(--t3); background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-full); padding:3px 10px;">
                                                {*chip}
                                            </span>
                                        }).collect::<Vec<_>>()}
                                    </div>

                                    <div style="background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-md); padding:1rem 1.25rem; backdrop-filter:blur(var(--glass-blur)); max-width:520px;">
                                        {visual}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Bottom CTA
                <div class="reveal" style="margin-top:3.5rem; padding:2rem 2.5rem; background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-xl); backdrop-filter:blur(var(--glass-blur)); display:flex; align-items:center; justify-content:space-between; flex-wrap:wrap; gap:1rem;">
                    <div>
                        <div style="font-size:1.15rem; font-weight:600; letter-spacing:-0.02em;">"Pret a demarrer ?"</div>
                        <div style="font-size:0.78rem; color:var(--t3);">"Premier atelier de diagnostic offert \u{2014} Reponse < 4h ouvrees"</div>
                    </div>
                    <div class="flex items-center gap-4">
                        <a href="#services" style="background:var(--glass-bg); color:var(--t1); border:1px solid var(--glass-border); padding:0.8rem 1.75rem; border-radius:var(--r-full); font-size:0.95rem; text-decoration:none; transition:all 0.2s;"
                           class="hover:-translate-y-[2px]">
                            "En savoir plus"
                        </a>
                        <a href="#contact" style="background:var(--apple-blue); color:white; border:none; padding:0.8rem 1.75rem; border-radius:var(--r-full); font-size:0.95rem; font-weight:500; text-decoration:none; transition:all 0.2s;"
                           class="hover:opacity-90 hover:-translate-y-[2px]">
                            "Demarrer le diagnostic \u{2192}"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
