import json
with open('cargo_errors.json', encoding='utf8') as f:
    for line in f:
        if line.strip():
            msg = json.loads(line)
            if msg.get('reason') == 'compiler-message' and msg['message']['level'] == 'error':
                spans = msg['message']['spans']
                if spans:
                    print(f"{msg['message']['message']} at {spans[0]['file_name']}:{spans[0]['line_start']}")
                else:
                    print(msg['message']['message'])
