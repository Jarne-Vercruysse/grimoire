# 🦀 Graduaatsproef

This is a web project built with **Leptos**, a full-stack, isomorphic web framework for Rust. Leptos allows you to build fast, reactive web applications using a component-based model.

---

## 🚀 Getting Started

To get this project up and running, you'll need to have Rust installed and some specific components for web development with Leptos.

### 📋 Prerequisites

1. **Install Rust:** If you don't have Rust installed, follow the instructions on the official Rust website:
   [Install Rust](https://www.rust-lang.org/tools/install)

2. **Install `cargo-leptos`:** This is the essential CLI tool for developing Leptos applications. Install it using Cargo:

   ```bash
   cargo install cargo-leptos --locked
   ```

3. **Rust Toolchain Configuration:**

   - **Nightly Toolchain:** Leptos often leverages features only available in Rust's nightly builds. Ensure you have the `nightly` toolchain installed:

     ```bash
     rustup toolchain install nightly --allow-downgrade
     ```

   - **WebAssembly Target:** To compile your Rust code for the browser, you need the WebAssembly target:

     ```bash
     rustup target add wasm32-unknown-unknown
     ```

---

## ▶️ Running the Project

Once you have the prerequisites set up, you can run your Leptos project with a single command:

```bash
cargo leptos watch
```

## Academic Contributions

This project was developed as part of a **graduaatsproef** at **HOGent**. The following documents provide a deeper dive into the theoretical background, methodology, and results of this project:

- **Paper:** [Paper](academic/paper.pdf)
- **Project Poster:** [Poster](academic/poster.pdf)

Please refer to these documents for a comprehensive understanding of the project's academic context and contributions.
