"use client"

export function Hero() {
  const stats = [
    { value: "60", suffix: "x", label: "Plus rapide que Python" },
    { value: "98", prefix: "-", suffix: "%", label: "Energie vs Python" },
    { value: "70", prefix: "-", suffix: "%", label: "Surface vulnerabilites" },
    { value: "10", suffix: "+", label: "Ans langue preferee devs" },
  ]

  return (
    <section className="min-h-screen flex flex-col items-center justify-center px-8 pt-32 pb-24 text-center relative overflow-hidden" id="hero">
      {/* Mesh background orbs */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div 
          className="absolute w-[700px] h-[700px] rounded-full blur-[80px] opacity-35 animate-float"
          style={{ 
            background: 'radial-gradient(circle, rgba(0,113,227,0.18), transparent 70%)',
            top: '-200px',
            left: '-150px'
          }}
        />
        <div 
          className="absolute w-[500px] h-[500px] rounded-full blur-[80px] opacity-35 animate-float-reverse"
          style={{ 
            background: 'radial-gradient(circle, rgba(191,90,242,0.1), transparent 70%)',
            bottom: '0',
            right: '-100px'
          }}
        />
        <div 
          className="absolute w-[300px] h-[300px] rounded-full blur-[80px] opacity-35 animate-float-delay"
          style={{ 
            background: 'radial-gradient(circle, rgba(48,209,88,0.08), transparent 70%)',
            bottom: '20%',
            left: '15%'
          }}
        />
      </div>

      {/* Eyebrow */}
      <div 
        className="inline-flex items-center gap-[7px] bg-[var(--glass-bg)] border border-[var(--glass-border)] backdrop-blur-[var(--glass-blur)] rounded-full py-[0.35rem] px-4 text-[0.78rem] font-medium text-[var(--apple-blue-l)] tracking-[0.01em] mb-8 opacity-0 animate-fadeUp"
        style={{ animationDelay: '0.2s', animationFillMode: 'forwards' }}
      >
        <span className="w-[6px] h-[6px] bg-[var(--apple-blue-l)] rounded-full animate-pulse-dot" />
        ESN specialisee Rust - Paris - Worldwide
      </div>

      {/* Title */}
      <h1 
        className="text-[clamp(3rem,7.5vw,7rem)] font-semibold tracking-[-0.04em] leading-[1.0] max-w-[860px] gradient-text opacity-0 animate-fadeUp"
        style={{ animationDelay: '0.35s', animationFillMode: 'forwards' }}
      >
        Construit pour durer. Eprouve pour garantir votre securite.
      </h1>

      {/* Subtitle */}
      <p 
        className="text-[1.1rem] font-light text-[var(--t2)] max-w-[560px] mt-7 leading-[1.65] opacity-0 animate-fadeUp"
        style={{ animationDelay: '0.5s', animationFillMode: 'forwards' }}
      >
        Thrustcode construit des solutions systeme en Rust — performance extreme, securite memoire by design, empreinte energetique reduite de 98%.
      </p>

      {/* Actions */}
      <div 
        className="flex gap-4 justify-center flex-wrap mt-10 opacity-0 animate-fadeUp"
        style={{ animationDelay: '0.65s', animationFillMode: 'forwards' }}
      >
        <button 
          className="bg-[var(--apple-blue)] text-white border-none py-[0.8rem] px-7 rounded-full text-[0.95rem] font-medium tracking-[-0.01em] hover:bg-[var(--apple-blue-l)] hover:-translate-y-[2px] hover:shadow-[0_8px_32px_rgba(204,75,32,0.4)] transition-all"
          onClick={() => document.getElementById('contact')?.scrollIntoView({ behavior: 'smooth' })}
        >
          Demarrer un projet →
        </button>
        <button 
          className="bg-[var(--glass-bg)] text-[var(--t1)] border border-[var(--glass-border)] py-[0.8rem] px-7 rounded-full text-[0.95rem] font-normal tracking-[-0.01em] backdrop-blur-[var(--glass-blur)] hover:bg-[var(--glass-bg-hover)] hover:border-[var(--glass-border-h)] hover:-translate-y-[2px] transition-all"
          onClick={() => document.getElementById('process')?.scrollIntoView({ behavior: 'smooth' })}
        >
          Voir notre approche
        </button>
      </div>

      {/* Stats */}
      <div 
        className="flex flex-col md:flex-row gap-[1px] mt-16 bg-[var(--glass-border)] border border-[var(--glass-border)] rounded-[var(--r-lg)] overflow-hidden backdrop-blur-[var(--glass-blur)] opacity-0 animate-fadeUp"
        style={{ animationDelay: '0.8s', animationFillMode: 'forwards' }}
      >
        {stats.map((stat, index) => (
          <div 
            key={index}
            className="flex-1 py-5 px-7 bg-[var(--glass-bg)] text-center hover:bg-[var(--glass-bg-hover)] transition-colors min-w-[140px]"
          >
            <div className="text-[2rem] font-semibold tracking-[-0.04em] bg-gradient-to-br from-white to-white/70 bg-clip-text text-transparent">
              {stat.prefix}<span className="text-[var(--apple-blue-l)]" style={{ WebkitTextFillColor: 'var(--apple-blue-l)' }}>{stat.value}</span>{stat.suffix}
            </div>
            <div className="text-[0.72rem] font-normal text-[var(--t3)] tracking-[0.04em] mt-[3px] uppercase">
              {stat.label}
            </div>
          </div>
        ))}
      </div>
    </section>
  )
}
