import init, { update } from '/pkg/frontend.js';

async function main() {
    await init();
    update();
}

main();
