<script>
    import { resolve } from '$app/paths';
	let { data } = $props();
</script>

<div class="post-container">
	<h1 class="title">{data.metadata.title}</h1>
	<p class="subtitle">{data.metadata.date}</p>

	<div class="post-content">
		<!-- svelte-ignore svelte_component_deprecated -->
		<svelte:component this={data.component} />
	</div>

	<div class="mt-8">
		<a href={resolve("/")} class="link">← Back to home</a>
	</div>
</div>
