import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { Analytics } from '@vercel/analytics/next'
import './globals.css'

const inter = Inter({ 
  subsets: ['latin'],
  variable: '--font-inter'
})

export const metadata: Metadata = {
  title: 'Thrustcode — Performance · Security · Green IT',
  description: 'ESN spécialisée Rust. Migration de goulots, sécurité mémoire, Green IT. Audit de performance gratuit en 48h.',
  openGraph: {
    type: 'website',
    url: 'https://thrustcode.dev/',
    title: 'Thrustcode — Performance · Security · Green IT',
    description: 'ESN spécialisée Rust. Migration de goulots, sécurité mémoire, Green IT. Audit de performance gratuit en 48h.',
    images: ['https://thrustcode.dev/og-image.png'],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Thrustcode — Performance · Security · Green IT',
    description: 'ESN spécialisée Rust. Migration de goulots, sécurité mémoire, Green IT. Audit de performance gratuit en 48h.',
    images: ['https://thrustcode.dev/og-image.png'],
  },
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="fr">
      <body className={`${inter.variable} font-sans antialiased`}>
        {children}
        <Analytics />
      </body>
    </html>
  )
}
