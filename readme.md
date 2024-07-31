![SahomeDB Use Case](https://i.postimg.cc/SR0MJRFF/sahomedb.png)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](/docs/code_of_conduct.md) [![Discord](https://img.shields.io/discord/1182432298382131200?logo=discord&logoColor=%23ffffff&label=Discord&style=for-the-badge)](https://discord.gg/bDhQrkqNdsP4)

# 👋 Meet SahomeDB

SahomeDB is an **embeddable**, **efficient**, and **easy to use** vector database. It is designed to be used as a library and embedded inside your AI application. It is written in Rust and uses [Sled](https://docs.rs/sled) as its persistence storage engine to save vector collections to the disk.

SahomeDB implements **HNSW** (Hierachical Navigable Small World) as its indexing algorithm. It is a state-of-the-art algorithm that is used by many vector databases. It is fast, memory efficient, and it scales well to large datasets.

## Why SahomeDB?

SahomeDB is very flexible for use cases related with vector search such as using RAG (Retrieval-Augmented Generation) method with an LLM to generate a context-aware output. SahomeDB offers 2 major features that make it stand out from other vector databases or libraries:

- **Incremental vector operations**: SahomeDB allows you to add, remove, or modify vectors from the collections without having to rebuild their indexes. This allows for a more flexible and efficient approach on storing your vector data.
- **Flexible persistence options**: You can choose to persist the vector collection to the disk or to keep it in memory. By default, whenever you use a collection, it will be loaded to the memory to ensure that the search performance is high.

🚀 Quickstart

This is a code snippet that you can use as a reference to get started with SahomeDB. In short, use `Collection` to store your vector records or search similar vector and use `Database` to persist a vector collection to the disk.

```rust
// This is a complete, runnable example
use sahomedb::collection::*;
use sahomedb::database::Database;
use sahomedb::vector::*;

fn main() {
    // Vector dimension must be uniform.
    let dimension = 128;

    // Replace with your own data.
    let records = Record::many_random(dimension, 100);
    let query = Vector::random(dimension);

    // Open the database and create a collection.
    let mut db = Database::open("data/readme").unwrap();
    let collection =
        db.create_collection("vectors", None, Some(&records)).unwrap();

    // Search for the nearest neighbors.
    let result = collection.search(&query, 5).unwrap();
    println!("Nearest ID: {}", result[0].id);
}
```

# 🏁 Benchmarks

SahomeDB has a built-in benchmarking suite using Rust's [Criterion](https://docs.rs/criterion) crate that can be used to measure the performance of the vector database.

Currently, the benchmarks are focused on the performance of the collection's vector search functionality. We are working on adding more benchmarks to measure the performance of other operations.

If you are curious and want to run the benchmarks, you can use the following command which will download the benchmarking dataset and run the benchmarks:

```bash
cargo bench
```

# 🤝 Contributing

The easiest way to contribute to this project is to star this project and share it with your friends. This will help us grow the community and make the project more visible to others.

If you want to go further and contribute your expertise, we will gladly welcome your code contributions. For more information and guidance about this, please see [contributing.md](/docs/contributing.md).


If you have deep experience in the space but don't have the free time to contribute codes, we also welcome advices, suggestions, or feature requests. We are also looking for advisors to help guide the project direction and roadmap.

This project is still in the early stages of development. We are actively working on it and we expect the API and functionality to change. We do not recommend using this in production yet.

If you are interested about the project in any way, please join us on [Discord](https://discord.gg/bDhQdfgfrkqNP4). Help us grow the community and make SahomeDB better 😁


## Code of Conduct

We are committed to creating a welcoming community. Any participant in our project is expected to act respectfully and to follow the [Code of Conduct](/docs/code_of_conduct.md).

## Disclaimer

This project is still in the early stages of development. We are actively working on it and we expect the API and functionality to change. We do not recommend using this in production yet.