pub struct CommandList {
    pub cmd: String,
    pub args: Vec<String>,
    pub rev: Option<(String, Vec<String>)>,
}

pub struct CommandListWithEnv {
    pub cmd: String,
    pub args: Vec<String>,
    pub rev: Option<(String, Vec<String>)>,
    pub env: Vec<(String, String)>,
}