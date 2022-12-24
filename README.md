# ascii-art

Convert images to ascii art and write it to text files using Rust 🦀.

## Install from git
Just make sure you have the rust toolchain installed and then run:

```bash
cargo install --git https://github.com/codeaye/ascii_art --profile=release
```

## Install Locally

Clone the project

```bash
  git clone https://github.com/codeaye/ascii_art
```

Go to the project directory

```bash
  cd ascii_art
```

Install the cli globally with cargo

```bash
  make
```

Or if you dont have make, just run
```bash
  cargo install --path . --profile release
```

## Usage
Then run:

```bash
  ascii-art -i=input.png -o=output.txt
```

You can also provide a target width:

```bash
  ascii-art -i=input.png -o=output.txt -t=1024
```

For more help:

```bash
    ascii-art --help
```
