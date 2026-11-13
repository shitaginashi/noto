#!/bin/bash
COMFY_ROOT="/home/shiro/ComfyUI"

echo "Clearing ghosts..."
pkill -9 -f "main.py" || true

# Force PyTorch to reclaim every fragmented byte
export PYTORCH_CUDA_ALLOC_CONF="expandable_segments:True,max_split_size_mb:128"
export PYTHONNOUSERSITE=1
unset PYTHONPATH
unset PYTHONHOME

cd "$COMFY_ROOT"

# Activate environment
source ~/miniconda3/etc/profile.d/conda.sh
conda activate py310

LOG_FILE="/tmp/comfy_headless.log"
echo "Igniting ComfyUI. Routing output to $LOG_FILE..."

# Run in background, pass all script arguments ($@), and pipe to log
python main.py \
    --cuda-malloc \
    --port 8188 "$@" > "$LOG_FILE" 2>&1 &

# Launch browser
sleep 6
firefox "http://localhost:8188" &

echo "from the depths of HELL, headless comfy rides for vengeance... Streaming live output (Ctrl+C to exit stream, Comfy will stay alive):"
echo "========================================================================================"
tail -f "$LOG_FILE"
