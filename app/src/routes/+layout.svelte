<script lang="ts">
	import './style.css';
	import { ModeWatcher, toggleMode } from 'mode-watcher';
	import Icon from '@iconify/svelte';

	let { children } = $props();
</script>

<ModeWatcher />

<div class="navbar">
	<div class="subtitle">Blog</div>
	<button class="theme-toggle" onclick={toggleMode}>
		<Icon icon="ix:light-dark" width="1.5em" height="1.5em" />
	</button>
</div>

<main class="p-6">
	{@render children()}
</main>
