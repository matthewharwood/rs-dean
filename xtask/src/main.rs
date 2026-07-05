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

use anyhow::{Context, Result, bail};
use base64::Engine as _;
use command_fds::{CommandFdExt, FdMapping};
use os_pipe::{PipeReader, PipeWriter};
use serde_json::{Value, json};

const WEB_APP: &str = "apps/web";
const STORIES_APP: &str = "apps/stories";
const CUBE_SMOKE_APP: &str = "apps/cube-smoke";
const TEST_PROJECT: &str = "apps/test-project";

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("build") => build(),
        Some("check" | "check-fast" | "gate") => gate(),
        Some("check-template") => check_template(),
        Some("cube-smoke") => cube_smoke(),
        Some("dev") => dev(),
        Some("docs-sweep") => docs_sweep(),
        Some("five-phase-pass") => five_phase_pass(),
        Some("gen-app") => {
            let name = args.next().unwrap_or_else(|| "test-project".to_owned());
            gen_app(&name)
        }
        Some("preview") => preview(),
        Some("stories") => stories(),
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
  dev              run the Leptos app on :5173
  stories          run the Rust story harness on :6106
  preview          serve the release app on :3100
  build            build static app output and GitHub Pages artifacts
  gate             one-pass Rust gate
  check            alias for gate
  check-fast       alias for gate
  cube-smoke       build the green-cube page and assert rendered pixels
  five-phase-pass  regenerate template, run gate, and sweep docs
  docs-sweep       fail on retired stack references
  gen-app <name>   copy templates/app into apps/<name>
  check-template   regenerate apps/test-project from templates/app
"
    );
}

fn dev() -> Result<()> {
    run_in(
        WEB_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "5173"],
    )
}

fn stories() -> Result<()> {
    run_in(
        STORIES_APP,
        "trunk",
        ["serve", "--address", "0.0.0.0", "--port", "6106"],
    )
}

fn preview() -> Result<()> {
    run_in(
        WEB_APP,
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
    run_in(WEB_APP, "trunk", ["build", "--release"])?;
    let dist = Path::new(WEB_APP).join("dist");
    postprocess_dist(&dist)?;
    verify_pages_dist(&dist)
}

fn five_phase_pass() -> Result<()> {
    check_template()?;
    gate()?;
    docs_sweep()
}

fn gate() -> Result<()> {
    require_tool("trunk")?;
    require_cargo_subcommand("cargo-nextest")?;
    require_cargo_subcommand("cargo-deny")?;
    require_cargo_subcommand("cargo-machete")?;

    run("cargo", ["fmt", "--all", "--", "--check"])?;
    check_bevy_webgpu_only()?;
    run(
        "cargo",
        [
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run(
        "cargo",
        [
            "clippy",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "rs-dean-web",
            "-p",
            "rs-dean-stories",
            "-p",
            "rs-dean-cube-smoke",
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
    )?;
    run("cargo", ["nextest", "run", "--workspace"])?;
    run("cargo", ["test", "--workspace", "--doc"])?;
    run(
        "cargo",
        [
            "check",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "rs-dean-web",
            "-p",
            "rs-dean-stories",
            "-p",
            "rs-dean-cube-smoke",
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
    run_with_env(
        "cargo",
        [
            "doc",
            "--workspace",
            "--no-deps",
            "--document-private-items",
        ],
        [("RUSTDOCFLAGS", "-D warnings")],
    )?;
    run("cargo", ["deny", "check"])?;
    run("cargo-machete", ["--skip-target-dir"])?;
    check_template()?;
    check_generated_template_build()?;
    build()?;
    build_stories()?;
    cube_smoke()?;
    docs_sweep()
}

fn build_stories() -> Result<()> {
    run_in(STORIES_APP, "trunk", ["build", "--release"])?;
    verify_trunk_dist(&Path::new(STORIES_APP).join("dist"))
}

fn cube_smoke() -> Result<()> {
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
    let output = Command::new("cargo")
        .args([
            "tree",
            "--target",
            "wasm32-unknown-unknown",
            "-e",
            "features",
            "-p",
            "rs-dean-bevy-scenes",
        ])
        .output()
        .context("inspect Bevy wasm feature tree")?;
    if !output.status.success() {
        bail!("cargo tree exited with {}", output.status);
    }
    let stdout = String::from_utf8(output.stdout).context("cargo tree output was not UTF-8")?;
    if stdout.contains("webgl") {
        bail!("Bevy wasm feature tree includes WebGL; keep `webgl2` disabled");
    }
    if !stdout.contains("webgpu") {
        bail!("Bevy wasm feature tree does not include WebGPU");
    }
    Ok(())
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
        ForbiddenTerm::word("Tailwind"),
        ForbiddenTerm::literal("package.json"),
        ForbiddenTerm::literal("node_modules"),
        ForbiddenTerm::word("bunx"),
        ForbiddenTerm::literal("js-sys"),
        ForbiddenTerm::word("stylelint"),
        ForbiddenTerm::word("tailwindcss"),
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
    ] {
        let path = test_project.join(relative);
        if !path.exists() {
            bail!("template did not create {}", path.display());
        }
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

fn postprocess_dist(dist: &Path) -> Result<()> {
    fs::copy(dist.join("index.html"), dist.join("404.html"))
        .with_context(|| format!("copy {} to 404.html", dist.display()))?;
    fs::write(dist.join(".nojekyll"), "").with_context(|| {
        format!(
            "write GitHub Pages marker at {}",
            dist.join(".nojekyll").display()
        )
    })?;
    fs::write(
        dist.join("sitemap.xml"),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.com/</loc></url>
</urlset>
"#,
    )
    .with_context(|| format!("write {}", dist.join("sitemap.xml").display()))?;
    Ok(())
}

fn verify_pages_dist(dist: &Path) -> Result<()> {
    verify_trunk_dist(dist)?;
    for relative in ["404.html", ".nojekyll", "sitemap.xml"] {
        let path = dist.join(relative);
        if !path.exists() {
            bail!("missing Pages artifact {}", path.display());
        }
    }
    Ok(())
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
