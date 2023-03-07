<script setup lang="ts">
import Switch from "./Switch.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { Command } from "@tauri-apps/api/shell";

const greetMsg = ref("");
const name = ref("");
const name2 = ref("");

const isActive: boolean = false;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });

  const command = new Command("git", ["-v"]);

  //   const command = new Command("ls");
  //   console.log("============ command =============");
  //   console.log(command);
  const child = await command.spawn();
  console.log("============ child =============");
  console.log(child);
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <input id="greet-input2" v-model="name2" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
    <div type="button">
      <p v-if="isActive">The switch is on.</p>
      <p v-if="!isActive">The switch is off.</p>
      <Switch v-model="isActive" @click="greet()" />
    </div>
  </div>

  <p>{{ greetMsg }}</p>
</template>
