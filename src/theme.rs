use crate::{Error, PromptBody, PromptInput, PromptState, Terminal};

/// A struct aggregating the content for rendering.
#[derive(Debug)]
pub struct RenderSnapshot<'a> {
    pub state: &'a PromptState,
    pub message: String,
    pub hint: Option<String>,
    pub placeholder: Option<String>,
    pub input: PromptInput,
    pub body: PromptBody,
}

/// A trait for the Theme that determines what Promptuity renders.
pub trait Theme<W: std::io::Write> {
    /// Output of messages without decoration.
    fn log(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;
    /// Output of messages with info decoration.
    fn info(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;
    /// Output of messages with warning decoration.
    fn warn(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;
    /// Output of messages with error decoration.
    fn error(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;
    /// Output of messages with success decoration.
    fn success(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;

    /// Output of messages with a step decoration.
    fn step(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error>;

    /// Renders the start of a prompt session.  
    /// It can render a title or message received as `intro`.
    fn begin(&mut self, term: &mut dyn Terminal<W>, intro: Option<String>) -> Result<(), Error>;

    /// Renders the prompt.
    fn render(&mut self, term: &mut dyn Terminal<W>, payload: RenderSnapshot) -> Result<(), Error>;

    /// Renders the end of a prompt session.  
    /// It can render a message received as `outro`.
    fn finish(
        &mut self,
        term: &mut dyn Terminal<W>,
        state: &PromptState,
        outro: Option<String>,
    ) -> Result<(), Error>;
}
