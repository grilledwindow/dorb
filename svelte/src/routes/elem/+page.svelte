<script lang="ts">
    import * as JSZip from 'jszip';
    import wasm, { main } from '$lib/init';

    main(() => {});
    let files: FileList;
    let srcs: string[] = [];
    let width: number;
    let maxSize: number;
    let padding = 0;

    let zipped: File;

    // TODO: make this more efficient
    $: srcs = Array.from(files ?? []).map(URL.createObjectURL);

    const processImages = async (files: FileList | null): Promise<File[] | undefined> => {
        if (!files || !files.length) return;
        const zip = new JSZip();

        await Promise.all(Array.from(files).map(async file => {
            const buf = await file.arrayBuffer();
            const bytes = new Uint8Array(buf);
            const newBytes = wasm.files(padding, bytes);
            
            zip.file("dorb_" + file.name, newBytes);
        }));

        const blob = await zip.generateAsync({ type: 'blob'});
        zipped = new File([blob], "dorb.zip");
    };

    $: {
        const scaledPadding = padding * width / 800 * 2;
        const frameWidth = width + scaledPadding * 2;
        const percentage = width / frameWidth;
        maxSize = width * percentage;
    }
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />
<button on:click={() => processImages(files)} class="border border-black p-1">Process images</button>

<div class="w-full mt-4 space-y-2">
    <input type="number" bind:value={padding} placeholder="padding">
    
    {#if zipped}
        <a href={URL.createObjectURL(zipped)} download={zipped.name} class="border border-black p-1">
            Download {zipped.name}
        </a>
    {/if}

    {#each srcs as src}
        <div bind:clientWidth={width} class="w-1/2 border border-5 border-black bg-white flex justify-center items-center" style="height: {width}px">
            <img {src} alt="" class="left-0" style="max-width: {maxSize}px; max-height: {maxSize}px" />
        </div>
    {/each}
</div>