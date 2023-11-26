
import init, { greet } from '../pkg/dorb.js';

(async () => {
    await init();

    greet("dorb");
})();
