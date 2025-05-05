# DuckDB Data Types and Functions

This document provides a comprehensive overview of DuckDB's data types and built-in functions.

- [DuckDB Data Types and Functions](#duckdb-data-types-and-functions)
  - [Data Types](#data-types)
    - [Numeric Types](#numeric-types)
    - [String Types](#string-types)
    - [Temporal Types](#temporal-types)
    - [Collection Types](#collection-types)
    - [Other Types](#other-types)
    - [Type Examples](#type-examples)
  - [Type Conversion](#type-conversion)
  - [Functions by Category](#functions-by-category)
    - [Mathematical Functions](#mathematical-functions)
    - [String Functions](#string-functions)
    - [Regular Expressions](#regular-expressions)
    - [Date and Time Functions](#date-and-time-functions)
    - [Conditional and Comparison Functions](#conditional-and-comparison-functions)
    - [Array and List Functions](#array-and-list-functions)
    - [Struct and Map Functions](#struct-and-map-functions)
    - [Aggregate Functions](#aggregate-functions)
    - [Window Functions](#window-functions)
    - [JSON Functions](#json-functions)
    - [Text Search Functions](#text-search-functions)
    - [Bitwise Operations](#bitwise-operations)
    - [Other Utility Functions](#other-utility-functions)

## Data Types

DuckDB supports a variety of data types for different kinds of data storage and manipulation needs.

### Numeric Types

| Type | Description | Range/Precision |
|------|-------------|-----------------|
| `BOOLEAN` | Logical boolean (true/false) | 1 byte |
| `TINYINT` | 1-byte signed integer | -128 to 127 |
| `SMALLINT` | 2-byte signed integer | -32,768 to 32,767 |
| `INTEGER`, `INT` | 4-byte signed integer | -2,147,483,648 to 2,147,483,647 |
| `BIGINT` | 8-byte signed integer | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `HUGEINT` | 16-byte signed integer | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `UTINYINT` | 1-byte unsigned integer | 0 to 255 |
| `USMALLINT` | 2-byte unsigned integer | 0 to 65,535 |
| `UINTEGER`, `UINT` | 4-byte unsigned integer | 0 to 4,294,967,295 |
| `UBIGINT` | 8-byte unsigned integer | 0 to 18,446,744,073,709,551,615 |
| `REAL`, `FLOAT4` | 4-byte floating-point | ~7 digits of precision |
| `DOUBLE`, `FLOAT8` | 8-byte floating-point | ~15 digits of precision |
| `DECIMAL`, `NUMERIC` | Fixed-point decimal | Configurable precision and scale |

### String Types

| Type | Description |
|------|-------------|
| `VARCHAR`, `TEXT`, `CHAR`, `CHARACTER` | Variable-length character string |
| `VARCHAR(n)`, `CHAR(n)`, `CHARACTER(n)` | Variable-length character string with maximum length |
| `BLOB` | Binary data (variable-length) |
| `BIT` | Bit string type |
| `BIT VARYING` | Variable-length bit string |

### Temporal Types

| Type | Description | Format |
|------|-------------|--------|
| `DATE` | Calendar date (year, month, day) | 'YYYY-MM-DD' |
| `TIME` | Time of day (hour, minute, second, microsecond) | 'HH:MM:SS.mmmmmm' |
| `TIMESTAMP` | Date and time | 'YYYY-MM-DD HH:MM:SS.mmmmmm' |
| `TIMESTAMP WITH TIME ZONE` | Date and time with timezone | 'YYYY-MM-DD HH:MM:SS.mmmmmm±TZ' |
| `INTERVAL` | Time interval | 'Y-M D H:M:S.mmmmmm' |

### Collection Types

| Type | Description |
|------|-------------|
| `LIST` | Ordered collection of values (array) |
| `STRUCT` | Record with named fields of different types |
| `MAP` | Key-value pairs |
| `UNION` | Value of one of multiple possible types |

### Other Types

| Type | Description |
|------|-------------|
| `UUID` | Universally unique identifier |
| `ENUM` | Enumerated set of string values |
| `JSON` | JSON data |

### Type Examples

```sql
-- Numeric types
CREATE TABLE numeric_examples (
    flag BOOLEAN,
    tiny TINYINT,
    small SMALLINT,
    standard INTEGER,
    big BIGINT,
    unsigned_int UINTEGER,
    floating REAL,
    precise DOUBLE,
    exact DECIMAL(10,2)
);

-- String types
CREATE TABLE string_examples (
    variable_text VARCHAR,
    fixed_char CHAR(10),
    binary_data BLOB,
    bits BIT(8)
);

-- Temporal types
CREATE TABLE temporal_examples (
    calendar_date DATE,
    day_time TIME,
    timestamp_val TIMESTAMP,
    timestamp_tz TIMESTAMP WITH TIME ZONE,
    duration INTERVAL
);

-- Collection types
CREATE TABLE collection_examples (
    array_col LIST(INTEGER),
    record_col STRUCT(name VARCHAR, age INTEGER),
    dictionary MAP(VARCHAR, INTEGER),
    variant_col UNION(num INTEGER, text VARCHAR)
);
```

## Type Conversion

DuckDB provides functions to convert between data types:

```sql
-- Explicit type casting
SELECT 
    CAST(123 AS VARCHAR),                -- '123'
    CAST('123' AS INTEGER),              -- 123
    CAST('2023-01-15' AS DATE),          -- DATE '2023-01-15'
    CAST(42.5 AS INTEGER),               -- 42 (truncated)
    CAST(TIMESTAMP '2023-01-15 14:30:00' AS DATE);  -- DATE '2023-01-15'

-- Alternative casting syntax
SELECT 
    123::VARCHAR,
    '123'::INTEGER,
    '2023-01-15'::DATE;

-- TRY_CAST (returns NULL instead of error)
SELECT TRY_CAST('not a number' AS INTEGER);  -- NULL
```

## Functions by Category

### Mathematical Functions

```sql
-- Basic arithmetic
SELECT 
    ABS(-42),              -- 42
    CEIL(42.3),            -- 43
    FLOOR(42.7),           -- 42
    ROUND(42.4),           -- 42
    ROUND(42.5),           -- 43
    ROUND(42.567, 2),      -- 42.57
    SQRT(25),              -- 5
    CBRT(27),              -- 3
    POWER(2, 3),           -- 8
    MOD(5, 2),             -- 1
    FACTORIAL(5),          -- 120
    GREATEST(1, 5, 3),     -- 5
    LEAST(1, 5, 3);        -- 1

-- Trigonometric functions
SELECT 
    PI(),                  -- 3.141592...
    DEGREES(1.0),          -- 57.29577...
    RADIANS(180.0),        -- 3.141592...
    SIN(RADIANS(30)),      -- 0.5
    COS(RADIANS(60)),      -- 0.5
    TAN(RADIANS(45)),      -- 1.0
    ASIN(0.5),             -- 0.523598... (radians)
    ACOS(0.5),             -- 1.047197... (radians)
    ATAN(1.0);             -- 0.785398... (radians)

-- Logarithmic functions
SELECT 
    LN(2.718281828),       -- 1.0
    LOG(10),               -- 1.0 (base 10)
    LOG(2, 8),             -- 3.0 (base 2)
    LOG10(100),            -- 2.0
    LOG2(8);               -- 3.0

-- Statistical functions
SELECT 
    RANDOM(),              -- Random value between 0 and 1
    SETSEED(0.5),          -- Set random seed
    RANDOM_UUID();         -- Generate a random UUID
```

### String Functions

```sql
-- String manipulation
SELECT 
    LOWER('HELLO'),                         -- 'hello'
    UPPER('hello'),                         -- 'HELLO'
    INITCAP('hello world'),                 -- 'Hello World'
    CONCAT('Hello', ' ', 'World'),          -- 'Hello World'
    'Hello' || ' ' || 'World',              -- 'Hello World'
    LENGTH('hello'),                        -- 5
    SUBSTRING('hello world', 7, 5),         -- 'world'
    LEFT('hello world', 5),                 -- 'hello'
    RIGHT('hello world', 5),                -- 'world'
    TRIM(' hello '),                        -- 'hello'
    LTRIM(' hello'),                        -- 'hello'
    RTRIM('hello '),                        -- 'hello'
    REPLACE('hello world', 'world', 'DuckDB'), -- 'hello DuckDB'
    REVERSE('hello'),                       -- 'olleh'
    CONTAINS('hello world', 'world'),       -- true
    POSITION('world' IN 'hello world'),     -- 7
    STARTS_WITH('hello world', 'hello'),    -- true
    ENDS_WITH('hello world', 'world');      -- true

-- String splitting
SELECT 
    SPLIT('hello,world', ','),              -- ['hello', 'world']
    STRING_SPLIT('hello,world', ','),       -- ['hello', 'world']
    REGEXP_MATCHES('abc123def', '\d+');     -- ['123']

-- String aggregation
SELECT 
    department,
    STRING_AGG(name, ', ') as employee_list
FROM employees
GROUP BY department;
```

### Regular Expressions

```sql
-- Regular expression matching
SELECT 
    REGEXP_MATCHES('abc123def', '\d+'),                -- ['123']
    REGEXP_REPLACE('hello world', 'world', 'DuckDB'),  -- 'hello DuckDB'
    REGEXP_EXTRACT('abc123def', '\d+', 0),             -- '123'
    'abc123def' SIMILAR TO '%\d+%',                    -- true
    'hello' ~ 'h.*o';                                  -- true

-- Extract all matches
SELECT UNNEST(REGEXP_MATCHES('abc123def456', '\d+'));  -- Multiple rows: '123', '456'
```

### Date and Time Functions

```sql
-- Current date and time
SELECT 
    CURRENT_DATE,                      -- Today's date
    CURRENT_TIME,                      -- Current time
    CURRENT_TIMESTAMP,                 -- Current date and time
    NOW();                             -- Current date and time (same as CURRENT_TIMESTAMP)

-- Date and time parts
SELECT 
    EXTRACT(YEAR FROM DATE '2023-05-15'),             -- 2023
    EXTRACT(MONTH FROM DATE '2023-05-15'),            -- 5
    EXTRACT(DAY FROM DATE '2023-05-15'),              -- 15
    EXTRACT(HOUR FROM TIMESTAMP '2023-05-15 14:30:00'), -- 14
    EXTRACT(MINUTE FROM TIMESTAMP '2023-05-15 14:30:00'), -- 30
    EXTRACT(SECOND FROM TIMESTAMP '2023-05-15 14:30:15.5'), -- 15.5
    EXTRACT(DOW FROM DATE '2023-05-15'),              -- 1 (Monday)
    EXTRACT(DOY FROM DATE '2023-05-15'),              -- 135 (day of year)
    DATE_PART('year', DATE '2023-05-15');             -- 2023

-- Date and time manipulation
SELECT 
    -- Add intervals
    DATE '2023-05-15' + INTERVAL '1 day',             -- 2023-05-16
    TIMESTAMP '2023-05-15 14:30:00' + INTERVAL '2 hours', -- 2023-05-15 16:30:00
    
    -- Subtract dates
    DATE '2023-05-15' - DATE '2023-05-10',            -- INTERVAL '5 days'
    
    -- Date/time arithmetic
    DATE_ADD('day', 5, DATE '2023-05-15'),            -- 2023-05-20
    DATE_SUB('month', 1, DATE '2023-05-15'),          -- 2023-04-15
    DATE_TRUNC('month', DATE '2023-05-15'),           -- 2023-05-01
    
    -- Format dates
    STRFTIME(DATE '2023-05-15', '%Y-%m-%d'),          -- '2023-05-15'
    STRFTIME(TIMESTAMP '2023-05-15 14:30:00', '%Y-%m-%d %H:%M'), -- '2023-05-15 14:30'
    
    -- Parse strings to dates
    STRPTIME('2023-05-15', '%Y-%m-%d'),               -- DATE '2023-05-15'
    STRPTIME('2023-05-15 14:30:00', '%Y-%m-%d %H:%M:%S'); -- TIMESTAMP '2023-05-15 14:30:00'

-- Range of dates
SELECT * FROM generate_series(
    DATE '2023-01-01', 
    DATE '2023-01-10', 
    INTERVAL '1 day'
) AS date_series;
```

### Conditional and Comparison Functions

```sql
-- Conditional expressions
SELECT 
    CASE 
        WHEN salary > 100000 THEN 'High'
        WHEN salary > 70000 THEN 'Medium'
        ELSE 'Low'
    END as salary_category
FROM employees;

-- Simple CASE
SELECT 
    CASE department
        WHEN 'Engineering' THEN 'Tech'
        WHEN 'Marketing' THEN 'Business'
        ELSE department
    END as department_category
FROM employees;

-- Comparison functions
SELECT 
    COALESCE(NULL, NULL, 'hello', 'world'),    -- 'hello' (first non-NULL value)
    NULLIF('hello', 'hello'),                  -- NULL (if equal)
    NULLIF('hello', 'world'),                  -- 'hello' (if not equal)
    GREATEST(5, 3, 9, 1),                      -- 9
    LEAST(5, 3, 9, 1),                         -- 1
    
    -- Conditional value
    IF(salary > 80000, 'High', 'Standard') as salary_tier,
    
    -- Short-circuit operations
    NULL OR 'hello',                          -- 'hello'
    'hello' OR NULL,                          -- 'hello'
    NULL AND 'hello',                         -- NULL
    'hello' AND 'world';                     -- 'world'
```

### Array and List Functions

```sql
-- Create and manipulate lists
SELECT 
    LIST_VALUE(1, 2, 3),                      -- [1, 2, 3]
    ARRAY[1, 2, 3],                          -- [1, 2, 3] (alternative syntax)
    LIST_CONCAT([1, 2], [3, 4]),             -- [1, 2, 3, 4]
    ARRAY_LENGTH([1, 2, 3]),                 -- 3
    LIST_ELEMENT([1, 2, 3], 2),              -- 2 (1-based indexing)
    [1, 2, 3][2],                            -- 2 (alternative syntax)
    ARRAY_CONTAINS([1, 2, 3], 2),            -- true
    ARRAY_POSITION([1, 2, 3], 2),            -- 2
    LIST_SLICE([1, 2, 3, 4, 5], 2, 4);       -- [2, 3, 4]

-- Array aggregation
SELECT 
    department,
    ARRAY_AGG(name) as employee_names,
    ARRAY_AGG(DISTINCT name) as unique_employee_names
FROM employees
GROUP BY department;

-- Unnesting arrays
SELECT 
    department,
    UNNEST(employee_names) as employee
FROM (
    SELECT 
        department,
        ARRAY_AGG(name) as employee_names
    FROM employees
    GROUP BY department
) as dept_employees;
```

### Struct and Map Functions

```sql
-- Create and access structs
SELECT 
    STRUCT_PACK(name := 'Alice', age := 30) as person,
    STRUCT_EXTRACT(STRUCT_PACK(name := 'Alice', age := 30), 'name'), -- 'Alice'
    STRUCT_PACK(name := 'Alice', age := 30)['name']; -- 'Alice' (alternative syntax)

-- Create and access maps
SELECT 
    MAP([['name', 'Alice'], ['age', '30']]) as person_map,
    MAP_EXTRACT(MAP([['name', 'Alice'], ['age', '30']]), 'name'), -- 'Alice'
    MAP([['name', 'Alice'], ['age', '30']])['name']; -- 'Alice' (alternative syntax)
```

### Aggregate Functions

```sql
-- Basic aggregates
SELECT 
    COUNT(*),                           -- Count all rows
    COUNT(salary),                      -- Count non-NULL salary values
    COUNT(DISTINCT department),         -- Count unique departments
    SUM(salary),                        -- Sum of salaries
    AVG(salary),                        -- Average salary
    MIN(salary),                        -- Minimum salary
    MAX(salary),                        -- Maximum salary
    MEDIAN(salary),                     -- Median salary
    STDDEV(salary),                     -- Standard deviation
    VARIANCE(salary),                   -- Variance
    STRING_AGG(name, ', '),             -- Comma-separated list of names
    ARRAY_AGG(name),                    -- Array of names
    FIRST(name ORDER BY hire_date),     -- First name by hire date
    LAST(name ORDER BY hire_date)       -- Last name by hire date
FROM employees;

-- Conditional aggregates
SELECT 
    department,
    COUNT(CASE WHEN salary > 80000 THEN 1 END) as high_salary_count,
    AVG(CASE WHEN gender = 'F' THEN salary END) as avg_female_salary
FROM employees
GROUP BY department;

-- Statistical aggregates
SELECT 
    CORR(years_experience, salary),     -- Correlation
    REGR_SLOPE(salary, years_experience), -- Regression slope
    REGR_INTERCEPT(salary, years_experience), -- Regression intercept
    PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY salary) -- 50th percentile (median)
FROM employees;
```

### Window Functions

```sql
-- Ranking functions
SELECT 
    name,
    department,
    salary,
    ROW_NUMBER() OVER (PARTITION BY department ORDER BY salary DESC) as dept_row_num,
    RANK() OVER (PARTITION BY department ORDER BY salary DESC) as dept_rank,
    DENSE_RANK() OVER (PARTITION BY department ORDER BY salary DESC) as dept_dense_rank,
    PERCENT_RANK() OVER (PARTITION BY department ORDER BY salary DESC) as dept_percent_rank,
    NTILE(4) OVER (PARTITION BY department ORDER BY salary DESC) as dept_quartile
FROM employees;

-- Analytic functions
SELECT 
    name,
    department,
    salary,
    hire_date,
    AVG(salary) OVER (PARTITION BY department) as dept_avg_salary,
    MAX(salary) OVER (PARTITION BY department) as dept_max_salary,
    SUM(salary) OVER (PARTITION BY department) as dept_total_salary,
    COUNT(*) OVER (PARTITION BY department) as dept_employee_count
FROM employees;

-- Navigation functions
SELECT 
    name,
    hire_date,
    salary,
    LAG(name, 1) OVER (ORDER BY hire_date) as prev_hired,
    LEAD(name, 1) OVER (ORDER BY hire_date) as next_hired,
    FIRST_VALUE(name) OVER (ORDER BY hire_date) as first_hired,
    LAST_VALUE(name) OVER (ORDER BY hire_date ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) as last_hired
FROM employees;

-- Running calculations
SELECT 
    name,
    hire_date,
    salary,
    SUM(salary) OVER (ORDER BY hire_date) as running_total_salary,
    AVG(salary) OVER (ORDER BY hire_date ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING) as moving_avg_salary
FROM employees;
```

### JSON Functions

```sql
-- JSON creation and extraction
SELECT 
    JSON_OBJECT('name', 'Alice', 'age', 30) as person_json,
    JSON_EXTRACT('{"name":"Alice","age":30}', '$.name') as extracted_name,
    JSON_EXTRACT_STRING('{"name":"Alice","age":30}', '$.name') as name_string,
    JSON_EXTRACT_INTEGER('{"name":"Alice","age":30}', '$.age') as age_int;

-- Convert between JSON and other types
SELECT 
    JSON_SERIALIZE(STRUCT_PACK(name := 'Alice', age := 30)), -- Convert struct to JSON
    JSON_DESERIALIZE('{"name":"Alice","age":30}', 'STRUCT(name VARCHAR, age INTEGER)'); -- JSON to struct
```

### Text Search Functions

```sql
-- Full-text search
SELECT * 
FROM documents 
WHERE LOWER(content) LIKE '%duckdb%';

-- Custom search with pattern matching
SELECT * 
FROM employees 
WHERE REGEXP_MATCHES(name, 'A.*a');
```

### Bitwise Operations

```sql
-- Bitwise operators
SELECT 
    5 & 3,  -- 1 (AND)
    5 | 3,  -- 7 (OR)
    5 # 3,  -- 6 (XOR)
    ~5,     -- -6 (NOT)
    5 << 1, -- 10 (left shift)
    5 >> 1; -- 2 (right shift)
```

### Other Utility Functions

```sql
-- Type checking
SELECT 
    TYPEOF(42),                        -- 'INTEGER'
    TYPEOF('hello'),                   -- 'VARCHAR'
    TYPEOF(CURRENT_DATE);              -- 'DATE'

-- UUID generation
SELECT RANDOM_UUID();                 -- Generate a UUID

-- System information
SELECT 
    version(),                         -- DuckDB version
    current_user,                      -- Current user
    current_schema;                    -- Current schema
```

This reference covers the most commonly used data types and functions in DuckDB. For more detailed information and the latest additions, consult the [official DuckDB documentation](https://duckdb.org/docs/).