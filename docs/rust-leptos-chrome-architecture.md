# Rust + Leptos + Extensão do Google Chrome

Este guia mostra uma forma prática de implementar o fluxo de importação de músicas do Suno para um app local em Rust com interface em Leptos.

## 1) Arquitetura recomendada

```text
[Extensão Chrome]
      ↓ (HTTP POST para localhost)
[API local Axum em Rust]
      ↓
[Serviço de importação]
  - baixar página
  - extrair metadados (título, letra, áudio, capa)
  - baixar MP3
  - salvar no SQLite
      ↓
[UI Leptos]
  - biblioteca
  - player
  - status das importações
```

## 2) Estrutura de projeto sugerida

```text
music-importer/
  app/
    src/
      main.rs            # sobe API local + UI
      api.rs             # endpoints /import, /health
      importer.rs        # reqwest + scraper
      db.rs              # sqlite
      models.rs          # Song, ImportRequest, ImportResult
  extension/
    manifest.json
    popup.html
    popup.js
    content.js
```

## 3) Backend Rust (Axum) — endpoint local

```rust
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct ImportRequest {
    url: String,
}

async fn import(Json(payload): Json<ImportRequest>) -> String {
    // TODO: chamar serviço de importação com reqwest/scraper
    format!("Importando: {}", payload.url)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/import", post(import));
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    println!("API local em http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
```

## 4) Serviço de importação (Rust)

Crates recomendados:

- `axum`: API HTTP local
- `tokio`: runtime async
- `reqwest`: download da página e arquivos
- `scraper`: parse HTML
- `serde`: serialização/deserialização
- `sqlx` (ou `rusqlite`): persistência SQLite

Fluxo:

1. Recebe URL da extensão.
2. Faz `GET` da página do Suno.
3. Tenta extrair metadados no HTML inicial.
4. Se necessário, busca dados em endpoint interno consumido pela página.
5. Faz download do áudio.
6. Salva arquivo e metadados no banco.
7. Retorna status para extensão e UI.

## 5) Extensão Chrome (Manifest v3)

### `extension/manifest.json`

```json
{
  "manifest_version": 3,
  "name": "Music Importer",
  "version": "1.0.0",
  "permissions": ["activeTab", "tabs"],
  "host_permissions": ["http://localhost:7878/*", "https://suno.com/*"],
  "action": {
    "default_popup": "popup.html"
  },
  "content_scripts": [
    {
      "matches": ["https://suno.com/song/*"],
      "js": ["content.js"]
    }
  ]
}
```

### `extension/popup.html`

```html
<!doctype html>
<html>
  <body>
    <button id="import">Importar música atual</button>
    <p id="status"></p>
    <script src="popup.js"></script>
  </body>
</html>
```

### `extension/popup.js`

```javascript
const statusEl = document.getElementById("status");

document.getElementById("import").addEventListener("click", async () => {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  if (!tab?.url) {
    statusEl.textContent = "Não foi possível obter a URL da aba.";
    return;
  }

  const resp = await fetch("http://localhost:7878/import", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ url: tab.url })
  });

  statusEl.textContent = resp.ok ? "Importação enviada!" : "Falha ao enviar.";
});
```

### `extension/content.js` (opcional para UX)

```javascript
if (window.location.pathname.startsWith("/song/")) {
  const btn = document.createElement("button");
  btn.textContent = "Importar para biblioteca";
  btn.style.position = "fixed";
  btn.style.bottom = "24px";
  btn.style.right = "24px";
  btn.style.zIndex = "999999";

  btn.onclick = async () => {
    const res = await fetch("http://localhost:7878/import", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url: window.location.href })
    });
    alert(res.ok ? "Importação enviada" : "Falha na importação");
  };

  document.body.appendChild(btn);
}
```

## 6) Onde o Leptos entra

Recomendação prática:

- Use Leptos no app desktop/web local para:
  - listar biblioteca
  - mostrar progresso de importação
  - reproduzir áudio
  - editar metadados
- Mantenha a extensão em JS simples (mais rápida de evoluir e debugar).

## 7) Pontos importantes de produção

1. **CORS/localhost**: normalmente extensão chama `localhost` sem dor, mas trate erros e timeouts.
2. **App offline**: se o app não estiver rodando, extensão deve mostrar erro amigável.
3. **Robustez do parser**: HTML pode mudar; prefira fontes estruturadas quando disponíveis.
4. **Fila de importação**: evitar duplicidade e permitir retries.
5. **Segurança**: aceite apenas URLs esperadas (`https://suno.com/song/...`).

## 8) Próximo passo recomendado

Começar com MVP:

- botão na extensão para enviar a aba atual;
- endpoint `/import` funcionando;
- log de importação no backend;
- salvar metadados básicos + arquivo de áudio.

Depois evoluir para:

- detecção automática por content script;
- progresso em tempo real via WebSocket;
- biblioteca completa em Leptos.

## 9) Estado atual do MVP no repositório

Implementado no projeto:

- API local em Rust (`app/`) com rotas:
  - `GET /health`
  - `POST /import`
- Validação de URL para aceitar somente `https://suno.com/song/...` (ou subdomínios).
- Extração inicial de metadados do HTML:
  - `title`
  - `og:audio` (quando disponível)
  - descrição/letra básica
- Extensão Chrome (`extension/`) com:
  - popup para enviar URL da aba atual
  - validação de URL no cliente
  - botão in-page via `content.js`

## 10) Como rodar localmente

### Backend Rust

```bash
cd app
cargo run
```

API disponível em `http://127.0.0.1:7878`.

### Extensão Chrome

1. Abra `chrome://extensions`
2. Ative **Developer mode**
3. Clique em **Load unpacked**
4. Selecione a pasta `extension/`
5. Abra uma URL `https://suno.com/song/...`
6. Clique no popup da extensão para importar

## 11) Próximas melhorias (ordem sugerida)

1. Persistir metadados e jobs em SQLite.
2. Implementar fila de importação com retries.
3. Extrair áudio/letra por fontes estruturadas (JSON de dados da página).
4. Fazer download do MP3 e salvar no filesystem.
5. Expor progresso em tempo real (SSE/WebSocket) para UI Leptos.


## 12) Evoluções de robustez implementadas

Nesta fase, o projeto passou a ter uma base mais robusta:

- Persistência SQLite (`music_importer.db`) com tabela `imports`.
- Endpoint `GET /imports?limit=10` para listar histórico recente.
- Endpoint `POST /import` agora persiste cada import e retorna `import_id`.
- Popup da extensão mostra os últimos imports para feedback rápido.

Exemplo de resposta do `POST /import`:

```json
{
  "accepted": true,
  "import_id": 42,
  "song": {
    "source_url": "https://suno.com/song/abc123",
    "title": "Minha Música",
    "lyrics": null,
    "audio_url": "https://.../track.mp3"
  },
  "message": "Importação iniciada e persistida com sucesso"
}
```

## 13) Novas implementações de robustez (fase atual)

A arquitetura local foi evoluída com foco em operação contínua:

- **Deduplicação por URL** (`source_url` único no SQLite) com `upsert`.
- **Estados de processamento** em banco:
  - `parsed` (metadados extraídos)
  - `downloaded` (áudio salvo localmente)
  - `failed` (erro durante import)
- **Download local de áudio** quando `audio_url` está disponível.
- **Novo endpoint** `GET /imports/:id` para consulta de item específico.
- **Registro de erro** no banco quando o import falha.

### Endpoints atuais

- `GET /health`
- `POST /import`
- `GET /imports?limit=20`
- `GET /imports/:id`

### Estrutura local de arquivos gerados

```text
/workspace/CODEX/
  music_importer.db
  downloads/
    <song_id>.mp3
```

## 14) Frontend da extensão (UI nova)

A extensão agora possui uma interface visual mais completa no popup:

- Cabeçalho com estado da API (`API ON` / `API OFF`).
- Card de ação com botões de import e atualização de histórico.
- Lista de imports com tags de status (`parsed`, `downloaded`, `failed`).
- Indicador visual `🎵` quando existe `local_audio_path` (áudio salvo localmente).

Arquivos de frontend:

- `extension/popup.html`
- `extension/popup.css`
- `extension/popup.js`
