import init, { send_prompt } from "./pkg/worker.js";

await init();
let form = document.querySelector("form");

form.addEventListener("submit", async function (e) {
  e.preventDefault();

  const meme = new FormData(e.target);
  const apiKey = meme.get("apiKey");
  const response = await send_prompt(apiKey);

  document.querySelector("#response").innerHTML += response;
});
