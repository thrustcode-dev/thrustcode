import Link from "next/link"

const footerLinks = {
  services: [
    { label: "Performance", href: "#services" },
    { label: "Securite", href: "#services" },
    { label: "Green IT", href: "#services" },
    { label: "Embedded", href: "#services" },
    { label: "Refactoring", href: "#services" },
  ],
  resources: [
    { label: "Blog", href: "#" },
    { label: "Academy", href: "#" },
    { label: "Case studies", href: "#" },
    { label: "The Rust Report", href: "#magnets" },
  ],
  company: [
    { label: "A propos", href: "#" },
    { label: "Manifeste", href: "#" },
    { label: "Contact", href: "#contact" },
    { label: "GitHub", href: "#" },
  ]
}

export function Footer() {
  return (
    <>
      <footer className="bg-[var(--bg0)] border-t border-[var(--glass-border)] py-12 px-10 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12">
        {/* Logo and description */}
        <div>
          <div className="mb-3">
            <Link href="#" className="flex items-center gap-[7px] text-[1rem] font-semibold tracking-[-0.02em] text-[var(--t1)] no-underline">
              <div className="w-[22px] h-[22px] bg-[var(--apple-blue)] rounded-[6px] flex items-center justify-center">
                <svg viewBox="0 0 12 12" className="w-3 h-3 fill-white">
                  <path d="M6 1L11 4v4L6 11 1 8V4z"/>
                </svg>
              </div>
              thrustcode
            </Link>
          </div>
          <p className="text-[0.83rem] text-[var(--t3)] leading-[1.6] max-w-[240px]">
            ESN specialisee Rust. Performance extreme, securite memoire, Green IT. Basee a Paris, active a l&apos;international.
          </p>
        </div>

        {/* Services */}
        <div>
          <h5 className="text-[0.72rem] font-medium text-[var(--t3)] tracking-[0.1em] uppercase mb-[0.85rem]">
            Services
          </h5>
          <ul className="flex flex-col gap-2">
            {footerLinks.services.map((link, index) => (
              <li key={index}>
                <Link href={link.href} className="text-[0.85rem] text-[var(--t2)] no-underline hover:text-[var(--t1)] transition-colors">
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>

        {/* Resources */}
        <div>
          <h5 className="text-[0.72rem] font-medium text-[var(--t3)] tracking-[0.1em] uppercase mb-[0.85rem]">
            Ressources
          </h5>
          <ul className="flex flex-col gap-2">
            {footerLinks.resources.map((link, index) => (
              <li key={index}>
                <Link href={link.href} className="text-[0.85rem] text-[var(--t2)] no-underline hover:text-[var(--t1)] transition-colors">
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>

        {/* Company */}
        <div>
          <h5 className="text-[0.72rem] font-medium text-[var(--t3)] tracking-[0.1em] uppercase mb-[0.85rem]">
            Entreprise
          </h5>
          <ul className="flex flex-col gap-2">
            {footerLinks.company.map((link, index) => (
              <li key={index}>
                <Link href={link.href} className="text-[0.85rem] text-[var(--t2)] no-underline hover:text-[var(--t1)] transition-colors">
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      </footer>

      {/* Footer bottom */}
      <div className="bg-[var(--bg0)] border-t border-[var(--glass-border)] py-5 px-10 flex flex-col sm:flex-row justify-between items-center gap-2">
        <span className="text-[0.72rem] text-[var(--t3)] tracking-[0.04em]">
          © 2025 Thrustcode SAS - thrustcode.dev
        </span>
        <span className="text-[0.72rem] text-[var(--t3)] tracking-[0.04em]">
          Built with <span className="text-[var(--apple-blue-l)]">Rust</span> - Powered by <span className="text-[var(--apple-blue-l)]">Claude</span>
        </span>
      </div>
    </>
  )
}
