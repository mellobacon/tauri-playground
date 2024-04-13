<script lang="ts">
	import Terminal from "../src/lib/Terminal.svelte";

	$: boxes = [];
	function add() {
		boxes = [...boxes, "e"]
	}
</script>
<button on:click={add}>Add terminal</button>
<div class="app">
	{#each boxes as _}
		<div class="box">
			<Terminal />
		</div>
	{/each}
</div>

<style>
	.app {
		height: 100vh;
		display: flex;
		flex-direction: row;
		gap: 15px;
		align-items: center;
		justify-content: center;
	}
	.box {
		height: 300px;
		width: 300px;
		background-color: rgb(161, 159, 159);
	}
</style>
