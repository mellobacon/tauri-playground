<script lang="ts">
	import "xterm/css/xterm.css";
	import { onMount } from "svelte";
	import {Terminal, type ITheme} from "xterm";
	import { FitAddon } from "xterm-addon-fit";
	import { spawn, type IPty } from "tauri-pty";

	export let term: PtyTerminal;
	let terminalElement: HTMLElement;

	async function initializeXterm() {
		term.htmlEl = terminalElement;
		await term.init();
	}

	onMount(async () => {
		await initializeXterm();
	});
</script>
<script lang="ts" context="module">
	export class PtyTerminal {
		private pty: IPty;
		private terminal: Terminal;
		private termfit: FitAddon;
		private options = {};
		private id = 0;
		htmlEl;
		constructor(id: number, font: string, fontsize: number, theme: ITheme) {
			this.options = {
				fontFamily: font,
				fontSize: fontsize,
				theme: theme
			}
			this.terminal = new Terminal(this.options);
			this.termfit = new FitAddon();
			this.id = id;
			const pty = spawn("powershell.exe", [], {
				cols: this.terminal.cols,
				rows: this.terminal.rows,
			})
			this.pty = pty;
		}
		async init() {
			this.terminal.loadAddon(this.termfit);
			this.terminal.open(this.htmlEl);
			this.pty.onData(data => this.terminal.write(data));
			this.terminal.onData(data => this.pty.write(data));
		}
		kill() {
			this.pty.kill();
		}
	}
</script>

<div id="terminal" bind:this={terminalElement} />

<style>
	#terminal {
		height: 100%;
		width: 100%;
	}
</style>
