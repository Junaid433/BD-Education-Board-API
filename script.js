const form = document.getElementById("fetchForm");
const resultBox = document.getElementById("resultBox");
const statusText = document.getElementById("statusText");
const submitBtn = document.getElementById("submitBtn");
const btnText = submitBtn?.querySelector(".btn-text");
const btnLoading = submitBtn?.querySelector(".btn-loading");
const copyCurlBtn = document.getElementById("copyCurl");
const curlSnippet = document.getElementById("curlSnippet");

copyCurlBtn?.addEventListener("click", async () => {
  const snippet = curlSnippet?.innerText ?? "";

  try {
    await navigator.clipboard.writeText(snippet);
    copyCurlBtn.textContent = "Copied";
    setTimeout(() => {
      copyCurlBtn.textContent = "Copy";
    }, 1200);
  } catch (_error) {
    copyCurlBtn.textContent = "Cannot copy";
    setTimeout(() => {
      copyCurlBtn.textContent = "Copy";
    }, 1200);
  }
});

submitBtn?.addEventListener("click", async (event) => {
  event.preventDefault();

  const formData = new FormData(form);
  const payload = {
    exam: String(formData.get("exam") ?? "").trim(),
    year: String(formData.get("year") ?? "").trim(),
    board: String(formData.get("board") ?? "").trim(),
    roll: String(formData.get("roll") ?? "").trim(),
    reg: String(formData.get("reg") ?? "").trim(),
  };

  submitBtn.disabled = true;
  btnText.style.display = "none";
  btnLoading.style.display = "inline";
  statusText.textContent = "Requesting...";
  resultBox.textContent = "Loading...";

  try {
    const response = await fetch("/fetch", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    const textBody = await response.text();
    let parsed;

    try {
      parsed = JSON.parse(textBody);
    } catch (_error) {
      parsed = { raw: textBody };
    }

    statusText.textContent = `${response.status} ${response.statusText}`;
    resultBox.textContent = JSON.stringify(parsed, null, 2);
  } catch (error) {
    statusText.textContent = "Network error";
    resultBox.textContent = JSON.stringify(
      { error: error instanceof Error ? error.message : "Unknown error" },
      null,
      2
    );
  } finally {
    submitBtn.disabled = false;
    btnText.style.display = "inline";
    btnLoading.style.display = "none";
  }
});
