import duckdb from "duckdb";
import { readFileSync } from "fs";
import path from "path";

const dbPath = path.resolve(
  process.env.HOME || ".",
  ".triage",
  "triage.duckdb"
);
const db = new duckdb.Database(dbPath);

export async function queryIssuesByProject(projectKey: string): Promise<any[]> {
  return new Promise((resolve, reject) => {
    db.all(
      `SELECT id, key, summary, description, status, created, updated FROM issues WHERE key LIKE ?`,
      [`${projectKey}-%`],
      (err, rows) => {
        if (err) reject(err);
        else resolve(rows);
      }
    );
  });
}

export async function getIssues(): Promise<any[]> {
  return new Promise((resolve, reject) => {
    db.all(`SELECT * FROM triage.issues LIMIT 100;`, [], (err, rows) => {
      if (err) reject(err);
      else resolve(rows);
    });
  });
}
