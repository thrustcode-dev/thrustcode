"use client"

import { useScrollReveal } from "@/hooks/use-scroll-reveal"

const services = [
  {
    category: "Performance",
    categoryColor: "var(--apple-blue-l)",
    name: "Migration de goulots",
    description: "Identification et reecriture des chemins critiques (Python/Java/Go → Rust). ROI immediat sur CPU et latence.",
    kpi: "-40 a -80% CPU - x2 a x10 debit"
  },
  {
    category: "Performance",
    categoryColor: "var(--apple-blue-l)",
    name: "Refactoring de code vers Rust",
    isNew: true,
    description: "Modernisation progressive d'un codebase existant par modules. Methode Strangler Fig, zero downtime, tests de non-regression continus.",
    kpi: "-40 a -70% dette technique - couverture tests ≥ 80%"
  },
  {
    category: "Performance",
    categoryColor: "var(--apple-blue-l)",
    name: "Microservices haute performance",
    description: "Services Rust natifs (Actix-web, Axum, Tokio) optimises pour le throughput et la latence p99 sous charge massive.",
    kpi: "60k rps - p99 <15ms - cold start <30ms"
  },
  {
    category: "Securite",
    categoryColor: "var(--apple-purple)",
    name: "Audit securite memoire",
    description: "Analyse selon les 20 criteres CISA/NSA 2025. Rapport CVE potentiels, unsafe blocks, surface d'attaque.",
    kpi: "100% codebase - livraison <10j - NIS2 / DORA"
  },
  {
    category: "Securite",
    categoryColor: "var(--apple-purple)",
    name: "Migration C/C++ → Rust",
    description: "Reecriture securisee de composants critiques exposes reseau. Elimine les classes entieres de CVE memoire.",
    kpi: "-70% surface vulnerabilites - conformite NIS2"
  },
  {
    category: "Green IT",
    categoryColor: "var(--apple-green)",
    name: "Audit energetique numerique",
    description: "Mesure reelle de la consommation de vos workloads (RAPL, Scaphandre). Baseline CO₂ par service, par API, par batch.",
    kpi: "Mesure milliseconde - rapport CSRD-ready"
  },
  {
    category: "Green IT",
    categoryColor: "var(--apple-green)",
    name: "Migration green",
    description: "Remplacement des workloads les plus energivores par des equivalents Rust. Economies mesurees et exportables.",
    kpi: "-50% vs Java - -98% vs Python (source AWS)"
  },
  {
    category: "Transversal",
    categoryColor: "var(--apple-teal)",
    name: "Embedded & firmware Rust",
    description: "Firmware sur microcontroleurs (STM32, ESP32, RISC-V) en Rust bare-metal. Fiabilite et securite sans RTOS.",
    kpi: "Zero unsafe non justifie - industrie - medical"
  },
  {
    category: "Transversal",
    categoryColor: "var(--apple-teal)",
    name: "Formation Rust equipes",
    description: "Parcours de montee en competences sur mesure : 3 niveaux (decouverte, intermediaire, expert). Intra ou inter-entreprise.",
    kpi: "3 jours → autonomie basique - 10j → prod-ready"
  }
]

export function Services() {
  const { ref } = useScrollReveal()

  return (
    <section className="py-24 px-10 bg-[var(--bg1)]" id="services">
      <div className="max-w-[1100px] mx-auto">
        <div ref={ref} className="reveal text-[0.72rem] font-medium text-[var(--apple-blue-l)] tracking-[0.12em] uppercase mb-3">
          Nos services
        </div>
        <h2 ref={ref} className="reveal text-[clamp(2rem,4vw,3.2rem)] font-semibold tracking-[-0.03em] leading-[1.1]">
          Ce que nous <span className="gradient-text-accent">construisons.</span>
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-14">
          {services.map((service, index) => (
            <div 
              key={index}
              ref={ref}
              className="reveal bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-lg)] p-7 backdrop-blur-[var(--glass-blur)] transition-all hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1 relative overflow-hidden group"
            >
              {/* Gradient on hover */}
              <div 
                className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
                style={{ background: 'radial-gradient(circle at 0% 0%, rgba(204,75,32,0.08), transparent 60%)' }}
              />

              <div 
                className="text-[0.7rem] font-medium tracking-[0.1em] uppercase mb-[0.6rem] relative z-10"
                style={{ color: service.categoryColor }}
              >
                {service.category}
              </div>

              <div className="text-[1rem] font-semibold tracking-[-0.02em] mb-2 leading-[1.3] relative z-10">
                {service.name}
                {service.isNew && (
                  <span className="inline-block text-[0.65rem] font-medium bg-[rgba(232,105,42,0.12)] text-[var(--apple-blue-l)] border border-[rgba(232,105,42,0.25)] rounded-full py-[2px] px-2 ml-[7px] tracking-[0.04em] align-middle">
                    Nouveau
                  </span>
                )}
              </div>

              <p className="text-[0.83rem] text-[var(--t2)] leading-[1.6] mb-4 relative z-10">
                {service.description}
              </p>

              <div className="text-[0.72rem] text-[var(--t3)] pt-[0.85rem] border-t border-[var(--glass-border)] tracking-[0.02em] relative z-10">
                {service.kpi}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
