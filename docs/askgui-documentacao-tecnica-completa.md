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

---

## 4. Sistema de Design

> Tokens, tipografia, espaçamentos, opacidades e padrões seguem o design descrito no whitepaper original.

- Paleta baseada em HSL com `--background: 0 0% 3%` e `--primary: 24 95% 53%`.
- Tipografia principal: `Space Mono` com `Courier Prime` como fallback.
- Border radius global: `0px`.
- Labels e headings em uppercase com tracking expandido.

---

## 5. Sistema de Tema

- **Tema padrão (BLACK):** usa tokens CSS globais em `:root`.
- **Tema WHITE (Overlay e Reader):** usa classes hardcoded (`bg-amber-50`, `text-stone-800`, etc.).
- Accent (`hsl(24,95%,53%)`) permanece consistente nos dois temas.

---

## 6. Componentes UI

O sistema utiliza componentes shadcn/ui com padrões visuais unificados:

- Botões primários com accent laranja.
- Botões outline com hover para accent.
- Navbar fixa com estrutura esquerda/centro/direita.
- Cards com `border` + `transition-all duration-300`.

---

## 7. Documentação Completa da Página Overlay

Arquivo crítico: `src/pages/OverlayPage.tsx`.

### Recursos principais
- 5 tipos de widget: text, clock, ticker, banner, now-playing.
- Posicionamento por `x/y` percentual.
- Seleção visual com ring laranja.
- Editor lateral para conteúdo, nome e coordenadas.
- Preview com checkerboard para simular transparência.
- Toggle de tema BLACK/WHITE.
- Fullscreen do preview.

---

## 8. Fluxo de Renderização

1. Rota `/overlay` carrega `OverlayPage`.
2. Estado inicial: tema dark, sem widgets, sem seleção.
3. Usuário adiciona widget por template.
4. Widget é renderizado no canvas e selecionado.
5. Edição no painel lateral atualiza render em tempo real.

---

## 9. Lógica do Sistema

Estados principais:
- `theme`
- `widgets`
- `selectedWidget`
- `previewFullscreen`

Geração de ID:

```ts
let widgetIdCounter = 0;
const genId = () => `w-${++widgetIdCounter}-${Date.now()}`;
```

---

## 10. Sistema de Animação

- Framer Motion para entrada/saída de painéis e itens.
- AnimatePresence para transições de remoção.
- CSS animations para marquee, pulse, bounce e accordion.

---

## 11. Responsividade

- **Mobile-first** com breakpoints Tailwind (`sm`, `md`, `lg`).
- Overlay: sidebar empilhada no mobile e fixa em desktop.
- Portfolio e Reader adaptam tipografia e controles por viewport.

---

## 12. Assets

- `src/assets/profile.png`
- `public/favicon.ico`
- Ícones via `lucide-react`
- Fontes via Google Fonts (`Space Mono`, `Courier Prime`)

---

## 13. Processo de Build

Scripts esperados:
- `npm run dev`
- `npm run build`
- `npm run preview`
- `npm run test`

Vite configurado para porta `8080` com alias `@` → `./src`.

---

## 14. Guia de Reprodução Completa

Passos resumidos:
1. Criar app Vite React + TS + SWC.
2. Instalar dependências de UI, animação, router e testes.
3. Configurar Tailwind, `index.css`, `vite.config.ts`, TS configs.
4. Copiar componentes shadcn/ui e páginas do sistema.
5. Configurar rotas e assets.
6. Executar `npm run dev` e validar rotas.

---

## 15. Checklist de Fidelidade

Checklist completo de validação inclui:
- Design tokens, tipografia, tema, responsividade
- Portfolio (Matrix Rain + typewriter)
- Library e BookReader
- Overlay Studio (widgets, preview, tema, editor)
- Build e rotas

---

## Apêndice A

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
