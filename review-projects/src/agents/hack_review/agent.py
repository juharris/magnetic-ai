from google.adk.agents import Agent

# Models: https://ai.google.dev/gemini-api/docs/models
# model = "gemini-2.5-flash"
model = "gemini-2.5-flash-lite"

root_agent = Agent(
    name="root_agent",
    model=model,
    description="The root agent for the hackathon project review system",
    instruction="You are a helpful assistant that reviews hackathon projects and rates them based on a set of criteria.",
)
