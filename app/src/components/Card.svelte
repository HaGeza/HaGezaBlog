<script lang="ts">
	import Icon from '@iconify/svelte';

	let { title, preview, slug } = $props();

	let isExpanded = $state(false);
	const PREVIEW_LENGTH = 150;

	let displayPreview = $derived(isExpanded
		? preview
		: preview.length > PREVIEW_LENGTH
			? preview.slice(0, PREVIEW_LENGTH) + '...'
			: preview);

	let isExpandable = $derived(preview.length > PREVIEW_LENGTH);

	function toggleExpand(e: Event) {
		e.preventDefault();
		e.stopPropagation();
		isExpanded = !isExpanded;
	}
</script>

<a href={`/posts/${slug}`}>
	<div class="card">
		<h3 class="subtitle">{title}</h3>
		<p class="description">{displayPreview}</p>
		{#if isExpandable}
			<button
				onclick={toggleExpand}
				aria-label={isExpanded ? 'Show less' : 'Show more'}
			>
				<Icon 
					icon={isExpanded ? 'tabler:chevron-up' : 'tabler:chevron-down'} 
					width="1.5em" 
					height="1.5em"
				/>
			</button>
		{/if}
	</div>
</a>
