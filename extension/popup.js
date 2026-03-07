const statusEl = document.getElementById("status");
const importButton = document.getElementById("import");

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

document.getElementById("import").addEventListener("click", async () => {
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

    const body = await response.json();

    if (response.ok) {
      setStatus(body.message || "Importação enviada com sucesso.");
    } else {
      setStatus(body.message || "Falha ao importar.", false);
    }
  } catch (error) {
    setStatus("API local indisponível. Inicie o app Rust primeiro.", false);
  } finally {
    importButton.disabled = false;
  }
});
