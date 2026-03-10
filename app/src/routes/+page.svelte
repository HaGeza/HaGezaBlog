<script>
	import Card from '../components/Card.svelte';

	let { data } = $props();
</script>

<div>
	<h1 class="title">Welcome to my Blog</h1>
	<h2 class="subtitle">I will put stuff here (hopefully)</h2>

	<div class="card-grid mt-8">
		{#each data.posts as post}
			<Card title={post.title} preview={post.preview} slug={post.slug} />
		{/each}
	</div>
</div>
