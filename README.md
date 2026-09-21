# Welcome to searchit
searchit is a local search engine for locally stored documentation and files. This tool help get relevant information depending on your search query using the `tf-idf` Algorithm.

Documentations about `tf-idf` can be found there: [tf-idf](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)

# Note
This project is still in development.

# Usage

### 1- Building the project
```
cargo build --release
```

### 2- Scan your local files (only pickup pdfs for now)
```
cargo run --release -- scan <path>
```

### 3- Search locally via the terminal
```
cargo run --release -- search <query>
```

### 4- Serve the webui
```
cargo run --release -- serve
```

