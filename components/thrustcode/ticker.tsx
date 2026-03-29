export function Ticker() {
  const items = [
    "Performance extreme",
    "Securite memoire by design",
    "Green IT - -98% vs Python",
    "NSA & CISA recommandent Rust",
    "AWS - Microsoft - Android - Linux",
    "Zero-cost abstractions",
    "Borrow checker - pas de GC",
  ]

  return (
    <div className="bg-white/[0.03] border-y border-[var(--glass-border)] py-[0.8rem] overflow-hidden whitespace-nowrap">
      <div className="inline-flex animate-ticker">
        {[...items, ...items].map((item, index) => (
          <span 
            key={index}
            className="text-[0.75rem] font-normal text-[var(--t3)] tracking-[0.06em] px-10 inline-flex items-center gap-4"
          >
            {item}
            <span className="w-[3px] h-[3px] bg-[var(--apple-blue-l)] rounded-full flex-shrink-0" />
          </span>
        ))}
      </div>
    </div>
  )
}
