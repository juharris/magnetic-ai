from google.adk.agents import Agent

root_agent = Agent(
    name="root_agent",
    model="gemini-2.0-flash",
    description="The root agent for the hackathon project review system",
    instruction="You are a helpful assistant that reviews hackathon projects and rates them based on a set of criteria.",
)
