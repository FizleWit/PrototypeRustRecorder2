//const { invoke } = window.__TAURI__.tauri; for npm package managment
const invoke = window.__TAURI__.invoke //bc cargo




window.addEventListener("DOMContentLoaded", () => {
  document
  .querySelector("#screenshotbtn")
  .addEventListener("click", () => invoke("screen_shot"));
});

window.addEventListener("DOMContentLoaded", () => {
  document
  .querySelector("#recordbtn")
  .addEventListener("click", () => invoke("start_stream"));
});

window.addEventListener("DOMContentLoaded", () => {
  document
  .querySelector("#actionReplaybtn")
  .addEventListener("click", () => invoke("start_stream"));
});

  //invoke("start_stream");
  //invoke("start_recording");
  //invoke("end_stream");
  //invoke("save_stream1");
  //invoke("display_stream");
  //invoke("screenshot_stream");
