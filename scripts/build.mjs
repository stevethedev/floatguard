import console from "node:console";
import {readdir, stat, writeFile} from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { exit } from "node:process";
import { fileURLToPath } from "node:url";

function getRootDir(...p) {
    return resolve(fileURLToPath(new URL(join("..", ...p), import.meta.url)));
}

async function main() {
    const benchmarksDir = getRootDir("benchmarks");
    const benchmarkList = await Array.fromAsync(getBenchmarks(benchmarksDir));
    const benchmarkHtml = await buildBenchmarkHtml(benchmarkList);

    const indexPath = join(benchmarksDir, "index.html");
    await writeFile(indexPath, benchmarkHtml, "utf8");
}

async function* getBenchmarks(benchmarksDir) {
    const dirs = (await readdir(benchmarksDir, { withFileTypes: true }))
        .filter(dirent => dirent.isDirectory())
        .map(dirent => dirent.name);

    for (const dir of dirs) {
        const reportPath = join(dir, "report");
        const reportPathStats = await stat(join(benchmarksDir, reportPath));
        if (reportPathStats.isDirectory()) {
            yield { dir, reportPath };
        }
    }
}

async function buildBenchmarkHtml(benchmarkList) {
    return `
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>Index - Criterion.rs</title>
    <style>
        body {
            font: 14px Helvetica Neue;
            text-rendering: optimizelegibility;
        }

        .body {
            width: 960px;
            margin: auto;
        }

        a:link {
            color: #1F78B4;
            text-decoration: none;
        }

        h2 {
            font-size: 36px;
            font-weight: 300;
        }

        h3 {
            font-size: 24px;
            font-weight: 300;
        }

        #footer {
            height: 40px;
            background: #888;
            color: white;
            font-size: larger;
            font-weight: 300;
        }

        #footer a {
            color: white;
            text-decoration: underline;
        }

        #footer p {
            text-align: center
        }

        table {
            border-collapse: collapse;
        }

        table,
        th,
        td {
            border: 1px solid #888;
        }
    </style>
</head>

<body>
    <div class="body">
        <h2>FloatGuard's Benchmark Index</h2>
        <ul>
            ${benchmarkList.map((benchmark) => `
                <li><a href="${benchmark.reportPath.split(sep).map(encodeURIComponent).join("/")}">${benchmark.dir}</a></li>`
            ).join("").trim()}
        </ul>
    </div>
</body>
</html>
    `.trim()
}

main()
    .catch((err) => {
        console.error("Build failed:", err);
        exit(1);
    });
