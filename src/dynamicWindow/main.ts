
let greetMsgEl: HTMLElement | null;

window.onload = () => {
  console.log('loaded');
  greetMsgEl = document.querySelector("#greet-msg");
  const data = localStorage.getItem("greetMsg");

  console.log('sup', data);

  if (data && greetMsgEl) {
    greetMsgEl.innerHTML = data;
  }
};
