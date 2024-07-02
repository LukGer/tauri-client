<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let userName = "";
  let realName = "";
  let deviceName = "";
  let stampedIn = false;

  onMount(async () => {
    userName = await invoke("get_username");
    realName = await invoke("get_realname");
    deviceName = await invoke("get_devicename");

    const stampInUnlisten = await listen('stamp_in', () => {
      stampedIn = true
    });

    const stampOutUnlisten = await listen('stamp_out', () => {
      stampedIn = false
    });

    return () => {
      stampInUnlisten();
      stampOutUnlisten();
    }
  });

</script>

<div class="container">
  <h1>Username: {userName}</h1>
  <h1>Real name: {realName}</h1>
  <h1>deviceName: {deviceName}</h1>

  {#if stampedIn}
    <h1>Stamped in</h1>
  {:else}
    <h1>Not stamped in</h1>
  {/if}

</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
