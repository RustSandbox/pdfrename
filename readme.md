Given the additional details you've provided, I'll revise the README to include the links to the Ollama installation and model pulling instructions. Here's an updated version that incorporates these references:

---

# PDF Renamer Script

## Overview

This script reads a list of PDF files from a specified directory and renames each file based on its content. The renaming logic uses the content of the PDF to generate a more descriptive and meaningful filename.

## Prerequisites

Before launching the script, ensure the following requirements are met:

- **Ollama Installation**: The script requires the `ollama` tool to be installed on your machine. This tool handles the parsing and extraction of text from PDF files.
- **Llama3:70 Model**: You must pull the `llama3:70` model. This model is used by `ollama` for processing the PDFs. Note that due to the complexity and size of the model, the script's execution might be slow.

## Installation

### 1. Install Ollama

Ensure that `ollama` is installed on your system. If not installed, you can find the installation guide at:
[Ollama Installation Guide](https://ollama.com/download)

### 2. Pull Llama3:70 Model

Pull the necessary model using the following command:

```bash
ollama pull llama3:70
```

For more details on pulling the model, visit:
[How to Pull Llama3:70 Model](https://ollama.com/library/llama3:70b)

## Usage

To run the script, navigate to the directory where the script is located and execute the following command:

```bash
./pdf-rename /path/to/your/pdf/directory
```

Replace `/path/to/your/pdf/directory` with the actual path to the directory containing your PDF files.

## Note for Developers

This script was developed as part of a learning exercise in Rust programming. As such, it is not optimized for production use. The code may contain inefficiencies, particularly due to the heavy processing requirements of the `llama3:70` model. Users are advised to review and modify the code as needed before any production deployment.

## Contributions

Contributions are welcome! If you have suggestions for improvement or have found a bug, please feel free to create an issue or submit a pull request.
