<script>
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
		<a href="/" class="link">← Back to home</a>
	</div>
</div>

<style>
	.post-container {
		max-width: 800px;
		margin: 0 auto;
	}

	.link {
		color: var(--primary);
		text-decoration: none;
	}

	.link:hover {
		text-decoration: underline;
	}
</style>
