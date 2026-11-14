fn main() {
    println!("cargo:rerun-if-changed=assets/helixbench.ico");
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/helixbench.ico");
        if let Err(e) = res.compile() {
            eprintln!("warning: could not embed icon: {e}");
        }
    }
}