fn main() {
    let wasm = std::fs::read("target/wasm32-unknown-unknown/release/keystrokes_custom.wasm")
        .expect("read core wasm");
    let comp = wit_component::ComponentEncoder::default()
        .module(&wasm)
        .expect("set module")
        .validate(true)
        .encode()
        .expect("encode component");
    std::fs::write("mod.wasm", comp).expect("write mod.wasm");
    println!("Component mod.wasm encoded successfully!");
}
