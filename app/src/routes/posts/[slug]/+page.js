export const load = async ({ params }) => {
    const post = await import(`../../../lib/posts/${params.slug}.svelte`);
    return {
        component: post.default,
        metadata: post.metadata
    };
};