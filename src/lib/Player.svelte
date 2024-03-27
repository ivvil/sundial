<div class="container">
    <div class="Info">
       <img src={imgSrc} alt="{albumName} artwork" />
       <b>{songName}</b>
       <p>{artistName}</p>
    </div>
</div>

<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    
    let imgSrc = "/vite.svg";
    let albumName = "";
    let songName = "";
    let artistName = "";

    async function getPlaying() {
        
    }
</script>

<style>
    .container {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-rows: auto;
        gap: 0px 0px;
        grid-auto-flow: row;
        grid-template-areas: "Info Player Controls";
    }

    .Info {
        grid-area: Info;
    }

    .Player {
        grid-area: Player;
    }

    .Controls {
        grid-area: Controls;
    }
</style>
