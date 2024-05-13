<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window'
    import Terminal, { PtyTerminal } from '../src/lib/Terminal.svelte';
	import { invoke } from "@tauri-apps/api";

	let terminals: PtyTerminal[] = [];

	$: terms = terminals;

	let id = 0;
	function add() {
		let term = new PtyTerminal(id, "Cascadia Mono", 14, {
			background: "rgb(4, 47, 47)",
			foreground: "white"
		});
		terminals = [...terminals, term];
		id++;
	}
	function remove() {
		let x = terminals.pop();
		x?.kill();
		terms = terminals;
	}
	function send() {
		appWindow.emit("button", {
			message: "test"
		})
	}

	function but1(){
		invoke("test1");
	}

	function but2(){
		invoke("test2");
		appWindow.emit("test2");
	}
</script>
<button on:click={add}>Add terminal</button>
<button on:click={remove}>Remove terminal</button>
<button on:click={send}>Send event</button>
<button on:click={but1}>button1</button>
<button on:click={but2}>button2</button>


<div class="app">
	{#each terms as box}
		<div class="box">
			<Terminal term={box}></Terminal>
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
