<script>
  import { invoke } from '@tauri-apps/api/tauri';
  let id = '';
  let result = '';

  async function load() {
    result = await invoke('query_memory', { id });
  }
</script>

<main class="p-8">
  <h1 class="text-2xl font-bold">ThreadSpace α</h1>
  <input bind:value={id} placeholder="memory id" class="input input-bordered" />
  <button on:click={load} class="btn btn-primary ml-2">load</button>
  <pre class="mt-4">{JSON.stringify(result)}</pre>
</main>