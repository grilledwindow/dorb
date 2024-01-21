<script lang="ts">
    import wasm, { main } from '$lib/init';

    main(() => {});
    let files: FileList;
    let srcs: string[] = [];
    let processedImages: File[] = [];
    let width: number;
    let padding = 8;

    // TODO: make this more efficient
    $: srcs = Array.from(files ?? []).map(URL.createObjectURL);

    const processImage = async (file: File): Promise<File> => {
        const buf = await file.arrayBuffer();
        const bytes = new Uint8Array(buf);
        const newBytes = wasm.files(padding, bytes);
        return new File([newBytes], "dorb_" + file.name);
    }

    const processImages = async (files: FileList | null): Promise<File[] | undefined> => {
        if (!files || !files.length) return;
        processedImages = await Promise.all(Array.from(files).map(processImage));
        console.log(processedImages)
    };
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />
<button on:click={() => processImages(files)} class="border border-black p-1">Process images</button>

<div class="w-full mt-4 space-y-2">
    <input type="number" bind:value={padding}>
    
    {#each processedImages as img}
        <a href={URL.createObjectURL(img)} download={img.name} class="border border-black p-1">
            Download {img.name}
        </a>
    {/each}

    {#each srcs as src}
        <div bind:clientWidth={width} class="w-1/2 bg-black flex justify-center items-center" style="height: {width}px">
            <img {src} alt="" class="left-0" style="max-width: {width - padding * 2}px; max-height: {width - padding * 2}px" />
        </div>
    {/each}
</div>