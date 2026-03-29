import { Navbar } from "@/components/thrustcode/navbar"
import { Hero } from "@/components/thrustcode/hero"
import { Ticker } from "@/components/thrustcode/ticker"
import { Pillars } from "@/components/thrustcode/pillars"
import { Process } from "@/components/thrustcode/process"
import { Services } from "@/components/thrustcode/services"
import { LeadMagnets } from "@/components/thrustcode/lead-magnets"
import { Testimonials } from "@/components/thrustcode/testimonials"
import { FinalCTA } from "@/components/thrustcode/final-cta"
import { Footer } from "@/components/thrustcode/footer"
import { ScrollRevealProvider } from "@/components/thrustcode/scroll-reveal-provider"
import { CustomCursor } from "@/components/thrustcode/custom-cursor"

export default function Home() {
  return (
    <ScrollRevealProvider>
      <CustomCursor />
      <main>
        <Navbar />
        <Hero />
        <Ticker />
        <Pillars />
        <Process />
        <Services />
        <LeadMagnets />
        <Testimonials />
        <FinalCTA />
        <Footer />
      </main>
    </ScrollRevealProvider>
  )
}
