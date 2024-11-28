# Contributing to Rustic

Thank you for your interest in contributing to Rustic! Contributions of all kinds are welcome, from reporting bugs and suggesting features to submitting pull requests and improving documentation. Follow this guide to understand our process and get started.

---

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
  - [Reporting Issues](#reporting-issues)
  - [Suggesting Features](#suggesting-features)
  - [Improving Documentation](#improving-documentation)
  - [Submitting Code](#submitting-code)
- [Development Workflow](#development-workflow)
  - [Setting Up Your Environment](#setting-up-your-environment)
  - [Running and Testing Rustic](#running-and-testing-rustic)
  - [Submitting Pull Requests](#submitting-pull-requests)
- [Contact Us](#contact-us)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please review our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

---

## Getting Started

If you're new to contributing or need clarification, don’t hesitate to open a discussion or reach out. No contribution is too small!

1. **Check for Open Issues**: Take a look at the [Issues page](https://github.com/BersisSe/Rustic/issues) to see if there's something you'd like to work on.
2. **Open a Discussion**: Start a discussion if you’re unsure about a particular feature or change.
3. **Claim an Issue**: If you find an issue you’d like to work on, leave a comment to let others know you’re on it!

---

## How to Contribute

### Reporting Issues

Found a bug or an unexpected behavior? Open a [new issue](https://github.com/BersisSe/Rustic/issues/new) and provide the following information:
- **Description**: Explain the issue and expected behavior.
- **Steps to Reproduce**: Detail how to reproduce the issue.
- **Environment**: Include OS, Rustic version, and any other relevant details.
- **Logs or Screenshots**: Attach logs or screenshots if possible.

### Suggesting Features

To suggest a new feature, open a [feature request issue](https://github.com/BersisSe/Rustic/issues/new?template=feature_request.md). Please describe:
- **Problem**: The problem the feature would solve.
- **Proposed Solution**: Briefly outline how you think the feature could be implemented.
- **Additional Context**: Any other details that may be helpful.

### Improving Documentation

Good documentation is crucial for Rustic’s usability. If you see unclear instructions or missing information, feel free to improve it! Simply fork the repo, make your changes in the `docs` folder, and submit a pull request.

### Submitting Code

We welcome code contributions! Before you start working on any significant change, it’s a good idea to discuss it with us. See [Development Workflow](#development-workflow) for more information on how to submit code changes.

---

## Development Workflow

### Setting Up Your Environment

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/BersisSe/Rustic/rustic.git
   cd rustic
   ```
   
2. **Install Rust** (if you haven’t already):  
   Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

3. **Install Dependencies**:
   Rustic relies on a few crates like `pulldown-cmark` and `tera`. Run the following command to install dependencies:
   ```bash
   cargo build
   ```

### Running and Testing Rustic

1. **Run Rustic**:
   ```bash
   cargo run -- <command>
   ```
   
2. **Testing**:  
   All tests are in the `tests` folder. Run tests with:
   ```bash
   cargo test
   ```
   
3. **Manual Testing**:  
   After building, run commands like `rustic init` or `rustic build` to manually test features.

### Submitting Pull Requests

Once you’re ready to submit your code:

1. **Fork and Branch**:
   - Fork the repository and create a branch specific to your feature or fix.
   - Branch naming convention: `feature/feature-name` or `fix/issue-name`.
   
2. **Commit Messages**:  
   Write meaningful commit messages that explain the purpose of each change.
   
3. **Create a Pull Request**:
   - Push your branch to your forked repository.
   - Go to the Rustic repository and create a pull request.
   - Describe your changes and mention the related issue number (if applicable).

4. **Review and Feedback**:  
   Be ready to discuss and make updates based on feedback.

---

## Contact Us

For questions or further discussion, reach out via:
- **GitHub Discussions**: Join our community discussions on GitHub.
- **Issues**: Open an issue if you need help or guidance on a specific contribution.

---

Thank you for contributing to Rustic! We look forward to your feedback and ideas, which will help Rustic grow and improve.