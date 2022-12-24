# ascii-art

Convert images to ascii art and write it to text files using Rust 🦀.

## Run Locally

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

Then get some images.
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
