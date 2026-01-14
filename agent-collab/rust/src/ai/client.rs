use genai::Client;
use genai::chat::printer::{PrintChatStreamOptions, print_chat_stream};
use genai::chat::{ChatMessage, ChatRequest};

#[derive(Clone, Default)]
pub struct Ai {}

impl Ai {
    fn get_client(&self) -> genai::Client {
        Client::default()
    }

    fn create_request(&self, message: &str) -> genai::chat::ChatRequest {
        ChatRequest::new(vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user(message),
        ])
    }

    fn get_model(&self) -> String {
        let model = "llama3.2";
        // let model = "phi3";
        model.to_owned()
    }

    pub async fn send(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = self.get_client();

        let chat_request = self.create_request(message);
        let model = &self.get_model();
        let response = client.exec_chat(model, chat_request, None).await?;
        let content = response.first_text().expect("should be a text response");

        Ok(content.to_owned())
    }

    pub async fn print_stream(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.get_client();

        let chat_request = self.create_request(message);
        let model = &self.get_model();

        let stream = client.exec_chat_stream(model, chat_request, None).await?;
        let print_options = PrintChatStreamOptions::from_print_events(false);
        print_chat_stream(stream, Some(&print_options)).await?;

        Ok(())
    }
}
