# Span Queries: the SQL for GenAI

:rocket: [Playground](https://pages.github.ibm.com/cloud-computer/spnl/?qv=false) **|** [Research Poster](./docs/poster-20250529.pdf) **|** [About Span Queries](./docs/about.md) **|** [Contribute](./docs/dev.md)

What if we had a **SQL for GenAI**? Span Queries provide a declarative
query foundation for writing scale-up and scale-out interactions with
large language models (LLMs).  A span query allows messages to be
arranged into a [map/reduce](https://en.wikipedia.org/wiki/MapReduce)
tree of generation calls. When LLM calls are arranged in this way into
bulk (multi-generation) queries. 

Learn more about the [structure of a span
queries](./docs/about.md). And learn more about the research
possibilities for [span query planning](./docs/query-planning.md).

**Examples** [Judge/generator](https://pages.github.ibm.com/cloud-computer/spnl/?demo=email&qv=true) **|** [Judge/generator (optimized)](https://pages.github.ibm.com/cloud-computer/spnl/?demo=email2&qv=true) **|** [Policy-driven email generation](https://pages.github.ibm.com/cloud-computer/spnl/?demo=email3&qv=true)

## Getting Started

To kick the tires, you can use the Span Query CLI front-end. This CLI
is only geared towards demos, at this point. Using it, you can run one
of the built-in demos, or you can point it to a JSON file. Plans are
underway for integration backends (stay tuned!) and also with
user-facing libraries such as
[PDL](https://github.com/IBM/prompt-declaration-language). Please open
an issue documenting your use cases!

The span query system is written in
[Rust](https://www.rust-lang.org/). This choice was made to facilitate
flexible integration with backends, CLIs, and with Python
libraries. Rust is also awesome. Thus, step 1 in getting started is to
[configure your
environment](./https://www.rust-lang.org/tools/install) for Rust, if
you haven't already. Step 2 is to clone this repository.

Now you can run a quick demo with:

```shell
cargo run
```

The full usage is provided via `cargo run -- --help`, which also
specifies the available demos.

```
Usage: spnl [OPTIONS] [FILE]

Arguments:
  [FILE]  File to process

Options:
  -d, --demo <DEMO>
          Demo to run [possible values: chat, email, email2, email3, sweagent, gsm8k, rag]
  -m, --model <MODEL>
          Generative Model [default: ollama/granite3.2:2b]
  -e, --embedding-model <EMBEDDING_MODEL>
          Embedding Model [default: ollama/mxbai-embed-large:335m]
  -t, --temperature <TEMPERATURE>
          Temperature [default: 0.5]
  -l, --max-tokens <MAX_TOKENS>
          Max Completion/Generated Tokens [default: 100]
  -n, --n <N>
          Number of candidates to consider [default: 5]
  -k, --chunk-size <CHUNK_SIZE>
          Chunk size [default: 1]
      --vecdb-uri <VECDB_URI>
          Vector DB Url [default: data/spnl]
  -s, --show-query
          Re-emit the compiled query
  -v, --verbose
          Verbose output
  -h, --help
          Print help
  -V, --version
          Print version
```
