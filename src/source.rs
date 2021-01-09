#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    // This is needed to workaround #955 in compiler-explorer where it
    // may return objects without text field.
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExecResult {
    pub code: i32,
    pub stdout: Vec<Text>,
    pub stderr: Vec<Text>,
    #[serde(rename = "didExecute")]
    pub did_execute: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub code: i32,
    pub stderr: Vec<Text>,
    pub asm: Vec<Text>,
    #[serde(default, rename = "execResult")]
    pub exec_result: ExecResult,
}
