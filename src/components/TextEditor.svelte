<script lang="ts">
	import { textStore } from "./textStore";
</script>

<div class="text-editor">
    <div class="content">
        <textarea bind:value={$textStore} placeholder="Enter text"></textarea>
    </div>
</div>

<style>
    .text-editor {
        position: relative;
        width: 55%;
        box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.507);
        border-radius: 5px;
        background: #fff;
        min-height: 70px;
    }

    .content {
        background: #fff;
        border-radius: 5px;
        position: absolute;
        width: 100%;
        height: 100%;
    }

    textarea {
        background: none;
        border: none;
        resize: none;
        font-family: inherit;
        flex-grow: 1;
        width: 100%;
        height: 100%;
        border-radius: 5px;
        font-size: 2rem;
        padding: 1rem;
    }

    textarea::-webkit-scrollbar {
        background-color: #fff;
    }

    textarea:focus {
        border: none;
        outline: #8e44ad solid 3px;
    }

    @media (max-width: 770px) {
        .text-editor {
            width: 100%;
            flex-grow: 1;
        }
		textarea {
            font-size: 1rem;
            padding: 0.5rem;
        }
	}
</style>