//! PC side of gamepad-wheel. Phase 0: only proves the toolchain and CI work.

fn banner() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

fn main() {
    println!("{}", banner());
}

#[cfg(test)]
mod tests {
    #[test]
    fn banner_names_the_program() {
        assert!(super::banner().starts_with("gamepad-wheel "));
    }
}
