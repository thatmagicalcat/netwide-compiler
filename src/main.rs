#[tokio::main]
async fn main() {
    let com = netwide_compiler::NetWideCompiler::new().await;
    let output = com
        .run("lua", None, "io.write(\"Hello World From Lua\")".to_string())
        .await
        .unwrap();

    println!("{output:#?}");

    let output = com
        .run(
            "c++",
            None,   // target is automatically chosen
            "#include <iostream>\nint main() { std::cout << \"Hello World From C++\"; }".to_string(),
        )
        .await
        .unwrap();

    println!("{output:#?}");
}
