#[derive(Clone)]
pub struct Settings {
    invitation_input_symbol: String,
}

impl Settings {
    pub fn get_invitation_input(&self) -> String {
        format!(
            "\x1b[0;1;37m{}{}{}\x1b[0m",
            self.invitation_input_symbol,
            self.invitation_input_symbol,
            self.invitation_input_symbol
        )
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            invitation_input_symbol: ">".to_string(),
        }
    }
}
