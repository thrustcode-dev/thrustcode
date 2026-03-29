"use client"

import { useScrollReveal } from "@/hooks/use-scroll-reveal"

const magnets = [
  {
    type: "Outil interactif",
    color: "var(--apple-blue-l)",
    title: "Calculateur ROI cloud",
    description: "Saisissez votre stack, votre trafic mensuel et votre facture cloud. L'outil calcule les economies potentielles avec Rust (CPU, RAM, €/mois). Resultat envoye par email.",
    cta: "Obtenir mon estimation"
  },
  {
    type: "Simulateur CO₂",
    color: "var(--apple-green)",
    title: "Empreinte carbone numerique",
    description: "Calcule les tonnes de CO₂ economisees par an en migrant vers Rust. Export PDF avec donnees auditables pour vos rapports CSRD. Angle Green IT unique sur le marche ESN.",
    cta: "Simuler mon impact"
  },
  {
    type: "Rapport PDF",
    color: "var(--apple-purple)",
    title: "Audit securite memoire offert",
    description: "Checklist de 20 verifications basee sur les guidelines CISA/NSA juin 2025. Format PDF brande, genere automatiquement apres soumission du formulaire.",
    cta: "Telecharger l'audit"
  },
  {
    type: "Newsletter",
    color: "var(--apple-orange)",
    title: "The Rust Report",
    description: "Veille technique et reglementaire Rust bi-mensuelle : benchmarks, nouvelles crates, actualites NSA/CISA, cas d'usage industrie. Sequence d'onboarding en 5 emails.",
    cta: "S'abonner gratuitement"
  },
  {
    type: "Mini-cours",
    color: "var(--apple-teal)",
    title: "5 jours pour comprendre Rust",
    description: "Email drip de 5 lecons non-techniques pour decideurs DSI/CTO : pourquoi Rust, ce que ca change pour votre infra, les chiffres, les entreprises qui l'ont adopte.",
    cta: "Demarrer la formation"
  },
  {
    type: "Audit live",
    color: "var(--apple-blue-l)",
    title: "Demo 30 min offerte",
    description: "Un expert Thrustcode analyse en direct votre architecture et identifie 3 quick wins Rust concrets. Sans engagement. Booking Cal.com integre. Reponse <4h ouvrees.",
    cta: "Reserver ma demo"
  }
]

export function LeadMagnets() {
  const { ref } = useScrollReveal()

  return (
    <section className="py-24 px-10 bg-[var(--bg0)] relative" id="magnets">
      {/* Mesh orb */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div 
          className="absolute w-[500px] h-[500px] rounded-full blur-[80px] opacity-35 animate-float"
          style={{ 
            background: 'radial-gradient(circle, rgba(232,105,42,0.08), transparent 70%)',
            top: '0',
            left: '-100px'
          }}
        />
      </div>

      <div className="max-w-[1100px] mx-auto relative z-10">
        <div ref={ref} className="reveal text-[0.72rem] font-medium text-[var(--apple-blue-l)] tracking-[0.12em] uppercase mb-3">
          Ressources gratuites
        </div>
        <h2 ref={ref} className="reveal text-[clamp(2rem,4vw,3.2rem)] font-semibold tracking-[-0.03em] leading-[1.1]">
          Commencez par vous <span className="gradient-text-accent">convaincre.</span>
        </h2>
        <p ref={ref} className="reveal text-[1rem] text-[var(--t2)] max-w-[480px] leading-[1.65] mt-[0.85rem]">
          Six outils concrets pour mesurer, avant toute conversation commerciale, ce que Rust peut apporter a votre stack.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-14">
          {magnets.map((magnet, index) => (
            <div 
              key={index}
              ref={ref}
              className="reveal bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-lg)] p-7 backdrop-blur-[var(--glass-blur)] flex flex-col transition-all hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1 relative overflow-hidden group"
            >
              <div 
                className="text-[0.7rem] font-medium tracking-[0.1em] uppercase mb-3 flex items-center gap-[6px]"
                style={{ color: magnet.color }}
              >
                <div 
                  className="w-[5px] h-[5px] rounded-full flex-shrink-0"
                  style={{ background: magnet.color }}
                />
                {magnet.type}
              </div>

              <div className="text-[1rem] font-semibold tracking-[-0.02em] mb-2 leading-[1.3]">
                {magnet.title}
              </div>

              <p className="text-[0.83rem] text-[var(--t2)] leading-[1.6] flex-1">
                {magnet.description}
              </p>

              <div 
                className="text-[0.8rem] text-[var(--apple-blue-l)] mt-5 pt-[0.85rem] border-t border-[var(--glass-border)] flex items-center gap-[5px] transition-all group-hover:gap-2 cursor-pointer"
              >
                {magnet.cta} <span>→</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
