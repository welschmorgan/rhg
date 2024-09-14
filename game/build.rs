const DEFAULT_STYLE: &'static str = "fluent-dark";
const DEFAULT_BACKEND: &'static str = "GL";

fn main() {
  let style = std::env::var("SLINT_STYLE")
    .ok()
    .unwrap_or_else(|| DEFAULT_STYLE.to_string());

  let backend = std::env::var("SLINT_BACKEND")
    .ok()
    .unwrap_or_else(|| DEFAULT_BACKEND.to_string());

  println!("cargo::rustc-env={}={}", "SLINT_BACKEND", backend);
  #[cfg(debug_assertions)]
  println!(
    "cargo::rustc-env={}={}",
    "SLINT_DEBUG_PERFORMANCE", "refresh_lazy,overlay "
  );

  slint_build::compile_with_config(
    "ui/UI.slint",
    slint_build::CompilerConfiguration::new().with_style(style),
  )
  .unwrap()
}
