<div align="center">

# Bootleg

Bootleg is a simple cli tool to copy to the clipboard.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/hadronomy/bootleg/blob/main/LICENSE)

![the help flag output](.github/images/help.png)

</div>

## Installation

```bash
cargo install bootleg --locked
```

## Usage

Copy written text to the clipboard.

```bash
bootleg "Hello, World!"
```

Copy output of a command to the clipboard.

```bash
echo "Hello, World!" | bootleg
```

Copy the content of a file to the clipboard.

```bash
cat file.txt | bootleg
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) for details.
