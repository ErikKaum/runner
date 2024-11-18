import os

def list_files(path='.'):
    """List all files in the given directory and its subdirectories."""
    file_list = []
    for root, dirs, files in os.walk(path):
        for file in files:
            file_list.append(os.path.join(root, file))
    return file_list

files = list_files()
print("Files found:")
for file in sorted(files):
    print(f"- {file}")
