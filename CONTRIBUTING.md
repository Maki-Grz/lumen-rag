# Contributing to lumen-rag

First off, thank you for considering contributing! 🎉 Your help is what makes this project better for everyone.

We welcome contributions of all kinds: bug reports, feature requests, code improvements, documentation fixes, or examples.

## Table of Contents

* [How to Contribute](#how-to-contribute)
* [Code of Conduct](#code-of-conduct)
* [Setting Up the Development Environment](#setting-up-the-development-environment)
* [Making Changes](#making-changes)
* [Submitting Pull Requests](#submitting-pull-requests)
* [Reporting Issues](#reporting-issues)

---

## How to Contribute

There are several ways you can contribute:

* **Report bugs or issues**: Use the [issues page](https://github.com/Maki-Grz/lumen-rag/issues).
* **Suggest new features**: Open an issue or submit a pull request with your proposal.
* **Improve documentation**: Fix typos, add examples, or clarify explanations.
* **Fix bugs or implement features**: Fork the repository and submit a pull request.

---

## Code of Conduct

By participating, you agree to follow the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Please be respectful, inclusive, and collaborative.

---

## Setting Up the Development Environment

1. **Clone the repository:**

```bash
git clone https://github.com/Maki-Grz/lumen-rag.git
cd lumen-rag
```

2. **Install Rust (if not installed):**
   [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. **Run tests to make sure everything works:**

```bash
cargo test
```

---

## Making Changes

1. Create a new branch for your changes:

```bash
git checkout -b feat/my-new-feature
```

2. Make your changes in the code or documentation.

3. Use **Conventional Commits** for your commit messages. This project uses [Release Please](https://github.com/googleapis/release-please) to automate releases.

   Common prefixes:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `style:` for formatting changes
   - `refactor:` for code changes that neither fix a bug nor add a feature
   - `perf:` for performance improvements
   - `test:` for adding missing tests or correcting existing tests
   - `chore:` for updating build tasks, package manager configs, etc.

   Example: `feat: add support for Redis vector store`

4. Ensure your code is formatted and passes clippy checks:

```bash
cargo fmt
cargo clippy
```

4. Run tests locally:

```bash
cargo test
```

---

## Submitting Pull Requests

1. Push your branch to your fork:

```bash
git push origin feature/my-new-feature
```

2. Open a pull request on the main repository.

3. Include a clear description of what your PR does and why. Reference any related issues.

4. Be responsive to review comments so we can merge faster.

---

## Reporting Issues

When reporting a bug, please include:

* A clear **description** of the problem
* Steps to **reproduce** the issue
* **Expected vs actual behavior**
* Any relevant **logs or error messages**

Use [GitHub issues](https://github.com/Maki-Grz/lumen-rag/issues) for reporting.

---

## Tips for Contributors

* Keep pull requests **focused** and **atomic** (one feature/bug per PR).
* Write **descriptive commit messages**.
* Test your changes before submitting.

---

Thank you for helping improve **lumen-rag**! Your contributions make a difference. 🚀
