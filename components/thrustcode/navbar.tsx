"use client"

import { useState } from "react"
import Link from "next/link"
import { Menu, X } from "lucide-react"

export function Navbar() {
  const [isOpen, setIsOpen] = useState(false)

  const navLinks = [
    { href: "#pillars", label: "Piliers" },
    { href: "#process", label: "Processus" },
    { href: "#services", label: "Services" },
    { href: "#magnets", label: "Ressources" },
    { href: "#contact", label: "Contact" },
  ]

  return (
    <nav className="fixed top-0 left-0 right-0 z-[200] flex items-center justify-between px-6 md:px-10 h-[52px] bg-black/70 backdrop-blur-[20px] backdrop-saturate-[180%] border-b border-[var(--glass-border)]">
      <Link href="#" className="flex items-center gap-[7px] text-[1rem] font-semibold tracking-[-0.02em] text-[var(--t1)] no-underline">
        <div className="w-[22px] h-[22px] bg-[var(--apple-blue)] rounded-[6px] flex items-center justify-center">
          <svg viewBox="0 0 12 12" className="w-3 h-3 fill-white">
            <path d="M6 1L11 4v4L6 11 1 8V4z"/>
          </svg>
        </div>
        thrustcode
      </Link>

      {/* Desktop nav */}
      <ul className="hidden md:flex gap-0 list-none">
        {navLinks.map((link) => (
          <li key={link.href}>
            <Link 
              href={link.href}
              className="text-[0.8rem] font-normal text-[var(--t2)] px-[0.85rem] tracking-[-0.01em] hover:text-[var(--t1)] transition-colors"
            >
              {link.label}
            </Link>
          </li>
        ))}
      </ul>

      <div className="flex items-center gap-4">
        <button 
          className="bg-[var(--apple-blue)] text-white border-none py-[0.45rem] px-[1.1rem] rounded-full font-medium text-[0.82rem] tracking-[-0.01em] hover:bg-[var(--apple-blue-l)] hover:scale-[1.03] transition-all"
          onClick={() => document.getElementById('contact')?.scrollIntoView({ behavior: 'smooth' })}
        >
          Audit gratuit
        </button>

        {/* Mobile menu button */}
        <button 
          className="md:hidden text-[var(--t1)]"
          onClick={() => setIsOpen(!isOpen)}
        >
          {isOpen ? <X size={24} /> : <Menu size={24} />}
        </button>
      </div>

      {/* Mobile menu */}
      {isOpen && (
        <div className="absolute top-[52px] left-0 right-0 bg-black/95 backdrop-blur-xl border-b border-[var(--glass-border)] md:hidden">
          <ul className="flex flex-col py-4">
            {navLinks.map((link) => (
              <li key={link.href}>
                <Link 
                  href={link.href}
                  onClick={() => setIsOpen(false)}
                  className="block px-6 py-3 text-[var(--t2)] hover:text-[var(--t1)] hover:bg-[var(--glass-bg)] transition-colors"
                >
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      )}
    </nav>
  )
}
