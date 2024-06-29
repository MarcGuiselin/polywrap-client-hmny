# polywrap-client-multithreaded

Learning project. An experimental client that loads polywraps and then can run them very quickly in a multithreaded environment.

New wasmer instances of mods are just created as they are needed dynamically, for systems that require high throughput.

The wasm file is generated from this repository: https://github.com/MarcGuiselin/polywrap-client-test/
