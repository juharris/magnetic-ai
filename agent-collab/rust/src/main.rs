use agent_collab::agents::{Agent, get_agents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agents = get_agents()?;
    let agent_config = agents
        .values()
        .next()
        .expect("Should have at least one agent");

    let agent = Agent::new(agent_config.clone());
    let ai = agent.ai();

    let content = ai
        .send("What is the capital of France and what was the president's name in 2020?")
        .await?;
    println!("Answer: {content}");

    println!();
    println!("Streaming:");

    ai.print_stream("What is the capital of France and what was the president's name in 2020?")
        .await?;

    Ok(())
}
