"use client"

import { useScrollReveal } from "@/hooks/use-scroll-reveal"

const steps = [
  {
    num: "01",
    tag: "Decouverte",
    tagColor: "rgba(232,105,42,0.1)",
    tagTextColor: "var(--apple-blue-l)",
    duration: "Semaine 1",
    title: "Diagnostic & cadrage",
    description: "Nous analysons votre contexte en profondeur : stack actuelle, goulots identifies, contraintes reglementaires, equipe en place. Un atelier de 2 jours avec vos equipes techniques et decideurs pour aligner vision et faisabilite.",
    chips: ["Cartographie codebase", "Scoring performance / secu / green", "3 axes prioritaires"],
    visual: "code"
  },
  {
    num: "02",
    tag: "Planification",
    tagColor: "rgba(191,90,242,0.1)",
    tagTextColor: "var(--apple-purple)",
    duration: "Semaine 2",
    title: "Roadmap & proposition technique",
    description: "Sur la base du diagnostic, nous construisons un plan de migration priorise par ROI. Chaque module est evalue selon trois criteres : impact performance, reduction risque securite, gain energetique.",
    chips: ["Roadmap 30 / 60 / 90 jours", "ROI estime par module", "Architecture cible Rust"],
    visual: "bars"
  },
  {
    num: "03",
    tag: "Developpement",
    tagColor: "rgba(48,209,88,0.1)",
    tagTextColor: "var(--apple-green)",
    duration: "Semaines 3 → N",
    title: "Migration incrementale en sprints",
    description: "Nos ingenieurs Rust travaillent module par module, en cohabitation avec votre code existant via FFI. Chaque sprint de 2 semaines livre du code teste, documente et deployable.",
    chips: ["Strangler Fig Pattern", "Sprints de 2 semaines", "Tests non-regression continus", "FFI cohabitation"],
    visual: "sprints"
  },
  {
    num: "04",
    tag: "Validation",
    tagColor: "rgba(255,159,10,0.1)",
    tagTextColor: "var(--apple-orange)",
    duration: "J-14 avant go-live",
    title: "Hardening, tests & mesures",
    description: "Avant chaque mise en production : tests de charge (k6), audit securite (cargo-audit, fuzzing), mesure energetique reelle. Les resultats sont documentes et compares a la baseline initiale.",
    chips: ["Benchmark avant / apres", "cargo-audit + fuzzing", "Mesure CO₂ Scaphandre", "Tests k6 sous charge"],
    visual: "metrics"
  },
  {
    num: "05",
    tag: "Transfert",
    tagColor: "rgba(48,209,88,0.1)",
    tagTextColor: "var(--apple-green)",
    duration: "Post go-live",
    title: "Transfert de competences & suivi",
    description: "Nous ne disparaissons pas apres la livraison. Sessions de pair programming avec vos developpeurs, documentation architecture complete, et suivi de 30 jours post go-live.",
    chips: ["Pair programming sessions", "Documentation complete", "Suivi 30j - SLA garanti"],
    visual: "outcomes",
    isLast: true
  }
]

function CodeVisual() {
  return (
    <div className="flex flex-col gap-[3px]">
      <span className="text-[0.72rem] text-[var(--t4)] font-mono">// Analyse statique en cours...</span>
      <span className="text-[0.78rem] text-[var(--t2)] font-mono">
        <span className="text-[var(--apple-blue-l)]">cargo</span> audit --json | jq <span className="text-[var(--apple-green)]">&apos;.vulnerabilities&apos;</span>
      </span>
      <span className="text-[0.78rem] text-[var(--t2)] font-mono">
        <span className="text-[var(--apple-blue-l)]">cargo</span> clippy <span className="text-[var(--apple-green)]">-- -D warnings -W clippy::all</span>
      </span>
      <span className="text-[0.78rem] text-[var(--t2)] font-mono">
        <span className="text-[var(--apple-blue-l)]">cargo</span> flamegraph <span className="text-[var(--apple-green)]">--bin</span> api_server
      </span>
      <span className="text-[0.72rem] text-[var(--apple-green)] font-mono mt-[6px] pt-2 border-t border-[var(--glass-border)]">
        ✓ 147 modules - 12 points critiques - 3 quick wins identifies
      </span>
    </div>
  )
}

function BarsVisual() {
  const bars = [
    { label: "Auth service", width: "88%", color: "var(--apple-green)", value: "ROI x4.2" },
    { label: "Data pipeline", width: "72%", color: "var(--apple-blue-l)", value: "ROI x3.1" },
    { label: "API gateway", width: "55%", color: "var(--apple-orange)", value: "ROI x2.4" },
    { label: "Cache layer", width: "38%", color: "var(--apple-purple)", value: "ROI x1.8" },
  ]

  return (
    <div className="flex flex-col gap-[9px]">
      {bars.map((bar, i) => (
        <div key={i} className="flex items-center gap-[10px]">
          <span className="text-[0.72rem] text-[var(--t2)] min-w-[88px] font-normal">{bar.label}</span>
          <div className="flex-1 h-[5px] bg-white/[0.06] rounded-[3px] overflow-hidden">
            <div className="h-full rounded-[3px]" style={{ width: bar.width, background: bar.color }} />
          </div>
          <span className="text-[0.72rem] font-medium min-w-[54px] text-right" style={{ color: bar.color }}>{bar.value}</span>
        </div>
      ))}
    </div>
  )
}

function SprintsVisual() {
  const sprints = [
    { status: "done", label: "Sprint 1 - Auth service", badge: "✓ Livre", badgeStyle: "bg-[rgba(48,209,88,0.1)] text-[var(--apple-green)]" },
    { status: "done", label: "Sprint 2 - Data pipeline", badge: "✓ Livre", badgeStyle: "bg-[rgba(48,209,88,0.1)] text-[var(--apple-green)]" },
    { status: "live", label: "Sprint 3 - API gateway", badge: "● En cours", badgeStyle: "bg-[rgba(232,105,42,0.1)] text-[var(--apple-blue-l)]" },
    { status: "wait", label: "Sprint 4 - Cache layer", badge: "Planifie", badgeStyle: "bg-white/5 text-[var(--t3)]" },
    { status: "wait", label: "Sprint 5 - WebSocket service", badge: "Planifie", badgeStyle: "bg-white/5 text-[var(--t3)]" },
  ]

  return (
    <div className="flex flex-col gap-[7px]">
      {sprints.map((sprint, i) => (
        <div key={i} className="flex items-center gap-[9px] text-[0.78rem] text-[var(--t2)]">
          <div 
            className={`w-[7px] h-[7px] rounded-full flex-shrink-0 ${
              sprint.status === 'done' ? 'bg-[var(--apple-green)]' : 
              sprint.status === 'live' ? 'bg-[var(--apple-blue-l)] animate-pulse-dot' : 
              'bg-white/15'
            }`}
          />
          <span>{sprint.label}</span>
          <span className={`ml-auto text-[0.65rem] py-[2px] px-2 rounded-full ${sprint.badgeStyle}`}>
            {sprint.badge}
          </span>
        </div>
      ))}
    </div>
  )
}

function MetricsVisual() {
  return (
    <div className="flex gap-6 flex-wrap">
      <div className="flex items-center gap-3">
        <div className="flex flex-col gap-[2px]">
          <div className="text-[0.62rem] text-[var(--t3)] tracking-[0.06em]">AVANT</div>
          <div className="text-[0.95rem] font-semibold tracking-[-0.02em] text-[var(--t3)] line-through decoration-[rgba(255,59,48,0.5)]">2 840 ms</div>
          <div className="text-[0.62rem] text-[var(--t4)]">latence p99</div>
        </div>
        <div className="text-[var(--t3)] text-[0.9rem]">→</div>
        <div className="flex flex-col gap-[2px]">
          <div className="text-[0.62rem] text-[var(--t3)] tracking-[0.06em]">APRES</div>
          <div className="text-[0.95rem] font-semibold tracking-[-0.02em] text-[var(--apple-green)]">47 ms</div>
          <div className="text-[0.62rem] text-[var(--t4)]">latence p99</div>
        </div>
      </div>
      <div className="flex items-center gap-3">
        <div className="flex flex-col gap-[2px]">
          <div className="text-[0.62rem] text-[var(--t3)] tracking-[0.06em]">AVANT</div>
          <div className="text-[0.95rem] font-semibold tracking-[-0.02em] text-[var(--t3)] line-through decoration-[rgba(255,59,48,0.5)]">18 instances</div>
          <div className="text-[0.62rem] text-[var(--t4)]">EC2 t3.large</div>
        </div>
        <div className="text-[var(--t3)] text-[0.9rem]">→</div>
        <div className="flex flex-col gap-[2px]">
          <div className="text-[0.62rem] text-[var(--t3)] tracking-[0.06em]">APRES</div>
          <div className="text-[0.95rem] font-semibold tracking-[-0.02em] text-[var(--apple-green)]">6 instances</div>
          <div className="text-[0.62rem] text-[var(--t4)]">EC2 t3.large</div>
        </div>
      </div>
    </div>
  )
}

function OutcomesVisual() {
  const outcomes = [
    { icon: "✓", bg: "rgba(48,209,88,0.1)", color: "var(--apple-green)", text: "Equipe autonome sur le code Rust" },
    { icon: "✓", bg: "rgba(232,105,42,0.1)", color: "var(--apple-blue-l)", text: "Documentation architecture livree" },
    { icon: "✓", bg: "rgba(255,159,10,0.1)", color: "var(--apple-orange)", text: "SLA 30 jours post go-live inclus" },
    { icon: "✓", bg: "rgba(191,90,242,0.1)", color: "var(--apple-purple)", text: "Rapport de resultats signe & auditable" },
  ]

  return (
    <div className="flex flex-col gap-[7px]">
      {outcomes.map((outcome, i) => (
        <div key={i} className="flex items-center gap-[10px] text-[0.82rem] text-[var(--t2)]">
          <div 
            className="w-[22px] h-[22px] rounded-[6px] flex items-center justify-center text-[11px] flex-shrink-0"
            style={{ background: outcome.bg, color: outcome.color }}
          >
            {outcome.icon}
          </div>
          {outcome.text}
        </div>
      ))}
    </div>
  )
}

export function Process() {
  const { ref } = useScrollReveal()

  return (
    <section className="py-28 px-10 bg-[var(--bg0)] relative" id="process">
      {/* Mesh orb */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div 
          className="absolute w-[600px] h-[600px] rounded-full blur-[80px] opacity-35 animate-float"
          style={{ 
            background: 'radial-gradient(circle, rgba(48,209,88,0.07), transparent 70%)',
            top: '10%',
            right: '-150px'
          }}
        />
      </div>

      <div className="max-w-[900px] mx-auto relative z-10">
        <div ref={ref} className="reveal text-[0.72rem] font-medium text-[var(--apple-blue-l)] tracking-[0.12em] uppercase mb-3">
          Comment ca marche
        </div>
        <h2 ref={ref} className="reveal text-[clamp(2rem,4vw,3.2rem)] font-semibold tracking-[-0.03em] leading-[1.1]">
          De votre besoin a la <span className="gradient-text-accent">mise en production.</span>
        </h2>
        <p ref={ref} className="reveal text-[1rem] text-[var(--t2)] max-w-[480px] leading-[1.65] mt-[0.85rem]">
          Un processus structure en 5 etapes, concu pour minimiser les risques et maximiser la valeur livree a chaque sprint.
        </p>

        <div ref={ref} className="reveal mt-16 flex flex-col gap-[2px]">
          {steps.map((step, index) => (
            <div key={index} className="grid grid-cols-[56px_1fr] gap-6 pb-10 relative group">
              {/* Connecting line */}
              {!step.isLast && (
                <div 
                  className="absolute left-[27px] top-[52px] bottom-0 w-[1px]"
                  style={{ background: 'linear-gradient(to bottom, var(--glass-border-h), transparent)' }}
                />
              )}

              {/* Number */}
              <div className="flex flex-col items-center pt-[2px]">
                <div 
                  className={`w-11 h-11 rounded-full bg-[var(--glass-bg)] border border-[var(--glass-border)] backdrop-blur-[var(--glass-blur)] flex items-center justify-center text-[0.75rem] font-semibold text-[var(--t2)] flex-shrink-0 transition-all group-hover:bg-[rgba(0,113,227,0.15)] group-hover:border-[rgba(204,75,32,0.4)] group-hover:text-[var(--apple-blue-l)] ${
                    step.isLast ? 'bg-[rgba(48,209,88,0.1)] border-[rgba(48,209,88,0.3)] text-[var(--apple-green)]' : ''
                  }`}
                >
                  {step.num}
                </div>
              </div>

              {/* Content */}
              <div className="pt-2">
                <div>
                  <span 
                    className="inline-block text-[0.68rem] font-medium py-[2px] px-[10px] rounded-full mb-[0.6rem] tracking-[0.04em]"
                    style={{ background: step.tagColor, color: step.tagTextColor }}
                  >
                    {step.tag}
                  </span>
                  <span className="text-[0.72rem] text-[var(--t3)] ml-2">— {step.duration}</span>
                </div>
                
                <h3 className="text-[1.15rem] font-semibold tracking-[-0.02em] mb-2 leading-[1.2]">
                  {step.title}
                </h3>
                
                <p className="text-[0.88rem] text-[var(--t2)] leading-[1.7] max-w-[560px] mb-4">
                  {step.description}
                </p>
                
                <div className="flex flex-wrap gap-[6px] mb-5">
                  {step.chips.map((chip, i) => (
                    <span 
                      key={i}
                      className="text-[0.72rem] text-[var(--t3)] bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-full py-[3px] px-[10px] backdrop-blur-xl"
                    >
                      {chip}
                    </span>
                  ))}
                </div>

                <div className="bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-md)] p-4 px-5 backdrop-blur-[var(--glass-blur)] max-w-[520px]">
                  {step.visual === "code" && <CodeVisual />}
                  {step.visual === "bars" && <BarsVisual />}
                  {step.visual === "sprints" && <SprintsVisual />}
                  {step.visual === "metrics" && <MetricsVisual />}
                  {step.visual === "outcomes" && <OutcomesVisual />}
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* CTA */}
        <div ref={ref} className="reveal mt-14 p-8 px-10 bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-xl)] backdrop-blur-[var(--glass-blur)] flex items-center justify-between flex-wrap gap-4">
          <div>
            <div className="text-[1.15rem] font-semibold tracking-[-0.02em]">Pret a demarrer ?</div>
            <div className="text-[0.78rem] text-[var(--t3)]">Premier atelier de diagnostic offert - Reponse &lt; 4h ouvrees</div>
          </div>
          <div className="flex items-center gap-4">
            <button 
              className="bg-[var(--glass-bg)] text-[var(--t1)] border border-[var(--glass-border)] py-[0.8rem] px-7 rounded-full text-[0.95rem] font-normal backdrop-blur-[var(--glass-blur)] hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-[2px] transition-all"
              onClick={() => document.getElementById('services')?.scrollIntoView({ behavior: 'smooth' })}
            >
              En savoir plus
            </button>
            <button 
              className="bg-[var(--apple-blue)] text-white border-none py-[0.8rem] px-7 rounded-full text-[0.95rem] font-medium hover:bg-[var(--apple-blue-l)] hover:-translate-y-[2px] hover:shadow-[0_8px_32px_rgba(204,75,32,0.4)] transition-all"
              onClick={() => document.getElementById('contact')?.scrollIntoView({ behavior: 'smooth' })}
            >
              Demarrer le diagnostic →
            </button>
          </div>
        </div>
      </div>
    </section>
  )
}
