# DuckDB Connections and Integrations

DuckDB offers a wide range of connection options and integrations with various programming languages, tools, and database systems. This document provides an overview of the available connection methods and integrations.

- [DuckDB Connections and Integrations](#duckdb-connections-and-integrations)
  - [Command-line Interface (CLI)](#command-line-interface-cli)
    - [CLI Commands](#cli-commands)
  - [Programming Language Clients](#programming-language-clients)
    - [Python](#python)
      - [Installation](#installation)
      - [Basic Usage](#basic-usage)
      - [Pandas Integration](#pandas-integration)
      - [Working with Parquet, CSV, and JSON Files](#working-with-parquet-csv-and-json-files)
    - [R](#r)
      - [Installation](#installation-1)
      - [Basic Usage](#basic-usage-1)
      - [Data.frame Integration](#dataframe-integration)
    - [Java](#java)
      - [Maven Dependency](#maven-dependency)
      - [Basic Usage](#basic-usage-2)
    - [C/C++](#cc)
      - [Basic C++ Example](#basic-c-example)
    - [Node.js](#nodejs)
      - [Installation](#installation-2)
      - [Basic Usage](#basic-usage-3)
    - [Rust](#rust)
      - [Cargo.toml](#cargotoml)
      - [Basic Usage](#basic-usage-4)
  - [Database Integrations](#database-integrations)
    - [SQLite](#sqlite)
    - [PostgreSQL](#postgresql)
    - [MySQL/MariaDB](#mysqlmariadb)
    - [Apache Arrow](#apache-arrow)
  - [File Format Integrations](#file-format-integrations)
    - [Parquet](#parquet)
    - [CSV](#csv)
    - [JSON](#json)
    - [Excel](#excel)
  - [Web Integrations](#web-integrations)
    - [HTTP](#http)
    - [S3](#s3)
  - [BI Tool Integrations](#bi-tool-integrations)
    - [ODBC/JDBC](#odbcjdbc)
      - [ODBC Connection String](#odbc-connection-string)
      - [JDBC Connection URL](#jdbc-connection-url)
    - [Tableau](#tableau)
    - [Power BI](#power-bi)
  - [Extensions](#extensions)
    - [Installing and Loading Extensions](#installing-and-loading-extensions)
    - [Commonly Used Extensions](#commonly-used-extensions)
  - [Advanced Configurations](#advanced-configurations)
    - [Memory and Execution Parameters](#memory-and-execution-parameters)
    - [Temporary Tables and Views](#temporary-tables-and-views)
    - [Transaction Control](#transaction-control)
  - [Performance Optimization Tips](#performance-optimization-tips)
  - [Conclusion](#conclusion)

## Command-line Interface (CLI)

The DuckDB CLI is the simplest way to interact with DuckDB directly.

```bash
# Start with in-memory database
duckdb

# Start with a new or existing database file
duckdb mydb.db

# Execute commands from a SQL file
duckdb mydb.db -c ".read script.sql"

# Execute a single SQL command
duckdb mydb.db -c "SELECT * FROM mytable"

# Execute SQL and output to a file
duckdb mydb.db -c "SELECT * FROM mytable" -output result.csv
```

### CLI Commands

Inside the CLI, you can use special commands:

```sql
-- List tables
.tables

-- Show schema of a table
.schema tablename

-- Execute SQL from a file
.read script.sql

-- Output query results to a file
.output result.csv
.mode csv
SELECT * FROM mytable;
.output

-- Change output mode
.mode csv|tsv|column|markdown|table

-- Show help
.help
```

## Programming Language Clients

### Python

DuckDB has excellent Python integration and can be used with popular libraries like pandas and PyArrow.

#### Installation

```bash
pip install duckdb
```

#### Basic Usage

```python
import duckdb

# Connect to an in-memory database (default)
con = duckdb.connect()

# Connect to a file database
# con = duckdb.connect('mydb.db')

# Execute SQL queries
con.execute("CREATE TABLE items(id INTEGER, name VARCHAR)")
con.execute("INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')")

# Fetch results
result = con.execute("SELECT * FROM items").fetchall()
print(result)  # [(1, 'Item 1'), (2, 'Item 2')]

# Fetch as DataFrame
df = con.execute("SELECT * FROM items").df()
print(df)
```

#### Pandas Integration

```python
import duckdb
import pandas as pd

# Create a pandas DataFrame
df = pd.DataFrame({
    'id': [1, 2, 3],
    'name': ['Item 1', 'Item 2', 'Item 3']
})

# Query the DataFrame directly
result = duckdb.query_df(df, "SELECT * FROM df WHERE id > 1").df()
print(result)

# Register a DataFrame as a virtual table
con = duckdb.connect()
con.register('my_df', df)
result = con.execute("SELECT * FROM my_df WHERE id > 1").df()
print(result)

# Convert query results to DataFrame
result_df = con.execute("SELECT * FROM my_df").df()
```

#### Working with Parquet, CSV, and JSON Files

```python
import duckdb

con = duckdb.connect()

# Query Parquet files directly
result = con.execute("SELECT * FROM 'data.parquet'").df()

# Query CSV files directly
result = con.execute("SELECT * FROM 'data.csv'").df()

# Query JSON files directly
result = con.execute("SELECT * FROM 'data.json'").df()

# Query multiple files
result = con.execute("SELECT * FROM 'data/*.parquet'").df()

# Use SQL to transform data and write to a different format
con.execute("COPY (SELECT * FROM 'data.csv' WHERE id > 100) TO 'filtered_data.parquet'")
```

### R

DuckDB can be used in R with the `duckdb` package.

#### Installation

```r
install.packages("duckdb")
```

#### Basic Usage

```r
library(duckdb)

# Connect to an in-memory database (default)
con <- dbConnect(duckdb())

# Connect to a file database
# con <- dbConnect(duckdb(), "mydb.db")

# Execute SQL queries
dbExecute(con, "CREATE TABLE items(id INTEGER, name VARCHAR)")
dbExecute(con, "INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')")

# Fetch results
result <- dbGetQuery(con, "SELECT * FROM items")
print(result)

# Clean up
dbDisconnect(con)
```

#### Data.frame Integration

```r
library(duckdb)
library(DBI)

# Create a data.frame
df <- data.frame(
  id = 1:3,
  name = c("Item 1", "Item 2", "Item 3"),
  stringsAsFactors = FALSE
)

# Connect to DuckDB
con <- dbConnect(duckdb())

# Write data.frame to DuckDB table
dbWriteTable(con, "items", df)

# Query the table
result <- dbGetQuery(con, "SELECT * FROM items WHERE id > 1")
print(result)

# Clean up
dbDisconnect(con)
```

### Java

DuckDB provides a JDBC driver for Java integration.

#### Maven Dependency

```xml
<dependency>
    <groupId>org.duckdb</groupId>
    <artifactId>duckdb_jdbc</artifactId>
    <version>0.9.2</version>
</dependency>
```

#### Basic Usage

```java
import java.sql.*;

public class DuckDBExample {
    public static void main(String[] args) {
        try {
            // Load the DuckDB JDBC driver
            Class.forName("org.duckdb.DuckDBDriver");
            
            // Connect to an in-memory database
            Connection conn = DriverManager.getConnection("jdbc:duckdb:");
            
            // Connect to a file database
            // Connection conn = DriverManager.getConnection("jdbc:duckdb:mydb.db");
            
            // Create a table
            Statement stmt = conn.createStatement();
            stmt.execute("CREATE TABLE items(id INTEGER, name VARCHAR)");
            stmt.execute("INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')");
            
            // Query data
            ResultSet rs = stmt.executeQuery("SELECT * FROM items");
            while (rs.next()) {
                int id = rs.getInt("id");
                String name = rs.getString("name");
                System.out.println(id + ": " + name);
            }
            
            // Clean up
            rs.close();
            stmt.close();
            conn.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

### C/C++

DuckDB was originally written in C++ and provides C and C++ APIs.

#### Basic C++ Example

```cpp
#include "duckdb.hpp"

using namespace duckdb;

int main() {
    // Connect to an in-memory database
    DuckDB db(nullptr);
    Connection con(db);
    
    // Create a table
    con.Query("CREATE TABLE items(id INTEGER, name VARCHAR)");
    con.Query("INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')");
    
    // Query data
    auto result = con.Query("SELECT * FROM items");
    result->Print();
    
    return 0;
}
```

### Node.js

DuckDB can be used with Node.js via the `node-duckdb` package.

#### Installation

```bash
npm install node-duckdb
```

#### Basic Usage

```javascript
const duckdb = require('node-duckdb');

// Create a new database in memory
const db = new duckdb.Database(':memory:');

// Create a connection to the database
const conn = new duckdb.Connection(db);

// Execute queries
conn.executeSync('CREATE TABLE items(id INTEGER, name VARCHAR)');
conn.executeSync("INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')");

// Query data
const result = conn.executeSync('SELECT * FROM items');
console.log(result);

// Clean up
conn.close();
db.close();
```

### Rust

DuckDB provides a Rust crate for integration.

#### Cargo.toml

```toml
[dependencies]
duckdb = "0.9.0"
```

#### Basic Usage

```rust
use duckdb::{Connection, Result, params};

fn main() -> Result<()> {
    // Connect to an in-memory database
    let conn = Connection::open_in_memory()?;
    
    // Create a table
    conn.execute("CREATE TABLE items(id INTEGER, name VARCHAR)", [])?;
    conn.execute("INSERT INTO items VALUES (1, 'Item 1'), (2, 'Item 2')", [])?;
    
    // Query data
    let mut stmt = conn.prepare("SELECT * FROM items")?;
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(format!("{}: {}", id, name))
    })?;
    
    for item in rows {
        println!("{}", item?);
    }
    
    Ok(())
}
```

## Database Integrations

### SQLite

DuckDB can read from and write to SQLite databases directly.

```sql
-- Attach a SQLite database
ATTACH 'data.sqlite' AS sqlite_db;

-- Query SQLite tables
SELECT * FROM sqlite_db.table_name;

-- Copy data from SQLite to DuckDB
CREATE TABLE duckdb_table AS SELECT * FROM sqlite_db.sqlite_table;

-- Write data from DuckDB to SQLite
CREATE TABLE sqlite_db.new_table AS SELECT * FROM my_duckdb_table;
```

### PostgreSQL

DuckDB can connect to and query PostgreSQL databases using the `postgres` extension.

```sql
-- Install and load the postgres extension
INSTALL postgres;
LOAD postgres;

-- Create a connection to PostgreSQL
CALL postgres_attach('host=localhost port=5432 dbname=mydb user=postgres password=password', 'pg_db');

-- Query PostgreSQL tables
SELECT * FROM pg_db.table_name;

-- Copy data from PostgreSQL to DuckDB
CREATE TABLE duckdb_table AS SELECT * FROM pg_db.pg_table;
```

### MySQL/MariaDB

DuckDB can connect to and query MySQL/MariaDB databases using the `mysql` extension.

```sql
-- Install and load the mysql extension
INSTALL mysql;
LOAD mysql;

-- Create a connection to MySQL
CALL mysql_attach('host=localhost port=3306 dbname=mydb user=root password=password', 'mysql_db');

-- Query MySQL tables
SELECT * FROM mysql_db.table_name;

-- Copy data from MySQL to DuckDB
CREATE TABLE duckdb_table AS SELECT * FROM mysql_db.mysql_table;
```

### Apache Arrow

DuckDB has native integration with Apache Arrow, allowing for zero-copy data exchange.

```python
import duckdb
import pyarrow as pa

# Create an Arrow Table
data = [
    pa.array([1, 2, 3]),
    pa.array(['Item 1', 'Item 2', 'Item 3'])
]
table = pa.Table.from_arrays(data, names=['id', 'name'])

# Query the Arrow Table directly with DuckDB
con = duckdb.connect()
con.register('arrow_table', table)
result = con.execute("SELECT * FROM arrow_table WHERE id > 1").df()
print(result)

# Convert DuckDB result to Arrow Table
arrow_result = con.execute("SELECT * FROM arrow_table").arrow()
```

## File Format Integrations

### Parquet

DuckDB has native support for reading and writing Parquet files.

```sql
-- Query Parquet files directly
SELECT * FROM 'data.parquet';
SELECT * FROM 'data/*.parquet';

-- Write query results to Parquet
COPY (SELECT * FROM my_table) TO 'output.parquet';

-- Create a table from a Parquet file
CREATE TABLE my_table AS SELECT * FROM 'data.parquet';

-- Export a table to Parquet
COPY my_table TO 'output.parquet';
```

### CSV

DuckDB can read and write CSV files with various options.

```sql
-- Read a CSV file
SELECT * FROM 'data.csv';

-- Read a CSV file with options
SELECT * FROM read_csv('data.csv', 
    delimiter=',', 
    header=true, 
    auto_detect=true,
    sample_size=1000,
    quote='"',
    escape='\\'
);

-- Write to a CSV file
COPY (SELECT * FROM my_table) TO 'output.csv' (HEADER, DELIMITER ',');

-- Create a table from a CSV file
CREATE TABLE my_table AS SELECT * FROM 'data.csv';
```

### JSON

DuckDB can read and write JSON files.

```sql
-- Read a JSON file
SELECT * FROM 'data.json';

-- Read a JSON file with options
SELECT * FROM read_json('data.json', auto_detect=true);

-- Read a JSON file with a specific structure
SELECT * FROM read_json('data.json', format='array', columns={id: 'INTEGER', name: 'VARCHAR'});

-- Write to a JSON file
COPY (SELECT * FROM my_table) TO 'output.json';
```

### Excel

DuckDB can read Excel files using the `spatial` extension.

```sql
-- Install and load the spatial extension (includes Excel support)
INSTALL spatial;
LOAD spatial;

-- Read an Excel file
SELECT * FROM st_read_excel('data.xlsx');

-- Read a specific sheet
SELECT * FROM st_read_excel('data.xlsx', sheet=2);
```

## Web Integrations

### HTTP

DuckDB can query remote files over HTTP/HTTPS.

```sql
-- Query a remote Parquet file
SELECT * FROM 'https://example.com/data.parquet';

-- Query a remote CSV file
SELECT * FROM 'https://example.com/data.csv';
```

### S3

DuckDB can access files in Amazon S3 using the `httpfs` extension.

```sql
-- Install and load the httpfs extension
INSTALL httpfs;
LOAD httpfs;

-- Configure S3 credentials
SET s3_region='us-east-1';
SET s3_access_key_id='your-access-key';
SET s3_secret_access_key='your-secret-key';

-- Query a file from S3
SELECT * FROM 's3://my-bucket/data.parquet';
```

## BI Tool Integrations

### ODBC/JDBC

DuckDB provides ODBC and JDBC drivers, which can be used to connect DuckDB to various BI tools.

#### ODBC Connection String

```
Driver={DuckDB Driver};Database=mydb.db;
```

#### JDBC Connection URL

```
jdbc:duckdb:mydb.db
```

### Tableau

Tableau can connect to DuckDB using the ODBC driver.

1. Install the DuckDB ODBC driver
2. In Tableau, select "Other Databases (ODBC)"
3. Use the DuckDB ODBC connection string

### Power BI

Power BI can connect to DuckDB using the ODBC driver.

1. Install the DuckDB ODBC driver
2. In Power BI, select "Get Data" > "ODBC"
3. Use the DuckDB ODBC connection string

## Extensions

DuckDB has a rich extension ecosystem that provides additional functionality.

### Installing and Loading Extensions

```sql
-- List available extensions
SELECT * FROM duckdb_extensions();

-- Install an extension
INSTALL extension_name;

-- Load an extension
LOAD extension_name;
```

### Commonly Used Extensions

- **httpfs**: Enables access to remote files (S3, HTTP, etc.)
- **spatial**: Provides spatial data types and functions (including GIS functionality)
- **postgres**: Allows connecting to PostgreSQL databases
- **mysql**: Allows connecting to MySQL/MariaDB databases
- **sqlite**: Enhanced SQLite integration
- **json**: Enhanced JSON functionality
- **icu**: International Components for Unicode (advanced string functions)
- **excel**: Access to Excel files
- **fts**: Full-text search functionality
- **parquet**: Enhanced Parquet functionality
- **tpch**: TPC-H benchmark data generator
- **tpcds**: TPC-DS benchmark data generator

## Advanced Configurations

### Memory and Execution Parameters

```sql
-- Set memory limit
SET memory_limit='4GB';

-- Set thread count for parallel execution
SET threads=8;

-- Enable/disable profiling
SET enable_profiling=true;
SET profiling_output='profile.json';
```

### Temporary Tables and Views

```sql
-- Create a temporary table (exists only in the current connection)
CREATE TEMPORARY TABLE temp_data AS SELECT * FROM my_table;

-- Create a temporary view
CREATE TEMPORARY VIEW temp_view AS SELECT * FROM my_table WHERE id > 100;
```

### Transaction Control

```sql
-- Start a transaction
BEGIN TRANSACTION;

-- Commit changes
COMMIT;

-- Rollback changes
ROLLBACK;

-- Set transaction characteristics
SET autocommit=false;
```

## Performance Optimization Tips

1. **Use appropriate data types**: Choose the smallest data type that fits your needs.

2. **Create indexes for frequently queried columns**:
   ```sql
   CREATE INDEX idx_customer_id ON orders(customer_id);
   ```

3. **Use partitioned queries for large datasets**:
   ```sql
   SELECT * FROM 'data_*.parquet';
   ```

4. **Adjust memory settings for large operations**:
   ```sql
   SET memory_limit='16GB';
   ```

5. **Leverage parallel execution**:
   ```sql
   SET threads=12;
   ```

6. **Use columnar file formats** (Parquet) for large datasets.

7. **Push predicates down to data sources** when possible.

8. **Use EXPLAIN to understand query execution**:
   ```sql
   EXPLAIN SELECT * FROM orders WHERE customer_id = 100;
   ```

9. **For large imports, use COPY with appropriate options**:
   ```sql
   COPY large_table FROM 'data.csv' (DELIMITER ',', HEADER, PARALLEL true);
   ```

10. **Consider the appropriate index types** for your query patterns.

## Conclusion

DuckDB offers a comprehensive set of connection methods, integrations, and extensions that make it highly versatile for various use cases. Whether you're using DuckDB from a programming language like Python or R, connecting to external databases, or working with files in different formats, DuckDB provides efficient and convenient options for data processing and analysis.

For more information about specific integrations and extensions, refer to the [official DuckDB documentation](https://duckdb.org/docs/) and the [DuckDB extension repository](https://github.com/duckdb/duckdb/tree/master/extension).