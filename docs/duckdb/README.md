# DuckDB Documentation

A comprehensive guide to using DuckDB for analytical queries.

## Table of Contents

- [Overview](overview.md) - Introduction to DuckDB and its key features
- [Commands](commands.md) - Common DuckDB commands and syntax
- [Data Types & Functions](data_types_functions.md) - Reference for data types and built-in functions
- [Connections & Integrations](connections_integrations.md) - Working with DuckDB across languages and systems
- [CLI Output Modes](cli_modes.md) - Different output formats for CLI results

## About DuckDB

DuckDB is an embeddable SQL OLAP database management system designed for analytical queries on local datasets. It's a feature-rich, fast, and efficient in-process database with columnar storage and vectorized execution.

## Quick Start

```bash
# Installation
pip install duckdb  # Python
# OR
brew install duckdb  # macOS

# Start CLI
duckdb

# Basic commands
CREATE TABLE items(id INTEGER, name VARCHAR);
INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2');
SELECT * FROM items;

# Try different output modes
.mode markdown
SELECT * FROM items;
```

## Additional Resources

- [Official DuckDB Website](https://duckdb.org/)
- [GitHub Repository](https://github.com/duckdb/duckdb)
- [Online Documentation](https://duckdb.org/docs/)