<script>
    import wasm, { main } from '$lib/init';

    /** @type {FileList} */
    let files;

    /** @type {string[]} */
    let srcs = [];

    // todo: make this more efficient
    $: srcs = Array.from(files ?? []).map(URL.createObjectURL);
</script>

<input type="file" multiple accept="image/png, image/jpeg" bind:files />

<div class="w-full bg-black">
    {#each srcs as src}
        <div>
            <img {src} alt="" />
        </div>
    {/each}
</div>