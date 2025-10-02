from dotenv import load_dotenv
from google.adk.runners import Runner
from google.adk.sessions import InMemorySessionService
from google.genai import types

from agents.hack_review.agent import root_agent

APP_NAME = "review-projects"
USER_ID = "user_123"
SESSION_ID = "session_123"


async def setup_session_and_runner():
    session_service = InMemorySessionService()
    session = await session_service.create_session(app_name=APP_NAME, user_id=USER_ID, session_id=SESSION_ID)
    runner = Runner(agent=root_agent, app_name=APP_NAME,
                    session_service=session_service)
    return session, runner


async def call_agent_async(query):
    content = types.Content(role='user', parts=[types.Part(text=query)])
    session, runner = await setup_session_and_runner()
    events = runner.run_async(
        user_id=USER_ID, session_id=SESSION_ID, new_message=content)

    async for event in events:
        if event.is_final_response():
            # Was `final_response = event.content.parts[0].text`, but it's not clear what `content` is.
            final_response = event.content
            print("Agent Response: ", final_response)

# Note: In Colab, you can directly use 'await' at the top level.
# If running this code as a standalone Python script, you'll need to use asyncio.run() or manage the event loop.
if __name__ == "__main__":
    # Load .env
    load_dotenv()

    import asyncio
    asyncio.run(call_agent_async("""
        Rate the following project:
            name: "Magnetic AI"
            url: "https://github.com/juharris/magnetic-ai/tree/compare-hackathon-projects"
    """))
