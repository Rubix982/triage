# DuckDB Commands Reference

This document provides an overview of common DuckDB commands and their usage. DuckDB follows the SQL standard with some extensions specific to analytical workloads.

- [DuckDB Commands Reference](#duckdb-commands-reference)
  - [Database Connection and Management](#database-connection-and-management)
    - [Starting DuckDB CLI](#starting-duckdb-cli)
    - [Attaching/Detaching Databases](#attachingdetaching-databases)
    - [Database Information](#database-information)
  - [Table Operations](#table-operations)
    - [Creating Tables](#creating-tables)
    - [Altering Tables](#altering-tables)
    - [Dropping Tables](#dropping-tables)
  - [Data Manipulation](#data-manipulation)
    - [Inserting Data](#inserting-data)
    - [Updating Data](#updating-data)
    - [Deleting Data](#deleting-data)
  - [Querying Data](#querying-data)
    - [Basic SELECT](#basic-select)
    - [Filtering](#filtering)
    - [Aggregation](#aggregation)
    - [Joins](#joins)
    - [Subqueries](#subqueries)
    - [Common Table Expressions (CTEs)](#common-table-expressions-ctes)
  - [Transaction Control](#transaction-control)
  - [Data Import and Export](#data-import-and-export)
    - [Importing Data](#importing-data)
    - [Exporting Data](#exporting-data)
  - [Views](#views)
  - [Indexes](#indexes)
  - [Window Functions](#window-functions)
  - [Schema Management](#schema-management)
  - [Useful DuckDB-Specific Functions](#useful-duckdb-specific-functions)
  - [System Commands](#system-commands)

## Database Connection and Management

### Starting DuckDB CLI

```bash
# Start DuckDB with a new or existing database file
duckdb mydatabase.db

# Start DuckDB with in-memory database
duckdb
```

### Attaching/Detaching Databases

```sql
-- Attach another database file
ATTACH DATABASE 'path/to/other.db' AS other;

-- Detach a database
DETACH DATABASE other;
```

### Database Information

```sql
-- List all tables in the database
SHOW TABLES;

-- Show table schema
DESCRIBE table_name;
PRAGMA table_info(table_name);

-- Show database version
SELECT version();

-- Show attached databases
PRAGMA database_list;
```

## Table Operations

### Creating Tables

```sql
-- Create a simple table
CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100),
    department VARCHAR(50),
    salary DECIMAL(10, 2),
    hire_date DATE
);

-- Create a table from a query result
CREATE TABLE department_stats AS
    SELECT department, COUNT(*) as employee_count, AVG(salary) as avg_salary
    FROM employees
    GROUP BY department;

-- Create a temporary table (exists only during the session)
CREATE TEMPORARY TABLE temp_data (id INTEGER, value DOUBLE);
```

### Altering Tables

```sql
-- Add a column
ALTER TABLE employees ADD COLUMN email VARCHAR;

-- Rename a table
ALTER TABLE employees RENAME TO staff;

-- Drop a column
ALTER TABLE staff DROP COLUMN email;
```

### Dropping Tables

```sql
-- Drop a table
DROP TABLE staff;

-- Drop a table if it exists
DROP TABLE IF EXISTS department_stats;
```

## Data Manipulation

### Inserting Data

```sql
-- Insert a single row
INSERT INTO employees VALUES (1, 'Alice', 'Engineering', 85000, '2020-01-15');

-- Insert a row with specific columns
INSERT INTO employees (id, name, department) VALUES (2, 'Bob', 'Marketing');

-- Insert multiple rows
INSERT INTO employees VALUES 
    (3, 'Charlie', 'Engineering', 90000, '2019-03-20'),
    (4, 'Diana', 'HR', 70000, '2021-05-10');

-- Insert data from a query
INSERT INTO department_stats
    SELECT department, COUNT(*), AVG(salary)
    FROM employees
    GROUP BY department;
```

### Updating Data

```sql
-- Update all rows in a column
UPDATE employees SET salary = salary * 1.1;

-- Update with a condition
UPDATE employees SET salary = 95000 WHERE id = 3;

-- Update multiple columns
UPDATE employees 
SET department = 'Engineering', salary = 88000 
WHERE id = 2;
```

### Deleting Data

```sql
-- Delete all rows
DELETE FROM temp_data;

-- Delete with a condition
DELETE FROM employees WHERE department = 'Marketing';

-- Delete using a subquery
DELETE FROM employees 
WHERE department IN (SELECT department FROM departments WHERE is_active = false);
```

## Querying Data

### Basic SELECT

```sql
-- Select all columns from a table
SELECT * FROM employees;

-- Select specific columns
SELECT name, department, salary FROM employees;

-- Select with a condition
SELECT * FROM employees WHERE salary > 80000;

-- Select with sorting
SELECT * FROM employees ORDER BY salary DESC;

-- Select with limit
SELECT * FROM employees LIMIT 10;

-- Select with offset (pagination)
SELECT * FROM employees LIMIT 10 OFFSET 20;
```

### Filtering

```sql
-- Multiple conditions
SELECT * FROM employees 
WHERE department = 'Engineering' AND salary > 80000;

-- IN clause
SELECT * FROM employees 
WHERE department IN ('Engineering', 'Marketing', 'HR');

-- BETWEEN clause
SELECT * FROM employees 
WHERE salary BETWEEN 70000 AND 90000;

-- Pattern matching
SELECT * FROM employees 
WHERE name LIKE 'A%';  -- Names starting with 'A'

-- NULL handling
SELECT * FROM employees 
WHERE hire_date IS NULL;
```

### Aggregation

```sql
-- Basic aggregation
SELECT 
    COUNT(*) as employee_count,
    AVG(salary) as average_salary,
    MIN(salary) as min_salary,
    MAX(salary) as max_salary,
    SUM(salary) as total_salary
FROM employees;

-- Grouping
SELECT department, COUNT(*) as employee_count
FROM employees
GROUP BY department;

-- Having clause (filtering groups)
SELECT department, AVG(salary) as avg_salary
FROM employees
GROUP BY department
HAVING AVG(salary) > 75000;
```

### Joins

```sql
-- Inner join
SELECT e.name, d.department_name, e.salary
FROM employees e
JOIN departments d ON e.department = d.department_id;

-- Left join
SELECT e.name, d.department_name
FROM employees e
LEFT JOIN departments d ON e.department = d.department_id;

-- Right join
SELECT e.name, d.department_name
FROM employees e
RIGHT JOIN departments d ON e.department = d.department_id;

-- Full outer join
SELECT e.name, d.department_name
FROM employees e
FULL OUTER JOIN departments d ON e.department = d.department_id;

-- Cross join
SELECT e.name, p.project_name
FROM employees e
CROSS JOIN projects p;
```

### Subqueries

```sql
-- Subquery in WHERE
SELECT name, salary
FROM employees
WHERE salary > (SELECT AVG(salary) FROM employees);

-- Subquery in FROM
SELECT dept, avg_salary
FROM (
    SELECT department as dept, AVG(salary) as avg_salary
    FROM employees
    GROUP BY department
) AS dept_stats
WHERE avg_salary > 80000;

-- Correlated subquery
SELECT e.name, e.department, e.salary
FROM employees e
WHERE e.salary > (
    SELECT AVG(salary) 
    FROM employees 
    WHERE department = e.department
);
```

### Common Table Expressions (CTEs)

```sql
-- Simple CTE
WITH dept_stats AS (
    SELECT department, COUNT(*) as emp_count, AVG(salary) as avg_salary
    FROM employees
    GROUP BY department
)
SELECT * FROM dept_stats WHERE emp_count > 5;

-- Multiple CTEs
WITH 
dept_stats AS (
    SELECT department, COUNT(*) as emp_count, AVG(salary) as avg_salary
    FROM employees
    GROUP BY department
),
high_salary_depts AS (
    SELECT department FROM dept_stats WHERE avg_salary > 85000
)
SELECT e.* FROM employees e
JOIN high_salary_depts h ON e.department = h.department;
```

## Transaction Control

```sql
-- Start a transaction
BEGIN TRANSACTION;

-- Commit a transaction
COMMIT;

-- Rollback a transaction
ROLLBACK;
```

## Data Import and Export

### Importing Data

```sql
-- Import from CSV
COPY employees FROM 'path/to/employees.csv' (DELIMITER ',', HEADER);

-- Import from CSV with options
COPY employees FROM 'path/to/employees.csv' (
    DELIMITER ';', 
    HEADER, 
    NULL 'NA', 
    ESCAPE '\', 
    QUOTE '"'
);

-- Import from Parquet
COPY employees FROM 'path/to/employees.parquet';

-- Import from JSON
COPY employees FROM 'path/to/employees.json';
```

### Exporting Data

```sql
-- Export to CSV
COPY employees TO 'path/to/output.csv' (DELIMITER ',', HEADER);

-- Export query results to CSV
COPY (SELECT * FROM employees WHERE department = 'Engineering') 
TO 'path/to/engineers.csv' (DELIMITER ',', HEADER);

-- Export to Parquet
COPY employees TO 'path/to/output.parquet';

-- Export to JSON
COPY employees TO 'path/to/output.json';
```

## Views

```sql
-- Create a view
CREATE VIEW employee_summary AS
    SELECT department, COUNT(*) as employee_count, AVG(salary) as avg_salary
    FROM employees
    GROUP BY department;

-- Create or replace a view
CREATE OR REPLACE VIEW high_salary_employees AS
    SELECT * FROM employees WHERE salary > 80000;

-- Drop a view
DROP VIEW IF EXISTS employee_summary;
```

## Indexes

```sql
-- Create an index
CREATE INDEX idx_employee_department ON employees(department);

-- Create a unique index
CREATE UNIQUE INDEX idx_employee_id ON employees(id);

-- Drop an index
DROP INDEX idx_employee_department;
```

## Window Functions

```sql
-- Ranking employees by salary within department
SELECT name, department, salary,
    RANK() OVER (PARTITION BY department ORDER BY salary DESC) as dept_rank
FROM employees;

-- Running total of salary
SELECT name, department, salary,
    SUM(salary) OVER (ORDER BY id) as running_total
FROM employees;

-- Moving average of salary (last 3 employees)
SELECT name, salary,
    AVG(salary) OVER (ORDER BY id ROWS BETWEEN 2 PRECEDING AND CURRENT ROW) as moving_avg
FROM employees;
```

## Schema Management

```sql
-- Create a schema
CREATE SCHEMA analytics;

-- Create a table in a schema
CREATE TABLE analytics.dashboard_metrics (
    date DATE,
    metric_name VARCHAR,
    value DOUBLE
);

-- Drop a schema
DROP SCHEMA IF EXISTS analytics CASCADE;
```

## Useful DuckDB-Specific Functions

```sql
-- Generate series of dates
SELECT * FROM generate_series(DATE '2023-01-01', DATE '2023-01-10', INTERVAL '1 day');

-- Unnest arrays
SELECT UNNEST([1, 2, 3]) as value;

-- String aggregation
SELECT STRING_AGG(name, ', ') as employee_list FROM employees;

-- Regular expressions
SELECT * FROM employees WHERE name REGEXP '^[A-C]';

-- Date functions
SELECT date_part('month', hire_date) as hire_month, COUNT(*) 
FROM employees 
GROUP BY hire_month;
```

## System Commands

```sql
-- Set memory limit
SET memory_limit = '4GB';

-- Check system settings
SELECT * FROM pragma_database_size();

-- Enable profiling
SET enable_profiling = true;
PRAGMA profiling_output = 'profile.json';

-- Enable parallelism
SET threads = 8;
```

This reference covers the most commonly used commands and syntax in DuckDB. For more detailed information, consult the [official DuckDB documentation](https://duckdb.org/docs/).