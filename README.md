# blake3-node-neon-no-package

* Expose blake3 to node modules.
* Expose blake3 to browser contexts.

This implementation referes to the Rust crate for blake3.

**<u>The main reason for this version</u>:**

## Minimal packaging of code

It may be desirable to use blake3 hashing in upstream modules, such as [crypto-wraps]() or [ucwid](). But, there is a desire to keep those modules fairly lean. The basic version of those modules do not have dependencies outside of their execution contexts. Node.js provides **crypto.subtle** and so does the browser. But, **crypto.subtle** does not included some of the newer hashes and ciphers.

Some of the JavaScript packages for including these newer hashes and ciphers use extensive packaging libraries. That too, and they are not necessarily being maintained.


## Install

For node and browser:

```
npm install blake3-node-neon-no-package --save
```


For the browser, bring the assets up with [get-nmp-assets]().

```
$npm install -g get-npm-assets
$get-npm-assets blake3-node-neon-no-package
$ls ./assets
```

The output should list a wasm file for use in the browser.


## Usage


## Associated modules

## Build

### web

```
cargo build --release --target=wasm32-unknown-unknown
```

### node



