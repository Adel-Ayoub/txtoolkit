# txtoolkit
## A PDF text extraction tool built with Rust
> Fast and efficient text extraction from PDF documents with support for local files and web URLs.

It can be combined with AI / LLM tools like AiBro or Mods to evaluate and understand PDF documents. Just pipe the result on the command line from one tool to the other.

## How to Install txtoolkit

There are two methods to install txtoolkit:

```bash
# Clone the repository
git clone https://github.com/Adel-Ayoub/txtoolkit.git
cd txtoolkit

# Option 1: Manual Rust setup (if you have Rust installed)
cargo install --path .

# Option 2: Using Nix (uses the provided flake configuration)
nix develop
cargo install --path .
```

## CLI Options

| Flag | Command | Description |
| ---- | ------- | ----------- |
| **-m** | `--method` | Input method: `file` (default) or `web` |
| **-l** | `--limit` | Set byte limit on output (default: 100000) |
| **-i** | `--ignore-limit` | Ignore byte limit and output all text |
| **-h** | `--help` | Show help information |
| **-V** | `--version` | Show version |

## Requirements

- Rust 1.70+ and Cargo
- Internet connection (for web PDF downloads)
- Compatible with Windows, macOS, and Linux

## Input Methods

| Method | Description | Purpose |
| ------ | ----------- | ------- |
| **file** | Local PDF file | Default method for files on disk |
| **web** | Remote PDF URL | Downloads and extracts from web addresses |

## Command Reference

```
Usage: txtoolkit [OPTIONS] <INPUT>

Arguments:
  <INPUT>
          Input source

Options:
  -m, --method <METHOD>
          Selected input method

          [default: file]

          Possible values:
          - file: Input from file on disk
          - web:  Download input from web address

  -l, --limit <LIMIT>
          Set byte limit on output

          [default: 100000]

  -i, --ignore-limit
          Ignore byte limit

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

## Requirements

- Rust 1.70+ and Cargo
- Internet connection (for web PDF downloads)
- Compatible with Windows, macOS, and Linux

## Input Methods

| Method | Description | Usage |
| ------ | ----------- | ----- |
| **file** | Local PDF file | Default method for files on disk |
| **web** | Remote PDF URL | Downloads and extracts from web addresses |

## Usage

### Extract from Local File
```bash
# Basic extraction
txtoolkit document.pdf

# With byte limit
txtoolkit -l 5000 document.pdf

# Extract all text (no limit)
txtoolkit -i document.pdf
```

### Extract from Web URL
```bash
# Download and extract from web
txtoolkit -m web https://example.com/document.pdf

# With custom byte limit
txtoolkit -m web -l 2000 https://example.com/paper.pdf
```

## Example Usage

### Complete Text Extraction Workflow
```bash
# Create sample command
txtoolkit ~/Documents/report.pdf
```

```bash
# Extract limited text from local PDF
txtoolkit -l 1000 ~/Downloads/presentation.pdf
# Output: First 1000 characters of PDF text content
```

```bash
# Extract from web PDF
txtoolkit -m web "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
# Output: Full text content from the web PDF
```

```bash
# Pipe to other tools for analysis
txtoolkit document.pdf | grep "important keyword"
txtoolkit -m web https://example.com/paper.pdf | wc -w
```

### Integration with AI Tools
```bash
# Combine with AI analysis tools
txtoolkit document.pdf | mods "summarize this document"
txtoolkit -m web https://example.com/paper.pdf | aibro "what are the key findings?"
```

### Understanding Output
- **Format**: Plain text extracted from PDF content
- **Encoding**: UTF-8 text output
- **Limits**: Configurable byte limits prevent overwhelming output
- **Streaming**: Text is output directly to stdout for piping
- **Error Handling**: Clear error messages for network or file issues
