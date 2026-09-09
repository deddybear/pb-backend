pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n=== APLIKASI CRASH ===");
        eprintln!("{panic_info}");
        eprintln!("\nTekan ENTER untuk menutup window...");

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
    }));
}