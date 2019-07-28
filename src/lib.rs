use std::thread;

/// Main trait
pub trait Unchained
where
    Self: Iterator + Sized,
{
    /// apply the provided function to each item in the iterator using a thread per item
    fn unchained_for_each<F>(self, f: F) -> UnchainedForEach<Self, F>
    where
        F: FnMut(Self::Item) + Send + Sync + 'static,
    {
        UnchainedForEach::new(self, f)
    }
}

impl<I: Iterator> Unchained for I {}

/// this `struct` is created by [`unchained_for_each`] method on `Iterator`
///
/// [`unchained_for_each`]: trait.Unchained.html#method.unchained_for_each
pub struct UnchainedForEach<I: Iterator, F: FnMut(I::Item) + Send + Sync + 'static> {
    iter: I,
    f: F,
}

impl<I: Iterator, F: FnMut(I::Item) + Send + Sync + 'static> UnchainedForEach<I, F> {
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I: Iterator, F> Iterator for UnchainedForEach<I, F>
where
    F: FnMut(I::Item) + Sized + Send + Sync + Clone + 'static,
    I::Item: Send + 'static,
{
    type Item = thread::JoinHandle<()>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.iter.next() {
            Some(next) => next,
            None => return None,
        };
        let mut f = self.f.clone();
        Some(thread::spawn(move || {
            f(next);
        }))
    }
}

/// trait to join all of the iterator threads easily
pub trait Finisher {
    fn join(self);
}

impl<T> Finisher for T
where
    T: Iterator<Item = thread::JoinHandle<()>>,
{
    /// join all of the iterator threads
    fn join(self) {
        self.collect::<Vec<thread::JoinHandle<()>>>()
            .into_iter()
            .for_each(|t| {
                let _ = t.join();
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ddos_all_test() {
        let targets = vec!["192.168.1.2", "192.168.1.3", "192.168.1.4", "192.168.1.5"];
        ddos_all(targets);
    }

    fn ddos_all(targets: Vec<&'static str>) {
        targets
            .into_iter()
            .unchained_for_each(|t| {
                std::process::Command::new("ping")
                    .arg(t)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            })
            .join();
    }

    #[test]
    fn download_all_test() {
        let pages = vec![
            "https://doc.rust-lang.org/stable/std/",
            "https://doc.rust-lang.org/stable/std/#modules",
            "https://doc.rust-lang.org/stable/std/#primitives",
            "https://doc.rust-lang.org/stable/std/#macros",
            "https://doc.rust-lang.org/stable/std/prelude/index.html",
            "https://doc.rust-lang.org/stable/book/ch03-02-data-types.html",
        ];
        download_all(pages);
    }

    fn download_all(pages: Vec<&'static str>) {
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
            })
            .join();
    }
}
