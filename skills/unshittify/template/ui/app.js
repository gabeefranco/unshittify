// The whole frontend: a hash router that asks Rust for each page's HTML.
//
//   <a href="#/notes/3">        → invoke("render", {path}) → HTML into <main>
//   <form method="get">         → navigates to action?query
//   <form method="post">        → invoke("act", {path, form, from}) → path to show next
//   <a href="https://...">      → opens in the system browser
//
// Keep logic in Rust. This file should stay small enough to read in a minute.

const APP = "app";
const { invoke } = window.__TAURI__.core;
const main = document.getElementById("main");
const statusLine = document.getElementById("status");
let seq = 0;

const currentPath = () => location.hash.slice(1) || "/";

// Navigate to an in-app path. Re-renders if it's the page we're already on.
function go(path) {
  if (currentPath() === path) show();
  else location.hash = path;
}

async function show() {
  const mine = ++seq;
  document.body.classList.add("loading");
  let html;
  try {
    html = await invoke("render", { path: currentPath() });
  } catch (err) {
    html = null;
    console.error(err);
  }
  if (mine !== seq) return; // a newer navigation won
  document.body.classList.remove("loading");
  if (html === null) {
    main.replaceChildren(heading("error"), Object.assign(document.createElement("pre"), { textContent: "render failed — see the console" }));
  } else {
    main.innerHTML = html;
  }
  const h1 = main.querySelector("h1");
  document.title = h1 ? `${h1.textContent} - ${APP}` : APP;
  window.scrollTo(0, 0);
  main.querySelector("[autofocus]")?.focus();
  refreshStatus();
}

function heading(text) {
  return Object.assign(document.createElement("h1"), { textContent: text });
}

async function refreshStatus() {
  try {
    statusLine.innerHTML = await invoke("status");
  } catch (err) {
    console.error(err);
  }
}

document.addEventListener("submit", async (e) => {
  const form = e.target;
  e.preventDefault();
  const action = form.getAttribute("action") || currentPath().split("?")[0];
  const data = new FormData(form);
  if (form.method === "post") {
    const fields = {};
    for (const [k, v] of data) if (typeof v === "string") fields[k] = v;
    document.body.classList.add("loading");
    try {
      go(await invoke("act", { path: action, form: fields, from: currentPath() }));
    } catch (err) {
      console.error(err);
      show();
    }
  } else {
    const qs = new URLSearchParams(data).toString();
    go(qs ? `${action}?${qs}` : action);
  }
});

document.addEventListener("click", (e) => {
  const a = e.target.closest("a[href]");
  if (!a) return;
  if (a.dataset.nav) {
    e.preventDefault();
    a.dataset.nav === "back" ? history.back() : history.forward();
  } else if (/^(https?|mailto):/.test(a.getAttribute("href"))) {
    e.preventDefault();
    window.__TAURI__.opener.openUrl(a.href);
  }
});

document.addEventListener("keydown", (e) => {
  const typing = /^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName);
  if (e.altKey && e.key === "ArrowLeft") history.back();
  else if (e.altKey && e.key === "ArrowRight") history.forward();
  else if (e.key === "/" && !typing) {
    const search = main.querySelector("input[type=search]");
    if (search) {
      e.preventDefault();
      search.focus();
    }
  } else if (e.key === "Escape" && typing) document.activeElement.blur();
});

window.addEventListener("hashchange", show);
show();
