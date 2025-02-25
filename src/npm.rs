use std::{
    io::Write as _,
    process::{Command, Stdio},
    sync::LazyLock,
};

use anyhow::{Context as _, Result};

#[derive(Debug)]
enum PackageManager {
    Npm,
    Pnpm,
}

static PACKAGE_MANAGER: LazyLock<PackageManager> = LazyLock::new(|| {
    let pm = [PackageManager::Pnpm, PackageManager::Npm]
        .into_iter()
        .find(PackageManager::exists)
        .expect("npm / pnpm not found");
    pm.install();
    pm
});

impl PackageManager {
    fn exists(&self) -> bool {
        match self {
            Self::Npm => Command::new("npm")
                .arg("--version")
                .status()
                .is_ok_and(|s| s.success()),
            Self::Pnpm => Command::new("pnpm")
                .arg("--version")
                .status()
                .is_ok_and(|s| s.success()),
        }
    }

    fn install(&self) -> bool {
        log::info!("Install {self:?}");
        let mut command: Command;
        match self {
            Self::Npm => {
                command = Command::new("npm");
                command.arg("install");
            }
            Self::Pnpm => {
                command = Command::new("pnpm");
                command.arg("install");
            }
        }
        command.status().is_ok_and(|x| x.success())
    }
}

pub fn minify_html(source: &str) -> Result<Vec<u8>> {
    let _ = &*PACKAGE_MANAGER;
    pipe_cmd(
        Command::new("node_modules/html-minifier/cli.js")
            .arg("--collapse-whitespace")
            .arg("--remove-comments")
            .arg("--remove-optional-tags")
            .arg("--remove-redundant-attributes")
            .arg("--remove-script-type-attributes")
            .arg("--remove-tag-whitespace")
            .arg("--use-short-doctype")
            .arg("--minify-css")
            .arg("--minify-js"),
        source,
    )
}

pub fn minify_css(source: &str) -> Result<Vec<u8>> {
    let _ = &*PACKAGE_MANAGER;
    pipe_cmd(
        Command::new("node_modules/clean-css-cli/bin/cleancss").arg("-O2"),
        source,
    )
}

fn pipe_cmd(cmd: &mut Command, input: &str) -> Result<Vec<u8>> {
    let mut cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    std::thread::scope(|s| {
        let mut stdin = cmd.stdin.take().context("take stdin")?;
        let handle = s.spawn(move || -> Result<(), std::io::Error> {
            write!(stdin, "{input}")?;
            Ok(())
        });
        let out = cmd.wait_with_output()?.stdout;
        handle.join().expect("thread panicked")?;
        Result::Ok(out)
    })
}
