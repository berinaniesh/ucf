# UCF Formatter Tests

This directory contains test files and a script to verify that UCF formatters are working correctly.

## Directory Structure

```
test/
├── README.md              # This file
├── run_tests.sh           # Test runner script
├── ucf-test-ignore.toml   # Config to exclude test files (optional)
└── samples/               # Sample files with intentionally bad formatting
    ├── sample.c           # C (clang-format)
    ├── sample.cmake       # CMake (cmake-format)
    ├── sample.css         # CSS (prettier)
    ├── sample.go          # Go (gofmt)
    ├── sample.hs          # Haskell (stylish-haskell)
    ├── sample.html        # HTML (prettier)
    ├── sample.js          # JavaScript (prettier)
    ├── sample.json        # JSON (clang-format)
    ├── sample.lua         # Lua (stylua)
    ├── sample.py          # Python (black)
    ├── sample.rs          # Rust (rustfmt)
    ├── sample.sh          # Shell (shfmt)
    ├── sample.tex         # LaTeX (tex-fmt)
    ├── sample.tf          # Terraform (terraform fmt)
    ├── sample.toml        # TOML (taplo)
    ├── sample.ts          # TypeScript (prettier)
    ├── sample.xml         # XML (xmllint)
    ├── sample.yaml        # YAML (prettier)
    └── sample.zig         # Zig (zig fmt)
```

## Running Tests

```bash
./test/run_tests.sh
```

The test script will:
1. Build UCF in release mode
2. Check which formatters are installed on your system
3. Copy sample files to a temporary directory
4. Run UCF on each file
5. Verify that formatting changes were applied
6. Report results (PASS/FAIL/SKIP)

Formatters that are not installed will be skipped.

## Protecting Sample Files

The sample files contain intentionally bad formatting. To prevent them from being accidentally formatted by UCF, you can:

### Option 1: Don't run UCF on the test directory
Simply avoid running `ucf test/samples/*` manually.

### Option 2: Use the ignore config (not recommended for general use)
```bash
cp test/ucf-test-ignore.toml ~/.config/ucf/config.toml
```

**Warning:** This will prevent UCF from formatting ANY files with these extensions system-wide. Only use this temporarily if needed.

## Resetting Sample Files

If sample files get accidentally formatted, reset them with git:
```bash
git checkout test/samples/
```
