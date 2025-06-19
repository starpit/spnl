# Span Queries - For Developers

## A Quick Overview of this Repository

This repository consists of the following Rust workspaces:

- **spnl**: The core Span Query support. Consult [this page](feature-flags.md) for a summary of the feature
> flags you can use to selectively enable more complex features. 
- **cli**: A demonstration CLI that includes a handful of demo queries.
- **wasm**: Wraps `spnl` into a WASM build.
- **web**: A simple web UI that runs queries directly in a browser via [WebLLM](https://github.com/mlc-ai/web-llm).

