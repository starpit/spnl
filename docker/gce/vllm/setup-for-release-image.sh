#!/usr/bin/env bash

#
# Note: this script is executed inside the VM when using a custom release image.
# The custom image already has vLLM and ollama installed, so we just need to start the services.
#

set -eo pipefail

cleanup() {
    rc=$?
    echo "Exiting with exit_code=$rc"
    gsutil cp <(echo $rc) gs://$GCS_BUCKET/runs/$RUN_ID/status/exit_code
}
trap "cleanup" EXIT

export HOME=/root
cd $HOME

# Load environment
. /etc/environment

# Activate vLLM virtual environment (installed in custom image)
cd vllm
source .venv/bin/activate

# Start vLLM
VLLM_ATTENTION_BACKEND=TRITON_ATTN \
    VLLM_USE_V1=1 \
    VLLM_V1_SPANS_ENABLED=True \
    VLLM_V1_SPANS_TOKEN_PLUS=10 \
    VLLM_V1_SPANS_TOKEN_CROSS=13 \
    VLLM_SERVER_DEV_MODE=1 \
    nohup vllm serve $MODEL --enforce-eager &

# Start ollama (already installed in custom image)
nohup ollama serve &

# Wait till vllm is ready
timeout 5m bash -c 'until curl --output /dev/null --silent --fail http://localhost:8000/health; do sleep 3; done'
echo "vllm is ready"

# Wait till ollama is ready
timeout 5m bash -c 'until ollama ps; do sleep 3; done'
echo "ollama is ready"

echo "Services started successfully"

# Made with Bob
