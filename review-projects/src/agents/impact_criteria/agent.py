from google.adk.agents import Agent

# Models: https://ai.google.dev/gemini-api/docs/models
# model = "gemini-2.5-flash"
model = "gemini-2.5-flash-lite"


impact_criteria_agent = Agent(
    name="impact_criteria_agent",
    model=model,
    description="Decide how impactful a hackathon project is based on a set of criteria.",
    instruction="Impact & Innovation: Does it solve a meaningful problem? Unlock a new pattern? Show a powerful personal/local workflow others could adopt?",
)

print(impact_criteria_agent.disallow_transfer_to_parent)