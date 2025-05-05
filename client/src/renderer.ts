import { renderNotebook } from "./visualizer";
import { queryIssuesByProject } from "./db";

const input = document.getElementById("project-key") as HTMLInputElement;
const button = document.getElementById("load-btn")!;
const vizDiv = document.getElementById("viz")!;

button.addEventListener("click", async () => {
  const key = input.value.trim();
  if (!key) return alert("Please enter a project key");

  vizDiv.innerHTML = "⏳ Loading...";

  try {
    const rows = await queryIssuesByProject(key);
    renderNotebook(vizDiv, rows);
  } catch (err) {
    console.error(err);
    vizDiv.innerHTML = "❌ Failed to load data";
  }
});
