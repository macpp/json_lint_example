fn main(){
    let analysis = rls_data::Config{
        output_file: Some("saved_analysis.json".to_string()),
        full_docs: true,
        signatures: true,
        borrow_data: true,
        ..rls_data::Config::default()
    };
    let json = serde_json::to_string(&analysis).unwrap();
    println!("cargo:rustc-env=RUST_SAVE_ANALYSIS_CONFIG={}",json);
}