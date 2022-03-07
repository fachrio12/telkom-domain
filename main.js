import init, { run_app } from './pkg/telkom_domain_docs.js';
async function main() {
   await init('/pkg/telkom_domain_docs_bg.wasm');
   run_app();
}
main()