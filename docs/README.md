# WebAssembly Component Model Playground

This is a playground for WebAssembly, showing off some of the features and capabilities of WebAssembly Component Model.

## Project structure

The project is structured as follows:
* `wit` contains the WebAssembly Interface Types definitions which are shared between the guest (compiled to webassembly) and host (loading and running the webassembly file)
* `guest-logic` which is a rust based project that will be compiled into a component model compatible webassembly binary
* `host-processor` which is a rust based project that will load and run the guest module as webassembly file

## Running the playground

[![Built with Devbox](https://www.jetify.com/img/devbox/shield_galaxy.svg)](https://www.jetify.com/devbox/docs/contributor-quickstart/)

The project is built with Devbox to make it easy to run the project locally.
Just run `devbox shell` to install all the required locally and get ready to code.

To build the project just run `devbox run build`, which will
* create a markdown specification of the WebAssembly Interface Types located in `wit`
  * the file is located at `target/task.[md|html]`
* compile `guest-logic` with `wasm32-wasip2` target
* compile `host-processor` with the native toolchain target

To run the project just run `devbox run run`, which will take the compiled `guest-logic` and execute it in the `host-processor`.
For the exact commands just check the [devbox.json](/devbox.json) file's `scripts` section.

### Required Tools

Should you not use Devbox, you need to have all the tools installed which are listed in the [devbox.json](/devbox.json) file's `package` section.
