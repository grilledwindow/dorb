<script lang="ts">
    import wasm, { main } from '$lib/init';
	import { onMount } from 'svelte';

    let files: FileList;
    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;

    onMount(() => {
        context = canvas.getContext('2d');
    });

    $: if (files) {
        const img = new Image();
        img.onload = () => {
            if (!context) return;

            // set to full width (why doesn't w-full work???)
            canvas.width = window.innerWidth;

            // ratio of canvas' width to img's width
            // so we know how to adjust the height accordingly
            const scale = canvas.width / img.width;

            const scaledImgWidth = img.width * scale;
            const scaledImgHeight = img.height * scale;

            // adjust canvas' height to scaled img height
            canvas.height = scaledImgHeight;

            context.drawImage(img, 0, 0, scaledImgWidth, scaledImgHeight);
        };
        img.src = URL.createObjectURL(files[0]);
    }
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />

<div class="w-full bg-black">
    <canvas bind:this={canvas}>
    </canvas>
</div>