import { browser } from '$app/environment';
import init, * as wasm from '../wasm/dorb';

export const main = (main: () => any) => {
    if (browser) {
        (async () => {
            await init();
            main();
        })();
    }
}

export default wasm;