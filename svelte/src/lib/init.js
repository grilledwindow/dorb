import { browser } from '$app/environment';
import init, * as wasm from '../wasm/dorb';

/**
 * @param {() => any} main
*/
export const main = (main) => {
    if (browser) {
        (async () => {
            await init();
            main();
        })();
    }
}

export default wasm;