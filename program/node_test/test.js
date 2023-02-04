const { MasterEdition } = require('../pkg/zero_copy_test.js');
const wasm = require('../pkg/zero_copy_test.js');

// Cheating for speed
let me = new wasm.MasterEdition('1', '10', 'Keith');

console.log(MasterEdition.get_supply(me));
console.log(MasterEdition.get_master_edition(me));
