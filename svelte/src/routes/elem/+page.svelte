<script>
    import wasm, { main } from '$lib/init';

    main(() => {});
    /** @type {FileList} */
    let files;

    /** @type {string[]} */
    let srcs = [];

    /** @type {number} */
    let width;
    
    let padding = 8;

    // todo: make this more efficient
    $: srcs = Array.from(files ?? []).map(URL.createObjectURL);
    $: {
        if (files) {
            const p = Promise.all(Array.from(files).map(file => file.arrayBuffer().then(buf => new Uint8Array(buf))));
            p.then(b => wasm.files(padding, b[0]));
        } 
    }
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