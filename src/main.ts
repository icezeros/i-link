import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

(async () => {
  console.log("============ readDir =============");
  console.log(readDir);
  // Reads the `$APPDATA/users` directory recursively
  const entries = await readDir("/Users/huguosen/workspace/ice/tauri-app", {
    dir: BaseDirectory.AppData,
    recursive: true,
  });
  console.log("============ entries =============");
  console.log(entries);
})();

// function processEntries(entries) {
//   for (const entry of entries) {
//     console.log(`Entry: ${entry.path}`);
//     if (entry.children) {
//       processEntries(entry.children);
//     }
//   }
// }

createApp(App).mount("#app");
