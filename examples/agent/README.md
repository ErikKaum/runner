# Example with simple LLM Agent

A showcase of how to use the wasm sandbox with an LLM to generate and execute Python code. The wordflow is:
1. Take task descriptions
2. Generate corresponding Python code. I'm using the HuggingFace inference API, but anything that follows the OpenAI spec will work.
3. Execute the generated code on a local server
4. Return the stdout/stderr from the sandbox

## Requirements

- Python 3.x
- HuggingFace API token
- Running code execution server (on port 3000)
- (Optional but nice) uv

## Usage

1. Replace the placeholder API key in `main.py` with your HuggingFace token:
```python
llm_api_key = "your_huggingface_api_token"
```

2. Ensure the code execution server is running on `http://localhost:3000`

3. Run the agent (check below if you don't use uv):
```bash
uv run main.py
```

Your output should be something like this:
```
📝 Task: Write a Python function that calculates the factorial of a number and return the result for input 5

💻 Generated Code:
def factorial(n):
    if n == 0 or n == 1:
        return 1
    else:
        return n * factorial(n - 1)

result = factorial(5)
print(result)

🚀 Executing code:
Execution Result: {'res': {'status': 'Ok', 'output': '120\n'}}
```

And the server logs should be something like this:
```
2024-11-17T13:46:25.199232Z  INFO wasm_wasi_runner::runner: sandbox 4cfe60a2-fc96-4c14-827b-89a280eeef51 exited successfully after: 51.635167ms
python e784b236-85d6-497b-8405-65b2ccdf829f: 120
```

The agent can be configured with different LLM models from HuggingFace Inference API. The default model is `Qwen/Qwen2.5-Coder-32B-Instruct`, but you can use any other supported model from the [HuggingFace Model List](https://huggingface.co/docs/api-inference/en/supported-models#supported-models).


## Installing deps with pip

```bash
python3 -m venv venv
source venv/bin/activate
pip install .
python3 run main.py
```
