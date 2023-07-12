<div align="center">

  <h1><code>wasm-pack-template</code></h1>

<strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

The Chip-8 emulator is a project written in Rust that allows you to run and play games developed for the Chip-8 virtual machine. The emulator is compiled to WebAssembly (Wasm) and embedded into a website, providing a seamless and interactive experience for users.

### What is Chip-8?

Chip-8 is an interpreted programming language developed in the 1970s. It was designed for early microcomputers and used to create simple video games and applications. The Chip-8 virtual machine interprets instructions and executes them on the underlying system.

### Future Enhancements

I might come back to this project and extend the Chip-8 instruction set to the Super Chip-8 or even XO-chip extenstions.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### Run the webapp

```
cd ./webapp
npm install
npm start
```

## License

Licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
