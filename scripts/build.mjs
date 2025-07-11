import console from "node:console";
import { mkdir, cp, readdir, stat, writeFile } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { exit } from "node:process";
import { fileURLToPath } from "node:url";

/**
 * Main function to build the root index and benchmark index.
 * @returns {Promise<void>}
 */
async function main() {
  await buildRootIndex(getDistDir());
  await buildBenchmarkIndex(
    getSourceDir("benchmarks"),
    getDistDir("benchmarks"),
  );
}

/**
 * Get a path relative to the root directory of the project.
 * @param p {...string[]} - Path segments to join with the root directory.
 * @returns {string} - The absolute path to the specified directory.
 */
function getRootDir(...p) {
  return resolve(fileURLToPath(new URL(join("..", ...p), import.meta.url)));
}

/**
 * Get the source directory path relative to the root directory.
 * @param p {...string[]} - Path segments to join with the source directory.
 * @returns {string} - The absolute path to the source directory.
 */
function getSourceDir(...p) {
  return getRootDir("src", ...p);
}

/**
 * Get the distribution directory path relative to the root directory.
 * @param p {...string[]} - Path segments to join with the distribution directory.
 * @returns {string} - The absolute path to the distribution directory.
 */
function getDistDir(...p) {
  return getRootDir("dist", ...p);
}

/**
 * Ensure that a directory exists, creating it if necessary.
 * @param dir {string} - The directory path to ensure exists.
 * @returns {Promise<void>} - A promise that resolves when the directory has been created or already exists.
 */
async function ensureDirExists(dir) {
  await mkdir(dir, { recursive: true });
}

/**
 * Build the root index HTML file.
 * @param distDir {string} - The distribution directory where the index file will be created.
 * @returns {Promise<void>} - A promise that resolves when the index file has been created.
 */
async function buildRootIndex(distDir) {
  await ensureDirExists(distDir);
  const indexHtml = buildHtml(
    "FloatGuard Benchmarks",
    `
        <h2>Benchmarks</h2>
        <p>See the <a href="benchmarks/">benchmarks</a> for details.</p>
    `,
  );

  const indexPath = join(distDir, "index.html");
  await writeFile(indexPath, indexHtml, "utf8");
}

/**
 * Build the benchmark index HTML file.
 * @param sourceDir {string} - The source directory containing the benchmark reports.
 * @param distDir {string} - The distribution directory where the benchmark index file will be created.
 * @returns {Promise<void>} - A promise that resolves when the benchmark index file has been created.
 */
async function buildBenchmarkIndex(sourceDir, distDir) {
  await ensureDirExists(distDir);
  const benchmarkList = (await Array.fromAsync(getBenchmarks(sourceDir)))
      .toSorted(sortBenchmarks);
  const benchmarkHtml = buildBenchmarkHtml(benchmarkList);

  const indexPath = join(distDir, "index.html");
  await writeFile(indexPath, benchmarkHtml, "utf8");

  await cp(sourceDir, distDir, { recursive: true });
}

/**
 * Sort benchmark directories based on their version numbers or alphabetically.
 * @param a {{ dir: string }} - The first benchmark directory object to compare.
 * @param b {{ dir: string }} - The second benchmark directory object to compare.
 * @returns {number} -1, 0, or 1 depending on the comparison result.
 */
function sortBenchmarks(a, b) {
  const bVersion = b.dir.match(/^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)/)?.groups;
  const aVersion = a.dir.match(/^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)/)?.groups;

  // If both are versions, compare them numerically in descending order of major, minor, and patch.
  if (aVersion && bVersion) {
    const aMajor = parseInt(aVersion.major, 10);
    const aMinor = parseInt(aVersion.minor, 10);
    const aPatch = parseInt(aVersion.patch, 10);
    const bMajor = parseInt(bVersion.major, 10);
    const bMinor = parseInt(bVersion.minor, 10);
    const bPatch = parseInt(bVersion.patch, 10);

    return bMajor - aMajor || bMinor - aMinor || bPatch - aPatch;
  }

  // If one is a version and the other is not, prioritize the non-version.
  if (aVersion) return -1; // a is a version, b is not
  if (bVersion) return 1; // b is a version, a is not

  // If neither is a version, sort alphabetically.
  return a.dir.localeCompare(b.dir);
}

/**
 * Get an asynchronous generator that yields benchmark directories and their report paths.
 * @param benchmarksDir {string} - The directory containing benchmark reports.
 * @returns {AsyncGenerator<{ dir: string, reportPath: string }>} - An async generator yielding objects with directory names and report paths.
 */
async function* getBenchmarks(benchmarksDir) {
  const dirs = (await readdir(benchmarksDir, { withFileTypes: true }))
    .filter((dirent) => dirent.isDirectory())
    .map((dirent) => dirent.name);

  for (const dir of dirs) {
    const reportPath = join(dir, "report");
    const reportPathStats = await stat(join(benchmarksDir, reportPath));
    if (reportPathStats.isDirectory()) {
      yield { dir, reportPath };
    }
  }
}

/**
 * Build an HTML document with a given title and body content.
 * @param title {string} - The title of the HTML document.
 * @param body {string} - The body content of the HTML document.
 * @returns {string} - The complete HTML document as a string.
 */
function buildHtml(title, body) {
  return `
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>${title}</title>
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
        ${body}
    </div>
</body>
</html>
  `.trim();
}

/**
 * Build the HTML content for the benchmark index page.
 * @param benchmarkList {Array<{ dir: string, reportPath: string }>} - List of benchmark directories and their report paths.
 * @returns {string} - The HTML content for the benchmark index page.
 */
function buildBenchmarkHtml(benchmarkList) {
  const benchmarkData = benchmarkList
    .map(({ dir, reportPath }) => ({
      dir,
      reportPath: join(
        "benchmarks",
        reportPath.split(sep).map(encodeURIComponent).join("/"),
      ),
    }))
    .map(({ dir, reportPath }) => `<li><a href="${reportPath}">${dir}</a></li>`)
    .join("");

  return buildHtml("FloatGuard Benchmarks", `
    <h2>Benchmarks</h2>
    <ul>${benchmarkData}</ul>
  `);
}

// Execute the main function and handle any errors.
main().catch((err) => {
  console.error("Build failed:", err);
  exit(1);
});
