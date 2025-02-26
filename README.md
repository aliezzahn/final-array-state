# Final Array State

This Rust project computes the final state of an array after performing a series of multiplication operations on its elements. It is designed to be efficient and easy to use.

![CI](https://github.com/aliezzahn/final-array-state/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/final-array-state/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Table of Contents

- [Final Array State](#final-array-state)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Example Usage](#example-usage)
    - [Output](#output)
  - [Example](#example)
    - [Input](#input)
    - [Execution](#execution)
    - [Output](#output-1)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)
  - [Contact](#contact)

---

## Introduction

The project provides a function `get_final_state` that takes an array of integers, a number of operations `k`, and a `multiplier`. It performs `k` multiplication operations on the smallest element of the array and returns the final state of the array.

This is particularly useful for scenarios where you need to repeatedly modify the smallest elements of an array and track the changes efficiently.

---

## Features

- **Efficient Operations**: Uses a min-heap to efficiently find and update the smallest elements in the array.
- **Modular Arithmetic**: Ensures that results stay within bounds using modular arithmetic.
- **Easy to Use**: Simple API with clear documentation.

---

## Installation

To use this project, you need to have Rust installed on your system. If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone the repository:

```bash
git clone https://github.com/aliezzahn/final-array-state.git
cd final-array-state
```

Build the project:

```bash
cargo build
```

---

## Usage

The main function is `get_final_state`, which takes the following parameters:

- `nums`: A vector of integers representing the initial state of the array.
- `k`: The number of multiplication operations to perform.
- `multiplier`: The value by which the smallest element is multiplied in each operation.

### Example Usage

```rust
fn main() {
    let nums = vec![2, 1];
    let k = 3;
    let multiplier = 10;

    let result = get_final_state(nums, k, multiplier);
    println!("Final state: {:?}", result);
}
```

### Output

```
Final state: [20, 100]
```

---

## Example

### Input

```rust
let nums = vec![2, 1];
let k = 3;
let multiplier = 10;
```

### Execution

1. Initial array: `[2, 1]`.
2. After 1st operation: `[2, 10]` (multiply `1` by `10`).
3. After 2nd operation: `[20, 10]` (multiply `2` by `10`).
4. After 3rd operation: `[20, 100]` (multiply `10` by `10`).

### Output

```
Final state: [20, 100]
```

---

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them.
4. Push your changes to your fork.
5. Submit a pull request.

Please ensure your code follows the project's coding standards and includes appropriate tests.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Thanks to the Rust community for creating such an amazing programming language.
- Special thanks to everyone who contributed to this project.

---

## Contact

If you have any questions or suggestions, feel free to reach out:

- **GitHub**: [aliezzahn](https://github.com/aliezzahn)
- **Email**: [aliezzahn@gmail.com](mailto:aliezzahn@gmail.com)

---
