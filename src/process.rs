use needletail::{parse_fastx_file};
use rustc_hash::FxHashMap as HashMap;
use serde_json::json;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::path::Path;

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
) -> Result<(), Box<dyn Error>> {

 	let mut complexity_content = HashMap::default();
 	let mut total_mismatch_count = 0_u64;
 	let mut total_base_count = 0_u64;
    let mut read_count = 0_u64;
    let mut reader = parse_fastx_file(&filename).expect("Invalid path/file");

    // Gather data from every record
    while let Some(record) = reader.next() {

        if let Ok(seqrec) = record {

            read_count += 1;
			total_base_count += seqrec.num_bases() as u64;

            let sequence = seqrec.seq();

            let mut mismatch_count = 0;

			for (offset, chara) in sequence.iter().enumerate() {

				if offset < seqrec.num_bases() - 1 {

					if &sequence[offset+1] != chara {
						mismatch_count += 1;
					}
				}
			}

			total_mismatch_count += mismatch_count;

			let complexity = mismatch_count as f32 / seqrec.num_bases() as f32;
			complexity_content.insert(read_count, complexity);

        } else {
        }

    }

    let file = Path::new(&filename).file_name().unwrap().to_str().unwrap();

    let meta = json!({
        "file name": file,
        "total reads": read_count,
        "complexity_per_read": complexity_content,
        "complexity": total_mismatch_count as f32 / total_base_count as f32,
    });

    io::stdout().write_all(meta.to_string().as_bytes())?;
    Ok(())

}

