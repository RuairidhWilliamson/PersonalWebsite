use std::{collections::HashMap, hash::Hash, num::NonZeroUsize};

use anyhow::Result;

use crate::{Cache, JobCtx, JobId, RootJobOutput};

#[derive(Default)]
struct CallCounter {
    call_count: HashMap<&'static str, usize>,
}

impl Hash for CallCounter {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl CallCounter {
    fn inc(&mut self, name: &'static str) {
        let e = self.call_count.entry(name).or_default();
        *e += 1;
    }

    fn count(&self, name: &'static str) -> usize {
        self.call_count.get(name).cloned().unwrap_or_default()
    }
}

#[test]
fn basic_job() {
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("parent_job", 0), |_ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            Ok(())
        })
    }

    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);
}

#[test]
fn basic_child_job() {
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("parent_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            child_job(ctx, sys)?;
            child_job(ctx, sys)?;
            Ok(())
        })
    }

    fn child_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("child_job", 0), |_ctx: &mut JobCtx<'_>| {
            println!("Run child");
            sys.inc("child_job");
            Ok(())
        })
    }

    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);
    assert_eq!(sys.count("child_job"), 1);

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);
    assert_eq!(sys.count("child_job"), 1);
}

#[test]
fn basic_job_deps() {
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("parent_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            ctx.depends_file("test1.txt")?;
            Ok(())
        })
    }

    std::fs::write("test1.txt", "abc").unwrap();
    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 1, "parent_job called");

    std::fs::write("test1.txt", "abcdef").unwrap();

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 2, "parent_job called");
}

#[test]
fn basic_child_job_deps() {
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("parent_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            child_job(ctx, sys)?;
            Ok(())
        })
    }

    fn child_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("child_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run child");
            sys.inc("child_job");
            ctx.depends_file("test_basic_child_job_deps.txt")?;
            Ok(())
        })
    }

    std::fs::write("test_basic_child_job_deps.txt", "abc").unwrap();
    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 1, "parent_job called");
    assert_eq!(sys.count("child_job"), 1, "child_job called");

    std::fs::write("test_basic_child_job_deps.txt", "abcdef").unwrap();

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 2, "parent_job called");
    assert_eq!(sys.count("child_job"), 2, "child_job called");
}

#[test]
fn advanced_child_job_deps() {
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("parent_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            ctx.depends_file("test_advanced_child_job_deps_parent.txt")?;
            child_job(ctx, sys)?;
            child_job(ctx, sys)?;
            Ok(())
        })
    }

    fn child_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        ctx.job(JobId::new("child_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run child");
            sys.inc("child_job");
            ctx.depends_file("test_advanced_child_job_deps_child.txt")?;
            Ok(())
        })
    }

    std::fs::write("test_advanced_child_job_deps_parent.txt", "abc").unwrap();
    std::fs::write("test_advanced_child_job_deps_child.txt", "abc").unwrap();
    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 1, "parent_job called");
    assert_eq!(sys.count("child_job"), 1, "child_job called");

    std::fs::write("test_advanced_child_job_deps_child.txt", "abcdef").unwrap();

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 2, "parent_job called");
    assert_eq!(sys.count("child_job"), 2, "child_job called");

    std::fs::write("test_advanced_child_job_deps_parent.txt", "abcdef").unwrap();

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();

    assert_eq!(sys.count("parent_job"), 3, "parent_job called");
    assert_eq!(sys.count("child_job"), 2, "child_job called");
}

#[test]
fn basic_job_macro() {
    // Required to make macro work
    use crate as jobber;

    #[jobber_derive::job]
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter) -> Result<()> {
        println!("Run parent");
        sys.inc("parent_job");
        Ok(())
    }

    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());
    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);

    parent_job(&mut cache.root_ctx(&()), &mut sys).unwrap();
    assert_eq!(sys.count("parent_job"), 1);
}

#[test]
fn root_job() {
    fn parent_job(cache: &Cache, sys: &mut CallCounter) -> Result<RootJobOutput<()>> {
        cache.root_job(JobId::new("parent_job", 0), |ctx: &mut JobCtx<'_>| {
            println!("Run parent");
            sys.inc("parent_job");
            ctx.depends_file("test_root_job.txt")?;
            Ok(())
        })
    }

    std::fs::write("test_root_job.txt", "abc").unwrap();
    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());
    let h1 = parent_job(&cache, &mut sys).unwrap().hash;
    assert_eq!(sys.count("parent_job"), 1);

    std::fs::write("test_root_job.txt", "abcdef").unwrap();

    let h2 = parent_job(&cache, &mut sys).unwrap().hash;
    assert_eq!(sys.count("parent_job"), 2);
    assert_ne!(h1, h2);
}

#[test]
fn basic_job_args_macro() {
    // Required to make macro work
    use crate as jobber;

    #[jobber_derive::job]
    fn parent_job(ctx: &mut JobCtx<'_>, sys: &mut CallCounter, x: usize) -> Result<()> {
        println!("Run parent {x}");
        sys.inc("parent_job");
        Ok(())
    }

    let mut sys = CallCounter::default();
    let cache = crate::Cache::new(NonZeroUsize::new(16).unwrap());
    parent_job(&mut cache.root_ctx(&()), &mut sys, 0).unwrap();
    assert_eq!(sys.count("parent_job"), 1);
    parent_job(&mut cache.root_ctx(&()), &mut sys, 1).unwrap();
    assert_eq!(sys.count("parent_job"), 2);
    parent_job(&mut cache.root_ctx(&()), &mut sys, 0).unwrap();
    assert_eq!(sys.count("parent_job"), 2);
    parent_job(&mut cache.root_ctx(&()), &mut sys, 1).unwrap();
    assert_eq!(sys.count("parent_job"), 2);
}
