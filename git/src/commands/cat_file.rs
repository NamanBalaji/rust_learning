use std::ffi::CStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::copy;

use anyhow::Context;
use flate2::read::ZlibDecoder;

enum Kind {
    Blob,
}

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        pretty_print,
        "mode must be given without -p and we don't support mode"
    );

    // TODO: support shortest unique object hashes
    let f = File::open(format!(
        ".git/objects/{}/{}",
        &object_hash[..2],
        &object_hash[2..]
    ))
    .context("open in ./git/objects")?;

    let z = ZlibDecoder::new(f);
    let mut z = BufReader::new(z);

    let mut buf = Vec::new();
    z.read_until(0, &mut buf)
        .context("read header from .git/objects")?;

    let header =
        CStr::from_bytes_with_nul(&buf).expect("know that there is exactly one nul, at the end");
    let header = header
        .to_str()
        .context(".git/objects file header isnt a valid UTF-8")?;

    let Some((kind, size)) = header.split_once(' ') else {
        anyhow::bail!(".git/objects file header did not start with a known type: '{header}'");
    };
    let kind = match kind {
        "blob" => Kind::Blob,
        _ => anyhow::bail!("we do not yet know how to print a '{kind}'"),
    };

    let size = size
        .parse::<u64>()
        .context(".git/objects file header has invalid size: {size}")?;

    let mut z = z.take(size);
    match kind {
        Kind::Blob => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n = copy(&mut z, &mut stdout).context("write .git/objects file to stdout")?;
            anyhow::ensure!(
                n == size,
                ".git/object file was npt the expected size (expected: {size}, actual: {n} )"
            );
        }
    }

    Ok(())
}
