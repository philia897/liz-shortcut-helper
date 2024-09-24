import json

def convert_json(input_file, output_file):
    # Read the JSON data from the input file
    with open(input_file, 'r') as f:
        data = json.load(f)

    # Convert the data to the desired format
    converted_data = [
        {
            "description": item["description"],
            "shortcut": item["shortcut"],
            "application": item["application"],
            "comment": item["comment"]
        }
        for item in data
    ]

    # Write the converted data to the output file
    with open(output_file, 'w') as f:
        json.dump(converted_data, f, indent=4)

if __name__ == "__main__":
    input_file = 'shortcuts.json'  # Replace with your input JSON file
    output_file = 'output.json'  # Replace with your desired output JSON file
    convert_json(input_file, output_file)