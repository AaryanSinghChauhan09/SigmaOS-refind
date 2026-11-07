//! refind model — rEFInd boot manager.

#[derive(Debug, Clone)]
pub struct Entry {
    pub title: String,
    pub kernel: String,
    pub initrd: String,
    pub params: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub timeout: u32,
    pub default: usize,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub config: Config,
}

impl Default for Model {
    fn default() -> Self { Self { config: Config { timeout: 5, default: 0, entries: vec![] } } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_entry(&mut self, title: &str, kernel: &str, initrd: &str, params: &str, icon: &str) {
        self.config.entries.push(Entry { title: title.into(), kernel: kernel.into(), initrd: initrd.into(), params: params.into(), icon: icon.into() });
    }

    pub fn set_default(&mut self, index: usize) -> bool {
        if index < self.config.entries.len() { self.config.default = index; true } else { false }
    }

    pub fn boot(&self) -> Option<&Entry> {
        self.config.entries.get(self.config.default)
    }

    pub fn list(&self) -> &[Entry] { &self.config.entries }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entry_default_boot() {
        let mut r = Model::new();
        r.add_entry("Linux", "/vmlinuz", "/initrd", "root=/dev/sda1", "linux.png");
        r.add_entry("Windows", "/bootmgfw", "", "", "windows.png");
        assert!(r.set_default(0));
        assert_eq!(r.boot().unwrap().title, "Linux");
    }
}
