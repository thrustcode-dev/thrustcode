"use client"

import { useState } from "react"
import { useScrollReveal } from "@/hooks/use-scroll-reveal"

export function FinalCTA() {
  const { ref } = useScrollReveal()
  const [formState, setFormState] = useState<'idle' | 'loading' | 'success' | 'error'>('idle')
  const [formData, setFormData] = useState({
    name: '',
    company: '',
    email: '',
    message: ''
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!formData.name || !formData.email) {
      return
    }

    setFormState('loading')
    
    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1200))
      setFormState('success')
      setFormData({ name: '', company: '', email: '', message: '' })
    } catch {
      setFormState('error')
    }
  }

  return (
    <section className="py-36 px-10 bg-[var(--bg1)] text-center relative overflow-hidden" id="contact">
      {/* Mesh orb */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div 
          className="absolute w-[700px] h-[400px] rounded-full blur-[80px]"
          style={{ 
            background: 'radial-gradient(ellipse, rgba(204,75,32,0.12), transparent 70%)',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)'
          }}
        />
      </div>

      <div 
        ref={ref}
        className="reveal max-w-[640px] mx-auto bg-[var(--glass-bg)] border border-[var(--glass-border)] rounded-[var(--r-xl)] py-14 px-12 backdrop-blur-[var(--glass-blur)] relative z-10"
      >
        {/* Eyebrow */}
        <div className="inline-flex items-center gap-[7px] bg-[var(--glass-bg)] border border-[var(--glass-border)] backdrop-blur-[var(--glass-blur)] rounded-full py-[0.35rem] px-4 text-[0.78rem] font-medium text-[var(--apple-blue-l)] tracking-[0.01em] mb-6 justify-center">
          <span className="w-[6px] h-[6px] bg-[var(--apple-blue-l)] rounded-full animate-pulse-dot" />
          Premier atelier offert - Sans engagement
        </div>

        <h2 className="text-[clamp(2rem,4.5vw,3rem)] font-semibold tracking-[-0.04em] leading-[1.05] mb-4">
          Votre prochain projet<br />en <span className="gradient-text-accent">Rust.</span>
        </h2>

        <p className="text-[0.95rem] text-[var(--t2)] leading-[1.65] mb-8">
          Recevez votre audit de performance gratuit en 48h. Nos experts analysent votre stack actuelle et identifient 3 quick wins concrets — mesures, chiffres, actionnables.
        </p>

        <form onSubmit={handleSubmit} noValidate>
          <div className="flex gap-2 mb-2 flex-col sm:flex-row">
            <input
              type="text"
              placeholder="Votre nom"
              required
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              className="flex-1 bg-white/[0.06] border border-[var(--glass-border)] text-[var(--t1)] py-[0.8rem] px-4 rounded-full text-[0.9rem] outline-none placeholder:text-[var(--t3)] focus:border-[rgba(0,113,227,0.5)] focus:bg-[rgba(204,75,32,0.06)] transition-all"
            />
            <input
              type="text"
              placeholder="Entreprise"
              value={formData.company}
              onChange={(e) => setFormData({ ...formData, company: e.target.value })}
              className="flex-1 bg-white/[0.06] border border-[var(--glass-border)] text-[var(--t1)] py-[0.8rem] px-4 rounded-full text-[0.9rem] outline-none placeholder:text-[var(--t3)] focus:border-[rgba(0,113,227,0.5)] focus:bg-[rgba(204,75,32,0.06)] transition-all"
            />
          </div>

          <div className="flex gap-2 mt-2">
            <input
              type="email"
              placeholder="votre@email.com"
              required
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              className="flex-1 bg-white/[0.06] border border-[var(--glass-border)] text-[var(--t1)] py-[0.8rem] px-4 rounded-full text-[0.9rem] outline-none placeholder:text-[var(--t3)] focus:border-[rgba(0,113,227,0.5)] focus:bg-[rgba(204,75,32,0.06)] transition-all"
            />
          </div>

          <div className="flex gap-2 mt-2">
            <textarea
              placeholder="Decrivez votre stack et vos enjeux (optionnel)"
              value={formData.message}
              onChange={(e) => setFormData({ ...formData, message: e.target.value })}
              className="w-full min-h-[90px] resize-y bg-white/[0.06] border border-[var(--glass-border)] text-[var(--t1)] py-[0.8rem] px-4 rounded-[var(--r-md)] text-[0.9rem] outline-none placeholder:text-[var(--t3)] focus:border-[rgba(0,113,227,0.5)] focus:bg-[rgba(204,75,32,0.06)] transition-all leading-[1.5]"
            />
          </div>

          <div className="mt-3">
            <button
              type="submit"
              disabled={formState === 'loading'}
              className="w-full bg-[var(--apple-blue)] text-white border-none py-[0.9rem] px-6 rounded-full text-[0.95rem] font-medium hover:bg-[var(--apple-blue-l)] hover:shadow-[0_4px_20px_rgba(204,75,32,0.4)] transition-all disabled:opacity-70"
            >
              {formState === 'loading' ? 'Envoi en cours...' : 'Obtenir mon audit gratuit →'}
            </button>
          </div>

          {formState === 'success' && (
            <div className="mt-3 p-[0.85rem] rounded-[var(--r-md)] bg-[rgba(48,209,88,0.08)] border border-[rgba(48,209,88,0.2)] flex items-center gap-[10px] text-[0.85rem] text-[var(--t2)]">
              <span className="text-[var(--apple-green)] text-[1.3rem]">✓</span>
              <span>Message recu ! Nous revenons vers vous dans les 4h ouvrees.</span>
            </div>
          )}

          {formState === 'error' && (
            <div className="mt-3 p-[0.85rem] rounded-[var(--r-md)] bg-[rgba(255,59,48,0.08)] border border-[rgba(255,59,48,0.2)] flex items-center gap-[10px] text-[0.85rem] text-[var(--t2)]">
              Une erreur est survenue. Ecrivez-nous directement a{' '}
              <a href="mailto:contact@thrustcode.dev" className="text-[var(--apple-blue-l)]">
                contact@thrustcode.dev
              </a>
            </div>
          )}
        </form>

        <p className="text-[0.72rem] text-[var(--t3)] tracking-[0.03em] mt-4">
          Reponse &lt;4h ouvrees - Sans engagement - 100% confidentiel
        </p>
      </div>
    </section>
  )
}
