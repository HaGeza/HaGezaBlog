<script>
	let { title, preview, slug } = $props();
</script>

<a href={`/posts/${slug}`} class="card-link">
	<div class="card">
		<h3 class="subtitle">{title}</h3>
		<p class="description">{preview}</p>
	</div>
</a>

<style>
	.card-link {
		text-decoration: none;
		color: inherit;
	}

	.card-link:hover .card {
		opacity: 0.8;
		transition: opacity 0.2s ease-in-out;
	}
</style>
