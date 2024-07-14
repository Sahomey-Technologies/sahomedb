# Contributing to SahomeDB

First of all, thank you for considering contributing to SahomeDB! We welcome contributions from the community, and this document outlines the process for contributing to our project.

## Code of Conduct

We are committed to creating a welcoming community. Any participant in our project is expected to act respectfully and to follow the [Code of Conduct](code_of_conduct.md).

## Have questions or suggestions?

## Encounter a bug? Have a feature request?

If you encounter a bug or have a feature request, please open an issue on [GitHub](https://github.com/sahomedb/sahomedb/issues).

Please include as much information as possible in your issue. This includes:

- A description of the bug or feature request.
- If it's a bug, steps to reproduce the bug. If it's a feature request, include the use case and expected behavior of the feature.
- Screenshots or screen recording, if applicable.

## Want to contribute code?

Before you start working on a pull request, we encourage you to check out the existing issues and pull requests to make sure that someone else isn't already working on the same thing. After all, we don't want you to waste your time!

We try to prioritize features and bug fixes that are requested by the community. If you want to work on a feature or bug fix that isn't already in the issue tracker, please open an issue first to discuss it with the community.

For features, we try to prioritize features that are backed by real-world use cases. If you have a use case for a feature, please include it in the issue.

# Getting started

Getting started with SahomeDB development is easy.

You will need to have Rust installed. We recommend using [rustup](https://www.rust-lang.org/tools/install) to install Rust. We also recommend having rust-analyzer installed for your editor.

To run SahomeDB locally, clone the repository and add `.env` file to the root of the project (refer to `.env.example`). After that, you can run the following command to start SahomeDB:

```bash
cargo run
```

By default, this will start SahomeDB on port 3141. You can change this by setting the `SAHOMEDB_PORT` environment variable.

Then, you can use any HTTP client to interact with SahomeDB like curl or Postman.

## Style guide

We use mostly the default linting and style guide for Rust except for some linting changes listed in [rustfmt.toml](rustfmt.toml) file. For more information, see the [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/index.html).

For commit messages, we use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format. This allows us to maintain consistency and readability in our commit messages.

## Submitting a pull request

Once you have made your changes, you can submit a pull request. We will review your pull request and provide feedback. If your pull request is accepted, we will merge it into the main branch.

For organization purposes, we ask that you use the following format for your pull request title in lowercase:

```
<type>: <description>
```

For example:

```
feat: add support ...
fix: fix issue ...
```

This is similar to the format used by [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## Conclusion

Thank you for taking the time to read this document. We look forward to your contributions!

If you want to support us, star this project, share it with your circles.

Best regards,<br />
Elijah Samson