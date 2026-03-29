use leptos::prelude::*;
use crate::components::{
    navbar::Navbar,
    hero::Hero,
    ticker::Ticker,
    pillars::Pillars,
    process::Process,
    services::Services,
    lead_magnets::LeadMagnets,
    testimonials::Testimonials,
    final_cta::FinalCta,
    footer::Footer,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Navbar />
        <main>
            <Hero />
            <Ticker />
            <Pillars />
            <Process />
            <Services />
            <LeadMagnets />
            <Testimonials />
            <FinalCta />
        </main>
        <Footer />
    }
}
