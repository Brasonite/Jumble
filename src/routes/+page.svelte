<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let input = $state("");
    let output = $state("");

    async function jumble() {
        output = await invoke("jumble", { input: input });
    }

    let input_placeholder = $state("");
    let output_placeholder = $state("");
    let button_text = $state("");

    onMount(async () => {
        input_placeholder = await invoke("native_localize", {
            key: "input_placeholder",
        });
        output_placeholder = await invoke("native_localize", {
            key: "output_placeholder",
        });
        button_text = await invoke("native_localize", {
            key: "button_text",
        });
    });
</script>

<main class="container">
    <div id="main-content">
        <img class="header" src="/header.svg" alt="Swirl" />
        <div id="options">
            <textarea
                class="text-input"
                placeholder={input_placeholder}
                spellcheck="false"
                bind:value={input}
            ></textarea>
            <textarea
                class="text-input"
                placeholder={output_placeholder}
                spellcheck="false"
                bind:value={output}
            ></textarea>
        </div>
        <button onclick={jumble}>{button_text}</button>
    </div>
</main>

<style>
    @font-face {
        font-family: "Funnel Sans";
        src: url("/fonts/FunnelSans-VariableFont_wght.ttf");
    }

    :root {
        font-family: "Funnel Sans";
        font-size: 16px;
        line-height: 24px;
        font-weight: normal;

        box-sizing: border-box;

        color: white;
        background-color: #17171d;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;

        user-select: none;
        -webkit-user-drag: none;
    }

    * {
        box-sizing: border-box;
    }

    .container {
        display: flex;
        flex-direction: column;
        align-items: center;

        position: absolute;
        top: 0;
        left: 0;

        width: 100vw;
        height: 100vh;
        margin: 0;

        text-align: center;
    }

    #main-content {
        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 1rem;

        width: calc(100% - 4rem);
        max-width: 64rem;
        height: calc(100% - 4rem);
        margin: 2rem;
    }

    #options {
        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 1rem;

        height: 100%;
    }

    .header {
        height: 4rem;

        pointer-events: none;
    }

    .text-input {
        width: 100%;
        height: 3.5rem;
        min-height: 3.5rem;
        padding: 1rem;
        resize: none;

        border-radius: 0.5rem;

        overflow: hidden;

        font-family: "Funnel Sans";
        font-size: 1rem;
        font-weight: normal;

        transition: 0.2s;
    }

    .text-input:focus {
        height: 100%;

        overflow-y: auto;
    }

    button {
        padding: 0.6rem 1.2rem;

        color: white;
        background-color: transparent;

        border-radius: 0.5rem;
        border: 1px solid rgba(255, 255, 255, 0.25);
        outline: none;

        font-family: "Funnel Sans";
        font-size: 1rem;
        font-weight: normal;

        cursor: pointer;

        transition: 0.2s;
    }

    button:hover {
        border-color: rgba(255, 255, 255, 0.5);
    }

    button:active {
        border-color: white;
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>
