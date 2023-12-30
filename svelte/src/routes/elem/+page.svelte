<script>
    import wasm, { main } from '$lib/init';

    /** @type {FileList} */
    let files;

    /** @type {string[]} */
    let srcs = [];

    /** @type {number} */
    let width;
    
    let padding = 8;

    // todo: make this more efficient
    $: srcs = Array.from(files ?? []).map(URL.createObjectURL);
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />

<div class="w-full space-y-2">
    <input type="number" bind:value={padding}>
    {#each srcs as src}
        <div bind:clientWidth={width} class="w-1/2 bg-black flex justify-center items-center" style="height: {width}px">
            <img {src} alt="" class="left-0" style="max-width: {width - padding * 2}px; max-height: {width - padding * 2}px" />
        </div>
    {/each}
</div>