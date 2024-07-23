![Sahome](https://i.postimg.cc/X7rGVsBb/banner.png)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](/docs/code_of_conduct.md) [![Discord](https://img.shields.io/discord/1182432298382131200?logo=discord&logoColor=%23ffffff&label=Discord&style=for-the-badge)](https://discord.gg/bDhQrkqNdsP4)

# 👋 Meet SahomeDB

SahomeDB is an **embeddable**, **efficient**, and **easy to use** vector database. It is designed to be used as a library and embedded inside your AI application. It is written in Rust and uses [Sled](https://docs.rs/sled) as its persistence storage engine to save vector collections to the disk.

SahomeDB implements **HNSW** (Hierachical Navigable Small World) as its indexing algorithm. It is a state-of-the-art algorithm that is used by many vector databases. It is fast, memory efficient, and it scales well to large datasets.

## Why SahomeDB?

SahomeDB is very flexible for use cases related with vector search such as using RAG (Retrieval-Augmented Generation) method with an LLM to generate a context-aware output. SahomeDB offers 2 major features that make it stand out from other vector databases or libraries:

- **Incremental operations on the collection index**: You can add, remove, or modify vectors from the index without having to rebuild it.
- **Flexible persistence options**: You can choose to persist the collection to disk or to keep it in memory.

🚀 Quickstart

This is a code snippet that you can use as a reference to get started with SahomeDB. In short, use `Collection` to store your vector records or search similar vector and use `Database` to persist a vector collection to the disk.

```rust
// This is a complete, runnable example
use sahomedb::database::Database;
use sahomedb::collection::*;
use sahomedb::vector::*;
use rand::random; // Utility

fn main() {
    // Utility functions to generate random vector records.
    let records = gen_records::<128>(100);

    // Open the database and create a collection.
    let mut db = Database::open("data/readme").unwrap();
    let collection: Collection<usize, 128, 32> =
        db.create_collection("vectors", None, Some(&records)).unwrap();

    // Utility function to generate a random vector.
    let query = gen_vector::<128>();
    let result = collection.search(&query, 5).unwrap();

    println!("Nearest neighbor ID: {}", result[0].id);
}

fn gen_records<const N: usize>(len: usize) -> Vec<Record<usize, N>> {
    let mut records = Vec::with_capacity(len);

    for _ in 0..len {
        let vector = gen_vector::<N>();
        let data = random::<usize>();
        records.push(Record { vector, data });
    }

    records
}

fn gen_vector<const N: usize>() -> Vector<N> {
    let mut vec = [0.0; N];

    for float in vec.iter_mut() {
        *float = random::<f32>();
    }

    Vector(vec)
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

We welcome contributions from the community. Please see [contributing.md](/docs/contributing.md) for more information.

## Disclaimer

This project is still in the early stages of development. We are actively working on it and we expect the API and functionality to change. We do not recommend using this in production yet.

## Code of Conduct

We are committed to creating a welcoming community. Any participant in our project is expected to act respectfully and to follow the [Code of Conduct](/docs/code_of_conduct.md).
