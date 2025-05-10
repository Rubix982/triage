# DuckDB CLI Output Modes

The DuckDB Command Line Interface (CLI) supports various output modes that format query results differently. These modes can significantly improve readability and help with different use cases, such as transferring data, creating visualizations, or generating reports.

## Available Output Modes

You can change the output mode using the `.mode` command followed by the mode name:

```sql
.mode MODE_NAME
```

DuckDB CLI supports the following output modes:

| Mode | Description | Best for |
|------|-------------|----------|
| `ascii` | Simple ASCII table format | Basic terminal display |
| `box` | Table with box-drawing characters | Terminal with Unicode support |
| `column` | Column-aligned output (default) | Interactive querying |
| `csv` | Comma-separated values | Data export |
| `duckbox` | DuckDB-specific box format | Terminal with Unicode support |
| `html` | HTML table format | Web integration |
| `insert` | SQL INSERT statements | SQL script generation |
| `json` | JSON array format | API integration |
| `jsonlines` | JSON Lines format (one object per line) | Streaming data |
| `latex` | LaTeX tabular environment | Academic documents |
| `line` | One value per line format | Simple parsing |
| `list` | Values separated by .separator string | Custom parsing |
| `markdown` | Markdown table format | Documentation |
| `quote` | Quoted values separated by .separator | Safe CSV-like export |
| `table` | Fixed-width padded columns | Terminal display |
| `tabs` | Tab-separated values | Spreadsheet import |
| `tcl` | TCL list format | TCL integration |
| `trash` | Discard all output | Benchmark/performance testing |

## Examples of Each Mode

Below are examples of what the same query results look like in different output modes.

Consider a simple query:

```sql
SELECT id, name, price FROM products LIMIT 3;
```

### ascii

```
+----+----------------+-------+
| id |      name      | price |
+----+----------------+-------+
|  1 | Widget         |  9.99 |
|  2 | Gadget         | 19.99 |
|  3 | Super Gizmo    | 29.99 |
+----+----------------+-------+
```

### box

```
┌────┬────────────────┬───────┐
│ id │      name      │ price │
├────┼────────────────┼───────┤
│  1 │ Widget         │  9.99 │
│  2 │ Gadget         │ 19.99 │
│  3 │ Super Gizmo    │ 29.99 │
└────┴────────────────┴───────┘
```

### column (default)

```
id  name          price
--  ------------  -----
1   Widget        9.99 
2   Gadget        19.99
3   Super Gizmo   29.99
```

### csv

```
id,name,price
1,Widget,9.99
2,Gadget,19.99
3,Super Gizmo,29.99
```

### duckbox

```
┏━━━━┳━━━━━━━━━━━━━━━━┳━━━━━━━┓
┃ id ┃      name      ┃ price ┃
┡━━━━╇━━━━━━━━━━━━━━━━╇━━━━━━━┩
│  1 │ Widget         │  9.99 │
│  2 │ Gadget         │ 19.99 │
│  3 │ Super Gizmo    │ 29.99 │
└────┴────────────────┴───────┘
```

### html

```html
<table>
<tr><th>id</th><th>name</th><th>price</th></tr>
<tr><td>1</td><td>Widget</td><td>9.99</td></tr>
<tr><td>2</td><td>Gadget</td><td>19.99</td></tr>
<tr><td>3</td><td>Super Gizmo</td><td>29.99</td></tr>
</table>
```

### insert

```sql
INSERT INTO table VALUES(1,'Widget',9.99);
INSERT INTO table VALUES(2,'Gadget',19.99);
INSERT INTO table VALUES(3,'Super Gizmo',29.99);
```

### json

```json
[{"id":1,"name":"Widget","price":9.99},
{"id":2,"name":"Gadget","price":19.99},
{"id":3,"name":"Super Gizmo","price":29.99}]
```

### jsonlines

```
{"id":1,"name":"Widget","price":9.99}
{"id":2,"name":"Gadget","price":19.99}
{"id":3,"name":"Super Gizmo","price":29.99}
```

### latex

```latex
\begin{tabular}{rrr}
id & name & price \\
\hline
1 & Widget & 9.99 \\
2 & Gadget & 19.99 \\
3 & Super Gizmo & 29.99 \\
\end{tabular}
```

### line

```
id = 1
name = Widget
price = 9.99

id = 2
name = Gadget
price = 19.99

id = 3
name = Super Gizmo
price = 29.99
```

### list

By default, this uses the pipe (|) character as the separator:

```
1|Widget|9.99
2|Gadget|19.99
3|Super Gizmo|29.99
```

### markdown

```markdown
| id |      name      | price |
|---:|:---------------|------:|
|  1 | Widget         |  9.99 |
|  2 | Gadget         | 19.99 |
|  3 | Super Gizmo    | 29.99 |
```

### quote

```
'1','Widget','9.99'
'2','Gadget','19.99'
'3','Super Gizmo','29.99'
```

### table

```
id        name        price     
--------  ----------  ----------
1         Widget      9.99      
2         Gadget      19.99     
3         Super Gizmo  29.99     
```

### tabs

```
id	name	price
1	Widget	9.99
2	Gadget	19.99
3	Super Gizmo	29.99
```

### tcl

```
"1" "Widget" "9.99"
"2" "Gadget" "19.99" 
"3" "Super Gizmo" "29.99"
```

### trash

This mode discards all output (nothing is displayed).

## Customizing Output Formats

### Separator

For modes like `list` and `csv`, you can customize the separator:

```sql
.separator "|"  -- Changes the separator for list mode to |
```

### Headers

Control whether column headers are displayed:

```sql
.headers on  -- Show column headers (default)
.headers off -- Hide column headers
```

### NULL Values Display

Customize how NULL values are displayed:

```sql
.nullvalue "NULL"  -- Display NULL values as the string "NULL"
```

## Redirecting Output to Files

You can redirect the output of a query to a file using the `.output` command:

```sql
.output results.csv
.mode csv
SELECT * FROM products;
.output stdout  -- Return to standard output
```

This is particularly useful for exporting data in a specific format.

## Combined Examples

Here are some practical examples combining modes and other CLI commands:

### Export to CSV file:

```sql
.mode csv
.headers on
.output data_export.csv
SELECT * FROM products;
.output stdout
```

### Generate Markdown Table for Documentation:

```sql
.mode markdown
.output sales_report.md
SELECT date, product, sum(amount) as total
FROM sales
GROUP BY date, product
ORDER BY date, total DESC;
.output stdout
```

### Create SQL INSERT Statements for Data Migration:

```sql
.mode insert
.output migration_script.sql
SELECT * FROM customers;
.output stdout
```

### Generate HTML Report:

```sql
.mode html
.output sales_report.html
SELECT region, product_category, sum(sales) as total_sales
FROM sales
GROUP BY region, product_category
ORDER BY region, total_sales DESC;
.output stdout
```

## Advanced Mode Features

### Custom Header and Footer for HTML

```sql
.header on
.mode html
.htmltag on table "class='data-table'"
.htmltag on td "class='data-cell'"
SELECT * FROM products LIMIT 5;
```

### Output Format in Scripts

When running scripts, you can include these commands in your SQL files:

```sql
-- Export script.sql
.mode csv
.headers on
.output export_data.csv
SELECT * FROM products;

.mode json
.output product_api.json
SELECT id, name, description, price FROM products;
```

## Performance Considerations

- `trash` mode is useful for benchmarking as it skips all output formatting
- For very large result sets, binary formats or CSV may be faster than formatted outputs
- JSON and HTML formats may use more memory for large datasets

## Common Use Cases by Mode

| Use Case | Recommended Mode |
|----------|------------------|
| Interactive querying | `column` or `box` |
| Data export to spreadsheets | `csv` or `tabs` |
| API integration | `json` or `jsonlines` |
| Documentation | `markdown` |
| Database migration | `insert` |
| Web display | `html` |
| Academic papers | `latex` |
| Simple text processing | `line` or `list` |
| Performance testing | `trash` |

## Mode Selection Tips

1. For regular interactive use, stick with the default `column` mode or try `box`/`duckbox` for better visual separation.
2. When working with data exports, use `csv` for standard compatibility or `tabs` for Excel imports.
3. For programmatic consumption, `json` or `jsonlines` are ideal.
4. For documentation or sharing query results, `markdown` produces clean, readable tables.
5. When transferring data between databases, `insert` creates ready-to-use SQL statements.

These output modes make DuckDB CLI a versatile tool not just for querying data but also for data presentation, export, and integration with other tools in your workflow.