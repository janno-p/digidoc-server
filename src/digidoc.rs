#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("digidoc-server/include/digidoc.hpp");

        type DigiDoc;

        pub fn initialize() -> ();
        pub fn terminate() -> ();

        pub fn container_open_ptr(path: &str) -> Result<UniquePtr<DigiDoc>>;
    }
}

pub struct LibDigiDoc;

impl LibDigiDoc {
    pub fn load() -> Self {
        ffi::initialize();
        println!("DigiDoc initialized ...");
        Self
    }
}

impl Drop for LibDigiDoc {
    fn drop(&mut self) {
        ffi::terminate();
        println!("DigiDoc terminated!");
    }
}
