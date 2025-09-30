import json
import os

from google.adk.agents import Agent

from optify import OptionsProvider

path_to_current_dir = os.path.dirname(os.path.abspath(__file__))
provider = OptionsProvider.build(os.path.join(path_to_current_dir, '../../../config'))

features = [
    "tools",
]

root_agent_config_json = provider.get_options_json('root_agent', features)
config = json.loads(root_agent_config_json)
print(config)

model = config['agent_model']


root_agent = Agent(
    name="root_agent",
    model=model,
    description="The root agent for the hackathon project review system",
    instruction="You are a helpful assistant that reviews hackathon projects and rates them based on a set of criteria.",
)
