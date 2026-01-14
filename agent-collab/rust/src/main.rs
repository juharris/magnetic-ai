use agent_collab::ai::client::Ai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai = Ai::default();
    let content = ai
        .send("What is the capital of France and what was the president's name in 2020?")
        .await
        .unwrap();
    println!("Answer: {}", content);

    println!();
    println!("Streaming:");

    ai.print_stream("What is the capital of France and what was the president's name in 2020?")
        .await
        .unwrap();

    Ok(())
}
