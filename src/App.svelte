<script lang="ts">
	import QrCode from "./components/QRCode.svelte";
import TextEditor from "./components/TextEditor.svelte";
</script>

<main>
	<TextEditor />
	<QrCode />
</main>

<style>
	main {
		padding: 2rem;
		display: flex;
		min-height: 100%;
		justify-content: space-around;
		overflow: auto;
	}

	@media (max-width: 770px) {
		main {
			padding: 0.5rem;
			justify-content: flex-start;
			flex-direction: column;
			align-items: center;
		}
	}
</style>