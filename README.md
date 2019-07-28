# Unchained
Trait that allows iterating on a collection while appliyng a function with a thread per item

## Example

```rust
use unchained::Unchained;

// download all the pages at the same time by using a thread per item
fn download_all {
    let pages = vec![
        "https://doc.rust-lang.org/stable/std/",
        "https://doc.rust-lang.org/stable/std/#modules",
        "https://doc.rust-lang.org/stable/std/#primitives",
        "https://doc.rust-lang.org/stable/std/#macros",
        "https://doc.rust-lang.org/stable/std/prelude/index.html",
        "https://doc.rust-lang.org/stable/book/ch03-02-data-types.html",
    ];
    download_all_inner(pages);
}

fn download_all_inner(pages: Vec<&'static str>) {
    pages
        .into_iter()
        .unchained_for_each(|page| {
            println!(
                "{}:\n\n{}",
                &page,
                String::from_utf8(
                    std::process::Command::new("curl")
                        .arg(&page)
                        .output()
                        .unwrap()
                        .stdout
                )
                .unwrap()
            );
            println!("\n#######################\n");
        });
}
```
You can also checkout [rustman](https://github.com/sigmaSd/rustman) that uses `Unchained` to parallelize search requests
