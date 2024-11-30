<div align="center">
  <img src="/.github/images/github-header-image.webp" alt="GitHub Header Image" width="auto" />
  
  <!-- MIT License -->
  <a href="https://github.com/hadronomy/bootleg/blob/main/LICENSE">
    <img
      alt="Content License"
      src="https://img.shields.io/github/license/hadronomy/bootleg?style=for-the-badge&logo=starship&color=ee999f&logoColor=D9E0EE&labelColor=302D41"
    />
  </a>

  <!-- GitHub Repo Stars -->
  <a href="https://github.com/hadronomy/bootleg/stargazers">
    <img
      alt="Stars"
      src="https://img.shields.io/github/stars/hadronomy/bootleg?style=for-the-badge&logo=starship&color=c69ff5&logoColor=D9E0EE&labelColor=302D41"
    />
  </a>
  <p></p>
  <span>
    A simple cli tool to copy everything directly from the terminal.
  </span>
  <p></p>
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
  <hr />

</div>

<div align="center">

<img 
    alt="program example" 
    src="/.github/images/bootleg.gif" 
    width="auto" 
    height="auto"
  />

</div>

</div>

## Installation

Its recommended to install the tool using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall),
a cargo plugin that installs binaries from crates.io.

```bash
cargo binstall bootleg --locked
```

Alternatively, you can install the tool using cargo.

```bash
cargo install bootleg
```

Or with the installation script.

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hadronomy/bootleg/releases/latest/download/bootleg-installer.sh | sh
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
