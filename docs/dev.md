# Span Queries - For Developers

## A Quick Overview of this Repository

This repository consists of the following Rust workspaces:

- **spnl**: The core Span Query support, which you can customize with
  a set of [feature flags](feature-flags.md) to selectively enable
  more complex features.
- **cli**: A demonstration CLI that includes a handful of demo queries.
- **wasm**: Wraps `spnl` into a WASM build.
- **web**: A simple web UI that runs queries directly in a browser via
  [WebLLM](https://github.com/mlc-ai/web-llm).

