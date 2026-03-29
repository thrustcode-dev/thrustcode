"use client"

import { useScrollReveal } from "@/hooks/use-scroll-reveal"

const pillars = [
  {
    icon: "⚡",
    name: "Performance",
    description: "Compilation native, zero runtime, abstractions zero cout. Vos systemes tournent au niveau du silicium. 60x plus rapide que Python sur les taches CPU-intensives.",
    metric: "2x Go - 60x Python - latence <15ms @ 1k rps",
    color: "var(--apple-blue-l)",
    bgColor: "rgba(232,105,42,0.1)"
  },
  {
    icon: "🔒",
    name: "Securite",
    description: "Le borrow checker elimine a la compilation 70% des CVE critiques. Recommande par NSA et CISA depuis 2024. Android est passe de 76% a 24% de bugs memoire.",
    metric: "-70% surface vulnerabilites - conformite NIS2 / DORA",
    color: "var(--apple-purple)",
    bgColor: "rgba(191,90,242,0.1)"
  },
  {
    icon: "🌿",
    name: "Green IT",
    description: "-50% vs Java, -98% vs Python en consommation energetique. Moins de serveurs, moins de CO₂, moins de couts cloud. Donnees mesurees, exportables en CSRD.",
    metric: "-98% vs Python - -50% vs Java - CSRD-ready",
    color: "var(--apple-green)",
    bgColor: "rgba(48,209,88,0.1)"
  }
]

export function Pillars() {
  const { ref } = useScrollReveal()

  return (
    <section className="py-24 px-10 bg-[var(--bg1)]" id="pillars">
      <div className="max-w-[1100px] mx-auto">
        <div ref={ref} className="reveal text-[0.72rem] font-medium text-[var(--apple-blue-l)] tracking-[0.12em] uppercase mb-3">
          Nos piliers
        </div>
        <h2 ref={ref} className="reveal text-[clamp(2rem,4vw,3.2rem)] font-semibold tracking-[-0.03em] leading-[1.1]">
          Trois raisons de choisir <span className="gradient-text-accent">Rust.</span>
        </h2>

        <div 
          ref={ref} 
          className="reveal mt-14 grid grid-cols-1 md:grid-cols-3 gap-[1.5px] bg-[var(--glass-border)] border border-[var(--glass-border)] rounded-[var(--r-xl)] overflow-hidden"
        >
          {pillars.map((pillar, index) => (
            <div 
              key={index}
              className="bg-[var(--glass-bg)] backdrop-blur-[var(--glass-blur)] p-10 pt-10 pb-8 relative overflow-hidden hover:bg-[var(--glass-bg-hover)] transition-colors group"
              style={{ ['--card-line' as string]: pillar.color }}
            >
              {/* Accent line on hover */}
              <div 
                className="absolute top-0 left-0 right-0 h-[1px] opacity-0 group-hover:opacity-100 transition-opacity"
                style={{ background: pillar.color }}
              />
              
              <div 
                className="w-11 h-11 rounded-xl flex items-center justify-center mb-6 text-[1.3rem]"
                style={{ background: pillar.bgColor }}
              >
                {pillar.icon}
              </div>
              
              <div className="text-[1.1rem] font-semibold tracking-[-0.02em] mb-[0.65rem]">
                {pillar.name}
              </div>
              
              <p className="text-[0.88rem] text-[var(--t2)] leading-[1.65]">
                {pillar.description}
              </p>
              
              <div className="text-[0.72rem] text-[var(--t3)] mt-6 pt-4 border-t border-[var(--glass-border)] tracking-[0.03em]">
                {pillar.metric}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
