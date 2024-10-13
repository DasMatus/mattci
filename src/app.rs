use epi::egui::{CentralPanel, Grid, Label};
use epi::{egui::Context, Frame};
use epi::{
    egui::{self, SidePanel},
    App,
};
use std::fmt::format;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};
#[derive(Default)]
pub struct UI {
    path: String,
    id: String,
}
impl UI {
    fn populate(&mut self) {
        let var = "RECIPE_DIR";
        let dir = std::env::var(var).unwrap();
        self.path = Path::new(&dir).to_str().unwrap().to_string();
    }
}
impl App for UI {
    fn name(&self) -> &str {
        "MattCI"
    }
    fn update(&mut self, ctx: &egui::Context, frame: &Frame) {
        SidePanel::left("actions").show(ctx, |side| {
            for action in read_dir(&self.path).unwrap() {
                let a = action.unwrap();
                let cfg: crate::cfg::Cfg =
                    serde_yaml::from_str(read_to_string(a.path()).unwrap().as_str()).unwrap();
                if side.button(cfg.name.clone()).clicked() {
                    self.id = cfg.name.into();
                }
            }
        });
        CentralPanel::default().show(ctx, |center| {
            Grid::new("Header-grid").show(center, |heading| {
                heading.end_row();
                heading.heading(format!("Action"));
                heading.end_row();
            });
            center.centered_and_justified(|action| {
                for a in read_dir(&self.path).unwrap() {
                    let cfg: crate::cfg::Cfg =
                        serde_yaml::from_str(read_to_string(a.unwrap().path()).unwrap().as_str()).unwrap();
                    if action.button(cfg.name.clone()).clicked() {
                        self.id = cfg.name.into();
                    }
                    for step in cfg.steps {
                        let l: String = step.name.into(); 
                        action.collapsing(l, |workflow| {
                            
                        });
                        
                    }
                }
            })
        });
    }
}
