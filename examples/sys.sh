curl -X POST http://localhost:3000/run \
    -H "Content-Type: application/json" \
    -d "{\"code\": \"import sys; print(sys.platform); import os; print(os.uname().machine)\"}"