<script context="module">
	export const metadata = {
		title: 'Dummy post',
		date: '2026-03-10',
		preview: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus faucibus feugiat lectus et dictum. Integer viverra facilisis urna vel efficitur. Praesent non semper nunc. Donec nulla turpis, consectetur nec semper et, volutpat sed quam. Vestibulum vel orci felis. Nullam at dui pulvinar, eleifend leo eu, scelerisque tortor. Maecenas ultrices arcu at convallis blandit. Pellentesque eu leo et erat facilisis pellentesque non facilisis erat. Nullam a tortor bibendum, suscipit purus in, porttitor velit. Proin eget quam sapien. Pellentesque nulla tortor, tempor sit amet dolor quis, fringilla pharetra arcu. Morbi accumsan eros semper velit eleifend, sit amet varius elit molestie. Proin vitae metus placerat, tincidunt ex sollicitudin, luctus diam. Mauris nulla ipsum, vestibulum a velit a, laoreet egestas neque..'
	};
</script>

<script>

    import { onMount, onDestroy } from 'svelte';

    onMount(async () => {
        // Load the script dynamically to ensure the canvas #glcanvas is in the DOM
        if (!window.load) {
            const script = document.createElement("script");
            script.src = "https://cdn.jsdelivr.net/gh/not-fl3/macroquad@master/js/mq_js_bundle.js";
            script.onload = () => {
                window.load("/wasm/hello_world.wasm");
            };
            document.head.appendChild(script);
        } else {
            // If already loaded, just start the simulation
            // Note: Macroquad might need a fresh canvas if it already initialized one
            window.load("/wasm/hello_world.wasm");
        }
    });

    onDestroy(() => {
        if (window.stop_simulation) {
            window.stop_simulation();
        }
    });
</script>

<canvas id="glcanvas" class="w-full h-full"></canvas>

<style>
    #glcanvas {
        display: block;
        min-height: 400px;
    }
</style>



<p>Text</p>
