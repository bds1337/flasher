
fn main() {
    println!("hi");
    gio::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "compiled.gresource",
        );
}
