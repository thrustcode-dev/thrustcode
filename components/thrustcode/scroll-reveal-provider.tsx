"use client"

import { useEffect, type ReactNode } from "react"

interface ScrollRevealProviderProps {
  children: ReactNode
}

export function ScrollRevealProvider({ children }: ScrollRevealProviderProps) {
  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add("visible")
          }
        })
      },
      { threshold: 0.08 }
    )

    // Observe all reveal elements
    const revealElements = document.querySelectorAll(".reveal")
    revealElements.forEach((el) => observer.observe(el))

    return () => {
      revealElements.forEach((el) => observer.unobserve(el))
    }
  }, [])

  return <>{children}</>
}
