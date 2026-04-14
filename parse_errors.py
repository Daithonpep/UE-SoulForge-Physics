import json

with open('errors.json', 'r', encoding='utf-16') as f, open('parsed.txt', 'w', encoding='utf-8') as out:
    for line in f:
        try:
            msg = json.loads(line.strip())
            if msg.get('reason') == 'compiler-message':
                if msg['message']['level'] == 'error':
                    out.write(msg['message']['message'] + '\n')
                    for span in msg['message']['spans']:
                        if span['is_primary']:
                            out.write(f"  {span['file_name']}:{span['line_start']}\n")
        except:
            pass
