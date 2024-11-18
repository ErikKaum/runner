# 🏃 Runner

An experimental Rust-based server that executes Python code in a WebAssembly+WASI sandbox. The server provides a simple HTTP API to run Python code securely and with extremely fast startup times. This makes Runner ideal for running LLM-generated code safely.

The purpose of this repo is to explore the current state of WASM+WASI as a sandboxed runtime for code execution. I've always been amazed by the [Cloudflare Workers runtime](https://blog.cloudflare.com/cloud-computing-without-containers/), which builds on the V8 engine. You can think of this as an aspiration to build something:
- with similar startup times (and maybe similar memory footprint)
- that's not only for JavaScript 
- open-source for you to run locally and tinker with.

### What it does:
- Executes Python code in an isolated WASM+WASI environment.
- Provides an HTTP API for code execution:
    - Suitable for LLM code execution (see `examples/agent`).
- Enables safe and easy control over resource access (see `examples/list_files.sh`).

### Known limitations:
- Networking is not implemented (see `examples/fail_case.sh`).
- Installing Python modules with system dependencies won’t work, including popular libraries like NumPy or PyTorch.

## Get started 💻

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

With the development build, it takes around 15 seconds to load the Python runtime (about 1 second on release builds).

```bash
cargo run
```

Once the server is running...
```
2024-11-17T17:01:08.345563Z  INFO runner: starting server
2024-11-17T17:01:24.414272Z  INFO runner::runner: loading python runtime took: 16.0684225s
2024-11-17T17:01:24.414719Z  INFO runner: server running on port 3000 🚀
```

...send Python code to execute:

```bash
curl -X POST http://localhost:3000/run \
    -H "Content-Type: application/json" \
    -d '{"code": "print(\"hello world\")"}'
```

which returns:

```bash
{"res":{"status":"Ok","output":"hello world\n"}}                                                                                  
```

Check out the `examples/` directory for examples:
- Bubble sort.
- Listing files (only accesses those permitted by the WASM runtime).
- LLM Agent integration using the HuggingFace Inference API.

It should be easy for you to expand the examples and try your own tricks.
Some examples require `jq` and [GNU parallel](https://www.gnu.org/software/parallel/).

### Note

Using a release build significantly improves performance. If you want to go faster, run:

```bash
cargo build --release 
```

```bash
./target/release/runner
```

Enjoy 🏎️

## Under the hood ⚒️

### How does this work?
The WASM+WASI runtime is provided by [Wasmtime](https://docs.wasmtime.dev/) as a crate. To run Python code in this sandbox, we need to compile the Python interpreter (CPython in this case) to the `wasm32-unknown-wasi` target. The `wasm32-unknown-wasi` target is currently at Tier 2, but you still have to do the compilation yourself. Fortunately, there's a [good guide](https://docs.wasmtime.dev/) on setting everything up in a devcontainer. To spare you the hassle, I’ve included the compiled Python interpreter in the repo, despite the large file size.

If you're unfamiliar with target triplets, Brett Cannon has a [good blog post](https://snarky.ca/webassembly-and-its-platform-targets/) explaining them, particularly in the context of Python and `wasm32-unknown-wasi`.

This also means that Python libraries with system dependencies won’t work out of the box. Most popular Python libraries fall into this category. For example, `pytorch` relies on `libtorch`, a dynamically linked C++ library that needs to be built for the target architecture. To make it work in this setup, we would need a `libtorch` compiled to `wasm32-unknown-wasi`. Additionally, the Python interpreter would need to load this library at runtime using functions like `dlopen` or `dlsym`, which are unavailable in WASI. These functions are [stubbed](https://github.com/WebAssembly/wasi-libc/pull/443) until dynamic linking is implemented in WASM.

### Why not Docker?

You might be wondering "we already have docker; why do we need WASM+WASI?". I think there's some confusion around the similarities and differences between the two. Even more so after the founder of Docker [posted](https://x.com/solomonstre/status/1111004913222324225) that he wouldn't have needed to create Docker if WASM+WASI existed 🤔 

A few pointers on top of my head:
- **Docker doesn't run natively on macOS or Windows.** So you need a Linux VM that runs Docker inside of it, which isn't ideal
- **A minimal hello-world container typically starts in about 0.5-1.5 seconds.** While this is fast for most use cases, it's quite slow if you're just executing small functions here and there. For reference, the `example/fib.sh` takes around 44ms to run on my machine. So a startup time of 0.5 seconds is more than 10x the execution time of the code itself. Starting the Python WASM+WASI runtime from cache takes around 1-2ms and on server startup (without cache) it's typically just below 1 second. Of course, you should do your own benchmarking but the ratios should be similar.
- **With WASM+WASI you get more fine-grained control.** By default in wasm, none of the system APIs can be accessed by the guest code. So opening files, sockets etc results in an error. In WASM+WASI it's also possible to give read-access to files. Which is what I'm doing in this experiment. For the python runtime to work, it needs access to the `lib/python3.12`. And instead of making a copy of this directory for each sandbox, they all get read-access to it. Would this be safe to do in a multitenant service..? Maybe, maybe not 🤷‍♂️  
- The downside with WASM+WASI is that **it's more complicated**. Since everything needs to run within a VM that executes WASM bytecode you also need to compile it WASM.
- Docker is essentially a process running in a chroot jail + cgroup restrictions and many other things on top. But it's still a "normal" process, so there isn't a performance overhead in the same was as in WASM+WASI.
- **WASM+WASI is very early**.
    - The security and confidence in the security of software comes partially from running it in production for several years, with skin in the game. The V8 JS engine is a good example of this. And as you probably noticed, the WASM+WASI sanbox should be very secure but there isn't much real world evidence. It should be fine for two sandbox instances to have read access to the same file. But if there's a bug in this part of the wasm implementation, you'll inherit that bug. Similarly Docker has its security concerns, but through time they've been explored, patched and are better understood.
    - Tooling, documentation and that jazz. Don't expect to have a "it just works" experience. 