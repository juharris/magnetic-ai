import json
import os

from google.adk.agents import Agent
from google.adk.agents.llm_agent import ToolUnion
from google.adk.tools.mcp_tool.mcp_session_manager import StreamableHTTPConnectionParams
from google.adk.tools.mcp_tool.mcp_toolset import McpToolset
from google.adk.tools import google_search

from optify import OptionsWatcher

path_to_current_dir = os.path.dirname(os.path.abspath(__file__))
provider = OptionsWatcher.build(os.path.join(
    path_to_current_dir, '../../../config'))

features = [
    "root_agent",
]

root_agent_config_json = provider.get_options_json('root_agent', features)
config = json.loads(root_agent_config_json)

model = config['agent_model']
gh_mcp_pat = os.environ.get('GH_PAT')

gh_tool = McpToolset(
    connection_params=StreamableHTTPConnectionParams(
        url='https://api.githubcopilot.com/mcp/',
        headers={
            "Authorization": f"Bearer {gh_mcp_pat}"
        }
    )
)


def get_tools() -> list[ToolUnion]:
    result = []
    configured_tools = config['tools']
    for tool_name, is_enabled in configured_tools.items():
        if is_enabled:
            match tool_name:
                case 'google_search':
                    result.append(google_search)
                case 'GitHub':
                    result.append(gh_tool)
    return result


tools = get_tools()

projects = [
    {
        "name": "Magnetic AI",
        "url": "https://github.com/juharris/magnetic-ai/tree/compare-hackathon-projects",
    },
]

print("Making agent with model: ", model)
print("Tools: ", tools)

root_agent = Agent(
    name="project_review_agent",
    model=model,
    description=config['description'],
    instruction=config['instruction'],
    tools=tools,
)
