<script lang="ts">
	import './layout.css';
	import { ModeWatcher, toggleMode } from 'mode-watcher';
	import Icon from '@iconify/svelte';

	let { children } = $props();
</script>

<ModeWatcher />
<button onclick={toggleMode}>
	<Icon icon="ix:light-dark" width="2.5em" height="2.5em" />
</button>

{@render children()}
