// Lean4 subprocess management

use moonmath_types::{CompileError, CompileRequest, CompileResponse, ErrorSeverity};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum LeanCompileError {
    #[error("Lean4 binary not found — install from https://leanprover.github.io")]
    LeanNotFound,
    #[error("Lake binary not found — install from https://leanprover.github.io")]
    LakeNotFound,
    #[error("Lake project not ready at {0} — run `lake update && lake exe cache get`")]
    LakeProjectNotReady(PathBuf),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Compilation timed out after {0} seconds")]
    Timeout(u64),
}

/// Configuration for the Lean compiler.
#[derive(Debug, Clone)]
pub struct LeanConfig {
    /// Path to the Lake project directory (for Mathlib imports).
    pub lake_project_path: PathBuf,
    /// Compilation timeout in seconds.
    pub timeout_secs: u64,
}

impl Default for LeanConfig {
    fn default() -> Self {
        // Resolve lean-project/ relative to the workspace root.
        // At runtime the binary is in `target/…/moonmath-app`, so we walk
        // up from the executable or use the CARGO_MANIFEST_DIR fallback.
        let lake_project_path = find_lean_project().unwrap_or_else(|| PathBuf::from("lean-project"));
        Self {
            lake_project_path,
            timeout_secs: 120,
        }
    }
}

/// Walk up from the current working directory looking for `lean-project/lakefile.lean`.
fn find_lean_project() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();
    loop {
        let candidate = dir.join("lean-project").join("lakefile.lean");
        if candidate.exists() {
            return Some(dir.join("lean-project"));
        }
        dir = dir.parent()?;
    }
}

pub struct LeanCompiler {
    config: LeanConfig,
}

impl LeanCompiler {
    /// Create a compiler with default configuration.
    pub fn with_defaults() -> Self {
        Self {
            config: LeanConfig::default(),
        }
    }

    /// Create a compiler with custom configuration.
    pub fn with_config(config: LeanConfig) -> Self {
        Self { config }
    }

    /// Check whether the `lean` binary is available on PATH.
    pub async fn is_available() -> bool {
        tokio::process::Command::new("lean")
            .arg("--version")
            .output()
            .await
            .is_ok()
    }

    /// Check whether the `lake` binary is available on PATH.
    async fn is_lake_available() -> bool {
        tokio::process::Command::new("lake")
            .arg("--version")
            .output()
            .await
            .is_ok()
    }

    /// Detect whether code needs Lake context (imports Mathlib, Std, etc.).
    fn needs_lake_context(code: &str) -> bool {
        code.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("import Mathlib")
                || trimmed.starts_with("import Std")
                || trimmed.starts_with("import Batteries")
        })
    }

    /// Compile a Lean4 source string.
    pub async fn compile(&self, req: CompileRequest) -> Result<CompileResponse, LeanCompileError> {
        if !Self::is_available().await {
            return Err(LeanCompileError::LeanNotFound);
        }

        if Self::needs_lake_context(&req.code) {
            self.compile_with_lake(&req.code).await
        } else {
            self.compile_standalone(&req.code).await
        }
    }

    /// Compile using standalone `lean` (no Lake project needed).
    async fn compile_standalone(&self, code: &str) -> Result<CompileResponse, LeanCompileError> {
        let tmp_dir = std::env::temp_dir();
        let tmp_file = tmp_dir.join(format!("moonmath_{}.lean", std::process::id()));
        let result = self.run_lean_cmd("lean", &[tmp_file.to_str().unwrap()], code, &tmp_file, None).await;
        let _ = tokio::fs::remove_file(&tmp_file).await;
        result
    }

    /// Compile using `lake env lean` inside the Lake project directory.
    async fn compile_with_lake(&self, code: &str) -> Result<CompileResponse, LeanCompileError> {
        if !Self::is_lake_available().await {
            return Err(LeanCompileError::LakeNotFound);
        }

        let project_path = &self.config.lake_project_path;
        if !project_path.join("lakefile.lean").exists() {
            return Err(LeanCompileError::LakeProjectNotReady(project_path.clone()));
        }

        // Write scratch file inside the Lake project so it can resolve deps
        let scratch_file = project_path.join(format!("_scratch_{}.lean", std::process::id()));
        let result = self
            .run_lean_cmd(
                "lake",
                &["env", "lean", scratch_file.to_str().unwrap()],
                code,
                &scratch_file,
                Some(project_path),
            )
            .await;
        let _ = tokio::fs::remove_file(&scratch_file).await;
        result
    }

    /// Run a lean compilation command and parse the output.
    async fn run_lean_cmd(
        &self,
        program: &str,
        args: &[&str],
        code: &str,
        tmp_file: &Path,
        working_dir: Option<&Path>,
    ) -> Result<CompileResponse, LeanCompileError> {
        tokio::fs::write(tmp_file, code).await?;

        let mut cmd = tokio::process::Command::new(program);
        cmd.args(args);
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        let output = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.timeout_secs),
            cmd.output(),
        )
        .await
        .map_err(|_| LeanCompileError::Timeout(self.config.timeout_secs))??;

        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let errors = parse_lean_errors(&stderr);

        let success = output.status.success()
            && !errors
                .iter()
                .any(|e| matches!(e.severity, ErrorSeverity::Error));

        Ok(CompileResponse {
            success,
            output: if stdout.is_empty() {
                None
            } else {
                Some(stdout.into_owned())
            },
            errors,
            latex: None,
        })
    }
}

/// Parse Lean4 error output.
/// Format: `<file>:<line>:<col>: error|warning|information: <message>`
fn parse_lean_errors(stderr: &str) -> Vec<CompileError> {
    let mut errors = Vec::new();
    for line in stderr.lines() {
        // Skip lines that don't look like lean diagnostics
        let parts: Vec<&str> = line.splitn(5, ':').collect();
        if parts.len() < 5 {
            continue;
        }
        let line_num = match parts[1].trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let col = match parts[2].trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let severity_str = parts[3].trim();
        let severity = match severity_str {
            "error" => ErrorSeverity::Error,
            "warning" => ErrorSeverity::Warning,
            "information" => ErrorSeverity::Info,
            _ => continue,
        };
        let message = parts[4].trim().to_string();
        errors.push(CompileError {
            line: line_num,
            column: col,
            message,
            severity,
        });
    }
    errors
}
