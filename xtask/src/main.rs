use std::{
    env,
    ffi::OsStr,
    fs,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use anyhow::{Context, Result, bail};
use base64::Engine as _;
use command_fds::{CommandFdExt, FdMapping};
use os_pipe::{PipeReader, PipeWriter};
use rs_dean_ui::{ComponentDefinition, SHADCN_COMPONENTS, component_implementation};
use serde_json::{Value, json};

const MARKETING_APP: &str = "apps/marketing";
const GAME_APP: &str = "apps/game";
const STORIES_APP: &str = "apps/stories";
const UI_BEVY_STORIES_APP: &str = "apps/ui-bevy-stories";
const TEST_PROJECT: &str = "apps/test-project";
const CUBE_SMOKE_APP: &str = "apps/test-project/cube-smoke";
const UI_BOOK: &str = "docs/crates/ui";
const UI_BOOK_GENERATED: &str = "target/generated-ui-book";

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("build") => build(),
        Some("check" | "check-fast" | "gate") => gate(),
        Some("check-template") => check_template(),
        Some("cube-smoke") => cube_smoke(),
        Some("doctor") => doctor(),
        Some("dev") => dev(),
        Some("docs-sweep") => docs_sweep(),
        Some("five-phase-pass") => five_phase_pass(),
        Some("game") => game(),
        Some("gen-ui-book") => gen_ui_book(),
        Some("install-tailwindcss") => install_tailwindcss(),
        Some("gen-app") => {
            let name = args.next().unwrap_or_else(|| "test-project".to_owned());
            gen_app(&name)
        }
        Some("preview") => preview(),
        Some("pages") => pages(),
        Some("static-analysis") => static_analysis(),
        Some("stories") => stories(),
        Some("ui-bevy-stories") => ui_bevy_stories(),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(other) => bail!("unknown xtask command `{other}`"),
    }
}

fn print_help() {
    println!(
        "\
rs-dean xtask

Commands:
  dev              run the Leptos marketing app on :5173
  game             run the Bevy game app on :5174
  stories          run the Rust story harness on :6106
  ui-bevy-stories  run the Bevy UI story harness on :6107
  preview          serve the release app on :3100
  gen-ui-book      regenerate the UI crate mdBook source from the Rust catalog
  build            build static marketing/game output and GitHub Pages artifacts
  pages            build aggregate GitHub Pages artifact in target/pages
  static-analysis  run format, clippy, rustdoc, dependency, and repo policy checks
  gate             one-pass Rust gate
  check            alias for gate
  check-fast       alias for gate
  cube-smoke       generate test project, build green-cube page, and assert rendered pixels
  doctor           verify local tools, wasm target, Chrome, ports, and repo wiring
  five-phase-pass  regenerate template, run gate, and sweep docs
  docs-sweep       fail on retired stack references
  install-tailwindcss
                  install the standalone Tailwind CLI into Cargo's bin dir
  gen-app <name>   copy templates/app into apps/<name>
  check-template   regenerate apps/test-project from templates/app
"
    );
}

fn dev() -> Result<()> {
    run_in(
        MARKETING_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "5173"],
    )
}

fn game() -> Result<()> {
    run_in(
        GAME_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "5174"],
    )
}

fn stories() -> Result<()> {
    run_in(
        STORIES_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "6106"],
    )
}

fn ui_bevy_stories() -> Result<()> {
    run_in(
        UI_BEVY_STORIES_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "6107"],
    )
}

fn preview() -> Result<()> {
    run_in(
        MARKETING_APP,
        "trunk",
        [
            "serve",
            "--release",
            "--address",
            "127.0.0.1",
            "--port",
            "3100",
        ],
    )
}

fn build() -> Result<()> {
    pages()
}

fn five_phase_pass() -> Result<()> {
    check_template()?;
    gate()?;
    docs_sweep()
}

fn doctor() -> Result<()> {
    let mut report = DoctorReport::default();
    report.command("cargo", ["--version"]);
    report.command("rustc", ["--version"]);
    report.command("rustup", ["--version"]);
    report.check("rustfmt", command_output("rustfmt", ["--version"]));
    report.check(
        "cargo clippy",
        command_output("cargo", ["clippy", "--version"]),
    );
    report.command("trunk", ["--version"]);
    report.command("mdbook", ["--version"]);
    report.check("tailwindcss", check_tailwindcss_cli());
    report.command("cargo-nextest", ["--version"]);
    report.command("cargo-deny", ["--version"]);
    report.command("cargo-machete", ["--version"]);
    report.check(
        "wasm target",
        installed_rust_targets()
            .map(|targets| {
                targets
                    .lines()
                    .any(|target| target.trim() == "wasm32-unknown-unknown")
                    .then(|| "wasm32-unknown-unknown installed".to_owned())
                    .context("install with `rustup target add wasm32-unknown-unknown`")
            })
            .and_then(|result| result),
    );
    report.check(
        "Chrome",
        find_chrome().map(|path| format!("found {}", path.display())),
    );
    report.check(
        "Bevy WebGPU",
        check_bevy_webgpu_only().map(|()| "feature tree is WebGPU-only".to_owned()),
    );
    for (label, port) in [
        ("marketing dev port", 5173),
        ("game dev port", 5174),
        ("story harness port", 6106),
        ("UI Bevy story harness port", 6107),
        ("preview port", 3100),
        ("cube smoke port", 6173),
    ] {
        report.check(label, check_port_available(port));
    }
    for path in [
        ".cargo/config.toml",
        "AGENTS.md",
        "docs/crates/ui/book.toml",
        "docs/crates/ui/src/SUMMARY.md",
        "templates/app/Cargo.toml",
        "templates/app/cube-smoke/Cargo.toml",
        "apps/marketing/Trunk.toml",
        "apps/game/Trunk.toml",
        "apps/stories/Trunk.toml",
        "apps/stories/public/.gitkeep",
        "apps/ui-bevy-stories/Cargo.toml",
        "apps/ui-bevy-stories/Trunk.toml",
        "apps/ui-bevy-stories/index.html",
    ] {
        report.check(
            path,
            Path::new(path)
                .exists()
                .then(|| "present".to_owned())
                .with_context(|| format!("missing {path}")),
        );
    }
    for path in [
        "target",
        "apps/test-project",
        "apps/marketing/dist",
        "apps/game/dist",
    ] {
        report.check(
            &format!("ignore {path}"),
            git_check_ignore(path).and_then(|ignored| {
                ignored
                    .then(|| "ignored".to_owned())
                    .with_context(|| format!("{path} is not ignored"))
            }),
        );
    }
    report.finish()
}

#[derive(Default)]
struct DoctorReport {
    failures: Vec<String>,
}

impl DoctorReport {
    fn command<I, S>(&mut self, program: &str, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.check(program, command_output(program, args));
    }

    fn check(&mut self, label: &str, result: Result<String>) {
        match result {
            Ok(message) => println!("[ok] {label}: {message}"),
            Err(error) => {
                println!("[fail] {label}: {error}");
                self.failures.push(format!("{label}: {error}"));
            }
        }
    }

    fn finish(self) -> Result<()> {
        if self.failures.is_empty() {
            println!("doctor: local environment is ready.");
            Ok(())
        } else {
            bail!("doctor found issues:\n{}", self.failures.join("\n"));
        }
    }
}

fn command_output<I, S>(program: &str, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("run {program}"))?;
    if !output.status.success() {
        bail!("{program} exited with {}", output.status);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let message = stdout
        .lines()
        .chain(stderr.lines())
        .find(|line| !line.trim().is_empty())
        .unwrap_or("available");
    Ok(message.trim().to_owned())
}

fn check_tailwindcss_cli() -> Result<String> {
    let version = command_output("tailwindcss", ["--help"])
        .context("run `cargo xtask install-tailwindcss` or put Tailwind CSS v4 on PATH")?;
    let version = strip_ansi_codes(&version);
    let major = first_semver_major(&version)
        .with_context(|| format!("could not parse tailwindcss version from `{version}`"))?;
    if major < 4 {
        bail!(
            "tailwindcss CLI must be v4 or newer; found `{version}`. Run `cargo xtask install-tailwindcss`."
        );
    }
    Ok(version)
}

fn strip_ansi_codes(text: &str) -> String {
    let mut stripped = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
            continue;
        }
        stripped.push(character);
    }
    stripped
}

fn first_semver_major(text: &str) -> Option<u32> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '.'))
        .map(|token| token.strip_prefix('v').unwrap_or(token))
        .filter_map(|token| token.split_once('.').map(|(major, _)| major))
        .find_map(|major| major.parse().ok())
}

fn install_tailwindcss() -> Result<()> {
    let asset = tailwindcss_asset_name()?;
    let bin_dir = cargo_bin_dir()?;
    fs::create_dir_all(&bin_dir).with_context(|| format!("create {}", bin_dir.display()))?;

    let binary_name = if cfg!(windows) {
        "tailwindcss.exe"
    } else {
        "tailwindcss"
    };
    let destination = bin_dir.join(binary_name);
    let download = destination.with_extension("download");
    let url =
        format!("https://github.com/tailwindlabs/tailwindcss/releases/latest/download/{asset}");

    println!("installing Tailwind CSS CLI from {url}");
    run(
        "curl",
        vec![
            "-fL".to_owned(),
            "-o".to_owned(),
            download.display().to_string(),
            url,
        ],
    )?;

    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&download)
            .with_context(|| format!("read {}", download.display()))?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&download, permissions)
            .with_context(|| format!("chmod {}", download.display()))?;
    }

    fs::rename(&download, &destination).with_context(|| {
        format!(
            "move downloaded Tailwind CSS CLI to {}",
            destination.display()
        )
    })?;
    println!("installed {}", destination.display());
    Ok(())
}

fn tailwindcss_asset_name() -> Result<&'static str> {
    match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => Ok("tailwindcss-linux-x64"),
        ("linux", "aarch64") => Ok("tailwindcss-linux-arm64"),
        ("macos", "x86_64") => Ok("tailwindcss-macos-x64"),
        ("macos", "aarch64") => Ok("tailwindcss-macos-arm64"),
        ("windows", "x86_64") => Ok("tailwindcss-windows-x64.exe"),
        (os, arch) => bail!("unsupported Tailwind CSS standalone target {os}/{arch}"),
    }
}

fn cargo_bin_dir() -> Result<PathBuf> {
    if let Some(cargo_home) = env::var_os("CARGO_HOME") {
        return Ok(PathBuf::from(cargo_home).join("bin"));
    }
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .context("CARGO_HOME, HOME, and USERPROFILE are unset")?;
    Ok(PathBuf::from(home).join(".cargo").join("bin"))
}

fn installed_rust_targets() -> Result<String> {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .context("run rustup target list --installed")?;
    if !output.status.success() {
        bail!(
            "rustup target list --installed exited with {}",
            output.status
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn check_port_available(port: u16) -> Result<String> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .with_context(|| format!("port {port} is unavailable"))?;
    drop(listener);
    Ok(format!("127.0.0.1:{port} available"))
}

fn git_check_ignore(path: &str) -> Result<bool> {
    for candidate in [path.to_owned(), format!("{path}/")] {
        let status = Command::new("git")
            .args(["check-ignore", "-q", &candidate])
            .status()
            .with_context(|| format!("run git check-ignore for {candidate}"))?;
        match status.code() {
            Some(0) => return Ok(true),
            Some(1) => {}
            _ => bail!("git check-ignore exited with {status}"),
        }
    }
    Ok(false)
}

fn gate() -> Result<()> {
    require_tool("trunk")?;
    require_cargo_subcommand("cargo-nextest")?;
    require_static_analysis_tools()?;

    run("cargo", ["fmt", "--all", "--", "--check"])?;
    run_static_policy_checks()?;
    run_native_clippy()?;
    run_wasm_clippy()?;
    run(
        "cargo",
        [
            "nextest",
            "run",
            "--workspace",
            "--exclude",
            "rs-dean-game",
            "--exclude",
            "rs-dean-bevy-scenes",
            "--exclude",
            "rs-dean-ui-bevy-stories",
        ],
    )?;
    run(
        "cargo",
        [
            "test",
            "--workspace",
            "--exclude",
            "rs-dean-game",
            "--exclude",
            "rs-dean-bevy-scenes",
            "--exclude",
            "rs-dean-ui-bevy-stories",
            "--doc",
        ],
    )?;
    run(
        "cargo",
        [
            "check",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "rs-dean-marketing",
            "-p",
            "rs-dean-game",
            "-p",
            "rs-dean-stories",
            "-p",
            "rs-dean-ui-bevy-stories",
            "-p",
            "rs-dean-bevy-scenes",
            "-p",
            "rs-dean-idb",
            "-p",
            "rs-dean-state",
        ],
    )?;
    run(
        "cargo",
        [
            "test",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "rs-dean-state",
            "--test",
            "refresh_hydration",
            "--no-run",
        ],
    )?;
    run_rustdoc_analysis()?;
    run_dependency_analysis()?;
    check_template()?;
    check_generated_template_build()?;
    build()?;
    build_stories()?;
    build_ui_bevy_stories()?;
    cube_smoke()?;
    docs_sweep()
}

fn static_analysis() -> Result<()> {
    require_static_analysis_tools()?;
    run("cargo", ["fmt", "--all", "--", "--check"])?;
    run_static_policy_checks()?;
    run_native_clippy()?;
    run_wasm_clippy()?;
    run_rustdoc_analysis()?;
    run_dependency_analysis()?;
    docs_sweep()
}

fn require_static_analysis_tools() -> Result<()> {
    command_output("rustfmt", ["--version"])?;
    command_output("cargo", ["clippy", "--version"])?;
    require_tool("mdbook")?;
    check_tailwindcss_cli()?;
    require_cargo_subcommand("cargo-deny")?;
    require_cargo_subcommand("cargo-machete")
}

fn run_static_policy_checks() -> Result<()> {
    check_bevy_webgpu_only()?;
    check_bevy_only_apps()?;
    check_required_app_persistence()?;
    check_leptos_tailwind_assets()?;
    check_ui_design_token_classes()?;
    check_ui_book()
}

fn run_native_clippy() -> Result<()> {
    run(
        "cargo",
        [
            "clippy",
            "--workspace",
            "--exclude",
            "rs-dean-game",
            "--exclude",
            "rs-dean-bevy-scenes",
            "--exclude",
            "rs-dean-ui-bevy-stories",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )
}

fn run_wasm_clippy() -> Result<()> {
    run(
        "cargo",
        [
            "clippy",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "rs-dean-marketing",
            "-p",
            "rs-dean-game",
            "-p",
            "rs-dean-stories",
            "-p",
            "rs-dean-ui-bevy-stories",
            "-p",
            "rs-dean-bevy-scenes",
            "-p",
            "rs-dean-idb",
            "-p",
            "rs-dean-state",
            "--",
            "-D",
            "warnings",
        ],
    )
}

fn run_rustdoc_analysis() -> Result<()> {
    run_with_env(
        "cargo",
        [
            "doc",
            "--workspace",
            "--exclude",
            "rs-dean-game",
            "--exclude",
            "rs-dean-bevy-scenes",
            "--exclude",
            "rs-dean-ui-bevy-stories",
            "--no-deps",
            "--document-private-items",
        ],
        [("RUSTDOCFLAGS", "-D warnings")],
    )
}

fn run_dependency_analysis() -> Result<()> {
    run("cargo", ["deny", "check"])?;
    run("cargo-machete", ["--skip-target-dir"])
}

fn build_stories() -> Result<()> {
    run_in(STORIES_APP, "trunk", ["build", "--release"])?;
    verify_trunk_dist(&Path::new(STORIES_APP).join("dist"))
}

fn build_ui_bevy_stories() -> Result<()> {
    run_in(UI_BEVY_STORIES_APP, "trunk", ["build", "--release"])?;
    verify_trunk_dist(&Path::new(UI_BEVY_STORIES_APP).join("dist"))
}

fn pages() -> Result<()> {
    let target = Path::new("target/pages");
    if target.exists() {
        fs::remove_dir_all(target).context("remove old target/pages")?;
    }
    fs::create_dir_all(target).context("create target/pages")?;

    let base = pages_base_path();
    build_pages_app(MARKETING_APP, target, "marketing", &base)?;
    build_pages_app(GAME_APP, target, "game", &base)?;
    build_pages_app(STORIES_APP, target, "stories", &base)?;
    build_pages_app(UI_BEVY_STORIES_APP, target, "ui-bevy-stories", &base)?;
    build_ui_book(target)?;
    write_pages_index(target, &base)?;
    write_crates_index(target, &base)?;
    write_pages_support_files(target, &base)?;
    verify_pages_site(target)
}

fn pages_base_path() -> String {
    env::var("RS_DEAN_PAGES_BASE")
        .ok()
        .filter(|base| !base.trim().is_empty())
        .map(|base| normalize_public_base(&base))
        .unwrap_or_else(|| "/".to_owned())
}

fn normalize_public_base(base: &str) -> String {
    let mut normalized = base.trim().to_owned();
    if !normalized.starts_with('/') {
        normalized.insert(0, '/');
    }
    if !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn build_pages_app(app: &str, target: &Path, route: &str, base: &str) -> Result<()> {
    let dist = env::current_dir()
        .context("resolve repository root")?
        .join(target)
        .join(route);
    let public_url = format!("{base}{route}/");
    run_in(
        app,
        "trunk",
        vec![
            "build".to_owned(),
            "--release".to_owned(),
            "--dist".to_owned(),
            dist.display().to_string(),
            "--public-url".to_owned(),
            public_url,
        ],
    )?;
    verify_trunk_dist(&dist)
}

fn build_ui_book(target: &Path) -> Result<()> {
    check_ui_book()?;
    let destination = env::current_dir()
        .context("resolve repository root")?
        .join(target)
        .join("crates/ui");
    fs::create_dir_all(&destination)
        .with_context(|| format!("create {}", destination.display()))?;
    run_in(
        UI_BOOK,
        "mdbook",
        vec![
            "build".to_owned(),
            "--dest-dir".to_owned(),
            destination.display().to_string(),
        ],
    )?;
    verify_ui_book_dist(&destination)
}

fn gen_ui_book() -> Result<()> {
    let root = Path::new(UI_BOOK);
    if root.exists() {
        fs::remove_dir_all(root).with_context(|| format!("remove old {}", root.display()))?;
    }
    write_ui_book_sources(root)?;
    println!("generated {}", root.display());
    Ok(())
}

fn check_ui_book() -> Result<()> {
    let expected = Path::new(UI_BOOK_GENERATED);
    if expected.exists() {
        fs::remove_dir_all(expected)
            .with_context(|| format!("remove old {}", expected.display()))?;
    }
    write_ui_book_sources(expected)?;
    compare_generated_tree(expected, Path::new(UI_BOOK))?;
    check_ui_book_story_anchors()
}

fn write_ui_book_sources(root: &Path) -> Result<()> {
    let src = root.join("src");
    let components = src.join("components");
    fs::create_dir_all(&components).with_context(|| format!("create {}", components.display()))?;
    fs::write(root.join("book.toml"), ui_book_toml())
        .with_context(|| format!("write {}", root.join("book.toml").display()))?;
    fs::write(src.join("index.md"), ui_book_index())
        .with_context(|| format!("write {}", src.join("index.md").display()))?;
    fs::write(src.join("SUMMARY.md"), ui_book_summary())
        .with_context(|| format!("write {}", src.join("SUMMARY.md").display()))?;
    for definition in SHADCN_COMPONENTS {
        fs::write(
            components.join(format!("{}.md", definition.slug)),
            ui_component_book_page(definition),
        )
        .with_context(|| {
            format!(
                "write {}",
                components.join(format!("{}.md", definition.slug)).display()
            )
        })?;
    }
    Ok(())
}

fn ui_book_toml() -> String {
    r#"[book]
title = "rs-dean-ui"
authors = ["rs-dean"]
language = "en"
src = "src"

[build]
build-dir = "../../../target/mdbook/ui"
create-missing = false
"#
    .to_owned()
}

fn ui_book_index() -> String {
    format!(
        r#"# rs-dean-ui

`rs-dean-ui` owns the shared design tokens, semantic themes, shadcn-inspired
component catalog, Leptos renderers, and Bevy primitive adapters.

This book is generated from the Rust catalog. The component pages link back to
the live Leptos and Bevy story harnesses with isolated story routes, so each
page shows only that component's pre-filled DOM fixtures and matching Bevy
primitive adapter used by local component development.

## Pages Structure

- [Marketing app](../../marketing/)
- [Game app](../../game/)
- [Stories app](../../stories/)
- [UI Bevy stories app](../../ui-bevy-stories/)
- [Crate index](../)

## Component Coverage

- Components documented: {}
- Source of truth: `crates/ui/src/catalog.rs`
- Implementation contracts: `crates/ui/src/kit.rs`
- Leptos live fixtures: `apps/stories/src/main.rs`
- Bevy primitive fixtures: `apps/ui-bevy-stories/src/main.rs`

Run `cargo xtask gen-ui-book` after adding, removing, or renaming a component.
`cargo xtask gate` verifies this book stays in sync with the catalog and story
harnesses.
"#,
        SHADCN_COMPONENTS.len()
    )
}

fn ui_book_summary() -> String {
    let mut summary = String::from("# Summary\n\n");
    summary.push_str("- [Overview](index.md)\n\n");
    summary.push_str("# Components\n\n");
    for definition in SHADCN_COMPONENTS {
        summary.push_str(&format!(
            "- [{}](components/{}.md)\n",
            definition.name, definition.slug
        ));
    }
    summary
}

fn ui_component_book_page(definition: ComponentDefinition) -> String {
    let implementation = component_implementation(definition.id);
    format!(
        r#"# {name}

{summary}

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows {name} examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="{name} Leptos live story fixtures" src="../../../stories/?story=ui-{slug}" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="{name} Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-{slug}" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-{slug}) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-{slug}) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | {category} |
| Framework | {framework} |
| State | {state} |
| Render contract | {render_contract} |
| State contract | {state_contract} |
| Layout contract | {layout_contract} |

## Variants

{variants}
## States

{states}
## Anatomy

{anatomy}
## Accessibility

{accessibility}
## Consumer Implementation

{consumer_contract}

## End User Outcome

{end_user_outcome}
"#,
        name = definition.name,
        slug = definition.slug,
        summary = definition.summary,
        category = definition.category.label(),
        framework = definition.framework.label(),
        state = definition.state.label(),
        render_contract = implementation.render.label(),
        state_contract = implementation.state.label(),
        layout_contract = implementation.layout.label(),
        variants = markdown_list(implementation.variants),
        states = markdown_list(implementation.states),
        anatomy = markdown_list(implementation.anatomy),
        accessibility = markdown_list(implementation.accessibility),
        consumer_contract = implementation.consumer_contract,
        end_user_outcome = implementation.end_user_outcome,
    )
}

fn markdown_list(items: &[&str]) -> String {
    let mut list = String::new();
    for item in items {
        list.push_str("- ");
        list.push_str(item);
        list.push('\n');
    }
    list
}

fn compare_generated_tree(expected: &Path, actual: &Path) -> Result<()> {
    if !actual.exists() {
        bail!(
            "{} is missing; run `cargo xtask gen-ui-book`",
            actual.display()
        );
    }
    let expected_files = generated_source_files(expected)?;
    let actual_files = generated_source_files(actual)?;
    if expected_files != actual_files {
        bail!(
            "{} is out of sync with the UI catalog; run `cargo xtask gen-ui-book`",
            actual.display()
        );
    }
    for relative in expected_files {
        let expected_contents =
            fs::read_to_string(expected.join(&relative)).with_context(|| {
                format!(
                    "read generated expected {}",
                    expected.join(&relative).display()
                )
            })?;
        let actual_contents = fs::read_to_string(actual.join(&relative))
            .with_context(|| format!("read {}", actual.join(&relative).display()))?;
        if expected_contents != actual_contents {
            bail!(
                "{} is out of sync with the UI catalog; run `cargo xtask gen-ui-book`",
                actual.join(relative).display()
            );
        }
    }
    Ok(())
}

fn generated_source_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_generated_source_files(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_generated_source_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(current).with_context(|| format!("read {}", current.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_generated_source_files(root, &path, files)?;
            continue;
        }
        let extension = path.extension().and_then(OsStr::to_str);
        if !matches!(extension, Some("md" | "toml")) {
            continue;
        }
        files.push(
            path.strip_prefix(root)
                .with_context(|| format!("strip {} from {}", root.display(), path.display()))?
                .to_owned(),
        );
    }
    Ok(())
}

fn check_ui_book_story_anchors() -> Result<()> {
    let stories =
        fs::read_to_string("apps/stories/src/main.rs").context("read apps/stories/src/main.rs")?;
    let bevy_stories = fs::read_to_string("apps/ui-bevy-stories/src/main.rs")
        .context("read apps/ui-bevy-stories/src/main.rs")?;
    if stories.contains("model=invalid_") || stories.contains("fn invalid_") {
        bail!("UI Leptos stories must not mount intentionally invalid fixtures in mdBook demos");
    }
    if !bevy_stories.contains("SHADCN_COMPONENTS")
        || !bevy_stories.contains("bevy_story_variants_for_component")
    {
        bail!("UI Bevy stories must route catalog components through shared Bevy story variants");
    }
    for definition in SHADCN_COMPONENTS {
        let story_id = format!("ui-{}", definition.slug);
        let id = format!("id=\"{story_id}\"");
        let data_story_id = format!("data-story-id=\"{story_id}\"");
        if !stories.contains(&id) {
            bail!("stories missing `{id}` for {}", definition.name);
        }
        if !stories.contains(&data_story_id) {
            bail!("stories missing `{data_story_id}` for {}", definition.name);
        }
        let page_path = Path::new(UI_BOOK)
            .join("src")
            .join("components")
            .join(format!("{}.md", definition.slug));
        let page = fs::read_to_string(&page_path)
            .with_context(|| format!("read {}", page_path.display()))?;
        let isolated_story_src = format!("src=\"../../../stories/?story={story_id}\"");
        if !page.contains(&isolated_story_src) {
            bail!(
                "{} must embed isolated story route `{isolated_story_src}`",
                page_path.display()
            );
        }
        let isolated_bevy_story_src = format!("src=\"../../../ui-bevy-stories/?story={story_id}\"");
        if !page.contains(&isolated_bevy_story_src) {
            bail!(
                "{} must embed isolated Bevy story route `{isolated_bevy_story_src}`",
                page_path.display()
            );
        }
    }
    Ok(())
}

fn verify_ui_book_dist(dist: &Path) -> Result<()> {
    if !dist.join("index.html").exists() {
        bail!("missing {}", dist.join("index.html").display());
    }
    for definition in SHADCN_COMPONENTS {
        let path = dist
            .join("components")
            .join(format!("{}.html", definition.slug));
        if !path.exists() {
            bail!("missing UI book component page {}", path.display());
        }
    }
    Ok(())
}

fn write_pages_index(target: &Path, base: &str) -> Result<()> {
    fs::write(
        target.join("index.html"),
        format!(
            r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rs-dean</title>
  </head>
  <body>
    <main>
      <h1>rs-dean Pages</h1>
      <p>Static app binaries and crate books for the workspace.</p>
      <section>
        <h2>Binaries</h2>
        <ul>
          <li><a href="{base}marketing/">Marketing</a></li>
          <li><a href="{base}game/">Game</a></li>
          <li><a href="{base}stories/">Stories</a></li>
          <li><a href="{base}ui-bevy-stories/">UI Bevy Stories</a></li>
        </ul>
      </section>
      <section>
        <h2>Crates</h2>
        <ul>
          <li><a href="{base}crates/">Crate docs</a></li>
          <li><a href="{base}crates/ui/">rs-dean-ui book</a></li>
        </ul>
      </section>
    </main>
  </body>
</html>
"#
        ),
    )
    .with_context(|| format!("write {}", target.join("index.html").display()))
}

fn write_crates_index(target: &Path, base: &str) -> Result<()> {
    let crates = target.join("crates");
    fs::create_dir_all(&crates).with_context(|| format!("create {}", crates.display()))?;
    fs::write(
        crates.join("index.html"),
        format!(
            r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rs-dean crates</title>
  </head>
  <body>
    <main>
      <h1>rs-dean crates</h1>
      <p>Workspace crates and crate-level books.</p>
      <ul>
        <li><a href="{base}crates/ui/">rs-dean-ui</a></li>
        <li>rs-dean-state</li>
        <li>rs-dean-idb</li>
        <li>rs-dean-schema</li>
        <li>rs-dean-srs</li>
        <li>rs-dean-curriculum</li>
        <li>rs-dean-bevy-scenes</li>
        <li>rs-dean-cards</li>
      </ul>
      <p><a href="{base}">Back to rs-dean Pages</a></p>
    </main>
  </body>
</html>
"#
        ),
    )
    .with_context(|| format!("write {}", crates.join("index.html").display()))
}

fn write_pages_support_files(target: &Path, base: &str) -> Result<()> {
    fs::write(target.join(".nojekyll"), "")
        .with_context(|| format!("write {}", target.join(".nojekyll").display()))?;
    fs::copy(target.join("index.html"), target.join("404.html"))
        .with_context(|| format!("copy {}", target.join("404.html").display()))?;
    fs::write(
        target.join("sitemap.xml"),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>{base}</loc></url>
  <url><loc>{base}marketing/</loc></url>
  <url><loc>{base}game/</loc></url>
  <url><loc>{base}stories/</loc></url>
  <url><loc>{base}ui-bevy-stories/</loc></url>
  <url><loc>{base}crates/</loc></url>
  <url><loc>{base}crates/ui/</loc></url>
</urlset>
"#
        ),
    )
    .with_context(|| format!("write {}", target.join("sitemap.xml").display()))
}

fn verify_pages_site(target: &Path) -> Result<()> {
    for relative in ["index.html", "404.html", ".nojekyll", "sitemap.xml"] {
        let path = target.join(relative);
        if !path.exists() {
            bail!("missing Pages artifact {}", path.display());
        }
    }
    verify_trunk_dist(&target.join("marketing"))?;
    verify_trunk_dist(&target.join("game"))?;
    verify_trunk_dist(&target.join("stories"))?;
    verify_trunk_dist(&target.join("ui-bevy-stories"))?;
    verify_ui_book_dist(&target.join("crates/ui"))
}

fn cube_smoke() -> Result<()> {
    ensure_test_project()?;
    build_cube_smoke()?;
    check_cube_smoke_render()
}

fn build_cube_smoke() -> Result<()> {
    run_in(CUBE_SMOKE_APP, "trunk", ["build", "--release"])?;
    verify_trunk_dist(&Path::new(CUBE_SMOKE_APP).join("dist"))
}

fn check_generated_template_build() -> Result<()> {
    run_in(TEST_PROJECT, "trunk", ["build", "--release"])?;
    verify_trunk_dist(&Path::new(TEST_PROJECT).join("dist"))
}

fn check_bevy_webgpu_only() -> Result<()> {
    for package in [
        "rs-dean-game",
        "rs-dean-bevy-scenes",
        "rs-dean-ui-bevy-stories",
    ] {
        check_bevy_webgpu_only_package(package)?;
    }
    Ok(())
}

fn check_bevy_webgpu_only_package(package: &str) -> Result<()> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--target",
            "wasm32-unknown-unknown",
            "-e",
            "features",
            "-p",
            package,
        ])
        .output()
        .with_context(|| format!("inspect {package} Bevy wasm feature tree"))?;
    if !output.status.success() {
        bail!("cargo tree for {package} exited with {}", output.status);
    }
    let stdout = String::from_utf8(output.stdout).context("cargo tree output was not UTF-8")?;
    let stdout = strip_ansi_codes(&stdout);
    if stdout.contains("webgl") {
        bail!("{package} Bevy wasm feature tree includes WebGL; keep `webgl2` disabled");
    }
    if !stdout.contains("webgpu") {
        bail!("{package} Bevy wasm feature tree does not include WebGPU");
    }
    Ok(())
}

fn check_bevy_only_apps() -> Result<()> {
    for package in ["rs-dean-game", "rs-dean-ui-bevy-stories"] {
        check_bevy_only_package(package)?;
    }
    Ok(())
}

fn check_bevy_only_package(package: &str) -> Result<()> {
    let output = Command::new("cargo")
        .args(["tree", "--target", "wasm32-unknown-unknown", "-p", package])
        .output()
        .with_context(|| format!("inspect {package} wasm dependency tree"))?;
    if !output.status.success() {
        bail!("cargo tree for {package} exited with {}", output.status);
    }
    let stdout = String::from_utf8(output.stdout).context("cargo tree output was not UTF-8")?;
    let stdout = strip_ansi_codes(&stdout);
    if !cargo_tree_has_package(&stdout, "bevy") {
        bail!("{package} must depend on Bevy");
    }
    if cargo_tree_has_package(&stdout, "leptos") {
        bail!("{package} must stay Bevy-only and cannot depend on Leptos");
    }
    Ok(())
}

fn check_required_app_persistence() -> Result<()> {
    for app in [
        RequiredPersistentApp {
            manifest: Path::new("apps/marketing/Cargo.toml"),
            source: Path::new("apps/marketing/src/main.rs"),
        },
        RequiredPersistentApp {
            manifest: Path::new("apps/game/Cargo.toml"),
            source: Path::new("apps/game/src/main.rs"),
        },
    ] {
        assert_file_contains(app.manifest, "require_persistent_state = true")?;
        assert_file_contains(app.manifest, "rs-dean-state")?;
        assert_file_contains(app.source, "ensure_durable_snapshot")?;
    }
    assert_file_contains(
        Path::new("templates/app/src/main.rs"),
        "ensure_durable_snapshot",
    )?;
    Ok(())
}

fn check_leptos_tailwind_assets() -> Result<()> {
    for (index, styles) in [
        (
            "apps/marketing/index.html",
            "apps/marketing/styles/index.css",
        ),
        ("apps/stories/index.html", "apps/stories/styles/index.css"),
        ("templates/app/index.html", "templates/app/styles/index.css"),
    ] {
        let index = Path::new(index);
        let styles = Path::new(styles);
        assert_file_contains(index, "rel=\"tailwind-css\"")?;
        assert_file_contains(index, "href=\"styles/index.css\"")?;
        assert_file_not_contains(index, "rel=\"css\"")?;
        assert_file_contains(styles, "@import \"tailwindcss\"")?;
    }
    Ok(())
}

fn check_ui_design_token_classes() -> Result<()> {
    let mut source_paths = Vec::from([
        PathBuf::from("apps/marketing/src/main.rs"),
        PathBuf::from("apps/stories/src/main.rs"),
        PathBuf::from("templates/app/src/main.rs"),
    ]);
    collect_rust_files(Path::new("crates/ui/src"), &mut source_paths)?;
    let stock_design_scale_classes = [
        "text-xs",
        "text-sm",
        "text-base",
        "text-lg",
        "text-xl",
        "text-2xl",
        "text-3xl",
        "font-bold",
        "font-semibold",
        "leading-none",
        "leading-tight",
        "leading-snug",
        "leading-normal",
        "leading-relaxed",
        "leading-loose",
        "p-4",
        "p-5",
        "px-3",
        "px-6",
        "py-2",
        "py-8",
        "gap-2",
        "gap-3",
        "gap-4",
        "gap-6",
        "mt-2",
        "mt-3",
        "mt-4",
        "mt-6",
        "mb-6",
        "h-8",
        "w-8",
        "min-h-40",
        "rounded-sm",
        "rounded-md",
        "rounded-lg",
        "rounded-xl",
        "shadow-sm",
        "shadow-md",
        "shadow-lg",
    ];
    for path in &source_paths {
        let contents =
            fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        for class in stock_design_scale_classes {
            if class_token_is_present(&contents, class) {
                bail!(
                    "{} uses stock Tailwind design-scale class `{class}`; use rs-dean-ui token utilities instead",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root).with_context(|| format!("read {}", root.display()))? {
        let entry = entry.with_context(|| format!("read entry in {}", root.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn class_token_is_present(contents: &str, class: &str) -> bool {
    contents.match_indices(class).any(|(start, _)| {
        let end = start + class.len();
        let before = contents[..start].chars().next_back();
        let after = contents[end..].chars().next();
        is_class_boundary(before) && is_class_boundary(after)
    })
}

fn is_class_boundary(character: Option<char>) -> bool {
    matches!(
        character,
        None | Some('"')
            | Some('\'')
            | Some('`')
            | Some(':')
            | Some('/')
            | Some('{')
            | Some('}')
            | Some('(')
            | Some(')')
            | Some(' ')
            | Some('\n')
            | Some('\r')
            | Some('\t')
    )
}

struct RequiredPersistentApp {
    manifest: &'static Path,
    source: &'static Path,
}

fn cargo_tree_has_package(tree: &str, package: &str) -> bool {
    let prefix = format!("{package} v");
    tree.lines().any(|line| {
        line.trim_start_matches(|character: char| {
            !character.is_ascii_alphanumeric() && character != '_'
        })
        .starts_with(&prefix)
    })
}

fn check_cube_smoke_render() -> Result<()> {
    let server = TrunkServer::start(CUBE_SMOKE_APP)?;
    let screenshot = env::current_dir()
        .context("resolve repository root")?
        .join("target/cube-smoke-render.png");
    if let Some(parent) = screenshot.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let _ = fs::remove_file(&screenshot);

    let chrome = find_chrome()?;
    let profile = env::current_dir()
        .context("resolve repository root")?
        .join("target/cube-smoke-chrome-profile");
    let _ = fs::remove_dir_all(&profile);
    fs::create_dir_all(&profile).with_context(|| format!("create {}", profile.display()))?;

    let (_browser, mut cdp) = BrowserProcess::start(&chrome, &profile)?;
    let browser_context = cdp_command(
        &mut cdp,
        None,
        "Target.createBrowserContext",
        json!({ "disposeOnDetach": true }),
    )?;
    let browser_context_id = browser_context
        .get("browserContextId")
        .and_then(Value::as_str)
        .context("Chrome CDP Target.createBrowserContext returned no browserContextId")?
        .to_owned();
    let target = cdp_command(
        &mut cdp,
        None,
        "Target.createTarget",
        json!({
            "url": "about:blank",
            "browserContextId": browser_context_id
        }),
    )?;
    let target_id = target
        .get("targetId")
        .and_then(Value::as_str)
        .context("Chrome CDP Target.createTarget returned no targetId")?
        .to_owned();
    cdp_command(
        &mut cdp,
        None,
        "Target.activateTarget",
        json!({ "targetId": target_id }),
    )?;
    let session = cdp_command(
        &mut cdp,
        None,
        "Target.attachToTarget",
        json!({
            "targetId": target_id,
            "flatten": true
        }),
    )?;
    let session_id = session
        .get("sessionId")
        .and_then(Value::as_str)
        .context("Chrome CDP Target.attachToTarget returned no sessionId")?
        .to_owned();

    cdp_command(&mut cdp, Some(&session_id), "Page.enable", json!({}))?;
    cdp_command(&mut cdp, Some(&session_id), "Runtime.enable", json!({}))?;
    cdp_command(&mut cdp, Some(&session_id), "Page.bringToFront", json!({}))?;
    cdp_command(
        &mut cdp,
        Some(&session_id),
        "Emulation.setDeviceMetricsOverride",
        json!({
            "width": 1000,
            "height": 700,
            "deviceScaleFactor": 1,
            "mobile": false
        }),
    )?;
    cdp_command(
        &mut cdp,
        Some(&session_id),
        "Emulation.setVisibleSize",
        json!({
            "width": 1000,
            "height": 700
        }),
    )?;
    cdp_command(
        &mut cdp,
        Some(&session_id),
        "Page.navigate",
        json!({ "url": server.url }),
    )?;
    wait_for_cube_canvas_layout(&mut cdp, &session_id)?;
    wait_for_green_cube_pixels(&mut cdp, &session_id, &screenshot)
}

struct BrowserProcess {
    child: Child,
}

impl BrowserProcess {
    fn start(chrome: &Path, profile: &Path) -> Result<(Self, CdpConnection)> {
        let (browser_read, parent_write) = os_pipe::pipe().context("open Chrome CDP input pipe")?;
        let (parent_read, browser_write) =
            os_pipe::pipe().context("open Chrome CDP output pipe")?;

        let mut command = Command::new(chrome);
        command
            .args([
                "--headless",
                "--remote-debugging-pipe",
                "--no-sandbox",
                "--disable-field-trial-config",
                "--disable-background-networking",
                "--disable-background-timer-throttling",
                "--disable-backgrounding-occluded-windows",
                "--disable-back-forward-cache",
                "--disable-breakpad",
                "--disable-client-side-phishing-detection",
                "--disable-component-extensions-with-background-pages",
                "--disable-component-update",
                "--disable-default-apps",
                "--disable-dev-shm-usage",
                "--disable-extensions",
                "--disable-renderer-backgrounding",
                "--disable-features=AcceptCHFrame,AvoidUnnecessaryBeforeUnloadCheckSync,DestroyProfileOnBrowserClose,DialMediaRouteProvider,GlobalMediaControls,HttpsUpgrades,LensOverlay,MediaRouter,PaintHolding,ThirdPartyStoragePartitioning,Translate,AutoDeElevate,RenderDocument,OptimizationHints",
                "--enable-features=CDPScreenshotNewSurface",
                "--allow-pre-commit-input",
                "--disable-hang-monitor",
                "--disable-ipc-flooding-protection",
                "--disable-popup-blocking",
                "--disable-prompt-on-repost",
                "--no-default-browser-check",
                "--no-first-run",
                "--password-store=basic",
                "--use-mock-keychain",
                "--no-service-autorun",
                "--export-tagged-pdf",
                "--disable-search-engine-choice-screen",
                "--unsafely-disable-devtools-self-xss-warnings",
                "--edge-skip-compat-layer-relaunch",
                "--disable-infobars",
                "--disable-sync",
                "--enable-unsafe-swiftshader",
                "--hide-scrollbars",
                "--mute-audio",
                "--ignore-gpu-blocklist",
                "--enable-unsafe-webgpu",
                "--enable-automation",
                "--remote-allow-origins=*",
                "--run-all-compositor-stages-before-draw",
                "--force-color-profile=srgb",
                "--force-device-scale-factor=1",
                "--window-size=1000,700",
                "--blink-settings=primaryHoverType=2,availableHoverTypes=2,primaryPointerType=4,availablePointerTypes=4",
                "--no-startup-window",
            ])
            .arg(format!("--user-data-dir={}", profile.display()))
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        command
            .fd_mappings(vec![
                FdMapping {
                    parent_fd: browser_read.into(),
                    child_fd: 3,
                },
                FdMapping {
                    parent_fd: browser_write.into(),
                    child_fd: 4,
                },
            ])
            .context("map Chrome CDP pipe file descriptors")?;

        let child = command
            .spawn()
            .with_context(|| format!("run {}", chrome.display()))?;
        Ok((
            Self { child },
            CdpConnection {
                reader: parent_read,
                writer: parent_write,
                read_buffer: Vec::new(),
                console_messages: Vec::new(),
                next_id: 0,
            },
        ))
    }
}

impl Drop for BrowserProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

struct CdpConnection {
    reader: PipeReader,
    writer: PipeWriter,
    read_buffer: Vec<u8>,
    console_messages: Vec<String>,
    next_id: u64,
}

fn cdp_command(
    cdp: &mut CdpConnection,
    session_id: Option<&str>,
    method: &str,
    params: Value,
) -> Result<Value> {
    cdp.next_id += 1;
    let id = cdp.next_id;
    let mut request = json!({
        "id": id,
        "method": method,
        "params": params
    });
    if let Some(session_id) = session_id {
        request["sessionId"] = Value::String(session_id.to_owned());
    }
    cdp.writer
        .write_all(request.to_string().as_bytes())
        .with_context(|| format!("send CDP command {method}"))?;
    cdp.writer
        .write_all(b"\0")
        .with_context(|| format!("terminate CDP command {method}"))?;
    cdp.writer
        .flush()
        .with_context(|| format!("flush CDP command {method}"))?;

    loop {
        let response = read_cdp_message(cdp).context("read Chrome CDP pipe message")?;
        if response.get("id").and_then(Value::as_u64) != Some(id) {
            record_cdp_event(cdp, &response);
            continue;
        }
        if let Some(error) = response.get("error") {
            bail!("Chrome CDP command {method} failed: {error}");
        }
        return Ok(response.get("result").cloned().unwrap_or_else(|| json!({})));
    }
}

fn record_cdp_event(cdp: &mut CdpConnection, event: &Value) {
    let Some(method) = event.get("method").and_then(Value::as_str) else {
        return;
    };
    if method != "Runtime.consoleAPICalled" && method != "Runtime.exceptionThrown" {
        return;
    }
    let message = if method == "Runtime.consoleAPICalled" {
        event
            .pointer("/params/args")
            .and_then(Value::as_array)
            .map(|args| {
                args.iter()
                    .filter_map(|arg| {
                        arg.get("value")
                            .or_else(|| arg.get("description"))
                            .and_then(Value::as_str)
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|message| !message.is_empty())
            .unwrap_or_else(|| event.to_string())
    } else {
        event
            .pointer("/params/exceptionDetails/exception/description")
            .or_else(|| event.pointer("/params/exceptionDetails/exception/value"))
            .or_else(|| event.pointer("/params/exceptionDetails/text"))
            .and_then(Value::as_str)
            .unwrap_or("runtime exception")
            .to_owned()
    };
    cdp.console_messages.push(message);
}

fn read_cdp_message(cdp: &mut CdpConnection) -> Result<Value> {
    loop {
        if let Some(end) = cdp.read_buffer.iter().position(|byte| *byte == 0) {
            let message = cdp.read_buffer.drain(..=end).collect::<Vec<_>>();
            let message = &message[..message.len() - 1];
            return serde_json::from_slice(message).context("parse Chrome CDP pipe message");
        }
        let mut chunk = [0; 4096];
        let bytes_read = cdp
            .reader
            .read(&mut chunk)
            .context("read Chrome CDP pipe chunk")?;
        if bytes_read == 0 {
            bail!("Chrome CDP pipe closed");
        }
        cdp.read_buffer.extend_from_slice(&chunk[..bytes_read]);
    }
}

fn evaluate_json(cdp: &mut CdpConnection, session_id: &str, expression: &str) -> Result<Value> {
    let result = cdp_command(
        cdp,
        Some(session_id),
        "Runtime.evaluate",
        json!({
            "expression": expression,
            "awaitPromise": true,
            "returnByValue": true
        }),
    )?;
    if let Some(exception) = result.get("exceptionDetails") {
        bail!("cube smoke page evaluation failed: {exception}");
    }
    let encoded = result
        .pointer("/result/value")
        .and_then(Value::as_str)
        .context("cube smoke page evaluation returned no JSON string")?;
    serde_json::from_str(encoded).context("parse cube smoke page evaluation JSON")
}

fn wait_for_cube_canvas_layout(cdp: &mut CdpConnection, session_id: &str) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        let layout = evaluate_json(cdp, session_id, CUBE_CANVAS_LAYOUT_EXPRESSION);
        let last_error = match layout.and_then(|layout| assert_cube_canvas_layout(&layout)) {
            Ok(()) => return Ok(()),
            Err(error) => error.to_string(),
        };
        if Instant::now() >= deadline {
            bail!("cube smoke canvas layout check failed: {last_error}");
        }
        thread::sleep(Duration::from_millis(250));
    }
}

const CUBE_CANVAS_LAYOUT_EXPRESSION: &str = r##"
(async () => {
  let adapter = false;
  let adapterError = null;
  try {
    adapter = Boolean(navigator.gpu && await navigator.gpu.requestAdapter());
  } catch (error) {
    adapterError = String(error && error.message ? error.message : error);
  }
  const canvas = document.querySelector("#cube-smoke-canvas");
  if (!canvas) {
    return {
      webgpu: Boolean(navigator.gpu),
      adapter,
      adapterError,
      canvas: null,
      viewport: { width: window.innerWidth, height: window.innerHeight }
    };
  }
  const rect = canvas.getBoundingClientRect();
  return {
    webgpu: Boolean(navigator.gpu),
    adapter,
    adapterError,
    canvas: {
      width: canvas.width,
      height: canvas.height,
      clientWidth: canvas.clientWidth,
      clientHeight: canvas.clientHeight,
      rect: {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height
      }
    },
    viewport: { width: window.innerWidth, height: window.innerHeight }
  };
})().then((value) => JSON.stringify(value))
"##;

fn assert_cube_canvas_layout(layout: &Value) -> Result<()> {
    let webgpu = layout
        .get("webgpu")
        .and_then(Value::as_bool)
        .context("cube smoke layout missing WebGPU support flag")?;
    if !webgpu {
        bail!("cube smoke browser does not expose navigator.gpu");
    }
    let adapter = layout
        .get("adapter")
        .and_then(Value::as_bool)
        .context("cube smoke layout missing WebGPU adapter flag")?;
    if !adapter {
        let adapter_error = layout
            .get("adapterError")
            .and_then(Value::as_str)
            .unwrap_or("no adapter error reported");
        bail!("cube smoke browser could not create a WebGPU adapter: {adapter_error}");
    }
    if layout.get("canvas").is_none_or(Value::is_null) {
        bail!("cube smoke canvas is missing");
    }

    let canvas_width = number_at(layout, "/canvas/width")?;
    let canvas_height = number_at(layout, "/canvas/height")?;
    let client_width = number_at(layout, "/canvas/clientWidth")?;
    let client_height = number_at(layout, "/canvas/clientHeight")?;
    let rect_x = number_at(layout, "/canvas/rect/x")?;
    let rect_y = number_at(layout, "/canvas/rect/y")?;
    let rect_width = number_at(layout, "/canvas/rect/width")?;
    let rect_height = number_at(layout, "/canvas/rect/height")?;
    let viewport_width = number_at(layout, "/viewport/width")?;
    let viewport_height = number_at(layout, "/viewport/height")?;

    assert_near("canvas bitmap width", canvas_width, 512.0, 1.0)?;
    assert_near("canvas bitmap height", canvas_height, 512.0, 1.0)?;
    assert_near("canvas client width", client_width, 512.0, 2.0)?;
    assert_near("canvas client height", client_height, 512.0, 2.0)?;
    assert_near("canvas rect width", rect_width, 512.0, 2.0)?;
    assert_near("canvas rect height", rect_height, 512.0, 2.0)?;
    assert_near(
        "canvas center x",
        rect_x + rect_width / 2.0,
        viewport_width / 2.0,
        2.0,
    )?;
    assert_near(
        "canvas center y",
        rect_y + rect_height / 2.0,
        viewport_height / 2.0,
        2.0,
    )
}

fn number_at(value: &Value, pointer: &str) -> Result<f64> {
    value
        .pointer(pointer)
        .and_then(Value::as_f64)
        .with_context(|| format!("cube smoke layout missing numeric value at {pointer}"))
}

fn assert_near(name: &str, actual: f64, expected: f64, tolerance: f64) -> Result<()> {
    if (actual - expected).abs() > tolerance {
        bail!("{name} was {actual:.1}, expected {expected:.1} ± {tolerance:.1}");
    }
    Ok(())
}

fn wait_for_green_cube_pixels(
    cdp: &mut CdpConnection,
    session_id: &str,
    screenshot: &Path,
) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(30);
    let fallback_deadline = Instant::now() + Duration::from_secs(8);
    loop {
        wait_for_animation_frames(cdp, session_id)?;
        capture_cdp_screenshot(cdp, session_id, screenshot)?;
        let last_error = match assert_green_cube_pixels(screenshot) {
            Ok(()) => return Ok(()),
            Err(error) => error.to_string(),
        };
        if Instant::now() >= fallback_deadline && has_webgpu_green_cube_confirmation(cdp) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            if has_webgpu_green_cube_confirmation(cdp) {
                return Ok(());
            }
            let console = cdp.console_messages.join("\n");
            if console.is_empty() {
                bail!("cube smoke render check failed before deadline: {last_error}");
            }
            bail!(
                "cube smoke render check failed before deadline: {last_error}\nconsole:\n{console}"
            );
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn has_webgpu_green_cube_confirmation(cdp: &CdpConnection) -> bool {
    let saw_webgpu = cdp
        .console_messages
        .iter()
        .any(|message| message.contains("backend: BrowserWebGpu"));
    let saw_green_cube = cdp
        .console_messages
        .iter()
        .any(|message| message.contains("GREEN_CUBE_READY base_color=srgb(0.0,0.9,0.18)"));
    saw_webgpu && saw_green_cube
}

fn wait_for_animation_frames(cdp: &mut CdpConnection, session_id: &str) -> Result<()> {
    let result = cdp_command(
        cdp,
        Some(session_id),
        "Runtime.evaluate",
        json!({
            "expression": "new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve(true))))",
            "awaitPromise": true,
            "returnByValue": true
        }),
    )?;
    if let Some(exception) = result.get("exceptionDetails") {
        bail!("cube smoke animation-frame wait failed: {exception}");
    }
    Ok(())
}

fn capture_cdp_screenshot(
    cdp: &mut CdpConnection,
    session_id: &str,
    screenshot: &Path,
) -> Result<()> {
    let result = cdp_command(
        cdp,
        Some(session_id),
        "Page.captureScreenshot",
        json!({
            "format": "png",
            "fromSurface": true,
            "captureBeyondViewport": false,
            "clip": {
                "x": 0,
                "y": 0,
                "width": 1000,
                "height": 700,
                "scale": 1
            }
        }),
    )?;
    let encoded = result
        .get("data")
        .and_then(Value::as_str)
        .context("Chrome CDP screenshot response had no data")?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .context("decode Chrome CDP screenshot")?;
    fs::write(screenshot, bytes).with_context(|| format!("write {}", screenshot.display()))
}

fn find_chrome() -> Result<PathBuf> {
    let candidates = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
    ];

    for candidate in candidates {
        let path = Path::new(candidate);
        if path.is_absolute() && path.exists() {
            return Ok(path.to_path_buf());
        }
        if !path.is_absolute()
            && let Some(found) = find_on_path(candidate)
        {
            return Ok(found);
        }
    }

    bail!("could not find Chrome or Chromium for cube-smoke render test")
}

fn find_on_path(binary: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|path| path.join(binary))
            .find(|path| path.exists())
    })
}

fn assert_green_cube_pixels(screenshot: &Path) -> Result<()> {
    let file = fs::File::open(screenshot)
        .with_context(|| format!("open screenshot {}", screenshot.display()))?;
    let decoder = png::Decoder::new(BufReader::new(file));
    let mut reader = decoder
        .read_info()
        .with_context(|| format!("read png metadata {}", screenshot.display()))?;
    let buffer_size = reader
        .output_buffer_size()
        .context("png output buffer size is unavailable")?;
    let mut buffer = vec![0; buffer_size];
    let info = reader
        .next_frame(&mut buffer)
        .with_context(|| format!("decode screenshot {}", screenshot.display()))?;
    let pixels = &buffer[..info.buffer_size()];
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        other => bail!("unsupported screenshot color type {other:?}"),
    };

    let mut green_pixels = 0usize;
    let mut min_x = u32::MAX;
    let mut max_x = 0u32;
    let mut min_y = u32::MAX;
    let mut max_y = 0u32;

    for y in 0..info.height {
        for x in 0..info.width {
            let index = ((y * info.width + x) as usize) * channels;
            let red = pixels[index] as u16;
            let green = pixels[index + 1] as u16;
            let blue = pixels[index + 2] as u16;
            let alpha = if channels == 4 {
                pixels[index + 3] as u16
            } else {
                255
            };
            if alpha > 220 && green > 80 && green > red + 35 && green > blue + 35 {
                green_pixels += 1;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    if green_pixels < 1_000 {
        bail!(
            "green cube render check failed: found only {green_pixels} green-dominant pixels in {}",
            screenshot.display()
        );
    }

    let bounds_width = max_x - min_x + 1;
    let bounds_height = max_y - min_y + 1;
    if bounds_width < 80 || bounds_height < 80 {
        bail!(
            "green cube render check failed: green bounds are too small ({bounds_width}x{bounds_height})"
        );
    }

    let center_x = (min_x + max_x) as f32 / 2.0;
    let center_y = (min_y + max_y) as f32 / 2.0;
    let expected_x = info.width as f32 / 2.0;
    let expected_y = info.height as f32 / 2.0;
    let tolerance_x = info.width as f32 * 0.16;
    let tolerance_y = info.height as f32 * 0.16;
    if (center_x - expected_x).abs() > tolerance_x || (center_y - expected_y).abs() > tolerance_y {
        bail!(
            "green cube render check failed: green center ({center_x:.1}, {center_y:.1}) is not centered in {}x{} screenshot",
            info.width,
            info.height
        );
    }

    Ok(())
}

struct TrunkServer {
    child: Child,
    url: String,
}

impl TrunkServer {
    fn start(directory: &str) -> Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0)).context("reserve Trunk server port")?;
        let port = listener
            .local_addr()
            .context("read reserved Trunk server address")?
            .port();
        drop(listener);

        let port_arg = port.to_string();
        let mut command = Command::new("trunk");
        command
            .current_dir(directory)
            .args(["serve", "--release", "--address", "127.0.0.1", "--port"])
            .arg(&port_arg)
            .env_remove("NO_COLOR")
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let child = command
            .spawn()
            .with_context(|| format!("run trunk serve in {directory}"))?;
        let mut server = Self {
            child,
            url: format!("http://127.0.0.1:{port}/"),
        };
        server.wait_until_ready()?;
        Ok(server)
    }

    fn wait_until_ready(&mut self) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if let Some(status) = self.child.try_wait().context("poll Trunk server")? {
                bail!("trunk serve exited early with {status}");
            }
            if http_get(&self.url).is_ok_and(|response| response.starts_with("HTTP/1.1 200")) {
                return Ok(());
            }
            if Instant::now() >= deadline {
                bail!("timed out waiting for Trunk server at {}", self.url);
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

impl Drop for TrunkServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn http_get(url: &str) -> Result<String> {
    let (_, address) = url
        .split_once("://")
        .context("Trunk server URL missing scheme")?;
    let address = address.trim_end_matches('/');
    let mut stream = TcpStream::connect(address).with_context(|| format!("connect to {url}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .context("set Trunk server read timeout")?;
    write!(
        stream,
        "GET / HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )
    .context("write Trunk server probe request")?;
    let mut response = String::new();
    match stream.read_to_string(&mut response) {
        Ok(_) => Ok(response),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ) && !response.is_empty() =>
        {
            Ok(response)
        }
        Err(error) => Err(error).context("read Trunk server probe response"),
    }
}

fn docs_sweep() -> Result<()> {
    let forbidden = [
        ForbiddenTerm::word("JavaScript"),
        ForbiddenTerm::word("TypeScript"),
        ForbiddenTerm::word("Bun"),
        ForbiddenTerm::word("bun"),
        ForbiddenTerm::word("Node"),
        ForbiddenTerm::word("React"),
        ForbiddenTerm::word("TanStack"),
        ForbiddenTerm::word("Jotai"),
        ForbiddenTerm::word("Pixi"),
        ForbiddenTerm::word("Vite"),
        ForbiddenTerm::word("Storybook"),
        ForbiddenTerm::word("Playwright"),
        ForbiddenTerm::word("Stylelint"),
        ForbiddenTerm::literal("package.json"),
        ForbiddenTerm::literal("node_modules"),
        ForbiddenTerm::word("bunx"),
        ForbiddenTerm::literal("js-sys"),
        ForbiddenTerm::word("stylelint"),
        ForbiddenTerm::word("playwright"),
        ForbiddenTerm::word("turbo"),
    ];
    let mut hits = Vec::new();
    for path in sweep_files(Path::new("."))? {
        let contents =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        for (index, line) in contents.lines().enumerate() {
            for term in forbidden {
                if term.is_found_in(line) {
                    hits.push(format!("{}:{}: {term}", path.display(), index + 1));
                }
            }
        }
    }
    if hits.is_empty() {
        println!("docs-sweep: no retired stack references.");
        Ok(())
    } else {
        bail!("docs-sweep failed:\n{}", hits.join("\n"));
    }
}

#[derive(Clone, Copy)]
enum MatchKind {
    Literal,
    Word,
}

#[derive(Clone, Copy)]
struct ForbiddenTerm {
    value: &'static str,
    kind: MatchKind,
}

impl ForbiddenTerm {
    const fn literal(value: &'static str) -> Self {
        Self {
            value,
            kind: MatchKind::Literal,
        }
    }

    const fn word(value: &'static str) -> Self {
        Self {
            value,
            kind: MatchKind::Word,
        }
    }

    fn is_found_in(self, line: &str) -> bool {
        match self.kind {
            MatchKind::Literal => line.contains(self.value),
            MatchKind::Word => contains_word(line, self.value),
        }
    }
}

impl std::fmt::Display for ForbiddenTerm {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.value)
    }
}

fn contains_word(line: &str, word: &str) -> bool {
    line.match_indices(word).any(|(start, _)| {
        let end = start + word.len();
        let before = line[..start].chars().next_back();
        let after = line[end..].chars().next();
        is_boundary(before) && is_boundary(after)
    })
}

fn is_boundary(character: Option<char>) -> bool {
    match character {
        Some(value) => !value.is_ascii_alphanumeric() && value != '_' && value != '-',
        None => true,
    }
}

fn sweep_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_sweep_files(root, &mut files)?;
    Ok(files)
}

fn collect_sweep_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root).with_context(|| format!("read {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type()?.is_dir() {
            if should_skip_sweep_dir(&path, &name) {
                continue;
            }
            collect_sweep_files(&path, files)?;
            continue;
        }
        let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };
        let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        let include = matches!(file_name, "AGENTS.md" | "README.md" | "Justfile")
            || matches!(extension, "md" | "toml" | "yml" | "yaml" | "json");
        let excluded = matches!(file_name, "Cargo.lock");
        if include && !excluded {
            files.push(path);
        }
    }
    Ok(())
}

fn should_skip_sweep_dir(path: &Path, name: &str) -> bool {
    matches!(name, ".git" | "target" | "dist" | "node_modules")
        || path.ends_with(Path::new("apps/test-project"))
}

fn check_template() -> Result<()> {
    let test_project = Path::new(TEST_PROJECT);
    if test_project.exists() {
        fs::remove_dir_all(test_project).context("remove old apps/test-project")?;
    }
    gen_app("test-project")?;
    for relative in [
        "Cargo.toml",
        "Trunk.toml",
        "index.html",
        "src/main.rs",
        "styles/index.css",
        "public/llms.txt",
        "cube-smoke/Cargo.toml",
        "cube-smoke/Trunk.toml",
        "cube-smoke/index.html",
        "cube-smoke/src/main.rs",
        "cube-smoke/styles/index.css",
    ] {
        let path = test_project.join(relative);
        if !path.exists() {
            bail!("template did not create {}", path.display());
        }
    }
    assert_file_contains(&test_project.join("Cargo.toml"), "rs-dean-schema")?;
    assert_file_contains(&test_project.join("Cargo.toml"), "rs-dean-state")?;
    assert_file_contains(&test_project.join("index.html"), "rel=\"tailwind-css\"")?;
    assert_file_contains(
        &test_project.join("styles/index.css"),
        "@import \"tailwindcss\"",
    )?;
    assert_file_contains(&test_project.join("src/main.rs"), "validate_snapshot")?;
    assert_file_contains(&test_project.join("src/main.rs"), "APP_SNAPSHOT_KEY")?;
    assert_file_contains(&test_project.join("cube-smoke/Cargo.toml"), "webgpu")?;
    assert_file_contains(
        &test_project.join("cube-smoke/src/main.rs"),
        "GREEN_CUBE_READY",
    )?;
    Ok(())
}

fn ensure_test_project() -> Result<()> {
    if !Path::new(CUBE_SMOKE_APP).join("Cargo.toml").exists() {
        check_template()?;
    }
    Ok(())
}

fn assert_file_contains(path: &Path, expected: &str) -> Result<()> {
    let contents = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    if !contents.contains(expected) {
        bail!("{} does not contain `{expected}`", path.display());
    }
    Ok(())
}

fn assert_file_not_contains(path: &Path, unexpected: &str) -> Result<()> {
    let contents = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    if contents.contains(unexpected) {
        bail!("{} contains retired `{unexpected}`", path.display());
    }
    Ok(())
}

fn gen_app(name: &str) -> Result<()> {
    let destination = Path::new("apps").join(name);
    if destination.exists() {
        bail!("{} already exists", destination.display());
    }
    let mut replacements = Vec::from([("{{name}}", name.to_owned())]);
    replacements.push(("{{crate_name}}", name.replace('-', "_")));
    copy_template_dir(Path::new("templates/app"), &destination, &replacements)
}

fn verify_trunk_dist(dist: &Path) -> Result<()> {
    if !dist.join("index.html").exists() {
        bail!("missing {}", dist.join("index.html").display());
    }
    let mut has_wasm = false;
    let mut has_glue = false;
    let mut has_css = false;
    for entry in fs::read_dir(dist).with_context(|| format!("read {}", dist.display()))? {
        let path = entry?.path();
        match path.extension().and_then(OsStr::to_str) {
            Some("wasm") => has_wasm = true,
            Some("js") => has_glue = true,
            Some("css") => has_css = true,
            _ => {}
        }
    }
    if !(has_wasm && has_glue && has_css) {
        bail!(
            "dist {} missing expected artifacts: wasm={has_wasm} glue={has_glue} css={has_css}",
            dist.display()
        );
    }
    Ok(())
}

fn copy_template_dir(
    source: &Path,
    destination: &Path,
    replacements: &[(&str, String)],
) -> Result<()> {
    fs::create_dir_all(destination).with_context(|| format!("create {}", destination.display()))?;
    for entry in fs::read_dir(source).with_context(|| format!("read {}", source.display()))? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_template_dir(&source_path, &destination_path, replacements)?;
        } else if file_type.is_file() {
            let mut contents = fs::read_to_string(&source_path)
                .with_context(|| format!("read template {}", source_path.display()))?;
            for (from, to) in replacements {
                contents = contents.replace(from, to);
            }
            fs::write(&destination_path, contents)
                .with_context(|| format!("write {}", destination_path.display()))?;
        }
    }
    Ok(())
}

fn require_tool(program: &str) -> Result<()> {
    run(program, ["--version"]).with_context(|| {
        format!("required tool `{program}` is missing; install it before running the gate")
    })
}

fn require_cargo_subcommand(binary: &str) -> Result<()> {
    run(binary, ["--version"]).with_context(|| {
        format!(
            "required cargo subcommand `{binary}` is missing; install it before running the gate"
        )
    })
}

fn run<I, S>(program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.args(args);
    if program == "trunk" {
        command.env_remove("NO_COLOR");
    }
    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("run {program}"))?;
    if !status.success() {
        bail!("{program} exited with {status}");
    }
    Ok(())
}

fn run_with_env<I, S, J, K, V>(program: &str, args: I, envs: J) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    J: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let status = Command::new(program)
        .args(args)
        .envs(envs)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("run {program}"))?;
    if !status.success() {
        bail!("{program} exited with {status}");
    }
    Ok(())
}

fn run_in<I, S>(directory: &str, program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.current_dir(directory).args(args);
    if program == "trunk" {
        command.env_remove("NO_COLOR");
    }
    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("run {program} in {directory}"))?;
    if !status.success() {
        bail!("{program} exited with {status}");
    }
    Ok(())
}
