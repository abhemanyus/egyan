# Egyankosh Material Downloader

A Rust-based tool to systematically download study materials from Egyankosh.ac.in (IGNOU's digital repository).

## Features

- Structured download of complete SLM materials
- Automatic PDF detection and saving
- Skip existing files for resume capability
- Organized folder structure preservation

## Prerequisites

- Latest Rust ([Installation guide](https://www.rust-lang.org/tools/install))
- Network connection with access to https://egyankosh.ac.in

## Installation

```bash
# Clone repository or just download the zip
git clone https://github.com/abhemanyus/egyan.git
cd egyan

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build release version
cargo build --release
```

### Usage Example: Download MBA Materials

Simply replace the the Course URL and rename the folder in the command.

```bash
./target/release/egyan \
  "https://egyankosh.ac.in/handle/123456789/78830" \
  ./mba-materials
```

## Folder Structure

The tool creates this hierarchical structure:
```
output-directory/
└── Semester X/
    └── Course Name/
        └── Block Name/
            ├── Unit 1.pdf
            ├── Unit 2.pdf
            └── ...
```

## Troubleshooting

Possible Issues:
1. **Selector Errors**: Update CSS selectors in `src/main.rs` if website structure changes
2. **Build Errors**: Ensure Rust is properly installed with `rustc --version`
3. **SSL Errors**: Update root certificates on your system
4. **Complete Incompatibility**: In the unlikely scenario that eGyanKosh completely changes thi
