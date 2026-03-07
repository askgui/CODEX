const statusEl = document.getElementById("status");
const importsEl = document.getElementById("imports");
const importButton = document.getElementById("import");
const refreshButton = document.getElementById("refresh");
const countEl = document.getElementById("count");
const apiBadgeEl = document.getElementById("api-badge");
const statTotalEl = document.getElementById("stat-total");
const statParsedEl = document.getElementById("stat-parsed");
const statDownloadedEl = document.getElementById("stat-downloaded");
const statFailedEl = document.getElementById("stat-failed");
const playerEl = document.getElementById("audio-player");
const nowPlayingEl = document.getElementById("now-playing");

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function setStatus(text, ok = true) {
  statusEl.textContent = text;
  statusEl.style.color = ok ? "#166534" : "#991b1b";
}

function setApiBadge(state) {
  apiBadgeEl.className = "badge";
  if (state === "ok") {
    apiBadgeEl.classList.add("badge--ok");
    apiBadgeEl.textContent = "API ON";
  } else if (state === "error") {
    apiBadgeEl.classList.add("badge--error");
    apiBadgeEl.textContent = "API OFF";
  } else {
    apiBadgeEl.classList.add("badge--idle");
    apiBadgeEl.textContent = "API?";
  }
}

function setStats(stats = {}) {
  statTotalEl.textContent = String(stats.total || 0);
  statParsedEl.textContent = String(stats.parsed || 0);
  statDownloadedEl.textContent = String(stats.downloaded || 0);
  statFailedEl.textContent = String(stats.failed || 0);
}

function isValidSunoSongUrl(rawUrl) {
  try {
    const parsed = new URL(rawUrl);
    const validHost = parsed.hostname === "suno.com" || parsed.hostname.endsWith(".suno.com");
    return validHost && parsed.pathname.startsWith("/song/");
  } catch {
    return false;
  }
}

function statusTag(status) {
  const map = {
    parsed: { cls: "tag--parsed", label: "parsed" },
    downloaded: { cls: "tag--downloaded", label: "downloaded" },
    failed: { cls: "tag--failed", label: "failed" }
  };
  return map[status] || { cls: "tag--parsed", label: escapeHtml(status || "unknown") };
}

function renderImports(items) {
  countEl.textContent = String(items?.length || 0);
  importsEl.innerHTML = "";

  if (!items?.length) {
    const li = document.createElement("li");
    li.textContent = "Nenhum import registrado ainda.";
    importsEl.appendChild(li);
    return;
  }

  for (const item of items) {
    const li = document.createElement("li");
    const title = escapeHtml(item.title || "(sem título)");
    const tag = statusTag(item.status);
    const local = item.local_audio_path ? "🎵" : "";
    const playButton = item.local_audio_path
      ? `<button class="btn-play" data-play-id="${item.id}" data-play-title="${title}" title="Ouvir faixa">Ouvir</button>`
      : "";

    li.innerHTML = `
      <div class="import-item-row">
        <span class="import-text"><span class="tag ${tag.cls}">${tag.label}</span>${title} ${local}</span>
        <div class="import-actions">
          ${playButton}
          <button class="btn-remove" data-id="${item.id}" title="Remover import">Remover</button>
        </div>
      </div>
    `;
    li.title = `${item.source_url}\n${item.created_at}${item.error_message ? `\nErro: ${item.error_message}` : ""}`;
    importsEl.appendChild(li);
  }
}

async function playImport(id, title) {
  const streamUrl = `http://localhost:7878/imports/${id}/audio`;
  playerEl.src = streamUrl;
  nowPlayingEl.textContent = title || `import #${id}`;

  try {
    await playerEl.play();
    setStatus("Reproduzindo áudio local.");
  } catch {
    setStatus("Não foi possível reproduzir o áudio local.", false);
  }
}

async function checkHealth() {
  try {
    const response = await fetch("http://localhost:7878/health");
    setApiBadge(response.ok ? "ok" : "error");
  } catch {
    setApiBadge("error");
  }
}

async function loadStats() {
  try {
    const response = await fetch("http://localhost:7878/stats");
    if (!response.ok) {
      return;
    }

    const body = await response.json().catch(() => ({}));
    setStats(body);
  } catch {
    // best effort
  }
}

async function loadRecentImports() {
  refreshButton.disabled = true;

  try {
    const response = await fetch("http://localhost:7878/imports?limit=10");

    if (!response.ok) {
      const body = await response.json().catch(() => ({}));
      setStatus(body.error || "Falha ao carregar histórico de imports.", false);
      setApiBadge("error");
      return;
    }

    const body = await response.json();
    renderImports(body.items || []);
    setApiBadge("ok");
  } catch {
    setStatus("API local indisponível para listar imports.", false);
    setApiBadge("error");
  } finally {
    refreshButton.disabled = false;
  }
}

async function deleteImportById(id) {
  try {
    const response = await fetch(`http://localhost:7878/imports/${id}`, { method: "DELETE" });
    const body = await response.json().catch(() => ({}));

    if (!response.ok) {
      setStatus(body.message || "Falha ao remover import.", false);
      return;
    }

    setStatus(body.message || "Import removido com sucesso.");
    await Promise.all([loadRecentImports(), loadStats()]);
  } catch {
    setStatus("API local indisponível para remover import.", false);
  }
}

importsEl.addEventListener("click", async (event) => {
  const target = event.target;
  if (!(target instanceof HTMLElement)) return;

  if (target.classList.contains("btn-remove")) {
    const id = target.getAttribute("data-id");
    if (!id) return;
    target.setAttribute("disabled", "true");
    await deleteImportById(id);
    return;
  }

  if (target.classList.contains("btn-play")) {
    const id = target.getAttribute("data-play-id");
    const title = target.getAttribute("data-play-title") || "faixa";
    if (!id) return;
    await playImport(id, title);
  }
});

importButton.addEventListener("click", async () => {
  importButton.disabled = true;
  setStatus("Enviando para API local...");

  try {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

    if (!tab?.url) {
      setStatus("Não foi possível ler a URL da aba.", false);
      return;
    }

    if (!isValidSunoSongUrl(tab.url)) {
      setStatus("Abra uma página Suno no formato /song/...", false);
      return;
    }

    const response = await fetch("http://localhost:7878/import", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url: tab.url })
    });

    const body = await response.json().catch(() => ({}));

    if (response.ok) {
      setStatus(body.message || "Importação enviada com sucesso.");
      setApiBadge("ok");
      await Promise.all([loadRecentImports(), loadStats()]);
    } else {
      setStatus(body.message || "Falha ao importar.", false);
      setApiBadge("error");
    }
  } catch {
    setStatus("API local indisponível. Inicie o app Rust primeiro.", false);
    setApiBadge("error");
  } finally {
    importButton.disabled = false;
  }
});

refreshButton.addEventListener("click", async () => {
  await Promise.all([loadRecentImports(), loadStats()]);
});

checkHealth();
setStats();
Promise.all([loadRecentImports(), loadStats()]);
