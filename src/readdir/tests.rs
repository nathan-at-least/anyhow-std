use crate::PathAnyhow;
use anyhow::anyhow as error;
use anyhow::Context;

#[test]
fn readdir_item_err() -> anyhow::Result<()> {
    // Setup:
    let tempdir = tempfile::TempDir::new()?;
    let dir = tempdir.path();
    let dirstr = dir.display().to_string();

    dir.join("a").write_anyhow("apple")?;
    dir.join("b").write_anyhow("banana")?;

    // Begin iteration:
    let mut rd = dir.read_dir_anyhow()?;

    let res = rd
        .next()
        .ok_or_else(|| error!("unexpected end of iteration"))
        .context(dirstr.clone());

    // Assert we successfully read an entry:
    assert!(res.is_ok());

    // Set up an error condition for iteration by removing dir:
    println!("Removing {:?}", &dirstr);
    dir.remove_dir_all_anyhow()?;

    let res2 = rd
        .next()
        .ok_or_else(|| error!("unexpected end of iteration"))
        .context(dirstr.clone());

    if let Some(err) = res2.as_ref().err() {
        assert_eq!(
            format!("{:#}", err),
            format!("while processing path {:?}: FIXME", dir.display()),
        );
        Ok(())
    } else {
        Err(error!("unexpected successful dir iteration: {res2:#?}"))
    }
}
