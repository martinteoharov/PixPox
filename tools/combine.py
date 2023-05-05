import os

# Set your input and output directories
input_directories = ["../crates/", "../examples/", "../src/"]
output_file = "./merged-file.txt"

with open(output_file, 'w') as outfile:
    for input_directory in input_directories:
        for dirpath, dirnames, filenames in os.walk(input_directory):
            for filename in filenames:
                    filepath = os.path.join(dirpath, filename)
                    with open(filepath, 'r') as infile:
                        outfile.write(f"{filepath} begin\n")
                        outfile.write(infile.read())
                        outfile.write(f"\n{filepath} end\n")