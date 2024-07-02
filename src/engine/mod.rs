pub enum SearchType {
    Minimax,
    AlphaBeta,
}
pub enum CommunicationProtocol {
    Uci,
    XBoard
}

pub struct Engine {
    search: SearchType,
    com_protocol: CommunicationProtocol,
    depth: usize,
}
impl Default for Engine {
    fn default() -> Self {
        Self {
            search: SearchType::Minimax,
            com_protocol: CommunicationProtocol::Uci,
            depth: 6
        }
    }
}
pub struct EngineBuilder;
impl EngineBuilder {
    fn build(&self) -> Option<Engine> {
        todo!()
    }
}