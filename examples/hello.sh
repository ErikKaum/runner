curl -X POST http://localhost:3000/run \
    -H "Content-Type: application/json" \
    -d "{\"code\": \"print('hello world')\"}"