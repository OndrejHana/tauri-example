import {listen, emit} from "@tauri-apps/api/event";

let greetMsg = document.querySelector("#greet-msg");

console.log("Listening for onload-data event");

listen("greet-msg", (msg) => {
  if (greetMsg) {
    greetMsg.innerHTML = msg.payload as string;
  }
});

emit("window-ready");
