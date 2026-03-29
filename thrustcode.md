# Thrustcode — Documentation complète

> **ESN spécialisée Rust · Performance · Sécurité · Green IT**
> Paris, France · Active à l'international · Fondée 2025

---

## Table des matières

1. [Présentation](#1-présentation)
2. [Positionnement stratégique](#2-positionnement-stratégique)
3. [Les trois piliers](#3-les-trois-piliers)
4. [Catalogue de services](#4-catalogue-de-services)
5. [Processus d'accompagnement](#5-processus-daccompagnement)
6. [Lead magnets & stratégie de conversion](#6-lead-magnets--stratégie-de-conversion)
7. [Études de cas (résultats mesurés)](#7-études-de-cas-résultats-mesurés)
8. [Thrustcode Academy](#8-thrustcode-academy)
9. [Identité & valeurs](#9-identité--valeurs)
10. [Équipe fondatrice](#10-équipe-fondatrice)
11. [Architecture du site web](#11-architecture-du-site-web)
12. [Sources & références](#12-sources--références)

---

## 1. Présentation

**Thrustcode** est une Entreprise de Services du Numérique (ESN) entièrement spécialisée dans le développement de solutions logicielles en **Rust** — le langage de programmation systèmes le plus performant, le plus sûr du point de vue de la mémoire, et l'un des plus économes en énergie disponibles en 2025.

### Mission

Construire des logiciels qui ne flanquent pas : sous la charge (performance), face aux attaquants (sécurité), et face aux enjeux climatiques (Green IT).

### Proposition de valeur en une phrase

> Thrustcode réécrit les goulots d'étranglement, sécurise les composants critiques et réduit l'empreinte carbone numérique des organisations — avec des résultats mesurés, sourcés et auditables.

### Chiffres clés (sourcés)

| Métrique | Valeur | Source |
|---|---|---|
| Vitesse vs Python (CPU-intensif) | **×60** | BenchCraft / TechEmpower 2025 |
| Vitesse vs Go | **×2** | BenchCraft 2025 |
| Réduction énergie vs Python | **−98%** | AWS Open Source Blog, 2022 |
| Réduction énergie vs Java | **−50%** | AWS Open Source Blog, 2022 |
| Réduction surface CVE mémoire | **−70%** | NSA/CISA, juin 2025 |
| Rust langue préférée des devs | **10 ans consécutifs** | Stack Overflow Survey 2025 |
| Développeurs Rust dans le monde | **~2,27 millions** | JetBrains State of Dev Ecosystem 2024 |
| Croissance offres d'emploi Rust | **+35% YoY** | Stack Overflow / LinkedIn 2025 |

---

## 2. Positionnement stratégique

### Contexte marché

Le marché des ESN en France représente **34,5 milliards d'euros** en 2024 (source : Numeum/PAC, décembre 2024). Dans un contexte de croissance ralentie (+0,7% en 2024 vs +6,5% en 2023), les ESN généralistes sont sous pression. Les acteurs spécialisés sur des technologies à forte valeur ajoutée se différencient.

Les trois leviers de croissance identifiés par Numeum pour 2025 sont la cybersécurité, le numérique responsable et l'intelligence artificielle — trois domaines où Rust apporte un avantage technique démontrable.

### Différenciation

Thrustcode est **la première ESN française entièrement dédiée à Rust**, avec un positionnement sur trois axes inédits dans le marché ESN :

- **Performance mesurée** : chaque engagement de performance est accompagné d'un benchmark reproductible
- **Sécurité réglementaire** : conformité NIS2, DORA, critères CISA/NSA 2025
- **Green IT chiffré** : données d'empreinte carbone mesurées (non estimées), auditables CSRD

### Pourquoi Rust en 2025

Rust bénéficie d'une adoption institutionnelle sans précédent :

- **Microsoft** qualifie 2025 d'« année de Rust » et réécrit des parties d'Hyper-V (Azure) en Rust
- **AWS** utilise Rust pour Amazon S3, EC2, CloudFront, Nitro System
- **Android/Google** : après migration vers Rust pour tout nouveau code, les vulnérabilités mémoire sont passées de **76% en 2019 à 24% en 2024** (source CISA/NSA, juin 2025)
- **NSA & CISA** recommandent officiellement Rust depuis 2022, avec un guide mis à jour en juin 2025
- **Linux** intègre Rust comme second langage de développement noyau

---

## 3. Les trois piliers

### ⚡ Performance

**Principe technique** : Rust compile en code natif sans runtime ni garbage collector. Ses abstractions zéro-coût signifient que les fonctionnalités de haut niveau (itérateurs, closures, traits) ne génèrent aucun surcoût à l'exécution par rapport au code assembleur équivalent.

**Résultats mesurés** :
- **60× plus rapide que Python** sur des tâches CPU-intensives (BenchCraft 2025)
- **2× plus rapide que Go** sur les workloads computationnels
- **60 000 requêtes/seconde** par instance avec Actix-web (vs 40 000 pour Go)
- Latence p99 **< 15ms** à 1 000 requêtes/seconde concurrentes
- Cold start AWS Lambda : **~30ms** (vs ~325ms pour Python, ~100ms pour Java)

**Cas d'usage prioritaires** : systèmes financiers haute fréquence, pipelines de données temps réel, API haute disponibilité, edge computing, microservices critiques.

---

### 🔒 Sécurité

**Principe technique** : le *borrow checker* de Rust garantit à la compilation qu'aucune référence ne survit à la donnée qu'elle référence. Cette propriété élimine structurellement les classes entières de vulnerabilités mémoire : buffer overflows, use-after-free, double-free, null pointer dereferences.

**Résultats mesurés** :
- Les vulnérabilités mémoire représentent **~70% de toutes les failles critiques** dans les logiciels C/C++ (NSA/CISA, juin 2025)
- Android a réduit ses bugs mémoire de **76% → 24%** en prioritisant Rust pour les nouveaux développements (Google/CISA, 2024)
- Rust élimine ces classes de bugs **au niveau du compilateur**, sans discipline de code ni outils externes

**Réglementation** :
- **NSA/CISA** — Guide "Memory Safe Languages" (juin 2025) : Rust cité comme référence de langage à sécurité mémoire
- **Maison-Blanche / ONCD** (fév. 2024) : recommande la migration vers les langages à sécurité mémoire
- **NIS2** (directive UE 2022/0383) — Article 21 : impose des mesures techniques pour la gestion des risques de cybersécurité
- **DORA** (règlement UE 2022/2554) : exigences de résilience opérationnelle numérique pour le secteur financier

---

### 🌿 Green IT

**Principe technique** : Rust élimine le garbage collector et minimise les allocations mémoire. Un code sobre en ressources consomme moins d'énergie CPU, nécessite moins d'instances serveur et réduit mécaniquement l'empreinte carbone.

**Résultats mesurés** (source : AWS Open Source Blog, 2022 — étude sur 27 langages) :
- Rust et C sont **~50% plus efficaces énergétiquement que Java**
- Rust est **~98% plus efficace que Python**
- Une adoption large de Rust pourrait réduire la consommation énergétique des datacenters de **50%** selon AWS

**Outils de mesure utilisés** :
- **RAPL** (Running Average Power Limit) : mesure de la consommation CPU par Intel
- **Scaphandre** : outil open-source de mesure de la consommation logicielle par processus
- **Computer Language Benchmarks Game** : suite de benchmarks standardisés multi-langages

**Conformité CSRD** : les données produites par ces outils sont auditables et conformes aux exigences de la directive CSRD (Corporate Sustainability Reporting Directive, UE 2022/2464) applicable aux entreprises de plus de 250 salariés à partir de 2025.

---

## 4. Catalogue de services

### 4.1 Performance Engineering (7 services)

| Service | Maturité | KPI différenciant | Interlocuteurs |
|---|---|---|---|
| **Migration de goulots** (Python/Java/Go → Rust) | Core | −40 à −80% CPU · ×2 à ×10 débit | CTO, Direction technique |
| **Refactoring de code legacy vers Rust** | Core | −40 à −70% dette technique · tests ≥ 80% | CTO, VP Engineering, Lead Architect |
| **Microservices haute performance** | Core | 60k rps · p99 < 15ms · cold start < 30ms | CTO, VP Engineering |
| **Traitement de données temps réel** | Core | 1 Go JSON en ~2s · ×4 vs Java · ×6 vs Python | Data Engineer, Head of Platform |
| **Audit de performance & benchmarking** | Core | Livraison 5j · 3 quick wins garantis | CTO, VP Engineering |
| **WASM & edge computing** | Growth | Latence edge < 5ms · bundle léger | VP Product, Lead Frontend |
| **Optimisation infra cloud & FinOps** | Growth | −30 à −60% instances · ROI < 6 mois | CFO, CTO, FinOps Manager |

---

### 4.2 Security by Design (6 services)

| Service | Maturité | KPI différenciant | Interlocuteurs |
|---|---|---|---|
| **Audit sécurité mémoire** (20 critères CISA/NSA) | Core | 100% codebase · livraison < 10j | RSSI, CISO |
| **Migration C/C++ → Rust** (code critique) | Core | −70% surface vulnérabilités · conformité NIS2/DORA | RSSI, CTO, DPO |
| **Conformité NIS2 & DORA** | Core | Conformité certifiable · NIS2 art.21 · DORA RTS 2025 | DPO, RSSI, Direction juridique |
| **Développement sécurisé by design** (DevSecOps) | Core | Zéro faille mémoire en production · SBOM complet | Lead DevSecOps, RSSI |
| **Cryptographie & protocoles sécurisés** | Growth | Zéro dépendance C pour la crypto · auditabilité totale | Architecte sécurité, CTO |
| **Hyperviseur & sandboxing** | Future | Isolation OS-level · démarrage < 125ms | Architecte infra, CTO |

---

### 4.3 Green IT & Durabilité (5 services)

| Service | Maturité | KPI différenciant | Interlocuteurs |
|---|---|---|---|
| **Audit énergétique numérique** | Core | Mesure milliseconde · base CSRD | DSI, CDO, Responsable RSE, DAF |
| **Migration green** (optimisation énergétique) | Core | −50% vs Java · −98% vs Python | CTO, Responsable RSE, DAF |
| **Rapport CSRD numérique** | Core | Conformité CSRD 2025 · Scope 1-2-3 · données auditables | DPO, Direction RSE, CFO |
| **Green badge & certification partenaire** | Growth | Preuve chiffrée · valorisable en appel d'offres | Direction marketing, RSE |
| **Simulateur CO₂ on-demand** | Growth | Lead magnet SaaS · export PDF CSRD | CTO, DSI, Direction RSE |

---

### 4.4 Transversal & Conseil (5 services)

| Service | Maturité | KPI différenciant | Interlocuteurs |
|---|---|---|---|
| **Formation Rust équipes** (3 niveaux) | Core | 3j → autonomie basique · 10j → prod-ready | CTO, DRH, Lead technique |
| **Embedded & firmware Rust** | Core | Zéro unsafe non justifié · industrie · médical | CTO hardware, Lead embedded |
| **Architecture conseil & CTO as a Service** | Core | Engagement flexible · dès 2j/mois | CEO, Fondateur, Investisseur |
| **Blockchain & protocoles décentralisés** | Growth | Solana : < 400ms finalité · audit sécurité inclus | CTO Web3, Lead blockchain |
| **IA infrastructure en Rust** | Future | Inférence ×3 vs Python serving · −60% CPU | Head of AI, MLOps, CTO |

---

### Service phare : Refactoring de code legacy vers Rust

Ce service mérite une description étendue, car il représente le plus grand marché adressable immédiat.

**Description** : Modernisation progressive d'un codebase existant (Python, Java, Go, C++) vers Rust. Approche incrémentale par modules, sans interruption de service, via la méthode **Strangler Fig Pattern**.

**Méthodologie en 3 phases** :

**Phase 1 — Diagnostic (1-2 semaines)**
Analyse statique du codebase via `cargo clippy`, `cargo audit`, `cargo flamegraph`. Identification des modules à fort ROI (performance, sécurité, dette technique). Scoring et roadmap priorisée par ROI.

**Phase 2 — Migration incrémentale (4-16 semaines)**
Réécriture module par module. Cohabitation avec le code existant via FFI (Foreign Function Interface). Tests de non-régression à chaque sprint de 2 semaines. Zéro downtime garanti.

**Phase 3 — Hardening (2-4 semaines)**
Audit sécurité (`cargo-audit`, fuzzing `cargo-fuzz`), optimisations finales, documentation complète, sessions de pair programming et transfert de compétences à l'équipe interne.

**Livrables** :
- Cartographie et scoring du codebase (complexité, risque, ROI)
- Plan de migration priorisé par module
- Code Rust production-ready modulaire
- Tests de non-régression ≥ 80% de couverture
- Rapport qualité : clippy, cargo-audit, benchmarks
- Documentation d'architecture complète

**KPI** : zéro downtime · −40 à −70% dette technique · couverture tests ≥ 80%

---

## 5. Processus d'accompagnement

Thrustcode applique un processus en **5 étapes structurées** pour chaque projet.

### Étape 01 — Diagnostic & cadrage *(Semaine 1)*

**Objectif** : comprendre le contexte en profondeur avant d'écrire une ligne de code.

Déroulé : atelier de 2 jours avec les équipes techniques et les décideurs. Analyse statique du codebase (`cargo audit`, `cargo clippy`, `cargo flamegraph`). Identification des goulots, contraintes réglementaires, configuration de l'équipe.

**Livrables** : cartographie codebase · scoring performance/sécu/green · 3 axes prioritaires

---

### Étape 02 — Roadmap & proposition technique *(Semaine 2)*

**Objectif** : construire un plan priorisé par ROI avant toute réalisation.

Chaque module est évalué selon trois critères : impact performance, réduction du risque sécurité, gain énergétique. La roadmap 30/60/90 jours est présentée au client pour validation avant démarrage.

**Livrables** : roadmap 30/60/90 jours · ROI estimé par module · architecture cible Rust

---

### Étape 03 — Migration incrémentale en sprints *(Semaines 3 → N)*

**Objectif** : livrer de la valeur à chaque sprint, sans rupture de service.

Les ingénieurs Rust travaillent module par module via le **Strangler Fig Pattern**, en cohabitation avec le code existant via FFI. Chaque sprint de 2 semaines livre du code testé, documenté et déployable en production.

**Livrables** : code Rust production-ready par sprint · tests non-régression continus · zéro downtime

---

### Étape 04 — Hardening, tests & mesures *(J-14 avant go-live)*

**Objectif** : valider chaque mise en production avec des données auditables.

Phase de validation rigoureuse : tests de charge (k6/Locust), audit sécurité (`cargo-audit`, `cargo-fuzz`), mesure énergétique réelle (Scaphandre/RAPL). Tous les résultats sont comparés à la baseline initiale.

**Livrables** : rapport benchmark avant/après · audit cargo-audit + fuzzing · mesure CO₂ Scaphandre · tests k6

---

### Étape 05 — Transfert de compétences & suivi *(Post go-live)*

**Objectif** : rendre l'équipe cliente autonome sur le code Rust livré.

Sessions de pair programming avec les développeurs du client, documentation d'architecture complète, suivi de 30 jours post go-live avec SLA de réponse garanti.

**Livrables** : équipe autonome sur le code Rust · documentation architecture · SLA 30j post go-live · rapport de résultats signé et auditable

---

### Garanties du processus

- **Zéro downtime** via Strangler Fig Pattern et FFI
- **Premier atelier de diagnostic offert** (sans engagement)
- **Réponse < 4h ouvrées** pour toute demande
- **Résultats mesurés et auditables** à chaque étape

---

## 6. Lead magnets & stratégie de conversion

Thrustcode déploie 6 assets de génération de leads, conçus pour capturer des contacts qualifiés (décideurs techniques et managers) avant toute interaction commerciale.

| # | Type | Titre | Cible | Canal de capture |
|---|---|---|---|---|
| 1 | Outil interactif | **Calculateur ROI cloud** | CTO, CFO, FinOps | Formulaire email + résultat envoyé |
| 2 | Simulateur CO₂ | **Empreinte carbone numérique** | DSI, RSE, DAF | Export PDF CSRD après email |
| 3 | Rapport PDF | **Audit sécurité mémoire offert** (20 points CISA/NSA) | RSSI, CISO | Génération automatique post-formulaire |
| 4 | Newsletter | **The Rust Report** (bi-mensuel) | Tech leads, architectes | Inscription + séquence onboarding 5 emails |
| 5 | Mini-cours email | **5 jours pour comprendre Rust** (non-technique) | DSI, CTO non-technicien | Drip email · 1 leçon/jour |
| 6 | Audit live | **Démo 30 min offerte** | CTO, VP Engineering | Booking Cal.com intégré · réponse < 4h |

### Séquence email d'onboarding "5 jours pour comprendre Rust"

Destinée aux décideurs non-développeurs (DSI, CTO, CEO), cette séquence éducative non-technique :

- **J1** : Pourquoi Rust en 2025 — les chiffres qui comptent
- **J2** : Ce que ça change pour votre infra et vos coûts cloud
- **J3** : Sécurité : pourquoi NSA et CISA recommandent Rust
- **J4** : AWS, Microsoft, Android : comment ils l'ont fait
- **J5** : Comment évaluer et démarrer votre premier projet Rust

---

## 7. Études de cas (résultats mesurés)

### 7.1 FinTech · Paiements — Migration Python → Rust

**Contexte** : moteur de traitement de transactions Python. Plafond à 3 200 tx/seconde. Facture AWS : 42 000 €/mois.

**Résultats après migration** :

| Métrique | Avant | Après | Gain |
|---|---|---|---|
| Débit transactionnel | 3 200 tx/s | 57 600 tx/s | **×18** |
| Utilisation CPU | 100% (pic) | 33% (pic) | **−67%** |
| Coût AWS annuel | ~504k€ | ~446k€ | **−58k€/an** |

**Stack** : Python → Rust/Actix-web · Durée : 8 semaines

---

### 7.2 SaaS B2B · Analytics — Refactoring Java → Rust

**Contexte** : pipeline Java traitant 1 Go de logs JSON en 12 secondes. Latence p99 à 2 840 ms. 18 instances EC2 t3.large.

**Résultats après migration** :

| Métrique | Avant | Après | Gain |
|---|---|---|---|
| Traitement 1 Go JSON | 12 s | 2 s | **×6** |
| Latence p99 | 2 840 ms | 47 ms | **−98%** |
| Instances EC2 | 18 | 6 | **−67%** |

**Stack** : Java → Rust/Tokio · Durée : 12 semaines

---

### 7.3 Industrie · IoT — Firmware C → Rust (STM32)

**Contexte** : firmware C sur 800 capteurs industriels. 3 incidents critiques en 18 mois liés à des corruptions mémoire.

**Résultats après migration** :

| Métrique | Avant | Après | Gain |
|---|---|---|---|
| Incidents mémoire (18 mois) | 3 | 0 | **−100%** |
| Autonomie batterie | Baseline | +34% | **+34%** |
| Consommation flash | Baseline | −41% | **−41%** |

**Stack** : C → Rust bare-metal STM32 · Durée : 16 semaines

---

### 7.4 Infrastructure · Cybersécurité — Audit NIS2

**Contexte** : API C++ exposée réseau. 14 vulnérabilités mémoire critiques identifiées. Délai de mise en conformité NIS2 : 3 mois.

**Résultats** :

| Métrique | Avant | Après |
|---|---|---|
| CVE mémoire potentiels | 14 | **0** |
| Conformité NIS2 art.21 | Non | **100%** |
| Délai livraison rapport | — | **10 jours** |

**Stack** : C++ → Rust/cargo-fuzz · Durée : 10 semaines

---

### 7.5 Énergie · Green IT — Pipeline ML Node.js → Rust

**Contexte** : pipeline d'inférence Node.js consommant 8 400 kWh/mois sur 35 instances. Obligation CSRD à venir.

**Résultats après migration** :

| Métrique | Avant | Après | Gain |
|---|---|---|---|
| Consommation mensuelle | 8 400 kWh | 320 kWh | **−96%** |
| CO₂ évité | — | 2,4 t/an | |
| Économies cloud | — | 94k€/an | |

**Stack** : Node.js → Rust · Durée : 10 semaines

---

### 7.6 Scale-up · Streaming — Microservices WASM edge

**Contexte** : microservices de transcodage Go avec 450 ms de latence p99 sous pic. Coût Cloudflare Workers trop élevé.

**Résultats après migration** :

| Métrique | Avant | Après | Gain |
|---|---|---|---|
| Latence p99 edge | 450 ms | 38 ms | **−92%** |
| Coût Cloudflare Workers | Baseline | −71% | **−71%** |

**Stack** : Go → Rust/WASM · Durée : 4 semaines

---

## 8. Thrustcode Academy

### Parcours de formation

#### Niveau 1 — Rust Foundations *(3 jours)*

**Public** : développeurs Python, Java ou Go sans expérience Rust.

**Programme** :
- Ownership & borrow checker
- Types, enums, pattern matching
- Error handling avec `Result` et `Option`
- Cargo, crates & ecosystem
- Projet fil rouge : CLI tool en Rust

**Format** : intra / inter-entreprise · présentiel ou distanciel

---

#### Niveau 2 — Rust in Production *(5 jours)*

**Public** : développeurs ayant les bases de Rust, souhaitant développer des services production-ready.

**Programme** :
- Async Rust & runtime Tokio
- Actix-web & Axum frameworks
- Tests unitaires & d'intégration
- CI/CD avec GitHub Actions
- Profiling & optimisation (cargo flamegraph, perf)
- Projet fil rouge : API REST Rust complète avec tests

**Format** : intra obligatoire

---

#### Niveau 3 — Rust Systems Expert *(5 jours)*

**Public** : développeurs souhaitant maîtriser les aspects bas niveau et systèmes de Rust.

**Programme** :
- Unsafe Rust & raw pointers
- FFI & interopérabilité C/C++
- Embedded bare-metal Rust (STM32, ESP32, RISC-V)
- WASM : compilation et déploiement
- Cryptographie avec les crates Rust (rustls, ring)
- Projet fil rouge : système complet

**Format** : intra · sur devis

---

### Mini-cours décideurs : "5 jours pour comprendre Rust"

Séquence email non-technique destinée aux DSI, CTO et managers souhaitant évaluer une migration Rust sans compétence technique préalable.

- Gratuit · 1 email/jour · désabonnement en 1 clic
- Inclut les chiffres clés, les cas d'usage, les retours d'expérience industrie

---

### Webinaires mensuels

Sessions live gratuites sur inscription :

| Thème | Pilier | Fréquence |
|---|---|---|
| Retours d'expérience migration | Performance | Mensuel |
| NIS2 & DORA — comment Rust aide | Sécurité | Mensuel |
| CSRD numérique — mesurer sa stack | Green IT | Mensuel |

---

## 9. Identité & valeurs

### Manifeste

Thrustcode est née d'une conviction simple : la performance, la sécurité et la durabilité ne sont pas des compromis — elles sont les conséquences d'un choix technologique. Rust est ce choix.

Nous construisons des logiciels qui ne flanquent pas sous la charge, qui ne laissent pas passer les vulnérabilités, et qui n'épuisent pas inutilement les ressources de la planète.

Pas de hype. Pas de buzzwords. Des métriques, des benchmarks, des livrables.

### Les 6 valeurs fondamentales

**1. Mesurer avant d'affirmer**
Chaque promesse de performance est accompagnée d'un benchmark reproductible. Chaque amélioration sécurité d'un rapport sourcé. Toutes les sources sont citées.

**2. Livrer, pas promettre**
Sprints courts, résultats tangibles à chaque itération. Pas de projets-tunnels. Le client voit de la valeur avant la fin du premier mois.

**3. Transparence technique**
Les clients comprennent ce que nous construisons. Documentation, commentaires, formation — le code appartient au client, pas à Thrustcode.

**4. Sobriété numérique**
Un code sobre n'est pas un compromis, c'est une excellence. L'empreinte énergétique de chaque livrable est mesurée systématiquement.

**5. Sécurité par conception**
La sécurité n'est pas un audit post-lancement. Elle est dans le compilateur, dans chaque PR, dans chaque choix d'architecture.

**6. Transfert de compétences**
L'objectif est que les clients n'aient plus besoin de Thrustcode sur le code livré. Formation, documentation, autonomisation des équipes.

---

## 10. Équipe fondatrice

### Thomas Laurent — CEO & Lead Architect

12 ans de systems programming. Ex-contributeur kernel Linux. Ex-équipe AWS Nitro System. Spécialiste performance et allocations mémoire.

**Stack** : Rust, C, Linux kernel, AWS infra

---

### Sofia Caramel — CTO & Security Lead

Ex-ANSSI (Agence Nationale de la Sécurité des Systèmes d'Information). Spécialiste en sécurité mémoire et cryptographie. Contribue au projet Rust-for-Linux. Experte NIS2 & DORA.

**Stack** : Rust, cryptographie, sécurité systèmes, conformité réglementaire

---

### Marc Berthier — Head of Green IT

Ingénieur énergie reconverti développeur. Créateur de Scaphandre-reporter, outil de mesure de la consommation logicielle par processus. Expert CSRD et bilan carbone numérique.

**Stack** : Rust, Scaphandre, RAPL, mesure énergétique, CSRD

---

### Amara Ndiaye — Embedded Systems Engineer

10 ans de firmware embarqué sur STM32, RISC-V et ESP32. Pionnière de Rust bare-metal en environnement industriel et médical.

**Stack** : Rust bare-metal, STM32, RISC-V, ESP32, systèmes temps réel

---

## 11. Architecture du site web

**Domaine** : [thrustcode.dev](https://thrustcode.dev)

### Structure des pages

| Page | URL | Objectif | Lead magnet |
|---|---|---|---|
| Home | `/` | Conversion initiale, crédibilité | Audit de performance gratuit |
| Performance Engineering | `/services/performance` | Qualification leads perf | Calculateur ROI cloud |
| Secure by Design | `/services/security` | Qualification leads sécu | Rapport sécurité 20 points |
| Green IT | `/services/green-it` | Qualification leads RSE/DAF | Simulateur CO₂ interactif |
| Embedded & IoT | `/services/embedded` | Leads industrie | Guide embedded Rust PDF |
| Études de cas | `/case-studies` | Preuve sociale, crédibilité | Case study PDF détaillé |
| Blog | `/blog` | SEO, nurturing, autorité | Newsletter The Rust Report |
| Academy | `/academy` | Formation, nurturing long terme | Cours Rust offert (email gate) |
| À propos | `/about` | Confiance, recrutement | — |
| Contact | `/contact` | Conversion finale | Audit 30 min offert |

### Design system

**Palette** : Apple dark mode — `#000000` fond principal, `#0071E3` bleu système, `#30D158` vert, `#BF5AF2` violet, `#FF9F0A` orange, `#5AC8FA` teal.

**Glassmorphism** : toutes les surfaces en verre — `rgba(255,255,255,0.055)` + `backdrop-filter: blur(24px)` + border `rgba(255,255,255,0.10)`. Nav macOS Ventura : `saturate(180%) blur(20px)`.

**Typographie** : Inter (fallback SF Pro Display) — même rendu qu'une interface macOS.

**Animations** : scroll-triggered reveals (IntersectionObserver natif), curseur custom animé, ticker défilant, orbes de mesh gradient, compteurs animés au viewport.

**Performances** : zéro librairie JavaScript externe, CSS natif uniquement, animations sur `transform` et `opacity` exclusivement.

---

## 12. Sources & références

Toutes les données présentées par Thrustcode reposent sur des sources vérifiables et récentes.

### Performance

- BenchCraft (2025) — *Rust vs Go vs Python benchmark CPU-intensif* : Rust ×60 vs Python, ×2 vs Go
- TechEmpower Benchmarks Round 23 (2025) — *Web framework throughput*
- Netguru (2025) — *Golang vs Rust benchmark* : 60 000 rps Rust vs 40 000 rps Go
- Stack Overflow Developer Survey 2025 (n=49 000) — *Rust langue la plus admirée, 10 ans consécutifs*

### Sécurité

- NSA/CISA — *Memory Safe Languages: Reducing Vulnerabilities in Modern Software Development* (juin 2025) — [media.defense.gov](https://media.defense.gov/2025/Jun/23/2003742198/-1/-1/0/CSI_MEMORY_SAFE_LANGUAGES_REDUCING_VULNERABILITIES_IN_MODERN_SOFTWARE_DEVELOPMENT.PDF)
- CISA (2023) — *The Case for Memory Safe Roadmaps*
- White House ONCD (fév. 2024) — *Back to the Building Blocks: A Path Toward Secure and Measurable Software*
- Google/Android (2024) — Réduction bugs mémoire de 76% à 24% après adoption Rust/Java (cité dans le guide CISA/NSA juin 2025)

### Green IT

- AWS Open Source Blog (2022) — *Sustainability with Rust* — [aws.amazon.com](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/) : Rust ~50% plus efficace que Java, ~98% que Python
- Pereira et al. (2017, 2021) — *Energy Efficiency Across Programming Languages* — Computer Language Benchmarks Game, 27 langages
- ArXiv (oct. 2024) — *It's Not Easy Being Green: On the Energy Efficiency of Programming Languages*

### Marché ESN

- Numeum/PAC — *Bilan 2024 et perspectives 2025* (décembre 2024) : ESN = 34,5 Md€, +0,7% en 2024
- KPMG/Numeum — *Grand Angle ESN & ICT 2024* : cybersécurité et numérique responsable comme leviers prioritaires

### Adoption industrielle

- JetBrains — *State of Developer Ecosystem 2024* : ~2,27 millions de développeurs Rust
- Stack Overflow / LinkedIn (2025) : +35% d'offres Rust YoY
- Microsoft (2025) : migration Hyper-V (Azure) vers Rust ; "année de Rust"

---

*Document généré le 29 mars 2026 · Version 1.0*
*Thrustcode SAS · thrustcode.dev · hello@thrustcode.dev*
