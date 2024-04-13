<script lang="ts">
	import "xterm/css/xterm.css";
	import { onMount } from "svelte";
	import {Terminal} from "xterm";
	import {FitAddon} from "xterm-addon-fit";
	import { invoke } from "@tauri-apps/api";

	let terminalElement: HTMLElement;

	function initializeXterm() {
		terminalController.loadAddon(termFit);
		terminalController.open(terminalElement);

		initShell();
		terminalController.onData(writeToPty);
		addEventListener("resize", fitTerminal);
		fitTerminal();

		window.requestAnimationFrame(readFromPty);
	}

	onMount(async () => {
		initializeXterm();
	});
</script>
<script lang="ts" context="module">
	let terminalId = 0;
let terminalController = new Terminal({
	fontFamily: "Cascadia Mono",
	fontSize: 14,
	theme: {
		background: "rgb(4, 47, 47)",
		foreground: "white"
	}
});
let termFit = new FitAddon();
export function fitTerminal() {
	termFit.fit();
	void invoke<string>("async_resize_pty", {
		rows: terminalController.rows,
		cols: terminalController.cols,
		terminalId
	});
}
function writeToTerminal(data: string) {
	return new Promise<void>((r) => {
		terminalController.write(data, () => r());
	});
}

function writeToPty(data: string) {
	void invoke("async_write_to_pty", {
		data,
		terminalId
	});
	console.log("end");
}

async function initShell() {
	terminalId = await invoke("async_create_shell").catch((error) => {
		// on linux it seem to to "Operation not permitted (os error 1)" but it still works because echo $SHELL give /bin/bash
		console.error("Error creating shell:", error);
	});
}
async function readFromPty() {
	const data = await invoke<string>("async_read_from_pty", {terminalId});
	if (data) {
		await writeToTerminal(data);
	}
	window.requestAnimationFrame(readFromPty);
}
</script>

<div id="terminal" bind:this={terminalElement} />

<style>
	#terminal {
		height: 100%;
		width: 100%;
	}
</style>
