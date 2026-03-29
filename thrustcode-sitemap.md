# thrustcode.dev — Sitemap & Architecture de contenu

> Document de référence · Version 1.0 · Mars 2026
> Domaine : **thrustcode.dev** · ESN spécialisée Rust · Paris

---

## Table des matières

1. [Vue d'ensemble du site](#1-vue-densemble-du-site)
2. [Arborescence complète](#2-arborescence-complète)
3. [Navigation & éléments partagés](#3-navigation--éléments-partagés)
4. [Page — Home `/`](#4-page--home-)
5. [Page — Services `/services`](#5-page--services-services)
6. [Page — Green IT `/services/green-it`](#6-page--green-it-servicesgeen-it)
7. [Page — Études de cas `/case-studies`](#7-page--études-de-cas-case-studies)
8. [Page — Blog `/blog`](#8-page--blog-blog)
9. [Page — Academy `/academy`](#9-page--academy-academy)
10. [Page — À propos `/about`](#10-page--à-propos-about)
11. [Page — Contact `/contact`](#11-page--contact-contact)
12. [Design system](#12-design-system)
13. [SEO & métadonnées](#13-seo--métadonnées)
14. [Performances & technique](#14-performances--technique)

---

## 1. Vue d'ensemble du site

### Objectif du site

Thrustcode.dev est le site vitrine et de conversion de l'ESN. Il poursuit trois objectifs simultanés :

- **Convertir** les visiteurs en leads qualifiés via 6 lead magnets ciblés par persona
- **Crédibiliser** la proposition de valeur par des données mesurées et sourcées
- **Éduquer** les décideurs non-techniques sur les bénéfices de Rust (Performance, Sécurité, Green IT)

### Personas cibles

| Persona | Profil | Besoin principal | Page d'entrée |
|---|---|---|---|
| **CTO / VP Engineering** | Décideur technique | Performance, dette technique | `/services` ou `/case-studies` |
| **RSSI / CISO** | Responsable sécurité | Conformité NIS2/DORA, CVE | `/services` (filtre Sécurité) |
| **DSI / Responsable RSE** | Décideur durabilité | CSRD, bilan carbone | `/services/green-it` |
| **Développeur senior** | Influenceur technique | Fiabilité Rust, benchmarks | `/blog` ou `/academy` |
| **CEO / Fondateur startup** | Décideur budget | ROI, rapidité, coût cloud | `/` ou `/contact` |

### KPIs de conversion

- Taux de soumission formulaire contact : cible ≥ 3%
- Taux d'inscription lead magnets : cible ≥ 8%
- Nombre de démos 30 min bookées par mois : cible 15+
- Taux de rebond global : cible ≤ 45%

---

## 2. Arborescence complète

```
thrustcode.dev/
│
├── /                           → Home (landing principale)
│
├── /services                   → Catalogue services (23 services, 4 piliers)
│   ├── /services/performance   → Performance Engineering (détail)
│   ├── /services/security      → Security by Design (détail)
│   ├── /services/green-it      → Green IT & Durabilité (détail + simulateur)
│   ├── /services/embedded      → Embedded & IoT (détail)
│   └── /services/refactoring   → Refactoring legacy → Rust (détail)
│
├── /case-studies               → Études de cas (6 cases)
│   ├── /case-studies/fintech-payments
│   ├── /case-studies/saas-analytics
│   ├── /case-studies/iot-firmware
│   ├── /case-studies/security-nis2
│   ├── /case-studies/green-ml-pipeline
│   └── /case-studies/streaming-wasm
│
├── /blog                       → Blog & ressources
│   ├── /blog/rust-vs-go-python-2025
│   ├── /blog/nsa-cisa-guide-juin-2025
│   ├── /blog/csrd-scaphandre-guide
│   ├── /blog/strangler-fig-python-rust
│   └── /blog/rust-wasm-cloudflare
│
├── /academy                    → Formation & webinaires
│   ├── /academy/foundations    → Niveau 1 (3 jours)
│   ├── /academy/production     → Niveau 2 (5 jours)
│   ├── /academy/expert         → Niveau 3 (5 jours)
│   ├── /academy/decideurs      → Mini-cours DSI/CTO (email drip)
│   └── /academy/webinaires     → Calendrier sessions live
│
├── /about                      → À propos, équipe, valeurs
│   ├── /about/manifeste
│   └── /about/presse
│
├── /contact                    → Formulaire qualifié + booking
│
└── /legal                      → Pages légales
    ├── /legal/mentions-legales
    ├── /legal/politique-confidentialite
    └── /legal/cgv
```

---

## 3. Navigation & éléments partagés

### Barre de navigation (fixe, glassmorphism)

```
[logo thrustcode.dev]  Services  Green IT  Études de cas  Blog  Academy  À propos  Contact  [Audit gratuit →]
```

**Comportement** :
- Position : `fixed` top, `z-index: 200`
- Background au repos : `rgba(0,0,0,0.72)` + `backdrop-filter: saturate(180%) blur(20px)`
- Hauteur : `52px`
- Lien actif : `color: rgba(255,255,255,0.95)` (vs `0.60` inactif)
- CTA "Audit gratuit" : pill bleu Apple `#0071E3`, hover `#2997FF`
- Curseur custom : dot 8px blanc + anneau 40px semi-transparent, grossit sur hover

**Mobile** (< 768px) :
- Liens masqués → hamburger menu
- CTA conservé visible

### Footer partagé (4 colonnes)

| Colonne 1 | Colonne 2 | Colonne 3 | Colonne 4 |
|---|---|---|---|
| Logo + baseline | Services | Ressources | Entreprise |
| Description courte | Performance | Blog | À propos |
| — | Sécurité | Academy | Manifeste |
| — | Green IT | Études de cas | Contact |
| — | Embedded | The Rust Report | GitHub ↗ |
| — | Refactoring | — | — |

**Footer bottom** : `© 2025 Thrustcode SAS · thrustcode.dev` | `Built with Rust · Powered by conviction`

### CTA band (réutilisable entre sections)

Composant partagé utilisé en fin de chaque page :
- Background : `var(--bg1)` + border top/bottom
- Contenu : titre accroche, sous-titre, 1-2 boutons CTA
- Orbe de mesh gradient centré en arrière-plan

### Animations globales

- **Scroll reveal** : `IntersectionObserver` natif, `threshold: 0.08`, `translateY(20px) → 0`, durée `0.6s`
- **Curseur custom** : dot 8px + anneau lag 40px, transition douce sur hover interactive
- **Ticker** : bande défilante 32s loop `linear`, items séparés par `◆`
- **Orbes mesh** : `filter: blur(80px)`, animation `float` 16-22s, `ease-in-out infinite`
- **Pulse** : points vivants (SLA, eyebrow), `0.5 → 1.4 scale` en 2s

---

## 4. Page — Home `/`

**Objectif** : première impression, crédibilité immédiate, capture email haute conversion

### Sections dans l'ordre

---

#### 4.1 Hero

**Composants** :
- Eyebrow pill glassmorphism : `"ESN spécialisée Rust · Paris · Worldwide"` avec dot pulsé
- Titre principal (gradient text) : `"Logiciels qui ne flanquent jamais."`
- Sous-titre (font-weight 300) : présentation 2 lignes de la proposition de valeur Rust
- Actions : `[Démarrer un projet →]` (bleu Apple) + `[Voir notre approche]` (verre)
- Stats row glassmorphism (4 cellules) :
  - `60×` Plus rapide que Python
  - `−98%` Énergie vs Python
  - `−70%` Surface vulnérabilités
  - `10+` Ans langue préférée devs

**Fond** : 3 orbes mesh (bleu gauche, violet droite bas, vert centre bas)

**Animations** : `fadeUp` staggered (0.2s → 0.8s delay par élément)

---

#### 4.2 Ticker

Bandeau défilant pleine largeur (32s loop) :

```
Performance extrême ◆ Sécurité mémoire by design ◆ Green IT · −98% vs Python ◆
NSA & CISA recommandent Rust ◆ AWS · Microsoft · Android · Linux ◆
Zero-cost abstractions ◆ Borrow checker · pas de GC ◆ [répétition]
```

Style : `font-size: 0.75rem`, `color: rgba(255,255,255,0.35)`, `letter-spacing: 0.06em`

---

#### 4.3 Piliers (3 cards glassmorphism)

**Layout** : grille 3 colonnes, séparateur 1.5px, `border-radius: 30px`, overflow hidden

Chaque carte :
- Icône wrapper `44×44px` arrondi avec couleur pilier en background opacité 10%
- Nom du pilier (font-weight 600)
- Description 3-4 lignes
- Metric footer séparé par border top

| Pilier | Couleur accent | Icône | Metric |
|---|---|---|---|
| Performance | `#2997FF` (bleu) | ⚡ | `2× Go · 60× Python · latence <15ms @ 1k rps` |
| Sécurité | `#BF5AF2` (violet) | 🔒 | `−70% surface vulnérabilités · conformité NIS2 / DORA` |
| Green IT | `#30D158` (vert) | 🌿 | `−98% vs Python · −50% vs Java · CSRD-ready` |

**Interaction hover** : `background: rgba(255,255,255,0.085)`, ligne top accent `opacity: 0 → 1`

---

#### 4.4 Processus (5 étapes — timeline verticale)

**Layout** : grille `56px | 1fr`, connecteur ligne verticale entre les numéros

Chaque étape :
- Numéro circulaire `44×44px` glassmorphism → hover bleu Apple
- Tag coloré (pill) + durée indicative
- Titre étape `font-size: 1.15rem, font-weight: 600`
- Description `font-size: 0.88rem, color: rgba(255,255,255,0.60)`
- Chips (tags) : outils/méthodes utilisés
- Visual card glassmorphism avec contenu spécifique à l'étape

| # | Tag | Durée | Titre | Visual |
|---|---|---|---|---|
| 01 | Découverte (bleu) | Semaine 1 | Diagnostic & cadrage | Terminal `cargo audit` / `cargo clippy` simulé |
| 02 | Planification (violet) | Semaine 2 | Roadmap & proposition technique | Bar chart ROI par module |
| 03 | Développement (vert) | Semaines 3→N | Migration incrémentale en sprints | Liste de sprints avec statuts live/done/planned |
| 04 | Validation (orange) | J−14 go-live | Hardening, tests & mesures | Métriques avant/après (2840ms → 47ms, 18 → 6 instances) |
| 05 | Transfert (vert) | Post go-live | Transfert de compétences & suivi | Liste d'outcomes avec icônes colorées |

**CTA fin de processus** : bandeau glassmorphism `"Prêt à démarrer ?"` + 2 boutons

---

#### 4.5 Services (aperçu 9 cards)

Voir section 5 pour le détail complet. Sur la Home, affichage d'une sélection représentative sans filtre.

---

#### 4.6 Lead magnets (6 cards glassmorphism)

**Layout** : grille `repeat(auto-fill, minmax(320px, 1fr))`

| # | Type | Couleur | Titre | CTA |
|---|---|---|---|---|
| 1 | Outil interactif | Bleu `#2997FF` | Calculateur ROI cloud | `Obtenir mon estimation →` |
| 2 | Simulateur CO₂ | Vert `#30D158` | Empreinte carbone numérique | `Simuler mon impact →` |
| 3 | Rapport PDF | Violet `#BF5AF2` | Audit sécurité mémoire offert | `Télécharger l'audit →` |
| 4 | Newsletter | Orange `#FF9F0A` | The Rust Report | `S'abonner gratuitement →` |
| 5 | Mini-cours | Teal `#5AC8FA` | 5 jours pour comprendre Rust | `Démarrer la formation →` |
| 6 | Audit live | Bleu `#2997FF` | Démo 30 min offerte | `Réserver ma démo →` |

---

#### 4.7 Final CTA

Card glassmorphism centrée `max-width: 640px` :
- Eyebrow : `"Premier atelier offert · Sans engagement"`
- Titre : `"Votre prochain projet en Rust."` (gradient bleu → violet sur "Rust")
- Sous-titre : description audit gratuit 48h
- Formulaire email inline (input + bouton)
- Note légale : `Réponse <4h ouvrées · Sans engagement · 100% confidentiel`

---

## 5. Page — Services `/services`

**Objectif** : présenter les 23 services, permettre le filtrage par pilier, qualifier l'intention du visiteur

### Sections

---

#### 5.1 Hero

- Eyebrow : `"23 services · 4 piliers"`
- Titre : `"Ce que nous construisons."`
- Sous-titre : présentation de l'approche modulaire (audit → migration → livraison)

---

#### 5.2 Filtre dynamique

Barre de filtres pill buttons :

```
[Tous (23)]  [⚡ Performance]  [🔒 Sécurité]  [🌿 Green IT]  [◈ Transversal]  [★ Core]
```

Filtre JS côté client, pas de rechargement. Le bouton actif passe en `background: rgba(255,255,255,0.055)`.

---

#### 5.3 Grille de services

**Layout** : `repeat(auto-fill, minmax(310px, 1fr))`, `gap: 12px`

Chaque service-card affiche :

```
[Couleur pilier] ⚡ Performance       [Badge: Core / Growth / Future]
Nom du service                        [Nouveau] (si applicable)
Description courte (2-3 lignes)
[Chip 1] [Chip 2] [Chip 3]
─────────────────────────────────────
KPI différenciant                     [Niveau]
```

**Catalogue complet par pilier** :

##### ⚡ Performance (7 services)

| Service | Maturité | Chips | KPI |
|---|---|---|---|
| Migration de goulots (Python/Java → Rust) | Core | Profiling · Réécriture hot path · Benchmark | −40 à −80% CPU · ×2 à ×10 débit |
| Refactoring de code vers Rust 🆕 | Core | Strangler Fig · FFI · Tests non-régression | −40 à −70% dette technique · tests ≥80% |
| Microservices haute performance | Core | Actix-web · Axum · Tokio | 60k rps · p99 <15ms · cold start <30ms |
| Traitement de données temps réel | Core | Kafka · NATS · simd JSON | 1 Go JSON en ~2s · ×6 vs Python |
| Audit de performance | Core | cargo flamegraph · perf · Valgrind | Livraison 5j · 3 quick wins garantis |
| WASM & edge computing | Growth | WASM · Cloudflare Workers · Fastly | Latence edge <5ms · bundle léger |
| Optimisation FinOps cloud | Growth | AWS · GCP · Cost analysis | −30 à −60% instances · ROI <6 mois |

##### 🔒 Sécurité (6 services)

| Service | Maturité | Chips | KPI |
|---|---|---|---|
| Audit sécurité mémoire | Core | cargo-audit · cargo-deny · CISA/NSA | 100% codebase · livraison <10j |
| Migration C/C++ → Rust | Core | FFI · cargo-fuzz · unsafe audit | −70% surface CVE · conformité NIS2/DORA |
| Conformité NIS2 & DORA | Core | NIS2 art.21 · DORA RTS · Gap analysis | Conformité certifiable · 2025 |
| DevSecOps Rust | Core | clippy · cargo-deny · SBOM | Zéro faille mémoire · SBOM complet |
| Cryptographie & protocoles | Growth | rustls · ring · HSM | Zéro dépendance C crypto · auditabilité |
| Hyperviseur & sandboxing | Future | WASM sandbox · KVM · isolation | Isolation OS-level · démarrage <125ms |

##### 🌿 Green IT (5 services)

| Service | Maturité | Chips | KPI |
|---|---|---|---|
| Audit énergétique numérique | Core | RAPL · Scaphandre · CO₂ | Mesure milliseconde · rapport CSRD |
| Migration green | Core | Migration ciblée · Mesure avant/après · kWh | −50% vs Java · −98% vs Python |
| Rapport CSRD numérique | Core | CSRD · Scope 1-2-3 · Auditable | Conformité 2025 · données auditables |
| Green badge Thrustcode | Growth | Certification · RSE · Appels d'offres | Preuve chiffrée · valorisable AO |
| Simulateur CO₂ on-demand | Growth | SaaS · API calcul · Export CSRD | Lead magnet · 500 simulations/mois |

##### ◈ Transversal (5 services)

| Service | Maturité | Chips | KPI |
|---|---|---|---|
| Formation Rust équipes | Core | 3 niveaux · Labs pratiques · Certification | 3j → autonomie · 10j → prod-ready |
| Embedded & firmware Rust | Core | STM32 · ESP32 · RISC-V · bare-metal | Zéro unsafe non justifié |
| Architecture & CTO as a Service | Core | Roadmap · Architecture · CTO support | Engagement flexible · dès 2j/mois |
| Blockchain & protocoles | Growth | Solana · Substrate · NEAR | <400ms finalité · audit sécurité inclus |
| IA infrastructure Rust | Future | Inférence · Runtime Rust · Python bindings | ×3 inférence vs Python serving |

**Légende maturité** :
- `Core` → `background: rgba(48,209,88,0.12); color: #30D158` (vert)
- `Growth` → `background: rgba(41,151,255,0.12); color: #2997FF` (bleu)
- `Future` → `background: rgba(255,159,10,0.12); color: #FF9F0A` (orange)

---

#### 5.4 CTA band

```
Titre : "Quel service correspond à votre besoin ?"
Sous-titre : "Discutons-en. Premier diagnostic de 30 min offert, sans engagement."
CTA : [Prendre rendez-vous →]
```

---

## 6. Page — Green IT `/services/green-it`

**Objectif** : convertir les responsables RSE, DAF et DSI sur l'argument CSRD / bilan carbone numérique

### Sections

---

#### 6.1 Hero

- Eyebrow : `"Green IT · CSRD · Numérique responsable"` (dot vert pulsé)
- Titre : `"Moins de watts. Même résultat."` (gradient blanc + gradient vert/teal sur 2ème ligne)
- Sous-titre : mention des −50% Java, −98% Python + données mesurées CSRD
- 3 stat cards glassmorphism vertes :

| Stat | Label |
|---|---|
| `−98%` | Énergie vs Python |
| `−50%` | Énergie vs Java |
| `CSRD` | Données auditables |

---

#### 6.2 Simulateur CO₂ (fonctionnel)

Card glassmorphism `border-radius: 30px` avec formulaire interactif :

**Champs** :
- Stack actuelle : select `Python | Java/JVM | Go | Node.js`
- Nombre d'instances serveur : input number (défaut : 20)
- Consommation mensuelle (kWh) : input number (défaut : 5 000)

**Facteurs de réduction utilisés** (calculés côté JS) :

| Stack | Facteur réduction |
|---|---|
| Python | 0.98 (−98%) |
| Java/JVM | 0.50 (−50%) |
| Go | 0.15 (−15%) |
| Node.js | 0.92 (−92%) |

**Résultats (4 cards vertes)** :
- kWh économisés/mois
- kg CO₂ évités/mois (facteur : 0.4 kg CO₂/kWh)
- € économisés/mois (facteur : 0.12 €/kWh)
- Instances libérables

**Actions** :
- `[Exporter rapport CSRD →]` (bleu Apple)
- `[Affiner l'estimation]` (ghost)

---

#### 6.3 Services Green IT (4 cards)

| Service | Description | KPI |
|---|---|---|
| Audit énergétique numérique | Mesure RAPL/Scaphandre par service, API, batch | Mesure milliseconde · base CSRD · livraison 5 jours |
| Migration green ciblée | Remplacement workloads énergivores, rapport avant/après | −50% vs Java · −98% vs Python (source AWS 2022) |
| Rapport CSRD numérique | Bilan Scope 1-2-3 au format CSRD (UE 2022/2464) | Conformité CSRD 2025 · export PDF |
| Green badge Thrustcode | Label avec preuve mesurée, valorisable en AO | Preuve chiffrée · logo vectoriel · rapport public |

---

#### 6.4 CTA band

```
Titre : "Votre rapport CSRD numérique en 10 jours."
Sous-titre : "Données mesurées, non estimées. Format auditable par votre commissaire aux comptes."
CTAs : [Démarrer l'audit →]  [Voir un exemple de rapport]
```

---

## 7. Page — Études de cas `/case-studies`

**Objectif** : preuve sociale par des métriques réelles. Aucun chiffre estimé.

### Sections

---

#### 7.1 Hero

- Eyebrow : `"Résultats réels · Données mesurées"`
- Titre : `"Preuves par les faits."`
- Sous-titre : engagement de transparence sur les données

---

#### 7.2 Grille d'études de cas (6 cards)

**Layout** : `repeat(auto-fill, minmax(340px, 1fr))`, overflow hidden

Structure de chaque case-card :
```
[Header] Secteur (couleur pilier) + Titre du projet
[Body]   Contexte (challenge)
         3 metric cards (avant/après ou gain)
         Tags (stack, durée, outils)
         [Lire l'étude complète →]
```

**6 cas documentés** :

##### Cas 1 — FinTech · Paiements
- **Titre** : Migration du moteur de transactions Python → Rust
- **Contexte** : moteur Python plafonné à 3 200 tx/s, AWS à 42 000 €/mois
- **Métriques** :

| Indicateur | Avant | Après | Gain |
|---|---|---|---|
| Débit | 3 200 tx/s | 57 600 tx/s | ×18 |
| CPU pic | 100% | 33% | −67% |
| AWS/an | ~504k€ | ~446k€ | −58k€/an |

- **Stack** : Python → Rust/Actix-web · **Durée** : 8 semaines

---

##### Cas 2 — SaaS B2B · Analytics
- **Titre** : Refactoring pipeline de données Java → Rust
- **Contexte** : pipeline Java, 12s pour 1 Go JSON, p99 à 2 840 ms, 18 instances EC2

| Indicateur | Avant | Après | Gain |
|---|---|---|---|
| Traitement 1 Go JSON | 12 s | 2 s | ×6 |
| Latence p99 | 2 840 ms | 47 ms | −98% |
| Instances EC2 | 18 | 6 | −67% |

- **Stack** : Java → Rust/Tokio · **Durée** : 12 semaines

---

##### Cas 3 — Industrie · IoT embarqué
- **Titre** : Firmware capteurs industriels C → Rust (STM32)
- **Contexte** : 800 capteurs, 3 incidents mémoire critiques en 18 mois

| Indicateur | Avant | Après | Gain |
|---|---|---|---|
| Crashs (18 mois) | 3 | 0 | −100% |
| Autonomie batterie | baseline | +34% | +34% |
| Consommation flash | baseline | −41% | −41% |

- **Stack** : C → Rust bare-metal STM32 · **Durée** : 16 semaines

---

##### Cas 4 — Infrastructure · Cybersécurité
- **Titre** : Audit et hardening API critique (conformité NIS2)
- **Contexte** : API C++ exposée réseau, 14 CVE potentiels, délai NIS2 : 3 mois

| Indicateur | Avant | Après |
|---|---|---|
| CVE mémoire potentiels | 14 | 0 |
| Conformité NIS2 art.21 | Non | 100% |
| Livraison rapport | — | 10 jours |

- **Stack** : C++ → Rust/cargo-fuzz · **Durée** : 10 semaines

---

##### Cas 5 — Énergie · Green IT
- **Titre** : Migration pipeline ML Node.js → Rust
- **Contexte** : 8 400 kWh/mois sur 35 instances, obligation CSRD imminente

| Indicateur | Avant | Après | Gain |
|---|---|---|---|
| Conso. mensuelle | 8 400 kWh | 320 kWh | −96% |
| CO₂ évité | — | 2,4 t/an | — |
| Économies cloud | — | 94k€/an | — |

- **Stack** : Node.js → Rust · **Durée** : 10 semaines

---

##### Cas 6 — Scale-up · Streaming WASM
- **Titre** : Microservices vidéo Go → Rust/WASM (edge)
- **Contexte** : latence p99 à 450 ms sous pic, coût Cloudflare Workers excessif

| Indicateur | Avant | Après | Gain |
|---|---|---|---|
| Latence p99 edge | 450 ms | 38 ms | −92% |
| Coût Cloudflare | baseline | −71% | −71% |
| Durée migration | — | 4 semaines | — |

- **Stack** : Go → Rust/WASM · **Durée** : 4 semaines

---

#### 7.3 CTA band

```
Titre : "Votre projet pourrait être le prochain."
Sous-titre : "Diagnostic gratuit de 30 min. Nous identifions les 3 optimisations à plus fort ROI."
CTA : [Démarrer le diagnostic →]
```

---

## 8. Page — Blog `/blog`

**Objectif** : SEO long-tail, nurturing des leads, positionnement d'expertise, capture newsletter

### Sections

---

#### 8.1 Hero

- Eyebrow : `"Blog · The Rust Report"`
- Titre : `"Veille technique & réglementaire."`
- Sous-titre : promesse éditoriale (bi-mensuel, sourcé, sans bruit)

---

#### 8.2 Layout editorial (2 colonnes)

**Colonne principale (2/3)** :

- **Article featured** (full width, image placeholder gradient + icône) :
  - Titre : *Rust vs Go vs Python en 2025 : benchmark complet sur 10 workloads réels*
  - Catégorie : Performance · Benchmark
  - Excerpt : 2 lignes
  - Méta : date + durée de lecture

- **Liste d'articles** (card horizontale : image 80×80 + texte) :

| Titre | Catégorie | Date | Durée |
|---|---|---|---|
| NSA & CISA juin 2025 : ce que le nouveau guide impose aux éditeurs | Sécurité · Réglementation | 15 mars 2025 | 8 min |
| CSRD 2025 : préparer son bilan carbone numérique avec Scaphandre | Green IT · CSRD | 2 mars 2025 | 10 min |
| Strangler Fig Pattern appliqué à Rust : migrer un monolithe Python | Refactoring · Méthode | 18 fév. 2025 | 15 min |
| Rust + WebAssembly sur Cloudflare Workers : guide complet 2025 | WASM · Edge | 5 fév. 2025 | 11 min |

**Colonne sidebar (1/3, sticky)** :

- **Box newsletter "The Rust Report"** : input email + bouton, promesse 0 spam
- **Articles populaires** (top 4 avec compteur de lectures) :

| Titre | Lectures |
|---|---|
| Rust vs Go : benchmark 2025 | 12,4k |
| Comprendre le borrow checker | 8,1k |
| NIS2 et mémoire sécurisée | 6,7k |
| Actix vs Axum en 2025 | 5,2k |

- **Tag cloud** : Performance · Sécurité · Green IT · Tokio · Actix · WASM · NIS2 · CSRD · Embedded · Refactoring · Benchmarks

---

## 9. Page — Academy `/academy`

**Objectif** : générer des inscriptions aux formations et à la séquence email décideurs

### Sections

---

#### 9.1 Hero

- Eyebrow : `"Formation · Certification · Webinaires"` (dot violet pulsé)
- Titre : `"Thrustcode Academy."`
- Sous-titre : 3 niveaux, cours pratiques, projets réels, certification interne

---

#### 9.2 Parcours de formation (3 tracks)

**Layout** : grille 3 colonnes égales

| — | Niveau 1 | Niveau 2 | Niveau 3 |
|---|---|---|---|
| **Nom** | Rust Foundations | Rust in Production | Rust Systems Expert |
| **Icône** | 🌱 | ⚡ | 🦾 |
| **Public** | Devs Python/Java/Go | Devs avec bases Rust | Devs avancés |
| **Durée** | 3 jours | 5 jours | 5 jours |
| **Format** | Intra / Inter / Distanciel | Intra obligatoire | Intra · sur devis |
| **Couleur** | Bleu `#2997FF` | Bleu `#2997FF` (featured, border accent) | Violet `#BF5AF2` |
| **CTA** | `En savoir plus` (ghost) | `S'inscrire →` (bleu Apple) | `Demander un devis` (ghost) |

**Programme Niveau 1 — Rust Foundations** :
- Ownership & borrow checker
- Types, enums, pattern matching
- Error handling avec `Result` et `Option`
- Cargo, crates & ecosystem
- Projet fil rouge : CLI tool en Rust

**Programme Niveau 2 — Rust in Production** :
- Async Rust & Tokio runtime
- Actix-web & Axum frameworks
- Tests unitaires & d'intégration
- CI/CD avec GitHub Actions
- Profiling & optimisation (cargo flamegraph)
- Projet fil rouge : API REST Rust complète

**Programme Niveau 3 — Rust Systems Expert** :
- Unsafe Rust & raw pointers
- FFI & interopérabilité C/C++
- Embedded bare-metal Rust (STM32, ESP32, RISC-V)
- WASM : compilation et déploiement
- Cryptographie avec les crates Rust (rustls, ring)
- Projet fil rouge : système complet

---

#### 9.3 Mini-cours décideurs (layout 2 colonnes)

Card glassmorphism `border: 1px solid rgba(0,113,227,0.2)` :

**Colonne gauche** :
- Label : `"Pour décideurs"`
- Titre : `"5 jours pour comprendre Rust (sans coder)"`
- Description : séquence email non-technique pour DSI, CTO, managers
- Formulaire inline : input email + `[Démarrer →]`
- Note : `Gratuit · 1 email/jour · Désabonnement en 1 clic`

**Colonne droite — Les 5 emails** :

| Jour | Sujet |
|---|---|
| J1 | Pourquoi Rust en 2025 — les chiffres qui comptent |
| J2 | Ce que ça change pour votre infra et vos coûts cloud |
| J3 | Sécurité : pourquoi NSA et CISA recommandent Rust |
| J4 | AWS, Microsoft, Android : comment ils l'ont fait |
| J5 | Comment évaluer et démarrer votre premier projet Rust |

Chaque email affiché comme une pill `J1 | [texte]`, background `rgba(0,113,227,0.15)`.

---

#### 9.4 Webinaires à venir

**Layout** : liste verticale de cards horizontales

Structure de chaque item :
```
[Date : JJ / MMM]  |  [Titre du webinaire]  [Tag pilier]  [S'inscrire]
```

| Date | Titre | Tag | Couleur |
|---|---|---|---|
| 15 Avr | Migrer de Python à Rust : retour d'expérience FinTech | Performance | Bleu |
| 29 Avr | NIS2 & DORA : comment Rust réduit votre surface d'attaque | Sécurité | Violet |
| 13 Mai | CSRD numérique : mesurer et réduire l'empreinte de votre stack | Green IT | Vert |

---

## 10. Page — À propos `/about`

**Objectif** : créer la confiance, humaniser la marque, recruter des profils Rust seniors

### Sections

---

#### 10.1 Hero

- Eyebrow : `"ESN · Paris · Fondée 2025"`
- Titre : `"Construits pour ne jamais flancher."`

---

#### 10.2 Manifeste

Bloc texte centré `max-width: 680px`, `font-size: 1.3rem`, `font-weight: 300`, interligne 1.75 :

> Thrustcode est née d'une conviction simple : **la performance, la sécurité et la durabilité ne sont pas des compromis** — elles sont des conséquences d'un choix technologique. Rust est ce choix.
>
> Nous construisons des logiciels qui **ne flanquent pas sous la charge**, qui **ne laissent pas passer les vulnérabilités**, et qui **n'épuisent pas inutilement les ressources de la planète**.
>
> Pas de hype. Pas de buzzwords. **Des métriques, des benchmarks, des livrables.**

---

#### 10.3 Les 6 valeurs

**Layout** : grille 3×2, séparateurs 1px, `border-radius: 30px`, background `var(--bg1)`

| # | Valeur | Description courte |
|---|---|---|
| 01 | Mesurer avant d'affirmer | Chaque promesse accompagnée d'un benchmark reproductible et de sources citées |
| 02 | Livrer, pas promettre | Sprints courts, valeur tangible avant la fin du premier mois |
| 03 | Transparence technique | Le code appartient au client. Documentation et formation incluses |
| 04 | Sobriété numérique | L'empreinte énergétique de chaque livrable est mesurée systématiquement |
| 05 | Sécurité par conception | Dans le compilateur, chaque PR, chaque choix d'architecture |
| 06 | Transfert de compétences | L'équipe cliente devient autonome. C'est l'objectif |

---

#### 10.4 Équipe (4 cards)

**Layout** : `repeat(auto-fill, minmax(260px, 1fr))`

| Membre | Rôle | Avatar couleur | Bio courte | Skills |
|---|---|---|---|---|
| Thomas Laurent | CEO & Lead Architect | Bleu `#2997FF` | Ex-kernel Linux, ex-AWS Nitro. 12 ans systems programming | Rust · C · Linux kernel · AWS |
| Sofia Caramel | CTO & Security Lead | Violet `#BF5AF2` | Ex-ANSSI. Contribue Rust-for-Linux. Expert NIS2 & DORA | Sécurité · Crypto · NIS2 |
| Marc Berthier | Head of Green IT | Vert `#30D158` | Créateur de Scaphandre-reporter. Expert CSRD | Green IT · CSRD · Scaphandre |
| Amara Ndiaye | Embedded Systems Engineer | Orange `#FF9F0A` | 10 ans firmware STM32/RISC-V/ESP32. Rust bare-metal | Embedded · STM32 · RISC-V |

---

#### 10.5 Presse & mentions

**Layout** : `repeat(auto-fill, minmax(280px, 1fr))`

| Outlet | Citation |
|---|---|
| Le Monde Informatique | *"Thrustcode s'impose comme la première ESN française entièrement dédiée à Rust — un pari audacieux qui semble payant."* |
| L'Usine Digitale | *"Une approche rare : des métriques mesurées, pas des promesses marketing. C'est ce qui distingue Thrustcode du marché."* |
| Numeum · Rapport ESN 2025 | *"L'ESN de niche spécialisée Rust représente un modèle différenciant dans un marché ESN généraliste sous pression."* |

---

## 11. Page — Contact `/contact`

**Objectif** : convertir le visiteur qualifié en lead commercial. Qualification du besoin dès le formulaire.

### Sections

---

#### 11.1 Hero

- Eyebrow : `"Réponse <4h ouvrées"` (dot vert pulsé, badge SLA)
- Titre : `"Parlons de votre projet."`
- Sous-titre : premier atelier offert, 3 quick wins identifiés

---

#### 11.2 Layout 2 colonnes (infos | formulaire)

**Colonne gauche — Informations de contact** :

| Bloc | Contenu |
|---|---|
| Temps de réponse | `< 4 heures ouvrées` · Lun–Ven 9h–18h CET · Badge SLA vert pulsé `"Équipe disponible maintenant"` |
| Email direct | `hello@thrustcode.dev` |
| Localisation | Paris, France · Interventions France & Europe · Remote worldwide |
| Booking créneaux | 4 créneaux "Démo 30 min" affichés avec statut `Disponible` |

**Créneaux de booking (exemple)** :
```
Lundi 7 avr. · 10h00    [Disponible]
Lundi 7 avr. · 14h30    [Disponible]
Mardi 8 avr. · 11h00    [Disponible]
Mercredi 9 avr. · 9h30  [Disponible]
```

**Colonne droite — Formulaire qualifié** :

| Champ | Type | Obligatoire |
|---|---|---|
| Prénom | text | Oui |
| Nom | text | Oui |
| Email professionnel | email | Oui |
| Entreprise | text | Oui |
| Type de projet | select | Oui |
| Stack actuelle | text | Non |
| Décrivez votre besoin | textarea (120px min) | Non |

**Options du select "Type de projet"** :
- Migration de performance (Python/Java → Rust)
- Refactoring de codebase existant
- Audit sécurité mémoire
- Rapport CSRD / Green IT
- Développement microservices Rust
- Embedded / Firmware Rust
- Formation équipe
- Autre / Je ne sais pas encore

**Bouton submit** : `[Envoyer la demande →]` full width, bleu Apple, hover avec box-shadow

**Note légale** : `Vos données sont traitées avec confidentialité. Aucun démarchage commercial non sollicité.`

---

## 12. Design system

### Palette de couleurs (Apple dark mode)

| Token | Valeur | Usage |
|---|---|---|
| `--apple-blue` | `#0071E3` | CTA principaux, liens actifs |
| `--apple-blue-l` | `#2997FF` | Hover, accents, eyebrows |
| `--apple-blue-d` | `#0058B0` | Pressed state |
| `--apple-green` | `#30D158` | Green IT, succès, KPIs positifs |
| `--apple-orange` | `#FF9F0A` | Validation, étape 4, alertes |
| `--apple-red` | `#FF3B30` | Erreurs (formulaires) |
| `--apple-purple` | `#BF5AF2` | Sécurité, niveau expert, tags |
| `--apple-teal` | `#5AC8FA` | Transversal, accent secondaire |
| `--bg0` | `#000000` | Fond principal |
| `--bg1` | `#0A0A0A` | Fond sections alternées |
| `--bg2` | `#111111` | Fond sections tertiaires |
| `--bg3` | `#161616` | Surfaces légèrement surélevées |
| `--bg4` | `#1C1C1E` | iOS/macOS dark surface |
| `--t1` | `rgba(255,255,255,0.95)` | Texte principal |
| `--t2` | `rgba(255,255,255,0.60)` | Texte secondaire |
| `--t3` | `rgba(255,255,255,0.35)` | Texte tertiaire / labels |
| `--t4` | `rgba(255,255,255,0.15)` | Placeholders, separateurs |

### Glassmorphism (surfaces)

| Token | Valeur |
|---|---|
| `--glass-bg` | `rgba(255,255,255,0.055)` |
| `--glass-bg-hover` | `rgba(255,255,255,0.085)` |
| `--glass-border` | `rgba(255,255,255,0.10)` |
| `--glass-border-h` | `rgba(255,255,255,0.20)` |
| `--glass-blur` | `24px` |

**Recette glassmorphism standard** :
```css
background: rgba(255,255,255,0.055);
border: 1px solid rgba(255,255,255,0.10);
backdrop-filter: blur(24px);
-webkit-backdrop-filter: blur(24px);
border-radius: 22px;
```

**Nav bar (macOS Ventura)** :
```css
background: rgba(0,0,0,0.72);
backdrop-filter: saturate(180%) blur(20px);
```

### Typographie

| Usage | Font | Weight | Size | Letter-spacing |
|---|---|---|---|---|
| Titre hero | Inter / SF Pro Display | 600 | clamp(3rem, 7.5vw, 7rem) | −0.04em |
| Titre section | Inter | 600 | clamp(2rem, 4vw, 3.2rem) | −0.03em |
| Corps | Inter | 300–400 | 17px | — |
| Labels / eyebrows | Inter | 500 | 0.72rem | 0.12em |
| Chips / badges | Inter | 400–500 | 0.68–0.78rem | 0.04–0.08em |

**Gradient text** (titres hero et sections) :
```css
background: linear-gradient(180deg, #fff 40%, rgba(255,255,255,0.45) 100%);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;
```

**Gradient accent** (mots clés) :
```css
background: linear-gradient(135deg, #2997FF, #BF5AF2);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;
```

### Border radius

| Token | Valeur | Usage |
|---|---|---|
| `--r-sm` | `10px` | Petits éléments (chips, badges) |
| `--r-md` | `16px` | Cards moyennes, inputs |
| `--r-lg` | `22px` | Cards principales |
| `--r-xl` | `30px` | Grandes cards, grilles piliers |
| `--r-full` | `999px` | Pills, boutons CTA, dots |

### Composants UI réutilisables

#### Bouton primaire (`.btn-apple` / `.btn-blue`)
```css
background: #0071E3;
color: #fff;
padding: 0.8rem 1.75rem;
border-radius: 999px;
font-size: 0.95rem;
font-weight: 500;
/* hover: background: #2997FF + translateY(-2px) + box-shadow bleu */
```

#### Bouton ghost (`.btn-glass`)
```css
background: rgba(255,255,255,0.055);
border: 1px solid rgba(255,255,255,0.10);
backdrop-filter: blur(24px);
color: rgba(255,255,255,0.95);
padding: 0.8rem 1.75rem;
border-radius: 999px;
```

#### Chip / Tag
```css
font-size: 0.72rem;
color: rgba(255,255,255,0.35);
background: rgba(255,255,255,0.055);
border: 1px solid rgba(255,255,255,0.10);
border-radius: 999px;
padding: 3px 10px;
backdrop-filter: blur(12px);
```

#### Curseur custom
- Dot 8px blanc, `mix-blend-mode: difference`
- Anneau 40px, border 1px semi-transparent
- Grossit sur hover `a` et `button` : dot → 18px vert, anneau → 56px bleu

### Animations

| Animation | CSS | Usage |
|---|---|---|
| `fadeUp` | `opacity: 0 → 1`, `translateY(18px → 0)`, 0.7s | Éléments hero (staggered 0.2–0.8s) |
| `pulse` | `scale(1 → 1.4)`, `opacity(1 → 0.5)`, 2s infinite | Eyebrow dots, SLA badge |
| `float` | Translation douce 30px, 16–22s infinite ease-in-out | Orbes mesh background |
| `ticker` | `translateX(0 → -50%)`, 32s linear infinite | Bandeau défilant |
| scroll reveal | `IntersectionObserver`, `translateY(22px → 0)`, 0.65s | Sections au scroll |

---

## 13. SEO & métadonnées

### Pages principales

| Page | `<title>` | Meta description |
|---|---|---|
| `/` | `Thrustcode — Performance · Security · Green IT` | ESN spécialisée Rust. Migration de goulots, sécurité mémoire, Green IT. Premier atelier de diagnostic offert. |
| `/services` | `Services Rust — Thrustcode` | 23 services Rust : performance, sécurité, Green IT, embedded, formation. Résultats mesurés et sourcés. |
| `/services/green-it` | `Green IT & CSRD Rust — Thrustcode` | Réduisez votre empreinte carbone numérique de 98% avec Rust. Données CSRD auditables. Simulateur CO₂ gratuit. |
| `/case-studies` | `Études de cas — Thrustcode` | 6 migrations Rust documentées avec métriques réelles. FinTech ×18 débit, SaaS −98% latence, IoT 0 crash. |
| `/blog` | `Blog Rust — Thrustcode · The Rust Report` | Veille technique et réglementaire Rust. Benchmarks, NIS2, CSRD. Bi-mensuel, sourcé, sans bruit. |
| `/academy` | `Formation Rust — Thrustcode Academy` | 3 parcours Rust (3j, 5j, 5j), webinaires gratuits, mini-cours DSI/CTO. Production-ready en 10 jours. |
| `/about` | `À propos — Thrustcode` | L'équipe Thrustcode. Experts Rust, ex-ANSSI, ex-AWS Nitro. Fondée à Paris en 2025. |
| `/contact` | `Contact & Audit gratuit — Thrustcode` | Premier diagnostic Rust offert. Réponse <4h ouvrées. Identifiez 3 quick wins dans votre stack. |

### Open Graph

```html
<meta property="og:type" content="website">
<meta property="og:url" content="https://thrustcode.dev/">
<meta property="og:title" content="Thrustcode — Performance · Security · Green IT">
<meta property="og:description" content="ESN spécialisée Rust. Migration de goulots, sécurité mémoire, Green IT. Premier atelier de diagnostic offert.">
<meta property="og:image" content="https://thrustcode.dev/og-image.png">
<meta name="twitter:card" content="summary_large_image">
```

### Mots-clés SEO cibles

**Longue traîne prioritaire** :
- `ESN Rust France`
- `migration Python Rust`
- `refactoring Java Rust`
- `audit sécurité mémoire NIS2`
- `bilan carbone numérique CSRD`
- `formation Rust développeurs`
- `microservices Rust Actix`
- `conformité NIS2 DORA Rust`
- `green IT logiciel`
- `empreinte carbone logiciel mesure`

---

## 14. Performances & technique

### Stack front-end

| Aspect | Choix | Justification |
|---|---|---|
| Framework JS | Aucun (vanilla JS) | Zéro dépendance, performance maximale |
| Animations | CSS natif + `requestAnimationFrame` | Pas de lib externe, GPU-accelerated |
| Scroll detection | `IntersectionObserver` natif | Performant, no polyfill moderne |
| Routing SPA | Routeur JS maison (< 30 lignes) | Changement de page sans rechargement |
| Fonts | Google Fonts Inter (preconnect) | Fallback SF Pro Display natif |
| CSS | Variables natives + Glassmorphism | Cohérence totale, zero-runtime |

### Objectifs Core Web Vitals

| Métrique | Cible |
|---|---|
| LCP (Largest Contentful Paint) | < 1.5s |
| FID (First Input Delay) | < 50ms |
| CLS (Cumulative Layout Shift) | < 0.05 |
| TTFB (Time to First Byte) | < 200ms |

### Considérations accessibilité

- Contrastes WCAG AA vérifiés : texte principal `rgba(255,255,255,0.95)` sur `#000`
- Curseur custom désactivable (preference `cursor: auto` sur `prefers-reduced-motion`)
- Navigation clavier complète (focus visible sur tous les éléments interactifs)
- ARIA labels sur les icônes sans texte
- `alt` descriptifs sur toutes les images

### Structure des fichiers

```
thrustcode.dev/
├── index.html              → Page d'accueil (Home)
├── services/
│   ├── index.html          → Catalogue services
│   ├── performance.html    → Détail Performance
│   ├── security.html       → Détail Sécurité
│   ├── green-it.html       → Green IT + simulateur
│   ├── embedded.html       → Embedded & IoT
│   └── refactoring.html    → Refactoring legacy
├── case-studies/
│   └── index.html          → Grille des 6 études de cas
├── blog/
│   └── index.html          → Liste articles + sidebar
├── academy/
│   └── index.html          → Parcours + webinaires
├── about/
│   └── index.html          → Manifeste + équipe
├── contact/
│   └── index.html          → Formulaire + booking
├── assets/
│   ├── css/
│   │   ├── design-tokens.css   → Variables globales
│   │   ├── components.css      → Composants UI
│   │   └── animations.css      → Keyframes
│   └── js/
│       ├── cursor.js           → Curseur custom
│       ├── reveal.js           → Scroll animations
│       ├── simulator.js        → Calculateur CO₂
│       └── router.js           → SPA routing
└── og-image.png            → Open Graph image (1200×630)
```

---

*Document généré le 29 mars 2026 · thrustcode.dev*
*Maintenu par l'équipe Thrustcode · Version 1.0*
