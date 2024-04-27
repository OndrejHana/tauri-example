let greetMsgEl: HTMLElement | null;

window.onload = () => {
  greetMsgEl = document.querySelector("#greet-msg");
  const msg = localStorage.getItem("greetMsg");

  if (msg && greetMsgEl) {
    greetMsgEl.innerHTML = msg;
  }
};
