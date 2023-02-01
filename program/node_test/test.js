const wasm = require('../pkg/zero_copy_test.js');

let thingy = wasm.AccountThingy.new();
thingy.set_thing(BigInt(123));
console.log(thingy.get_thing());
