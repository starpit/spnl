# spnl build
FROM rust:latest AS spnl_builder
WORKDIR /app
RUN apt update && apt install -y protobuf-compiler patchelf
COPY Cargo.* .
COPY web/wasm web/wasm
COPY benchmarks/haystack benchmarks/haystack
COPY cli cli
COPY spnl spnl
RUN cargo build --release -F rag,spnl-api && strip target/release/spnl
RUN cargo install cargo-binstall && cargo binstall maturin
RUN maturin build --release -F tok,run_py -m spnl/Cargo.toml

# vLLM build
FROM nvcr.io/nvidia/pytorch:25.05-py3 AS vllm_builder
WORKDIR /app
ENV PATH="$PATH:$HOME/.local/bin"
RUN apt update && apt install -y git
RUN git clone https://github.com/starpit/vllm-ibm.git vllm -b spnl-ibm
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
RUN cd vllm && uv venv --seed
RUN cd vllm && grep -v spnl requirements/common.txt > /tmp/z && mv /tmp/z requirements/common.txt
COPY --from=spnl_builder /app/target/wheels/*.whl /tmp
RUN cd vllm && . .venv/bin/activate && uv pip install /tmp/*.whl && VLLM_USE_PRECOMPILED=1 uv pip install --editable .

# Main
FROM nvcr.io/nvidia/pytorch:25.05-py3 as release
WORKDIR /app

ENV PATH="$PATH:/app/vllm/.venv/bin"

COPY --from=spnl_builder /app/target/release/spnl /usr/local/bin
COPY --from=vllm_builder /app/vllm /app/vllm

CMD ["spnl"]
