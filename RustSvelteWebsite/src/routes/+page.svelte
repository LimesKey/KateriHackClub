<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<script lang="ts">
    let time = 'test';
        // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri';

    async function get_stuff() {
        try {
        time = await invoke('display_time')
        console.log(time)
        }
        catch(error) {
            console.error('Error occurred:', error);
        }
    }
    get_stuff()
</script>

<h2>Current time:  {time}</h2>