(() => {
  if (!window.location.pathname.startsWith("/song/")) return;

  const button = document.createElement("button");
  button.textContent = "Importar para biblioteca";
  Object.assign(button.style, {
    position: "fixed",
    right: "20px",
    bottom: "20px",
    zIndex: "999999",
    border: "none",
    borderRadius: "8px",
    padding: "10px 14px",
    background: "#111827",
    color: "#fff",
    cursor: "pointer"
  });

  button.addEventListener("click", async () => {
    button.disabled = true;

    try {
      const response = await fetch("http://localhost:7878/import", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ url: window.location.href })
      });

      const payload = await response.json().catch(() => ({}));

      if (!response.ok) {
        alert(payload.message || "Falha na importação. Veja o app local para detalhes.");
        return;
      }

      alert(payload.message || "Importação enviada para o app local.");
    } catch (_) {
      alert("API local não encontrada. Inicie o app Rust em localhost:7878.");
    } finally {
      button.disabled = false;
    }
  });

  document.body.appendChild(button);
})();
