# ASKGUI — Documentação Técnica Completa

**Versão:** 1.0.0  
**Data:** 2026-03-09  
**Classificação:** Whitepaper Técnico — Replicação Total  
**Autor do Sistema:** Guilherme (ASKGUI)

---

## Sumário

1. [Visão Geral do Projeto](#1-visão-geral-do-projeto)
2. [Arquitetura do Projeto](#2-arquitetura-do-projeto)
3. [Stack Tecnológica](#3-stack-tecnológica)
4. [Sistema de Design (Design System)](#4-sistema-de-design)
5. [Sistema de Tema](#5-sistema-de-tema)
6. [Componentes UI](#6-componentes-ui)
7. [Documentação Completa da Página Overlay](#7-documentação-completa-da-página-overlay)
8. [Fluxo de Renderização](#8-fluxo-de-renderização)
9. [Lógica do Sistema](#9-lógica-do-sistema)
10. [Sistema de Animação](#10-sistema-de-animação)
11. [Responsividade](#11-responsividade)
12. [Assets](#12-assets)
13. [Processo de Build](#13-processo-de-build)
14. [Guia de Reprodução Completa](#14-guia-de-reprodução-completa)
15. [Checklist de Fidelidade](#15-checklist-de-fidelidade)

---

## 1. Visão Geral do Projeto

### Propósito

ASKGUI é um portfólio de desenvolvedor com módulos integrados:

- **Portfolio** — Landing page pessoal com efeito Matrix Rain, typewriter, links rápidos via drawer
- **ASKGUI Library** — Biblioteca digital de livros com leitor embutido, controle de fonte, modo claro/escuro
- **Overlay Studio** — Interface para criação de overlays para transmissões ao vivo (OBS, Streamlabs)

### Conceito Visual

Estética **terminal/hacker monoespacada**, com influências brutalist e cyberpunk:
- Tipografia monoespacada (`Space Mono`)
- Tracking expandido em labels e headings
- Bordas retas (border-radius: 0)
- Background escuro quase puro preto (`hsl(0,0%,3%)`)
- Acento laranja vibrante (`hsl(24,95%,53%)`)
- Texto uppercase com letter-spacing exagerado em labels

### Filosofia de Design

- Inspiração em interfaces de terminal/CLI
- Minimalismo funcional — cada elemento tem propósito
- Zero border-radius (brutalist)
- Hierarquia visual por opacidade e tamanho de fonte, não por cor
- Animações sutis para feedback, não decoração

### Experiência do Overlay Studio

O Overlay Studio permite que streamers:
1. Adicionem widgets visuais (texto, relógio, ticker, banner, now-playing)
2. Posicionem via coordenadas X/Y percentuais
3. Editem conteúdo em tempo real
4. Alternem entre tema BLACK e WHITE
5. Exportem para uso como Browser Source no OBS

---

## 2. Arquitetura do Projeto

### Estrutura de Pastas

```text
/
├── public/
│   ├── favicon.ico
│   ├── placeholder.svg
│   └── robots.txt
├── src/
│   ├── assets/
│   │   └── profile.png              # Foto de perfil (Portfolio)
│   ├── components/
│   │   └── ui/                      # shadcn/ui components (50+ arquivos)
│   │       ├── accordion.tsx
│   │       ├── alert-dialog.tsx
│   │       ├── button.tsx
│   │       ├── card.tsx
│   │       ├── dialog.tsx
│   │       ├── drawer.tsx           # Usado no Portfolio (Quick Links)
│   │       ├── resizable.tsx
│   │       ├── scroll-area.tsx
│   │       ├── select.tsx
│   │       ├── separator.tsx
│   │       ├── sheet.tsx
│   │       ├── sidebar.tsx
│   │       ├── toast.tsx
│   │       ├── toaster.tsx
│   │       └── ... (demais componentes shadcn)
│   ├── data/
│   │   └── books.ts                 # Dados dos livros (5 livros, ~900 linhas)
│   ├── hooks/
│   │   ├── use-mobile.tsx           # Hook de detecção mobile
│   │   └── use-toast.ts             # Hook de toast
│   ├── lib/
│   │   └── utils.ts                 # cn() utility (clsx + tailwind-merge)
│   ├── pages/
│   │   ├── Portfolio.tsx            # Landing page principal
│   │   ├── BooksPage.tsx            # Listagem de livros
│   │   ├── BookDetail.tsx           # Detalhe do livro
│   │   ├── BookReader.tsx           # Leitor de capítulos
│   │   └── OverlayPage.tsx          # 🔴 OVERLAY STUDIO (componente crítico)
│   ├── test/
│   │   ├── setup.ts
│   │   └── example.test.ts
│   ├── App.tsx                      # Router principal
│   ├── App.css                      # CSS legado (não utilizado ativamente)
│   ├── index.css                    # 🔴 DESIGN SYSTEM PRINCIPAL
│   ├── main.tsx                     # Entry point
│   └── vite-env.d.ts
├── index.html                       # HTML entry
├── package.json
├── tailwind.config.ts               # 🔴 CONFIGURAÇÃO TAILWIND
├── tsconfig.json
├── tsconfig.app.json
├── tsconfig.node.json
├── vite.config.ts
├── vitest.config.ts
├── postcss.config.js
├── eslint.config.js
└── components.json                  # shadcn/ui config
```

### Organização do Código

- **Pages** — Componentes de página completos, auto-contidos
- **Components/ui** — Primitivos shadcn/ui reutilizáveis
- **Data** — Dados estáticos (books)
- **Hooks** — React hooks customizados
- **Lib** — Utilidades puras

### Roteamento

```tsx
// src/App.tsx
<Routes>
  <Route path="/" element={<Portfolio />} />
  <Route path="/portfolio" element={<Portfolio />} />
  <Route path="/books" element={<BooksPage />} />
  <Route path="/book/:id" element={<BookDetail />} />
  <Route path="/book/:id/read/:chapterId" element={<BookReader />} />
  <Route path="/overlay" element={<OverlayPage />} />
  <Route path="*" element={<Navigate to="/" replace />} />
</Routes>
```

---

## 3. Stack Tecnológica

| Camada | Tecnologia | Versão | Propósito |
|--------|-----------|--------|-----------|
| **Framework** | React | ^18.3.1 | UI library |
| **Linguagem** | TypeScript | ^5.8.3 | Type safety |
| **Bundler** | Vite | ^5.4.19 | Build tool + HMR |
| **Estilo** | Tailwind CSS | ^3.4.17 | Utility-first CSS |
| **Componentes** | shadcn/ui | — | Component primitives (Radix UI) |
| **Animação** | Framer Motion | ^12.35.0 | Declarative animations |
| **Roteamento** | React Router DOM | ^6.30.1 | Client-side routing |
| **Estado Server** | TanStack React Query | ^5.83.0 | Server state management |
| **Ícones** | Lucide React | ^0.462.0 | Icon library |
| **Drawer** | Vaul | ^0.9.9 | Bottom sheet / drawer |
| **Toast** | Sonner | ^1.7.4 | Toast notifications |
| **Forms** | React Hook Form + Zod | ^7.61.1 / ^3.25.76 | Form validation |
| **Animação CSS** | tailwindcss-animate | ^1.0.7 | CSS animation utilities |
| **Merge** | tailwind-merge | ^2.6.0 | Class deduplication |
| **CVA** | class-variance-authority | ^0.7.1 | Variant styling |
| **SWC** | @vitejs/plugin-react-swc | ^3.11.0 | Fast JSX transform |
| **Testes** | Vitest | ^3.2.4 | Test runner |
| **DOM Testing** | @testing-library/react | ^16.0.0 | Component testing |

### Dependências Completas (package.json)

```json
{
  "dependencies": {
    "@hookform/resolvers": "^3.10.0",
    "@radix-ui/react-accordion": "^1.2.11",
    "@radix-ui/react-alert-dialog": "^1.1.14",
    "@radix-ui/react-aspect-ratio": "^1.1.7",
    "@radix-ui/react-avatar": "^1.1.10",
    "@radix-ui/react-checkbox": "^1.3.2",
    "@radix-ui/react-collapsible": "^1.1.11",
    "@radix-ui/react-context-menu": "^2.2.15",
    "@radix-ui/react-dialog": "^1.1.14",
    "@radix-ui/react-dropdown-menu": "^2.1.15",
    "@radix-ui/react-hover-card": "^1.1.14",
    "@radix-ui/react-label": "^2.1.7",
    "@radix-ui/react-menubar": "^1.1.15",
    "@radix-ui/react-navigation-menu": "^1.2.13",
    "@radix-ui/react-popover": "^1.1.14",
    "@radix-ui/react-progress": "^1.1.7",
    "@radix-ui/react-radio-group": "^1.3.7",
    "@radix-ui/react-scroll-area": "^1.2.9",
    "@radix-ui/react-select": "^2.2.5",
    "@radix-ui/react-separator": "^1.1.7",
    "@radix-ui/react-slider": "^1.3.5",
    "@radix-ui/react-slot": "^1.2.3",
    "@radix-ui/react-switch": "^1.2.5",
    "@radix-ui/react-tabs": "^1.1.12",
    "@radix-ui/react-toast": "^1.2.14",
    "@radix-ui/react-toggle": "^1.1.9",
    "@radix-ui/react-toggle-group": "^1.1.10",
    "@radix-ui/react-tooltip": "^1.2.7",
    "@tanstack/react-query": "^5.83.0",
    "class-variance-authority": "^0.7.1",
    "clsx": "^2.1.1",
    "cmdk": "^1.1.1",
    "date-fns": "^3.6.0",
    "embla-carousel-react": "^8.6.0",
    "framer-motion": "^12.35.0",
    "input-otp": "^1.4.2",
    "lucide-react": "^0.462.0",
    "next-themes": "^0.3.0",
    "react": "^18.3.1",
    "react-day-picker": "^8.10.1",
    "react-dom": "^18.3.1",
    "react-hook-form": "^7.61.1",
    "react-resizable-panels": "^2.1.9",
    "react-router-dom": "^6.30.1",
    "recharts": "^2.15.4",
    "sonner": "^1.7.4",
    "tailwind-merge": "^2.6.0",
    "tailwindcss-animate": "^1.0.7",
    "vaul": "^0.9.9",
    "zod": "^3.25.76"
  },
  "devDependencies": {
    "@eslint/js": "^9.32.0",
    "@testing-library/jest-dom": "^6.6.0",
    "@testing-library/react": "^16.0.0",
    "@tailwindcss/typography": "^0.5.16",
    "@types/node": "^22.16.5",
    "@types/react": "^18.3.23",
    "@types/react-dom": "^18.3.7",
    "@vitejs/plugin-react-swc": "^3.11.0",
    "autoprefixer": "^10.4.21",
    "eslint": "^9.32.0",
    "eslint-plugin-react-hooks": "^5.2.0",
    "eslint-plugin-react-refresh": "^0.4.20",
    "globals": "^15.15.0",
    "jsdom": "^20.0.3",
    "lovable-tagger": "^1.1.13",
    "postcss": "^8.5.6",
    "tailwindcss": "^3.4.17",
    "typescript": "^5.8.3",
    "typescript-eslint": "^8.38.0",
    "vite": "^5.4.19",
    "vitest": "^3.2.4"
  }
}
```

---

## 4. Sistema de Design

### 4.1 Tokens de Cor (CSS Custom Properties)

Todas as cores são definidas em `src/index.css` como valores HSL **sem** a função `hsl()`, aplicados via `hsl(var(--token))` no Tailwind.

#### Paleta Completa

| Token | HSL | HEX | RGB | Uso |
|-------|-----|-----|-----|-----|
| `--background` | `0 0% 3%` | `#080808` | `rgb(8,8,8)` | Fundo global |
| `--foreground` | `0 0% 90%` | `#E6E6E6` | `rgb(230,230,230)` | Texto principal |
| `--card` | `0 0% 6%` | `#0F0F0F` | `rgb(15,15,15)` | Fundo de cards |
| `--card-foreground` | `0 0% 90%` | `#E6E6E6` | `rgb(230,230,230)` | Texto em cards |
| `--popover` | `0 0% 6%` | `#0F0F0F` | `rgb(15,15,15)` | Fundo de popovers |
| `--popover-foreground` | `0 0% 90%` | `#E6E6E6` | `rgb(230,230,230)` | Texto em popovers |
| `--primary` | `24 95% 53%` | `#F97316` | `rgb(249,115,22)` | Cor de acento principal (laranja) |
| `--primary-foreground` | `0 0% 3%` | `#080808` | `rgb(8,8,8)` | Texto sobre primary |
| `--secondary` | `0 0% 12%` | `#1F1F1F` | `rgb(31,31,31)` | Fundo secundário |
| `--secondary-foreground` | `0 0% 80%` | `#CCCCCC` | `rgb(204,204,204)` | Texto secundário |
| `--muted` | `0 0% 10%` | `#1A1A1A` | `rgb(26,26,26)` | Fundo mutado |
| `--muted-foreground` | `0 0% 55%` | `#8C8C8C` | `rgb(140,140,140)` | Texto mutado |
| `--accent` | `30 90% 50%` | `#F29D13` | `rgb(242,157,19)` | Acento secundário |
| `--accent-foreground` | `0 0% 3%` | `#080808` | `rgb(8,8,8)` | Texto sobre accent |
| `--destructive` | `0 84% 60%` | `#EF4444` | `rgb(239,68,68)` | Cor de erro/destruição |
| `--destructive-foreground` | `0 0% 98%` | `#FAFAFA` | `rgb(250,250,250)` | Texto sobre destructive |
| `--border` | `0 0% 15%` | `#262626` | `rgb(38,38,38)` | Cor de borda |
| `--input` | `0 0% 15%` | `#262626` | `rgb(38,38,38)` | Borda de inputs |
| `--ring` | `24 95% 53%` | `#F97316` | `rgb(249,115,22)` | Focus ring |
| `--radius` | `0px` | — | — | Border radius global |

#### Sidebar Tokens

| Token | HSL | Uso |
|-------|-----|-----|
| `--sidebar-background` | `0 0% 5%` | Fundo do sidebar |
| `--sidebar-foreground` | `0 0% 90%` | Texto do sidebar |
| `--sidebar-primary` | `24 95% 53%` | Primary do sidebar |
| `--sidebar-primary-foreground` | `0 0% 3%` | Texto sobre primary |
| `--sidebar-accent` | `0 0% 10%` | Accent do sidebar |
| `--sidebar-accent-foreground` | `0 0% 90%` | Texto sobre accent |
| `--sidebar-border` | `0 0% 15%` | Borda do sidebar |
| `--sidebar-ring` | `24 95% 53%` | Ring do sidebar |

### 4.2 Tipografia

#### Fontes

```css
@import url('https://fonts.googleapis.com/css2?family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&family=Courier+Prime:wght@400;700&display=swap');
```

| Variável | Stack | Uso |
|----------|-------|-----|
| `--font-mono` | `'Space Mono', 'Courier Prime', monospace` | Corpo de texto, labels, todo conteúdo |
| `--font-display` | `'Space Mono', monospace` | Headings, títulos |

#### Pesos Utilizados

- `400` (normal) — Corpo de texto, labels
- `700` (bold) — Headings, destaques, títulos

#### Escala de Tamanhos de Fonte (Tailwind)

| Classe | Tamanho | Uso |
|--------|---------|-----|
| `text-[8px]` | 8px | Micro tags |
| `text-[9px]` | 9px | Labels secundários, metadata |
| `text-[10px]` | 10px | Labels primários, nav links, tracking labels |
| `text-[11px]` | 11px | Botão CTA, link titles |
| `text-xs` | 12px | Corpo de texto, descrições |
| `text-sm` | 14px | Texto principal, typewriter |
| `text-lg` | 18px | Headings de seção |
| `text-xl` | 20px | Subheadings |
| `text-2xl` | 24px | Títulos |
| `text-3xl` | 30px | Títulos grandes |
| `text-4xl` | 36px | Hero titulo (mobile) |
| `text-5xl` | 48px | Hero titulo (base) |
| `text-6xl` | 60px | Hero titulo (md), Library title |
| `text-7xl` | 72px | Hero titulo (lg) |
| `text-8xl` | 96px | Cover emoji, hero (xl) |

#### Letter Spacing

| Valor | Uso |
|-------|-----|
| `tracking-[0.04em]` | Títulos grandes |
| `tracking-[0.06em]` | Títulos de capítulos |
| `tracking-[0.08em]` | Headings (h1-h6 global) |
| `tracking-[0.1em]` | Labels inline, tags, metadata |
| `tracking-[0.15em]` | Nav brand, genre badges, link subtitles |
| `tracking-[0.2em]` | CTAs, overlays labels, section headers |
| `tracking-[0.25em]` | Nav links, section labels |
| `tracking-[0.3em]` | CTA brackets |

#### Text Transform

Todos os headings (h1-h6) e labels utilizam `uppercase` globalmente via `index.css`:

```css
h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-display);
  letter-spacing: 0.08em;
  text-transform: uppercase;
}
```

#### Line Height

- `leading-[0.9]` — Hero title (comprimido)
- `leading-[1.8]` — Book reader content
- `leading-relaxed` — Synopsis, descrições

### 4.3 Escala de Espaçamento

Utiliza escala padrão Tailwind:

| Classe | Valor | Uso típico |
|--------|-------|------------|
| `gap-1` | 4px | Ícone + texto inline |
| `gap-2` | 8px | Itens de lista, botões grid |
| `gap-3` | 12px | Widgets panel spacing |
| `gap-4` | 16px | Seções menores |
| `gap-6` | 24px | Grid cards, stats |
| `py-1.5` | 6px | Badges, small buttons |
| `py-2` | 8px | Widgets, nav |
| `py-3` | 12px | Nav bars, CTAs |
| `py-8` | 32px | Footer |
| `py-16` | 64px | Seções |
| `px-3` | 12px | Botões, badges |
| `px-4` | 16px | Containers, cards |
| `px-6` | 24px | CTAs, cards grandes |
| `px-8` | 32px | Containers md+ |
| `p-4` | 16px | Sidebar, widget editor |
| `p-6` | 24px | Info sections |

### 4.4 Border Radius

```css
--radius: 0px;
```

**Border radius é ZERO em todo o sistema.** Todas as bordas são retas (estética brutalist/terminal). Exceções mínimas:
- Scroll indicator dot: `rounded-full` (2px circle)
- Scrollbar thumb: nenhum radius
- Pulse dot: `rounded-full`

### 4.5 Shadows

O sistema não utiliza box-shadows no design principal. A exceção é:
- Botão "Links Rápidos" fixo: `shadow-lg` (sombra para destacar do conteúdo)

### 4.6 Opacidade

| Valor | Uso |
|-------|-----|
| `opacity-30` | Ícone placeholder (overlay vazio) |
| `opacity-40` | Matrix rain canvas |
| `opacity-50` | Resolution indicator, textos fantasma |
| `/5` | Background hover sutil (selected state) |
| `/10` | TOC selected chapter bg |
| `/20` | Line numbers |
| `/30` | Badge borders, hover borders, marquee text |
| `/40` | Code comments, footer text |
| `/50` | Stats text, bottom bar muted |
| `/60` | TOC overlay backdrop |
| `/80` | Nav backdrop, widget backgrounds |
| `/90` | Widget backgrounds mais sólidos |

### 4.7 Blur

| Classe | Uso |
|--------|-----|
| `backdrop-blur-md` | Nav bars (todos), widget bg |
| `backdrop-blur-sm` | TOC overlay, widgets |

---

## 5. Sistema de Tema

### 5.1 Tema Escuro (DEFAULT — BLACK)

O tema escuro é o padrão do sistema, definido via tokens CSS em `:root`:

```css
:root {
  --background: 0 0% 3%;      /* #080808 */
  --foreground: 0 0% 90%;     /* #E6E6E6 */
  --primary: 24 95% 53%;      /* #F97316 — Laranja */
  --border: 0 0% 15%;         /* #262626 */
  --muted-foreground: 0 0% 55%; /* #8C8C8C */
}
```

#### Características
- Background quase preto puro
- Texto cinza claro (alto contraste)
- Acento laranja vibrante
- Bordas sutis em cinza escuro
- Sem gradientes
- Matrix Rain em laranja sobre preto

### 5.2 Tema Claro (WHITE) — Overlay & BookReader

O tema claro usa cores **hardcoded** (não tokens CSS) para as páginas de Overlay e BookReader:

#### Overlay Studio — Light Theme

```tsx
const themeStyles = {
  light: {
    bg: "bg-amber-50",          // #FFFBEB
    text: "text-stone-800",      // #292524
    muted: "text-stone-500",     // #78716C
    border: "border-stone-300",  // #D6D3D1
    card: "bg-white",            // #FFFFFF
    accent: "bg-[hsl(24,95%,53%)]", // #F97316 (mesmo accent)
    widgetBg: "bg-white/90",
    previewBg: "bg-amber-50",    // #FFFBEB
  }
};
```

#### BookReader — Light Mode

```tsx
const readerBg = lightMode ? "bg-amber-50" : "bg-background";
const readerText = lightMode ? "text-stone-800" : "text-foreground";
const readerMuted = lightMode ? "text-stone-500" : "text-muted-foreground";
```

#### Paleta Light (valores exatos)

| Elemento | Classe Tailwind | HEX Aproximado |
|----------|----------------|----------------|
| Background | `bg-amber-50` | `#FFFBEB` |
| Texto principal | `text-stone-800` | `#292524` |
| Texto mutado | `text-stone-500` | `#78716C` |
| Bordas | `border-stone-300` | `#D6D3D1` |
| Card / Widget bg | `bg-white` / `bg-white/90` | `#FFFFFF` |
| Accent | `hsl(24,95%,53%)` | `#F97316` |
| Input bg | `bg-amber-50` | `#FFFBEB` |
| Code inline | `bg-amber-100 text-amber-900` | `#FEF3C7` / `#78350F` |
| Checkerboard | `#e8e0d4` | — |

---

## 6. Componentes UI

### 6.1 Component Classes Customizadas (index.css)

```css
.nav-link {
  @apply text-xs tracking-[0.25em] uppercase text-muted-foreground 
         hover:text-foreground transition-colors duration-300;
  font-family: var(--font-mono);
}

.section-label {
  @apply text-xs tracking-[0.2em] text-muted-foreground uppercase;
  font-family: var(--font-mono);
}

.hero-title {
  @apply text-4xl md:text-6xl lg:text-7xl font-bold tracking-[0.06em] 
         uppercase text-foreground;
  font-family: var(--font-display);
}

.hero-subtitle {
  @apply text-xs md:text-sm tracking-[0.15em] uppercase 
         text-muted-foreground italic;
  font-family: var(--font-mono);
}

.cta-bracket {
  @apply text-xs tracking-[0.3em] uppercase text-foreground 
         border border-foreground/30 px-6 py-3 
         hover:bg-foreground hover:text-background 
         transition-all duration-300 inline-block;
  font-family: var(--font-mono);
}

.announcement-bar {
  @apply text-[10px] tracking-[0.3em] uppercase text-muted-foreground 
         py-2 text-center border-b border-border;
  font-family: var(--font-mono);
}
```

### 6.2 Scrollbar Customizado

```css
::-webkit-scrollbar { width: 4px; }
::-webkit-scrollbar-track { background: hsl(var(--background)); }
::-webkit-scrollbar-thumb { background: hsl(var(--muted-foreground)); }

.scrollbar-hide::-webkit-scrollbar { display: none; }
.scrollbar-hide { -ms-overflow-style: none; scrollbar-width: none; }
```

### 6.3 Padrão de Botão do Sistema

O sistema usa dois padrões de botão:

**Botão Primary (CTA fixo):**
```tsx
className="bg-primary text-primary-foreground px-6 py-3 text-[11px] 
           font-mono tracking-[0.2em] uppercase hover:bg-primary/90 
           transition-colors shadow-lg"
```

**Botão Outline (Widget/Tool):**
```tsx
className={`flex items-center gap-1.5 border ${t.border} px-3 py-1.5 
            text-[10px] font-mono tracking-[0.1em] uppercase ${t.muted} 
            hover:text-[hsl(24,95%,53%)] hover:border-[hsl(24,95%,53%)] 
            transition-all duration-300`}
```

**Botão Ghost (Nav):**
```tsx
className="text-muted-foreground hover:text-foreground transition-colors"
```

### 6.4 Nav Bar Pattern

Todas as páginas compartilham o mesmo padrão de nav:

```tsx
<nav className="fixed top-0 left-0 right-0 z-50 ...">
  {/* Left: Back button */}
  {/* Center: Title */}
  {/* Right: Actions */}
</nav>
```

### 6.5 Card/Item Pattern

```tsx
className="border border-border p-6 cursor-pointer group 
           hover:border-primary/40 transition-all duration-300"
```

### 6.6 Badge/Tag Pattern

```tsx
className="text-[10px] tracking-[0.15em] uppercase font-mono 
           border border-border px-3 py-1 text-muted-foreground 
           hover:text-primary-foreground hover:bg-primary 
           hover:border-primary transition-all duration-300 cursor-default"
```

---

## 7. Documentação Completa da Página Overlay

### 7.1 Arquivo

`src/pages/OverlayPage.tsx` — 511 linhas

### 7.2 Estrutura JSX

```text
OverlayPage
├── div.min-h-screen (root container com tema dinâmico)
│   ├── nav (barra de navegação fixa)
│   │   ├── Link VOLTAR (← ArrowLeft)
│   │   ├── h1 OVERLAY STUDIO (Tv icon)
│   │   └── button tema (Sun/Moon toggle)
│   │
│   └── div.pt-16.flex (layout principal)
│       ├── aside (sidebar — painel de widgets)
│       │   ├── div ADICIONAR WIDGET
│       │   │   └── grid 2-col (5 botões de tipo)
│       │   ├── div WIDGETS (lista)
│       │   │   └── AnimatePresence > motion.div[] (cards de widget)
│       │   └── AnimatePresence (editor do widget selecionado)
│       │       └── motion.div
│       │           ├── textarea CONTEÚDO
│       │           ├── grid 2-col (X%, Y%)
│       │           └── input NOME
│       │
│       └── main (área de preview)
│           ├── div header (PRÉ-VISUALIZAÇÃO + botões)
│           │   ├── button EXPANDIR/MINIMIZAR
│           │   └── button COPIAR CSS
│           ├── div preview-canvas (aspect-video)
│           │   ├── div checkerboard-bg
│           │   ├── div resolution-indicator "1920×1080"
│           │   ├── OverlayWidgetPreview[] (widgets renderizados)
│           │   └── div empty-state (quando sem widgets)
│           ├── div tema-buttons (BLACK / WHITE)
│           └── div info COMO USAR
```

### 7.3 Tipos TypeScript

```typescript
type OverlayTheme = "dark" | "light";

interface OverlayWidget {
  id: string;
  type: "text" | "clock" | "ticker" | "banner" | "now-playing";
  label: string;
  content: string;
  visible: boolean;
  x: number;    // Posição X em percentual (0-100)
  y: number;    // Posição Y em percentual (0-100)
}
```

### 7.4 Templates de Widget

```typescript
const WIDGET_TEMPLATES = [
  { type: "text",        label: "Texto",       icon: Type,    defaultContent: "Seu texto aqui" },
  { type: "clock",       label: "Relógio",     icon: Clock,   defaultContent: "" },
  { type: "ticker",      label: "Ticker",      icon: Activity,defaultContent: "Breaking news • Insira seu texto aqui •" },
  { type: "banner",      label: "Banner",      icon: Image,   defaultContent: "LIVE NOW" },
  { type: "now-playing", label: "Now Playing",  icon: Monitor, defaultContent: "♫ Música atual" },
];
```

### 7.5 Sistema de Tema do Overlay

```typescript
const themeStyles = {
  dark: {
    bg: "bg-[hsl(0,0%,3%)]",
    text: "text-[hsl(0,0%,90%)]",
    muted: "text-[hsl(0,0%,55%)]",
    border: "border-[hsl(0,0%,15%)]",
    card: "bg-[hsl(0,0%,6%)]",
    accent: "bg-[hsl(24,95%,53%)]",
    widgetBg: "bg-[hsl(0,0%,6%)]/90",
    previewBg: "bg-[hsl(0,0%,3%)]",
  },
  light: {
    bg: "bg-amber-50",
    text: "text-stone-800",
    muted: "text-stone-500",
    border: "border-stone-300",
    card: "bg-white",
    accent: "bg-[hsl(24,95%,53%)]",
    widgetBg: "bg-white/90",
    previewBg: "bg-amber-50",
  },
};
```

### 7.6 Checkerboard Background

O preview simula transparência com um padrão xadrez:

```typescript
// Dark theme
backgroundImage: "linear-gradient(45deg, hsl(0,0%,8%) 25%, transparent 25%), 
                  linear-gradient(-45deg, hsl(0,0%,8%) 25%, transparent 25%), 
                  linear-gradient(45deg, transparent 75%, hsl(0,0%,8%) 75%), 
                  linear-gradient(-45deg, transparent 75%, hsl(0,0%,8%) 75%)"

// Light theme
backgroundImage: "linear-gradient(45deg, #e8e0d4 25%, transparent 25%), 
                  linear-gradient(-45deg, #e8e0d4 25%, transparent 25%), 
                  linear-gradient(45deg, transparent 75%, #e8e0d4 75%), 
                  linear-gradient(-45deg, transparent 75%, #e8e0d4 75%)"

backgroundSize: "20px 20px"
backgroundPosition: "0 0, 0 10px, 10px -10px, -10px 0px"
```

### 7.7 Preview Canvas

```tsx
<div className={`relative aspect-video w-full overflow-hidden border ${t.border} ${t.previewBg}`}>
  {/* checkerboard + widgets */}
</div>
```

### 7.8 Componente OverlayWidgetPreview

Componente separado que renderiza cada widget no canvas:

**Props:**
```typescript
{
  widget: OverlayWidget;
  isDark: boolean;
  isSelected: boolean;
  onClick: () => void;
}
```

**Posicionamento:**
```tsx
style={{ left: `${widget.x}%`, top: `${widget.y}%` }}
```

**Base styles (comum a todos):**
```tsx
const baseStyles = `absolute cursor-pointer transition-all duration-300 ${
  isSelected ? "ring-1 ring-[hsl(24,95%,53%)]" : ""
}`;
```

#### Widget: Text
```tsx
<div className={`${baseStyles} ...`} onClick={onClick}>
  {widget.content}
</div>
```

#### Widget: Clock
```tsx
<div className={`${baseStyles} ...`} onClick={onClick}>
  <div>
    {currentTime.toLocaleTimeString("pt-BR", { hour: "2-digit", minute: "2-digit", second: "2-digit" })}
  </div>
</div>
```

**Clock update:**
```tsx
const [currentTime, setCurrentTime] = useState(new Date());
useState(() => {
  if (widget.type !== "clock") return;
  const interval = setInterval(() => setCurrentTime(new Date()), 1000);
  return () => clearInterval(interval);
});
```
> **Nota:** O clock usa `useState` como initializer (executado uma vez). O ideal seria `useEffect` — este é um padrão funcional mas não-convencional.

#### Widget: Ticker
```tsx
<div className={`${baseStyles} ...`} onClick={onClick}>
  <div className="animate-marquee whitespace-nowrap">
    {widget.content}
  </div>
</div>
```

#### Widget: Banner
```tsx
<div className={`${baseStyles} ...`} onClick={onClick}>
  <div className="flex items-center gap-2">
    <span className="h-2 w-2 rounded-full bg-red-500 animate-pulse" />
    <span>{widget.content}</span>
  </div>
</div>
```

#### Widget: Now Playing
```tsx
<div className={`${baseStyles} ...`} onClick={onClick}>
  <div className="text-[8px] tracking-[0.2em] uppercase">NOW PLAYING</div>
  <div>{widget.content}</div>
</div>
```

### 7.9 Hierarquia Z-Index

| z-index | Elemento |
|---------|----------|
| `z-10` | Resolution indicator |
| `z-40` | Preview fullscreen |
| `z-50` | Nav bar |

### 7.10 Ícones Utilizados (Lucide)

```typescript
import {
  ArrowLeft,    // Nav: voltar
  Sun,          // Tema light
  Moon,         // Tema dark
  Type,         // Widget: texto
  Image,        // Widget: banner
  Clock,        // Widget: relógio
  Activity,     // Widget: ticker
  Eye,          // Toggle visibilidade
  Copy,         // Copiar CSS
  Plus,         // (não usado diretamente, disponível)
  Trash2,       // Remover widget
  GripVertical, // Drag handle visual
  Monitor,      // Widget: now-playing, Expandir
  Tv,           // Título Overlay Studio
} from "lucide-react";
```

---

## 8. Fluxo de Renderização

### 8.1 Overlay Studio — Lifecycle

```text
1. Usuário acessa /overlay
2. React Router renderiza OverlayPage
3. Estado inicial:
   - theme: "dark"
   - widgets: []
   - selectedWidget: null
   - previewFullscreen: false
4. isDark = theme === "dark"
5. t = themeStyles[theme]  (objeto de classes)
6. Renderiza layout: nav + sidebar + preview
7. Preview mostra empty state
```

### 8.2 Adição de Widget

```text
1. Usuário clica botão de tipo (ex: "Texto")
2. addWidget("text") é chamado
3. Template é encontrado em WIDGET_TEMPLATES
4. Novo widget criado com:
   - id: genId() → "w-1-1709913600000"
   - type, label, content do template
   - visible: true
   - x: 10 + Math.random() * 30  (10-40%)
   - y: 10 + Math.random() * 40  (10-50%)
5. Widget adicionado ao estado
6. selectedWidget = novo widget id
7. Preview re-renderiza com OverlayWidgetPreview
8. Editor aparece no sidebar
```

### 8.3 Edição de Widget

```text
1. Usuário edita campo (content, x, y, label)
2. onChange → updateWidget(id, { field: value })
3. widgets.map() recria array com widget atualizado
4. Preview re-renderiza widget na nova posição/conteúdo
```

### 8.4 Toggle Tema

```text
1. Usuário clica Sun/Moon ou botões BLACK/WHITE
2. setTheme("light" | "dark")
3. isDark recalculado
4. t = themeStyles[newTheme]
5. Todas as classes dinâmicas mudam
6. transition-colors duration-500 suaviza
7. Checkerboard pattern muda
8. Todos os widgets re-renderizam com novos estilos
```

### 8.5 Widget Clock — Timer

```text
1. Clock widget é montado
2. useState initializer roda (uma vez)
3. setInterval(1000ms) atualiza currentTime
4. A cada segundo: toLocaleTimeString("pt-BR") formata HH:MM:SS
5. Cleanup não é garantido (bug: deveria usar useEffect)
```

---

## 9. Lógica do Sistema

### 9.1 Estado (OverlayPage)

| Estado | Tipo | Default | Propósito |
|--------|------|---------|-----------|
| `theme` | `OverlayTheme` | `"dark"` | Tema atual |
| `widgets` | `OverlayWidget[]` | `[]` | Lista de widgets |
| `selectedWidget` | `string \| null` | `null` | ID do widget selecionado |
| `previewFullscreen` | `boolean` | `false` | Preview em tela cheia |

### 9.2 Funções

| Função | Parâmetros | Comportamento |
|--------|-----------|---------------|
| `addWidget` | `type: OverlayWidget["type"]` | Cria widget do template, posição random, seleciona |
| `updateWidget` | `id: string, updates: Partial<OverlayWidget>` | Merge parcial no widget |
| `removeWidget` | `id: string` | Remove widget, limpa seleção se necessário |

### 9.3 ID Generation

```typescript
let widgetIdCounter = 0;
const genId = () => `w-${++widgetIdCounter}-${Date.now()}`;
```

Nota: `widgetIdCounter` é **module-level**, persiste entre re-renders mas reseta no hot-reload.

### 9.4 Estado (BookReader)

| Estado | Tipo | Default | Propósito |
|--------|------|---------|-----------|
| `showToc` | `boolean` | `false` | Table of contents visível |
| `fontSize` | `number` | `16` | Tamanho da fonte (12-24) |
| `lightMode` | `boolean` | `false` | Modo claro |
| `showControls` | `boolean` | `true` | Controles visíveis |
| `scrollProgress` | `number` | `0` | Progresso de scroll (0-100) |

### 9.5 Hooks Customizados

#### useTypewriter (Portfolio)

```typescript
const useTypewriter = (
  words: string[], 
  typingSpeed = 80,      // ms por caractere
  deletingSpeed = 50,    // ms por delete
  pauseDelay = 2000      // ms de pausa no final
) => string
```

Ciclo: digita → pausa → apaga → próxima palavra → loop

#### useIsMobile (hooks/use-mobile.tsx)

Detecção de viewport mobile via `matchMedia`.

---

## 10. Sistema de Animação

### 10.1 Framer Motion

Biblioteca principal de animação: `framer-motion ^12.35.0`

#### Padrões de Entrada

**Fade Up (padrão):**
```tsx
initial={{ opacity: 0, y: 20 }}
animate={{ opacity: 1, y: 0 }}
transition={{ duration: 0.6 }}
```

**Fade Up com delay sequencial (listas):**
```tsx
initial={{ opacity: 0, y: 30 }}
animate={{ opacity: 1, y: 0 }}
transition={{ delay: i * 0.1, duration: 0.5 }}
```

**Slide X (widget list):**
```tsx
initial={{ opacity: 0, x: -20 }}
animate={{ opacity: 1, x: 0 }}
exit={{ opacity: 0, x: -20 }}
```

**Slide Y (editor panel):**
```tsx
initial={{ opacity: 0, y: 10 }}
animate={{ opacity: 1, y: 0 }}
exit={{ opacity: 0, y: 10 }}
```

**Spring (TOC sidebar):**
```tsx
initial={{ x: 300, opacity: 0 }}
animate={{ x: 0, opacity: 1 }}
exit={{ x: 300, opacity: 0 }}
transition={{ type: "spring", damping: 25, stiffness: 250 }}
```

**Nav slide (controls):**
```tsx
initial={{ opacity: 0, y: -20 }}
animate={{ opacity: 1, y: 0 }}
exit={{ opacity: 0, y: -20 }}
```

#### AnimatePresence

Usado para exit animations em:
- Widget list items
- Widget editor panel
- BookReader controls
- TOC sidebar

### 10.2 CSS Animations (tailwind.config.ts)

```typescript
keyframes: {
  "accordion-down": {
    from: { height: "0" },
    to: { height: "var(--radix-accordion-content-height)" },
  },
  "accordion-up": {
    from: { height: "var(--radix-accordion-content-height)" },
    to: { height: "0" },
  },
  "fade-up": {
    from: { opacity: "0", transform: "translateY(30px)" },
    to: { opacity: "1", transform: "translateY(0)" },
  },
  "color-slide": {
    "0%": { backgroundPosition: "0% 50%" },
    "50%": { backgroundPosition: "100% 50%" },
    "100%": { backgroundPosition: "0% 50%" },
  },
  "marquee": {
    "0%": { transform: "translateX(0%)" },
    "100%": { transform: "translateX(-50%)" },
  },
},
animation: {
  "accordion-down": "accordion-down 0.2s ease-out",
  "accordion-up": "accordion-up 0.2s ease-out",
  "fade-up": "fade-up 0.8s ease-out forwards",
  "color-slide": "color-slide 4s ease-in-out infinite",
  "marquee": "marquee 20s linear infinite",
},
```

### 10.3 Animações Nativas CSS

| Classe | Uso |
|--------|-----|
| `animate-pulse` | Dot "available", banner live dot, cursor |
| `animate-bounce` | Scroll indicator (chevron) |
| `animate-marquee` | Ticker widget, footer marquee |
| `transition-colors duration-300` | Hover em links, botões |
| `transition-all duration-300` | Cards, widgets, badges |
| `transition-colors duration-500` | Mudança de tema |
| `transition-transform duration-300` | Cover emoji hover scale |

### 10.4 Matrix Rain (Portfolio)

Canvas animation em `Portfolio.tsx`:

```typescript
// Characters: binário + katakana
const chars = "01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
const fontSize = 12;
// Color: hsl(24, 95%, 53%) — primary orange
// Opacity: random 0.02-0.17
// Interval: 50ms
// Reset probability: Math.random() > 0.975
// Canvas opacity: 0.40
// Trail: rgba(8, 8, 8, 0.05) fill
```

---

## 11. Responsividade

### 11.1 Breakpoints (Tailwind defaults)

| Prefixo | Min-width | Uso |
|---------|-----------|-----|
| (base) | 0px | Mobile first |
| `sm` | 640px | Texto inline em nav |
| `md` | 768px | Grid 2-col, nav items, hero size up |
| `lg` | 1024px | Sidebar fixo, grid 3-col, hero max size |

### 11.2 Overlay Studio — Responsividade

```text
Mobile (< 1024px):
├── Sidebar: stacked verticalmente acima do preview
├── aside: width 100%, não fixo
├── Preview: aspect-video, width 100%
└── Widget buttons: grid 2-col

Desktop (≥ 1024px — lg):
├── aside: fixo, lg:w-80, lg:fixed lg:top-[49px] lg:bottom-0 lg:left-0
├── main: lg:ml-80 (compensa sidebar fixo)
└── Preview: max-w-5xl
```

### 11.3 BookReader — Responsividade

```text
Mobile:
├── Controls: botões de texto hidden, só ícones
├── Content: px-6
├── Font size: ajustável (12-24px)
└── Chapter title: truncate max-w-[200px]

Desktop (md+):
├── Controls: labels visíveis (sm:inline)
├── Content: px-8
└── Chapter nav: texto + ícone
```

### 11.4 Portfolio — Responsividade

```text
Mobile:
├── Hero: text-5xl
├── Nav: hamburger (Terminal icon → scroll to contact)
├── Code comments: hidden
├── Line numbers: hidden
├── Stats: hidden
└── Links drawer: full width

Desktop (md+):
├── Hero: text-7xl → lg:text-8xl
├── Nav: inline items
├── Code comments: visible (md:block)
├── Line numbers: visible (md:block)
├── Stats: visible (md:block)
└── Container: max-w-6xl
```

---

## 12. Assets

### 12.1 Imagens

| Arquivo | Tipo | Uso | Import |
|---------|------|-----|--------|
| `src/assets/profile.png` | PNG | Foto de perfil (Portfolio) | ES6 import |
| `public/favicon.ico` | ICO | Favicon | HTML ref |
| `public/placeholder.svg` | SVG | Placeholder genérico | URL ref |

### 12.2 Ícones

Todos via **Lucide React** (`lucide-react ^0.462.0`). Ícones usados:

**Portfolio:**
`ChevronDown`, `Terminal`, `LinkIcon`, `BookOpen`, `Rocket`, `IdCard`, `ShoppingBag`, `GraduationCap`, `Tag`, `Github`, `Linkedin`, `Tv`

**Books:**
`BookOpen`, `Star`, `ArrowLeft`, `Search`, `Clock`, `FileText`, `ChevronRight`

**BookReader:**
`ArrowLeft`, `ArrowRight`, `BookOpen`, `X`, `List`, `Minus`, `Plus`, `Sun`, `Moon`

**Overlay:**
`ArrowLeft`, `Sun`, `Moon`, `Type`, `Image`, `Clock`, `Activity`, `Eye`, `Copy`, `Plus`, `Trash2`, `GripVertical`, `Monitor`, `Tv`

### 12.3 Fontes (Google Fonts)

```text
Space Mono: 400, 400i, 700, 700i
Courier Prime: 400, 700
```

URL de import:
```text
https://fonts.googleapis.com/css2?family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&family=Courier+Prime:wght@400;700&display=swap
```

### 12.4 Emojis (Book Covers)

| Livro | Emoji |
|-------|-------|
| The Rust Systems Handbook | 🦀 |
| Clean Architecture in Practice | 🏗️ |
| Digital Stoicism | 🧠 |
| Neuromancer — Study Notes | 🌐 |
| 10x Developer Mindset | ⚡ |

---

## 13. Processo de Build

### 13.1 Scripts

```json
{
  "dev": "vite",           // Development server (port 8080)
  "build": "vite build",   // Production build
  "build:dev": "vite build --mode development",
  "lint": "eslint .",
  "preview": "vite preview",
  "test": "vitest run",
  "test:watch": "vitest"
}
```

### 13.2 Configuração Vite

```typescript
// vite.config.ts
export default defineConfig(({ mode }) => ({
  server: {
    host: "::",
    port: 8080,
    hmr: { overlay: false },
  },
  plugins: [react(), mode === "development" && componentTagger()].filter(Boolean),
  resolve: {
    alias: { "@": path.resolve(__dirname, "./src") },
  },
}));
```

### 13.3 Como Rodar

```bash
# 1. Clonar repositório
git clone <URL>

# 2. Instalar dependências
npm install

# 3. Development
npm run dev
# Abre em http://localhost:8080

# 4. Build
npm run build
# Output em /dist

# 5. Preview do build
npm run preview

# 6. Testes
npm run test
```

### 13.4 Path Aliases

```json
// tsconfig.json
{
  "compilerOptions": {
    "paths": { "@/*": ["./src/*"] }
  }
}
```

### 13.5 PostCSS

```javascript
// postcss.config.js
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
```

---

## 14. Guia de Reprodução Completa

### Passo 1: Criar Projeto

```bash
npm create vite@latest askgui-portfolio -- --template react-swc-ts
cd askgui-portfolio
```

### Passo 2: Instalar Dependências

```bash
# Core
npm install react-router-dom @tanstack/react-query framer-motion lucide-react

# UI System (shadcn deps)
npm install tailwindcss-animate class-variance-authority clsx tailwind-merge
npm install @radix-ui/react-slot @radix-ui/react-dialog @radix-ui/react-toast
npm install @radix-ui/react-tooltip @radix-ui/react-accordion
npm install @radix-ui/react-alert-dialog @radix-ui/react-aspect-ratio
npm install @radix-ui/react-avatar @radix-ui/react-checkbox
npm install @radix-ui/react-collapsible @radix-ui/react-context-menu
npm install @radix-ui/react-dropdown-menu @radix-ui/react-hover-card
npm install @radix-ui/react-label @radix-ui/react-menubar
npm install @radix-ui/react-navigation-menu @radix-ui/react-popover
npm install @radix-ui/react-progress @radix-ui/react-radio-group
npm install @radix-ui/react-scroll-area @radix-ui/react-select
npm install @radix-ui/react-separator @radix-ui/react-slider
npm install @radix-ui/react-switch @radix-ui/react-tabs
npm install @radix-ui/react-toggle @radix-ui/react-toggle-group

# Additional
npm install vaul sonner cmdk date-fns embla-carousel-react input-otp
npm install next-themes react-day-picker react-hook-form @hookform/resolvers zod
npm install react-resizable-panels recharts

# Dev
npm install -D tailwindcss postcss autoprefixer @tailwindcss/typography
npm install -D @testing-library/react @testing-library/jest-dom vitest jsdom
```

### Passo 3: Configurar Tailwind

Copiar `tailwind.config.ts` exatamente como documentado na seção 3 — incluindo:
- `fontFamily.mono` e `fontFamily.display`
- Todas as cores semânticas
- Keyframes e animations
- Border radius
- Container config

### Passo 4: Configurar index.css

Copiar `src/index.css` completo — incluindo:
- Google Fonts import
- Todos os CSS custom properties em `:root`
- Classes utilitárias (nav-link, section-label, hero-title, etc.)
- Scrollbar customizado

### Passo 5: Configurar Vite

Copiar `vite.config.ts` com alias `@` → `./src`.

### Passo 6: Configurar TypeScript

Copiar `tsconfig.json`, `tsconfig.app.json`, `tsconfig.node.json`.

### Passo 7: Inicializar shadcn/ui

Copiar `components.json` e todos os componentes de `src/components/ui/`.

### Passo 8: Criar Estrutura de Dados

Copiar `src/data/books.ts` (5 livros, 899 linhas).

### Passo 9: Criar Pages

Na ordem:
1. `src/pages/Portfolio.tsx` — Landing page com Matrix Rain, typewriter, drawer
2. `src/pages/BooksPage.tsx` — Listagem com filtros e busca
3. `src/pages/BookDetail.tsx` — Detalhe do livro
4. `src/pages/BookReader.tsx` — Leitor com markdown rendering
5. `src/pages/OverlayPage.tsx` — **OVERLAY STUDIO** (511 linhas, componente crítico)

### Passo 10: Configurar Router

Copiar `src/App.tsx` com todas as rotas.

### Passo 11: Entry Point

Copiar `src/main.tsx`:
```tsx
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
createRoot(document.getElementById("root")!).render(<App />);
```

### Passo 12: Assets

- Colocar `profile.png` em `src/assets/`
- Colocar `favicon.ico` em `public/`

### Passo 13: Testar

```bash
npm run dev
# Verificar: /, /books, /book/rust-handbook, /book/rust-handbook/read/ch1, /overlay
```

---

## 15. Checklist de Fidelidade

### Design System
- [ ] Font `Space Mono` carregando (400, 700, italic)
- [ ] Font `Courier Prime` carregando (400, 700)
- [ ] Background `hsl(0,0%,3%)` (#080808)
- [ ] Foreground `hsl(0,0%,90%)` (#E6E6E6)
- [ ] Primary `hsl(24,95%,53%)` (#F97316)
- [ ] Border `hsl(0,0%,15%)` (#262626)
- [ ] Border radius global = 0px
- [ ] Todos os 16 tokens CSS definidos em `:root`
- [ ] Sidebar tokens definidos
- [ ] Scrollbar customizado (4px width, cores do tema)

### Tipografia
- [ ] Headings em uppercase com letter-spacing 0.08em
- [ ] Labels em text-[10px] tracking-[0.25em] uppercase
- [ ] Corpo em font-mono
- [ ] Todo texto usa font-family do design system

### Portfolio
- [ ] Matrix Rain animation (canvas, laranja, katakana)
- [ ] Typewriter hook funcionando com 4 roles
- [ ] Tech badges com hover → primary
- [ ] Marquee footer animation
- [ ] Drawer de Links Rápidos com 9 links
- [ ] Link para Overlay Studio presente
- [ ] "Available for work" badge com pulse dot

### Library
- [ ] 5 livros com dados completos
- [ ] Filtro por gênero funcionando
- [ ] Busca por título e tag
- [ ] Grid responsivo 1/2/3 colunas
- [ ] Hover com border-primary/40

### BookReader
- [ ] Modo claro (amber-50) e escuro
- [ ] Font size ajustável (12-24px, step 2)
- [ ] Scroll progress bar (primary color, 2px)
- [ ] TOC sidebar com spring animation
- [ ] Keyboard navigation (← → Esc)
- [ ] Auto-hide controls (3s timeout)
- [ ] Markdown rendering (headers, lists, code, bold, blockquotes)

### Overlay Studio (CRÍTICO)
- [ ] Tema BLACK com cores do design system
- [ ] Tema WHITE com amber-50 (idêntico ao BookReader)
- [ ] 5 tipos de widget funcionando
- [ ] Widget Text: texto simples, font-mono
- [ ] Widget Clock: atualiza a cada segundo, formato pt-BR HH:MM:SS
- [ ] Widget Ticker: bg primary, animate-marquee
- [ ] Widget Banner: border-l-2 primary, pulse dot vermelho
- [ ] Widget Now Playing: label "NOW PLAYING" + conteúdo
- [ ] Posição X/Y percentual (0-100)
- [ ] Seleção visual com ring-1 primary
- [ ] Toggle visibilidade (Eye icon)
- [ ] Remoção de widget
- [ ] Editor de conteúdo, posição e nome
- [ ] Preview checkerboard background
- [ ] Resolution indicator "1920×1080"
- [ ] Fullscreen toggle (fixed inset-4)
- [ ] Empty state com ícone Tv
- [ ] Botões BLACK/WHITE com estado ativo
- [ ] Sidebar fixo em desktop (lg:w-80)
- [ ] Sidebar stacked em mobile
- [ ] Transição de tema duration-500
- [ ] AnimatePresence em widget list e editor

### Animações
- [ ] Framer Motion fade-up nas pages
- [ ] AnimatePresence para exit animations
- [ ] Spring animation no TOC (damping: 25, stiffness: 250)
- [ ] CSS marquee (20s linear infinite)
- [ ] CSS pulse (available dot, banner dot)
- [ ] CSS bounce (scroll indicator)
- [ ] transition-all duration-300 em cards e widgets

### Responsividade
- [ ] Mobile: sidebar stacked, texto reduzido
- [ ] Desktop: sidebar fixo, layout side-by-side
- [ ] Nav responsiva com hidden elements
- [ ] Max-w containers (5xl, 6xl, 7xl)

### Build & Deploy
- [ ] `npm run dev` funciona na porta 8080
- [ ] `npm run build` compila sem erros
- [ ] Todas as rotas acessíveis
- [ ] Alias `@/` resolvendo corretamente

---

## Apêndice A: Configurações Completas

### components.json

```json
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "default",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "tailwind.config.ts",
    "css": "src/index.css",
    "baseColor": "slate",
    "cssVariables": true,
    "prefix": ""
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  }
}
```

### index.html

O HTML deve conter:
```html
<!DOCTYPE html>
<html lang="pt-BR">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/x-icon" href="/favicon.ico" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>ASKGUI — Developer Portfolio</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

---

**FIM DA DOCUMENTAÇÃO**

*Documento gerado em 2026-03-09. Versão 1.0.0.*
*Qualquer alteração no projeto original deve ser refletida neste documento.*
