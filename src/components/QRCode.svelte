<script lang="ts">
	import { textStore } from "./textStore";
    import { invoke } from "@tauri-apps/api/tauri";

    let once = false;
    let src = "";
    let isStop = false;
    let timeOutId: NodeJS.Timeout | null = null;

    function wraperCreateQRCode() {
       clearTimeout(timeOutId);

        timeOutId = setTimeout(() => createQRCode(), 1000);
    }

    async function createQRCode() {
        if(!$textStore || isStop) return;

        isStop = true;
        let pastValue = $textStore;
        invoke("create_qr_code", {text: $textStore})
            .then((message: string) => {
                once = true;
                src = "data:image/png;base64," + message;
                console.log(src);
                isStop = false;
                if(pastValue !== $textStore) createQRCode();
            });
    }

    async function saveQRCode() {
        if(!$textStore) return;

        invoke("save_qr_code", {text: $textStore});
    }

    $: $textStore && wraperCreateQRCode();
</script>

<div class="qr-code">
    <div class="qr-code__container">
        {#if once}
            <img {src} alt="">   
        {:else}
            <img class="start" src="./start.jpg" alt="">
        {/if}
    </div>
    <button on:click={saveQRCode}>Save</button>
</div>

<div class:active={isStop} class="loader"></div>

<style>
    .qr-code {
        width: 40%;
        box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.507);
        border-radius: 5px;
        background: #fff;
        display: flex;
        justify-content: space-between;
        flex-direction: column;
    }

    .qr-code__container{
        display: flex;
        align-items: center;
        height: 100%;   
        padding: 1rem;
    }

    button {
        background-color: #8e44ad;
        border: none;
        outline: none;
        color: #fff;
        width: 100%;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0.3rem;
        border-radius: 0 0 5px 5px;
    }

    .start {
        opacity: 0.6;
    }

    img {
        width: 100%;
        box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.178);
    }

    .loader {
        opacity: 0;
        position: fixed;
        top: 0;
        width: 100%;
        height: 6px;
        transition: opacity 0.3s ease-in-out;
    }

    .loader.active {
        opacity: 1;
    }

    .loader::after, .loader::before {
        position: absolute;
        left: -10%;
        content: "";
        width: 10%;
        height: 6px;
        border-radius: 5px;
        background-color: #8e44ad;
    }

    .loader::after {
        animation: move 5s infinite linear;
    }

    .loader::before {
        animation: move 5s 2.5s infinite linear;
    }

    @keyframes move {
        0% {left: -10%;}
        100% {left: 100%;} 
    }

    @media (max-width: 770px) {
		.qr-code {
            margin-top: 1rem;
            width: 50%;
        }
	}
</style>