<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<script lang="ts">
    let time = 'test';
        // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri';
    // When using the Tauri global script (if not using the npm package)
    // Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true

    async function get_stuff() {
        time = await invoke('display_time')
        console.log(time)
    }
    get_stuff();
</script>

<h2>Current time:  {time}</h2>