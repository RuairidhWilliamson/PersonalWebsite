use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::LazyLock,
};

#[derive(Debug)]
enum PackageManager {
    Npm,
    Pnpm,
}

static PACKAGE_MANAGER: LazyLock<PackageManager> = LazyLock::new(|| {
    let pm = [PackageManager::Pnpm, PackageManager::Npm]
        .into_iter()
        .find(PackageManager::exists)
        .unwrap();
    pm.install();
    pm
});

impl PackageManager {
    fn exists(&self) -> bool {
        match self {
            PackageManager::Npm => Command::new("npm")
                .arg("--version")
                .status()
                .unwrap()
                .success(),
            PackageManager::Pnpm => Command::new("pnpm")
                .arg("--version")
                .status()
                .unwrap()
                .success(),
        }
    }

    fn install(&self) -> bool {
        println!("Install {self:?}");
        let mut command: Command;
        match self {
            PackageManager::Npm => {
                command = Command::new("npm");
                command.arg("install");
            }
            PackageManager::Pnpm => {
                command = Command::new("pnpm");
                command.arg("install");
            }
        }
        command.status().unwrap().success()
    }
}

pub fn minify_js(source: &str) -> Vec<u8> {
    let _ = &*PACKAGE_MANAGER;
    let mut cmd = Command::new("node_modules/terser/bin/terser")
        .arg("--compress")
        .arg("--mangle")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::scope(|s| {
        let mut stdin = cmd.stdin.take().unwrap();
        s.spawn(move || {
            write!(stdin, "{}", source).unwrap();
        });
        let mut stdout = cmd.stdout.take().unwrap();
        let mut v = Vec::new();
        stdout.read_to_end(&mut v).unwrap();
        v
    })
}

pub fn minify_html(source: &str) -> Vec<u8> {
    let _ = &*PACKAGE_MANAGER;
    let mut cmd = Command::new("node_modules/html-minifier/cli.js")
        .arg("--collapse-whitespace")
        .arg("--remove-comments")
        .arg("--remove-optional-tags")
        .arg("--remove-redundant-attributes")
        .arg("--remove-script-type-attributes")
        .arg("--remove-tag-whitespace")
        .arg("--use-short-doctype")
        .arg("--minify-css")
        .arg("--minify-js")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::scope(|s| {
        let mut stdin = cmd.stdin.take().unwrap();
        s.spawn(move || {
            write!(stdin, "{}", source).unwrap();
        });
        let mut stdout = cmd.stdout.take().unwrap();
        let mut v = Vec::new();
        stdout.read_to_end(&mut v).unwrap();
        v
    })
}

pub fn minify_css(source: &str) -> Vec<u8> {
    let _ = &*PACKAGE_MANAGER;
    let mut cmd = Command::new("node_modules/clean-css-cli/bin/cleancss")
        .arg("-O2")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::scope(|s| {
        let mut stdin = cmd.stdin.take().unwrap();
        s.spawn(move || {
            write!(stdin, "{}", source).unwrap();
        });
        let mut stdout = cmd.stdout.take().unwrap();
        let mut v = Vec::new();
        stdout.read_to_end(&mut v).unwrap();
        v
    })
}