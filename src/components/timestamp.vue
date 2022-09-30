<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const message = ref("");
const ts_input = ref("");

async function format() {
  try {
    message.value = await invoke("timestamp_format", { value: ts_input.value });
  } catch (e) {
    console.log("rust error" + e);
    message.value = e
  }
}
</script>

<template>

  <div class="card">
    <input id="ts-input" v-model="ts_input" placeholder="timestamp" />
    <button type="button" @click="format()">格式化</button>
  </div>
  <p>{{ message }}</p>

</template>


<style>
  body {
    margin: 0;
    padding: 0;
  }

  #app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    color: #2c3e50;
    margin: 0;
    display: flex;
    /* border-top: 1px solid #e1e4e8; */
    box-sizing: border-box;
    height: 100vh;
    overflow: hidden;
  }


</style>