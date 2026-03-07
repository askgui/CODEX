const statusEl = document.getElementById("status");
const importsEl = document.getElementById("imports");
const importButton = document.getElementById("import");
const refreshButton = document.getElementById("refresh");

function setStatus(text, ok = true) {
  statusEl.textContent = text;
  statusEl.style.color = ok ? "#0f5132" : "#842029";
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

function renderImports(items) {
  importsEl.innerHTML = "";

  if (!items?.length) {
    const li = document.createElement("li");
    li.textContent = "Nenhum import registrado ainda.";
    importsEl.appendChild(li);
    return;
  }

  for (const item of items) {
    const li = document.createElement("li");
    const title = item.title || "(sem título)";
    const status = item.status || "desconhecido";
    const local = item.local_audio_path ? " 🎵" : "";
    li.textContent = `#${item.id} • [${status}] ${title}${local}`;
    li.title = `${item.source_url}\n${item.created_at}${item.error_message ? `\nErro: ${item.error_message}` : ""}`;
    importsEl.appendChild(li);
  }
}

async function loadRecentImports() {
  refreshButton.disabled = true;

  try {
    const response = await fetch("http://localhost:7878/imports?limit=10");

    if (!response.ok) {
      const body = await response.json().catch(() => ({}));
      setStatus(body.error || "Falha ao carregar histórico de imports.", false);
      return;
    }

    const body = await response.json();
    renderImports(body.items || []);
  } catch {
    setStatus("API local indisponível para listar imports.", false);
  } finally {
    refreshButton.disabled = false;
  }
}

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
      await loadRecentImports();
    } else {
      setStatus(body.message || "Falha ao importar.", false);
    }
  } catch {
    setStatus("API local indisponível. Inicie o app Rust primeiro.", false);
  } finally {
    importButton.disabled = false;
  }
});

refreshButton.addEventListener("click", loadRecentImports);
loadRecentImports();
