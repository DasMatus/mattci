use std::ops::Deref;

use epi::egui::WidgetText;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub enum Severity {
    Note(String),
    Warn(String),
}
#[derive(Serialize, Deserialize)]
pub struct Steps(Step);
#[derive(Serialize, Deserialize)]
pub struct Step {
    pub imports: Vec<String>,
    pub name: Name,
    pub note: Note,
    pub cmd: Vec<String>,
}
#[derive(Serialize, Deserialize, Default)]
pub struct Name(String);
#[derive(Serialize, Deserialize)]
pub struct Note(Severity);
#[derive(Serialize, Deserialize)]
pub struct Runner(String);
#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub name: Name,
    pub runner: Runner,
    pub steps: Vec<Step>,
}
impl Into<WidgetText> for Name {
    fn into(self) -> WidgetText {
        WidgetText::from(self.0)
    }
}
impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}
impl Clone for Name {
    fn clone(&self) -> Self {
        self.clone()
    }
}
