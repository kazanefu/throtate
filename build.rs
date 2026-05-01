use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // target/debug に辿る
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap();

    let dest = target_dir.join("assets/courses_ron");
    let src = Path::new("src/courses_ron");

    // 既存削除
    if dest.exists() {
        fs::remove_dir_all(&dest).unwrap();
    }

    // コピー
    copy_dir_all(src, &dest).unwrap();
}

// 再帰コピー
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}