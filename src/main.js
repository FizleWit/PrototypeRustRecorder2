//const { invoke } = window.__TAURI__.tauri; for npm package managment
const invoke = window.__TAURI__.invoke //bc cargo
//let greetInputEl;
//let greetMsgEl;

//async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
 // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
//}


// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document
//     .querySelector("#greet-button")
//     .addEventListener("click", () => greet());
// });

const screenshotbtn = window.querySelector('#screenshotbtn');

  screenshotbtn.addEventListener("click", () => {
    invoke("screen_shot");
    invoke("start_stream");
    invoke("start_recording");
    invoke("end_stream");
    invoke("save_stream1");
    invoke("display_stream");
    invoke("screenshot_stream");
  });

//https://medium.com/@marm.nakamura/trying-to-tauri-on-rust-3-communicate-with-js-d56390116e1f