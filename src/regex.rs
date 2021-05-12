use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::dfa::DeterministicFiniteAutomaton;


pub struct Regex
{
    dfa: DeterministicFiniteAutomaton,
}

impl Regex
{
    pub fn new(regex: String) -> Result<Regex, String> {
        let lexer = Lexer::new(regex);
        let mut parser = Parser::new(lexer);
        let nfa = parser.expression()?;
        Ok(
            Regex {
                dfa: nfa.nfa2dfa()
            }
        )
    }

    pub fn matches(&self, string: String) -> bool {
        let mut runtime = self.dfa.get_runtime();
        runtime.does_accept(string.as_bytes())
    }
}
