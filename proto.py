import os

insert_text = 'syntax = "proto2";\n\npackage dota;\n\n'

proto_dir = 'protobuf'

for filename in os.listdir(proto_dir):
    if filename.endswith('.proto'):
        filepath = os.path.join(proto_dir, filename)
        
        with open(filepath, 'r') as file:
            content = file.read()

        with open(filepath, 'w') as file:
            file.write(insert_text + content)

print("Lines inserted successfully.")
