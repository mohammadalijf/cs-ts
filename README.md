# cs-ts

A Rust command-line tool that generates TypeScript type definitions from Contentstack CMS schemas.

## Features

- Generate TypeScript interfaces from Contentstack content types and global fields
- Support for all Contentstack field types (text, number, boolean, date, file, link, json, reference, global field, group, blocks)
- Parallel processing for improved performance
- Customizable type name prefixes and postfixes
- Multi-region support
- Output to file or stdout

## Installation

### Download from GitHub Releases

#### macOS (Apple Silicon)
```bash
curl -L -o cs-ts.zip "https://github.com/mohammadalijf/cs-ts/releases/latest/download/cs-ts-aarch64-apple-darwin.zip"
unzip cs-ts.zip
chmod +x cs-ts
sudo mv cs-ts /usr/local/bin/
rm cs-ts.zip
```

#### macOS (Intel)
```bash
curl -L -o cs-ts.zip "https://github.com/mohammadalijf/cs-ts/releases/latest/download/cs-ts-x86_64-apple-darwin.zip"
unzip cs-ts.zip
chmod +x cs-ts
sudo mv cs-ts /usr/local/bin/
rm cs-ts.zip
```

#### Linux (ARM64)
```bash
curl -L -o cs-ts.zip "https://github.com/mohammadalijf/cs-ts/releases/latest/download/cs-ts-aarch64-unknown-linux-gnu.zip"
unzip cs-ts.zip
chmod +x cs-ts
sudo mv cs-ts /usr/local/bin/
rm cs-ts.zip
```

#### Linux (x86_64)
```bash
curl -L -o cs-ts.zip "https://github.com/mohammadalijf/cs-ts/releases/latest/download/cs-ts-x86_64-unknown-linux-gnu.zip"
unzip cs-ts.zip
chmod +x cs-ts
sudo mv cs-ts /usr/local/bin/
rm cs-ts.zip
```

#### Windows (x86_64)
```bash
curl -L -o cs-ts.zip "https://github.com/mohammadalijf/cs-ts/releases/latest/download/cs-ts-x86_64-pc-windows-gnu.zip"
# Extract and add to PATH manually
```

### From source
```bash
git clone git@github.com:mohammadalijf/cs-ts.git
cd cs-ts
cargo build --release
```

The binary will be available at `target/release/cs-ts`.

## Usage

```bash
cs-ts -k <API_KEY> -t <ACCESS_TOKEN> [OPTIONS]
```

### Required Arguments

- `-k, --api-key <API_KEY>`: API key of target stack
- `-t, --access-token <ACCESS_TOKEN>`: Access token of target stack

### Optional Arguments

- `-o, --output <OUTPUT>`: Output file path (prints to stdout if not specified)
- `-r, --region <REGION>`: API region (default: europe)
- `--prefix <PREFIX>`: Prefix for generated type names
- `--postfix <POSTFIX>`: Postfix for generated type names

### Examples

Generate TypeScript types and print to stdout:
```bash
cs-ts -k your-api-key -t your-access-token
```

Generate TypeScript types and save to file:
```bash
cs-ts -k your-api-key -t your-access-token -o types.ts
```

Generate TypeScript types with custom prefixes and postfixes:
```bash
cs-ts -k your-api-key -t your-access-token --prefix "CS" --postfix "Type" -o types.ts
```

Use a different region:
```bash
cs-ts -k your-api-key -t your-access-token -r us
```

## Supported Regions

- `europe` (default)
- `us`
- `azure-na`
- `azure-eu`
- `gcp-na`

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build
```

For optimized release build:
```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
