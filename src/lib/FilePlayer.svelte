<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from "@tauri-apps/api/dialog"

  async function loadFile() {
	try {
	  const path = await open({
		multiple: false,
		title: "Select a file to play",
		filters: [{
		  name: "Audio",
		  extensions: ['mp3', 'opus', 'flac', 'acc']
		}]
	  });
	  return path;
	} catch (err) {
	  console.error(err);
	}
  }

  async function playFile() {
	const path = await loadFile();
	if (path) {
	  invoke("play", {path});
	}
  }
  
</script>


<div>
  <button on:click={playFile}>Select file</button>
</div>
