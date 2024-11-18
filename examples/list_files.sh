python_file="list_files.py"
json_payload=$(jq -Rs . < "$python_file")

curl -X POST http://localhost:3000/run \
    -H "Content-Type: application/json" \
    -d "{\"code\": $(jq -Rs . < "$python_file")}"