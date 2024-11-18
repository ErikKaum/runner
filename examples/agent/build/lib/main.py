import requests
from huggingface_hub import InferenceClient

class PythonLLMAgent:
    def __init__(self, llm_api_key: str, llm_model: str, server_url: str):
        """
        Initialize the agent with LLM and server details.
        
        Args:
            llm_api_key (str): HuggingFace API token
            llm_model (str): Model name (e.g., 'mistralai/Mistral-7B-Instruct-v0.2')
            server_url (str): URL of the server that executes Python code.
        """
        self.llm_api_key = llm_api_key
        self.llm_model = llm_model
        self.server_url = server_url
        self.client = InferenceClient(token=llm_api_key)

    def generate_code(self, prompt: str) -> str:
        """
        Generate Python code using the HuggingFace model.

        Args:
            prompt (str): Instruction or task description.

        Returns:
            str: Generated Python code.
        """
        try:
            system_prompt = "You are a helpful AI that writes Python code, return only Python code in markdown and nothing else. Always print the result at the end"
            full_prompt = f"{system_prompt}\n\nUser: {prompt}\n\nAssistant:"
            
            response = self.client.text_generation(
                full_prompt,
                model=self.llm_model,
                max_new_tokens=512,
                temperature=0.1,
                do_sample=True,
                seed=1337
            )

            # Hacky way of cleaning but works for demo
            cleaned_response = response
            if response.strip().startswith("```python"):
                cleaned_response = response.split("```python")[1]
            if cleaned_response.strip().endswith("```"):
                cleaned_response = cleaned_response.split("```")[0]
            return cleaned_response.strip()


        except Exception as e:
            print(f"Error generating code: {e}")
            return ""

    def execute_code(self, python_code: str) -> dict:
        """
        Execute Python code on the server.

        Args:
            python_code (str): The Python code to execute.

        Returns:
            dict: The JSON response from the server.
        """
        endpoint = f"{self.server_url}/run"
        headers = {"Content-Type": "application/json"}
        payload = {"code": python_code}

        try:
            response = requests.post(endpoint, json=payload, headers=headers)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Error communicating with the server: {e}")
            return {"error": str(e)}

    def run_task(self, task: str):
        """
        Perform the full workflow: generate code, send it to the server, and return the result.

        Args:
            task (str): Task description for the agent.

        Returns:
            dict: Execution results.
            
        """

        CYAN = '\033[96m'
        GREEN = '\033[92m'
        YELLOW = '\033[93m'
        RESET = '\033[0m'

        print(f"\n{CYAN}📝 Task:{RESET} {task}")
        python_code = self.generate_code(task)
        print(f"\n{GREEN}💻 Generated Code:{RESET}\n{python_code}")
        print(f"\n{YELLOW}🚀 Executing code:{RESET}")
        result = self.execute_code(python_code)

        return result

def main():
    llm_api_key = "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"
    # llm_api_key = "placeholder"
    
    if llm_api_key == "placeholder":
        import sys
        sys.exit("Replace the placeholder key with your huggingface API key.\nIf you have an account go to https://huggingface.co/settings/tokens")

    llm_model = "Qwen/Qwen2.5-Coder-32B-Instruct" # or any other supported model: https://huggingface.co/docs/api-inference/en/supported-models#supported-models
    server_url = "http://localhost:3000"

    agent = PythonLLMAgent(llm_api_key, llm_model, server_url)

    task_description = "Write a Python function that calculates the factorial of a number and return the result for input 5"
    
    result = agent.run_task(task_description)
    print("Execution Result:", result)


if __name__ == "__main__":
    main()