use leptos::prelude::*;

#[derive(Clone, PartialEq)]
enum FormState { Idle, Loading, Success, Error }

#[component]
pub fn FinalCta() -> impl IntoView {
    let (form_state, set_form_state) = signal(FormState::Idle);
    let (name_val,    set_name)    = signal(String::new());
    let (company_val, set_company) = signal(String::new());
    let (email_val,   set_email)   = signal(String::new());
    let (message_val, set_message) = signal(String::new());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if name_val.get().is_empty() || email_val.get().is_empty() {
            return;
        }

        set_form_state.set(FormState::Loading);

        // Simulate async API call — replace with real fetch to Formspree / backend
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(1200).await;
            set_form_state.set(FormState::Success);
            set_name.set(String::new());
            set_company.set(String::new());
            set_email.set(String::new());
            set_message.set(String::new());
        });
    };

    let input_style = "width:100%; background:rgba(255,255,255,0.06); border:1px solid var(--glass-border); color:var(--t1); padding:0.8rem 1rem; border-radius:var(--r-full); font-size:0.9rem; outline:none; transition:all 0.2s; box-sizing:border-box; font-family:inherit;";
    let textarea_style = "width:100%; min-height:90px; resize:vertical; background:rgba(255,255,255,0.06); border:1px solid var(--glass-border); color:var(--t1); padding:0.8rem 1rem; border-radius:var(--r-md); font-size:0.9rem; outline:none; transition:all 0.2s; box-sizing:border-box; font-family:inherit; line-height:1.5;";

    view! {
        <section id="contact" style="padding:9rem 2.5rem; background:var(--bg1); text-align:center; position:relative; overflow:hidden;">
            // Mesh orb centered
            <div style="position:absolute; inset:0; overflow:hidden; pointer-events:none; z-index:0;">
                <div style="position:absolute; width:700px; height:400px; border-radius:50%; filter:blur(80px); background:radial-gradient(ellipse,rgba(204,75,32,0.12),transparent 70%); top:50%; left:50%; transform:translate(-50%,-50%);"/>
            </div>

            <div class="reveal" style="max-width:640px; margin:0 auto; background:var(--glass-bg); border:1px solid var(--glass-border); border-radius:var(--r-xl); padding:3.5rem 3rem; backdrop-filter:blur(var(--glass-blur)); position:relative; z-index:10;">

                // Eyebrow
                <div style="display:inline-flex; align-items:center; gap:7px; background:var(--glass-bg); border:1px solid var(--glass-border); backdrop-filter:blur(var(--glass-blur)); border-radius:var(--r-full); padding:0.35rem 1rem; font-size:0.78rem; font-weight:500; color:var(--apple-blue-l); letter-spacing:0.01em; margin-bottom:1.5rem;">
                    <span class="animate-pulse-dot" style="width:6px; height:6px; background:var(--apple-blue-l); border-radius:50%;"/>
                    "Premier atelier offert \u{2014} Sans engagement"
                </div>

                <h2 style="font-size:clamp(2rem,4.5vw,3rem); font-weight:600; letter-spacing:-0.04em; line-height:1.05; margin:0 0 1rem;">
                    "Votre prochain projet"<br/>
                    "en " <span class="gradient-text-accent">"Rust."</span>
                </h2>

                <p style="font-size:0.95rem; color:var(--t2); line-height:1.65; margin:0 0 2rem;">
                    "Recevez votre audit de performance gratuit en 48h. Nos experts analysent votre stack actuelle et identifient 3 quick wins concrets \u{2014} mesures, chiffres, actionnables."
                </p>

                <form on:submit=handle_submit novalidate="true">
                    <div class="flex gap-2 mb-2 flex-col sm:flex-row">
                        <input
                            type="text"
                            placeholder="Votre nom"
                            required
                            style=input_style
                            prop:value=name_val
                            on:input=move |e| set_name.set(event_target_value(&e))
                        />
                        <input
                            type="text"
                            placeholder="Entreprise"
                            style=input_style
                            prop:value=company_val
                            on:input=move |e| set_company.set(event_target_value(&e))
                        />
                    </div>

                    <div style="margin:0.5rem 0;">
                        <input
                            type="email"
                            placeholder="votre@email.com"
                            required
                            style=input_style
                            prop:value=email_val
                            on:input=move |e| set_email.set(event_target_value(&e))
                        />
                    </div>

                    <div style="margin:0.5rem 0;">
                        <textarea
                            placeholder="Decrivez votre stack et vos enjeux (optionnel)"
                            style=textarea_style
                            prop:value=message_val
                            on:input=move |e| set_message.set(event_target_value(&e))
                        />
                    </div>

                    <div style="margin-top:0.75rem;">
                        <button
                            type="submit"
                            disabled=move || form_state.get() == FormState::Loading
                            style="width:100%; background:var(--apple-blue); color:white; border:none; padding:0.9rem 1.5rem; border-radius:var(--r-full); font-size:0.95rem; font-weight:500; cursor:pointer; transition:all 0.2s; font-family:inherit;"
                            class="hover:opacity-90 disabled:opacity-70">
                            {move || match form_state.get() {
                                FormState::Loading => "Envoi en cours...".to_string(),
                                _ => "Obtenir mon audit gratuit \u{2192}".to_string(),
                            }}
                        </button>
                    </div>

                    <Show when=move || form_state.get() == FormState::Success>
                        <div style="margin-top:0.75rem; padding:0.85rem; border-radius:var(--r-md); background:rgba(48,209,88,0.08); border:1px solid rgba(48,209,88,0.2); display:flex; align-items:center; gap:10px; font-size:0.85rem; color:var(--t2);">
                            <span style="color:var(--apple-green); font-size:1.3rem;">"\u{2713}"</span>
                            "Message recu ! Nous revenons vers vous dans les 4h ouvrees."
                        </div>
                    </Show>

                    <Show when=move || form_state.get() == FormState::Error>
                        <div style="margin-top:0.75rem; padding:0.85rem; border-radius:var(--r-md); background:rgba(255,59,48,0.08); border:1px solid rgba(255,59,48,0.2); font-size:0.85rem; color:var(--t2);">
                            "Une erreur est survenue. Ecrivez-nous directement a "
                            <a href="mailto:contact@thrustcode.dev" style="color:var(--apple-blue-l);">
                                "contact@thrustcode.dev"
                            </a>
                        </div>
                    </Show>
                </form>

                <p style="font-size:0.72rem; color:var(--t3); letter-spacing:0.03em; margin:1rem 0 0;">
                    "Reponse <4h ouvrees \u{2014} Sans engagement \u{2014} 100% confidentiel"
                </p>
            </div>
        </section>
    }
}
