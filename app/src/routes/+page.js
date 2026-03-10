export const load = async () => {
    const allPostFiles = import.meta.glob('/src/lib/posts/*.svelte');
    const iterablePostFiles = Object.entries(allPostFiles);

    const allPosts = await Promise.all(
        iterablePostFiles.map(async ([path, resolver]) => {
            // @ts-ignore
            const { metadata } = await resolver();
            // @ts-ignore
            const slug = path.split('/').pop().slice(0, -7);
            return { slug, ...metadata };
        })
    );

    return { posts: allPosts };
};
