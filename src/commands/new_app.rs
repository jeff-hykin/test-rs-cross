use anyhow::{bail, Context, Result};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{config, ui};

pub fn run() -> Result<()> {
    let cfg = config::load()?;
    if !cfg.init_completed {
        bail!("Environment not initialised — run `dimos init` first.");
    }

    ui::header(" Dimos — New Python App");

    // ── gather answers ────────────────────────────────────────────────────────

    let name: String = cliclack::input("Project name")
        .placeholder("my-project")
        .validate(|s: &String| {
            if s.trim().is_empty() {
                return Err("Name cannot be empty.");
            }
            if s.contains(' ') {
                return Err("Use hyphens instead of spaces (e.g. my-project).");
            }
            if !s
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            {
                return Err("Only alphanumeric characters, hyphens, and underscores.");
            }
            Ok(())
        })
        .interact()?;

    let description: String = cliclack::input("Short description")
        .placeholder("What does this project do? (optional, Enter to skip)")
        .default_input("")
        .interact()?;

    let python_version: &str = cliclack::select("Python version")
        .initial_value("3.13")
        .item("3.13", "Python 3.13", "latest stable")
        .item("3.12", "Python 3.12", "")
        .item("3.11", "Python 3.11", "")
        .item("3.10", "Python 3.10", "")
        .item("3.9", "Python 3.9", "")
        .interact()?;

    let project_type: &str = cliclack::select("Project type")
        .initial_value("app")
        .item("app", "Application", "runnable script / CLI")
        .item("lib", "Library", "importable package")
        .interact()?;

    let default_author = git_user_name().unwrap_or_default();
    let author: String = cliclack::input("Author name")
        .default_input(&default_author)
        .interact()?;

    let license: &str = cliclack::select("License")
        .initial_value("MIT")
        .item("MIT", "MIT", "permissive")
        .item("Apache-2.0", "Apache 2.0", "permissive with patent clause")
        .item("GPL-3.0", "GPL 3.0", "copyleft")
        .item("None", "None", "no license")
        .interact()?;

    let cwd = env::current_dir()?;
    let default_dir = cwd.join(&name).to_string_lossy().into_owned();
    let dir_str: String = cliclack::input("Create project in")
        .default_input(&default_dir)
        .interact()?;
    let project_dir = PathBuf::from(&dir_str);

    // ── summary ───────────────────────────────────────────────────────────────

    let mut lines = format!(
        "Name:    {name}\nType:    {project_type}\nPython:  {python_version}\nAuthor:  {author}\nLicense: {license}\nDir:     {}",
        project_dir.display()
    );
    if !description.is_empty() {
        lines = format!("Name:    {name}\nDesc:    {description}\nType:    {project_type}\nPython:  {python_version}\nAuthor:  {author}\nLicense: {license}\nDir:     {}", project_dir.display());
    }
    cliclack::note("Summary", lines)?;

    let confirmed = cliclack::confirm("Create this project?")
        .initial_value(true)
        .interact()?;

    if !confirmed {
        ui::outro_cancel("Cancelled.");
        return Ok(());
    }

    // ── create ────────────────────────────────────────────────────────────────

    scaffold(
        &name,
        &description,
        python_version,
        project_type,
        &author,
        license,
        &project_dir,
    )?;

    // ── open shell in project dir ─────────────────────────────────────────────

    ui::outro(format!(
        "Project ready — opening a shell in {}  (type `exit` to return)",
        project_dir.display()
    ));

    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
    Command::new(&shell)
        .current_dir(&project_dir)
        .status()
        .with_context(|| format!("failed to spawn {shell}"))?;

    Ok(())
}

// ── scaffolding steps ─────────────────────────────────────────────────────────

fn scaffold(
    name: &str,
    description: &str,
    python_version: &str,
    project_type: &str,
    author: &str,
    license: &str,
    dir: &Path,
) -> Result<()> {
    let sp = cliclack::spinner();
    sp.start("Creating directory…");
    std::fs::create_dir_all(dir)?;
    sp.stop("Directory created");

    let sp = cliclack::spinner();
    sp.start("Initialising git repository…");
    run_in(dir, "git", &["init", "-q"])?;
    sp.stop("git repository initialised");

    let sp = cliclack::spinner();
    sp.start("Installing git-lfs hooks…");
    run_in(dir, "git", &["lfs", "install", "--local", "--silent"])?;
    sp.stop("git-lfs installed");

    let sp = cliclack::spinner();
    sp.start("Creating uv project…");
    let mut uv_args: Vec<&str> = vec!["init", "--python", python_version, "--name", name];
    if project_type == "lib" {
        uv_args.push("--lib");
    } else {
        uv_args.push("--app");
    }
    uv_args.push(".");
    run_in(dir, "uv", &uv_args)?;
    sp.stop("uv project created");

    let sp = cliclack::spinner();
    sp.start("Creating virtual environment…");
    run_in(dir, "uv", &["sync"])?;
    sp.stop("Virtual environment ready");

    let sp = cliclack::spinner();
    sp.start("Writing project files…");
    std::fs::write(
        dir.join("README.md"),
        build_readme(name, description, python_version, author, license),
    )?;
    std::fs::write(dir.join(".gitignore"), GITIGNORE)?;
    if let Some(text) = license_text(license, author) {
        std::fs::write(dir.join("LICENSE"), text)?;
    }
    sp.stop("README, .gitignore, LICENSE written");

    let sp = cliclack::spinner();
    sp.start("Creating initial commit…");
    run_in(dir, "git", &["add", "."])?;
    run_in(
        dir,
        "git",
        &["commit", "-q", "-m", "Initial commit (dimos new-app)"],
    )?;
    sp.stop("Initial commit created");

    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn run_in(dir: &Path, cmd: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("failed to run `{cmd}`"))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        bail!("`{cmd} {}` failed:\n{}", args.join(" "), stderr.trim());
    }
    Ok(())
}

fn git_user_name() -> Option<String> {
    Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn current_year() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    (1970 + secs / 31_557_600) as u32
}

fn build_readme(
    name: &str,
    description: &str,
    python: &str,
    author: &str,
    license: &str,
) -> String {
    let desc_block = if description.is_empty() {
        String::new()
    } else {
        format!("\n{}\n", description)
    };
    let module = name.replace('-', "_");
    format!(
        "# {name}\n{desc_block}
## Requirements

- Python {python}+
- [uv](https://github.com/astral-sh/uv)

## Getting Started

```bash
uv sync
uv run python -m {module}
```

## Author

{author}

## License

{license}
"
    )
}

fn license_text(license: &str, author: &str) -> Option<String> {
    let year = current_year();
    match license {
        "MIT" => Some(format!(
            "MIT License\n\nCopyright (c) {year} {author}\n\n\
Permission is hereby granted, free of charge, to any person obtaining a copy\n\
of this software and associated documentation files (the \"Software\"), to deal\n\
in the Software without restriction, including without limitation the rights\n\
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\
copies of the Software, and to permit persons to whom the Software is\n\
furnished to do so, subject to the following conditions:\n\n\
The above copyright notice and this permission notice shall be included in all\n\
copies or substantial portions of the Software.\n\n\
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n\
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n\
SOFTWARE.\n"
        )),
        "Apache-2.0" => Some(format!(
            "Copyright {year} {author}\n\n\
Licensed under the Apache License, Version 2.0 (the \"License\");\n\
you may not use this file except in compliance with the License.\n\
You may obtain a copy of the License at\n\n\
    http://www.apache.org/licenses/LICENSE-2.0\n\n\
Unless required by applicable law or agreed to in writing, software\n\
distributed under the License is distributed on an \"AS IS\" BASIS,\n\
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n\
See the License for the specific language governing permissions and\n\
limitations under the License.\n"
        )),
        "GPL-3.0" => Some(format!(
            "Copyright (C) {year} {author}\n\n\
This program is free software: you can redistribute it and/or modify\n\
it under the terms of the GNU General Public License as published by\n\
the Free Software Foundation, either version 3 of the License, or\n\
(at your option) any later version.\n\n\
This program is distributed in the hope that it will be useful,\n\
but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the\n\
GNU General Public License for more details.\n\n\
You should have received a copy of the GNU General Public License\n\
along with this program. If not, see <https://www.gnu.org/licenses/>.\n"
        )),
        _ => None,
    }
}

const GITIGNORE: &str = "\
# Python
__pycache__/
*.py[cod]
*$py.class
*.so
*.egg
*.egg-info/
dist/
build/
.eggs/

# uv / virtual envs
.venv/
venv/
env/
.python-version

# IDEs
.vscode/
.idea/
*.swp
*.swo
.DS_Store

# Test / lint caches
.pytest_cache/
.coverage
htmlcov/
.mypy_cache/
.ruff_cache/
";
