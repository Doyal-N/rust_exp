mod dir;
mod file;

pub use dir::Dir;
pub use file::File;

pub trait Component {
    fn search(&self, keyword: &str);
}
