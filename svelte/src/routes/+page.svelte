<script>
    import wasm, { main } from '$lib/init';
	import { onMount } from 'svelte';

    /** @type {FileList} */
    let files;

    /** @type {HTMLCanvasElement} */
    let canvas;

    /**
	 * @type {CanvasRenderingContext2D | null}
	 */
    let context;

    onMount(() => {
        context = canvas.getContext('2d');
    });

    $: if (files) {
        const img = new Image();
        img.onload = () => {
            context?.drawImage(img, 0, 0);
        };
        img.src = URL.createObjectURL(files[0]);
    }
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />

<div class="h-[100vh] w-[100vw] bg-black">
    <canvas bind:this={canvas} class="w-full h-full">
    </canvas>
</div>