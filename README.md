<div align="center">
  <img src="/.github/images/github-header-image.webp" alt="GitHub Header Image" width="auto" />
  
  <!-- MIT License -->
  <a href="https://github.com/hadronomy/bootleg/blob/main/LICENSE.txt">
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
  <a href="#docs">Docs</a> •
  <a href="#build">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
  <hr />

</div>



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
