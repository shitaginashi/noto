import argparse
import yaml
import json
import sys
import os
import uuid
from pathlib import Path
import urllib.request

def ingest_args():
    parser = argparse.ArgumentParser(description="Gator Headless Execution")
    parser.add_argument("-a", "--anchor", type=Path, required=True, help="Target anchor matrix")
    parser.add_argument("-e", "--exec_group", type=str, default="upscale_and_detect", help="Target execution node group")
    parser.add_argument("-t", "--tensor_in", type=str, help="Direct pixel tensor input path")
    parser.add_argument("--dry_run", action="store_true", help="Dump compiled JSON payload to stdout without dispatching")
    return parser.parse_args()

def load_matrix(args):
    try:
        with open(args.anchor, 'r') as f:
            return yaml.safe_load(f)
    except Exception as e:
        sys.exit(f"FATAL: Matrix ingestion failed - {e}")

def upload_tensor(tensor_path):
    """Headless file ingestion to bypass ComfyUI path restrictions."""
    filename = os.path.basename(tensor_path)
    boundary = uuid.uuid4().hex
    
    with open(tensor_path, 'rb') as f:
        file_data = f.read()
        
    body = (
        f'--{boundary}\r\n'
        f'Content-Disposition: form-data; name="image"; filename="{filename}"\r\n'
        f'Content-Type: application/octet-stream\r\n\r\n'
    ).encode('utf-8') + file_data + f'\r\n--{boundary}--\r\n'.encode('utf-8')
    
    req = urllib.request.Request(
        "http://127.0.0.1:8188/upload/image",
        data=body,
        headers={'Content-Type': f'multipart/form-data; boundary={boundary}'}
    )
    
    try:
        with urllib.request.urlopen(req) as response:
            return json.loads(response.read())['name']
    except Exception as e:
        sys.exit(f"FATAL: Tensor upload failed - {e}")

def compile_graph(pipeline_state, target_tensor):
    nodes = pipeline_state.get('nodes', [])
    comfy_prompt = {}
    
    for node in nodes:
        node_id = str(node.get('id'))
        comfy_inputs = {}
        
        if 'params' in node:
            comfy_inputs.update(node['params'])
        if 'inputs' in node:
            comfy_inputs.update(node['inputs'])
            
        for key, val in comfy_inputs.items():
            if val == "$PIXEL_TENSOR_INPUT":
                comfy_inputs[key] = target_tensor
            elif isinstance(val, str) and ".output" in val:
                # Translate YAML syntax into ComfyUI link array [parent_id, port_index]
                parent_id = val.split(".")[0]
                comfy_inputs[key] = [parent_id, 0]
                
        comfy_prompt[node_id] = {
            "class_type": node.get('type'),
            "inputs": comfy_inputs
        }
                    
    return comfy_prompt

def dispatch_to_engine(payload):
    req = urllib.request.Request(
        "http://127.0.0.1:8188/prompt", 
        data=json.dumps({"prompt": payload}).encode('utf-8'),
        headers={'Content-Type': 'application/json'}
    )
    try:
        with urllib.request.urlopen(req) as response:
            return json.loads(response.read())
    except Exception as e:
        sys.exit(f"FATAL: Engine dispatch failed - {e}")

if __name__ == "__main__":
    args = ingest_args()
    pipeline_state = load_matrix(args)
    
    # Resolve the tensor source (CLI arg overrides YAML baseline)
    target_tensor = args.tensor_in if args.tensor_in else pipeline_state.get('anchor', {}).get('source_image')
    
    # 1. Upload raw file to Comfy buffer
    sys.stderr.write(f"[INFO] Uploading tensor to server buffer: {target_tensor}\n")
    server_filename = upload_tensor(target_tensor)
    
    # 2. Compile the graph mapping
    sys.stderr.write(f"[INFO] Compiling {args.exec_group} with resolved filename: {server_filename}\n")
    compiled_payload = compile_graph(pipeline_state, server_filename)
    
    # 3. Dispatch or Dry Run
    if args.dry_run:
        print(json.dumps(compiled_payload, indent=2))
    else:
        sys.stderr.write("[INFO] Dispatching payload to execution engine...\n")
        response = dispatch_to_engine(compiled_payload)
        print(json.dumps(response))