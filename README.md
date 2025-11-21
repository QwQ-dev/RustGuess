# RustGuess: A Starter Guessing Game with Persistence

**RustGuess** is a command-line guessing game that provides a **very interesting example** for **new Rust developers**
or those **transitioning from other languages**.

If you've completed the basic syntax lessons, this project is designed to be your **next step**—a practical, working
application to clone and study.

---

## ✨ What You Will Learn

This small project demonstrates **real-world Rust application structure** by tackling several key concepts:

1. **Modular Design:** Code is cleanly separated into modules (e.g., `user_data`, `data_handler`).
2. **Data Persistence:** Reading and writing user statistics to JSON files using `serde` and `std::fs`.
3. **Advanced Error Handling:** A robust error system using custom enums (`GameError`) and dynamic boxed errors (
   `BoxedError`) for clean error propagation.
4. **Idiomatic I/O:** Using the `?` operator and `Result` for safe input handling.
5. **Asynchronous Programming:** Introduction to `async`/`await` and `tokio` for non-blocking operations.
6. **Web Integration & Testing:** Basic setup for a local Web API and writing tests with mock inputs.

---

## 🚀 Getting Started

You need the Rust toolchain installed.

1. **Clone & Run:**
   ```bash
   git clone https://github.com/QwQ-dev/RustGuess.git
   cd RustGuess
   cargo run
   ```

2. **Usage:**
   The game will prompt you for a username, load or create your stats, and then begin the guessing loop. Type `exit`,
   `quit`, or `end` to quit the game gracefully.

**Further examine the code; this may be quite useful for newcomers diving into Rust!**