"use client"

import { useScrollReveal } from "@/hooks/use-scroll-reveal"

const testimonials = [
  {
    quote: "Thrustcode a reduit notre latence p99 de 2,8s a 47ms en 6 semaines. La methode est rigoureuse, les resultats sont auditables. C'est exactement ce qu'on cherchait.",
    name: "Jean-Marc Lefebvre",
    role: "CTO - Fintech scale-up - Paris",
    initials: "JM",
    gradient: "linear-gradient(135deg, var(--apple-blue), var(--apple-purple))",
    metric: "-98%",
    metricLabel: "latence"
  },
  {
    quote: "La migration C++ → Rust a elimine toute une classe de vulnerabilites memoire. Le rapport CISA etait exactement ce dont nous avions besoin pour notre audit NIS2.",
    name: "Sophie Bernard",
    role: "RSSI - Editeur logiciel - Lyon",
    initials: "SB",
    gradient: "linear-gradient(135deg, var(--apple-purple), var(--apple-teal))",
    metric: "-70%",
    metricLabel: "surface CVE"
  },
  {
    quote: "On est passe de 18 a 6 instances EC2. La facture AWS a baisse de 61% le mois suivant. L'equipe Thrustcode a livre dans les delais, avec une doc complete.",
    name: "Alexis Renard",
    role: "VP Engineering - SaaS B2B - Bordeaux",
    initials: "AR",
    gradient: "linear-gradient(135deg, var(--apple-green), var(--apple-teal))",
    metric: "-61%",
    metricLabel: "cout cloud"
  }
]

export function Testimonials() {
  const { ref } = useScrollReveal()

  return (
    <section className="py-24 px-10 bg-[var(--bg0)] relative" id="testimonials">
      {/* Mesh orb */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div 
          className="absolute w-[600px] h-[400px] rounded-full blur-[80px] opacity-35 animate-float"
          style={{ 
            background: 'radial-gradient(circle, rgba(191,90,242,0.06), transparent 70%)',
            top: '0',
            right: '-100px'
          }}
        />
      </div>

      <div className="max-w-[1100px] mx-auto relative z-10">
        <div ref={ref} className="reveal text-[0.72rem] font-medium text-[var(--apple-blue-l)] tracking-[0.12em] uppercase mb-3">
          Temoignages
        </div>
        <h2 ref={ref} className="reveal text-[clamp(2rem,4vw,3.2rem)] font-semibold tracking-[-0.03em] leading-[1.1]">
          Ce que disent nos <span className="gradient-text-accent">clients.</span>
        </h2>
        <p ref={ref} className="reveal text-[1rem] text-[var(--t2)] max-w-[480px] leading-[1.65] mt-[0.85rem]">
          Des equipes tech qui ont franchi le pas et mesurent les resultats.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-14">
          {testimonials.map((testimonial, index) => (
            <div 
              key={index}
              ref={ref}
              className="reveal bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-lg)] p-8 backdrop-blur-[var(--glass-blur)] flex flex-col gap-6 transition-all hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-1"
            >
              <div className="text-[0.9rem] text-[var(--t2)] leading-[1.7] italic relative pl-4 border-l-2 border-[var(--apple-blue-l)]">
                &quot;{testimonial.quote}&quot;
              </div>

              <div className="flex items-center gap-[0.85rem]">
                <div 
                  className="w-[38px] h-[38px] rounded-full flex items-center justify-center text-[0.75rem] font-semibold text-white flex-shrink-0"
                  style={{ background: testimonial.gradient }}
                >
                  {testimonial.initials}
                </div>
                <div>
                  <div className="text-[0.88rem] font-semibold tracking-[-0.01em]">
                    {testimonial.name}
                  </div>
                  <div className="text-[0.72rem] text-[var(--t3)] mt-[2px]">
                    {testimonial.role}
                  </div>
                </div>
                <div className="ml-auto text-center flex-shrink-0">
                  <div className="text-[1.3rem] font-semibold tracking-[-0.03em] gradient-text-accent">
                    {testimonial.metric}
                  </div>
                  <div className="text-[0.62rem] text-[var(--t3)] tracking-[0.06em] uppercase">
                    {testimonial.metricLabel}
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
