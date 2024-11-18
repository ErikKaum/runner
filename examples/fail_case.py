import urllib.request

def main():
    with urllib.request.urlopen('http://jsonplaceholder.typicode.com/todos/1') as response:
        data = response.read()
        print(data)

if __name__ == "__main__":
    main()