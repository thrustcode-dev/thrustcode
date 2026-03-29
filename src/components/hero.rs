use leptos::prelude::*;

struct Stat {
    value:  &'static str,
    prefix: &'static str,
    suffix: &'static str,
    label:  &'static str,
}

const STATS: &[Stat] = &[
    Stat { value: "60",  prefix: "",   suffix: "x",  label: "Plus rapide que Python"     },
    Stat { value: "98",  prefix: "-",  suffix: "%",  label: "Energie vs Python"           },
    Stat { value: "70",  prefix: "-",  suffix: "%",  label: "Surface vulnerabilites"      },
    Stat { value: "10",  prefix: "",   suffix: "+",  label: "Ans langue preferee devs"    },
];

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero"
            class="min-h-screen flex flex-col items-center justify-center px-8 pt-32 pb-24 text-center relative overflow-hidden">

            // ── Mesh orbs ──
            <div class="absolute inset-0 overflow-hidden pointer-events-none z-0">
                <div class="absolute w-[700px] h-[700px] rounded-full animate-float"
                     style="background:radial-gradient(circle,rgba(0,113,227,0.18),transparent 70%); filter:blur(80px); opacity:0.35; top:-200px; left:-150px;"/>
                <div class="absolute w-[500px] h-[500px] rounded-full animate-float-reverse"
                     style="background:radial-gradient(circle,rgba(191,90,242,0.10),transparent 70%); filter:blur(80px); opacity:0.35; bottom:0; right:-100px;"/>
                <div class="absolute w-[300px] h-[300px] rounded-full animate-float-delay"
                     style="background:radial-gradient(circle,rgba(48,209,88,0.08),transparent 70%); filter:blur(80px); opacity:0.35; bottom:20%; left:15%;"/>
            </div>

            // ── Eyebrow ──
            <div class="inline-flex items-center gap-[7px] rounded-full py-[0.35rem] px-4 mb-8 opacity-0 animate-fadeUp"
                 style="background:var(--glass-bg); border:1px solid var(--glass-border); backdrop-filter:blur(var(--glass-blur)); font-size:0.78rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.01em; animation-delay:0.2s; animation-fill-mode:forwards;">
                <span class="w-[6px] h-[6px] rounded-full animate-pulse-dot"
                      style="background:var(--apple-blue-l); flex-shrink:0;"/>
                "ESN specialisee Rust \u{2014} Paris \u{2014} Worldwide"
            </div>

            // ── Title ──
            <h1 class="gradient-text font-semibold opacity-0 animate-fadeUp"
                style="font-size:clamp(3rem,7.5vw,7rem); letter-spacing:-0.04em; line-height:1.0; max-width:860px; animation-delay:0.35s; animation-fill-mode:forwards;">
                "Construit pour durer. Eprouve pour garantir votre securite."
            </h1>

            // ── Subtitle ──
            <p class="opacity-0 animate-fadeUp"
               style="font-size:1.1rem; font-weight:300; color:var(--t2); max-width:560px; margin-top:1.75rem; line-height:1.65; animation-delay:0.5s; animation-fill-mode:forwards;">
                "Thrustcode construit des solutions systeme en Rust \u{2014} performance extreme, securite memoire by design, empreinte energetique reduite de 98%."
            </p>

            // ── CTAs ──
            <div class="flex gap-4 justify-center flex-wrap mt-10 opacity-0 animate-fadeUp"
                 style="animation-delay:0.65s; animation-fill-mode:forwards;">
                <a href="#contact"
                   style="background:var(--apple-blue); color:white; border:none; padding:0.8rem 1.75rem; border-radius:var(--r-full); font-size:0.95rem; font-weight:500; letter-spacing:-0.01em; text-decoration:none; transition:all 0.2s;"
                   class="hover:opacity-90 hover:-translate-y-[2px]">
                    "Demarrer un projet \u{2192}"
                </a>
                <a href="#process"
                   style="background:var(--glass-bg); color:var(--t1); border:1px solid var(--glass-border); padding:0.8rem 1.75rem; border-radius:var(--r-full); font-size:0.95rem; font-weight:400; letter-spacing:-0.01em; text-decoration:none; backdrop-filter:blur(var(--glass-blur)); transition:all 0.2s;"
                   class="hover:-translate-y-[2px]">
                    "Voir notre approche"
                </a>
            </div>

            // ── Stats ──
            <div class="flex flex-col md:flex-row mt-16 overflow-hidden opacity-0 animate-fadeUp"
                 style="background:var(--glass-border); border:1px solid var(--glass-border); border-radius:var(--r-lg); backdrop-filter:blur(var(--glass-blur)); gap:1px; animation-delay:0.8s; animation-fill-mode:forwards;">
                {STATS.iter().map(|s| view! {
                    <div class="flex-1 py-5 px-7 text-center"
                         style="background:var(--glass-bg); min-width:140px; transition:background 0.2s;">
                        <div style="font-size:2rem; font-weight:600; letter-spacing:-0.04em;">
                            {s.prefix}
                            <span style="color:var(--apple-blue-l);">{s.value}</span>
                            {s.suffix}
                        </div>
                        <div style="font-size:0.72rem; font-weight:400; color:var(--t3); letter-spacing:0.04em; margin-top:3px; text-transform:uppercase;">
                            {s.label}
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
