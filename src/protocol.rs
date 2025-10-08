// In a real system you’d define RESP protocol here.
// For now this file just reserves structure for future protocol features.
#[derive(Debug)]
pub enum CommandType {
    Ping,
    Set,
    Get,
    Del,
    Keys,
    ClusterJoin,
}

