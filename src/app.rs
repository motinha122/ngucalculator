/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    #[serde(skip)]
    energy: f64,
    magic: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            energy: 37500.0,
            magic: 10000.0,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { energy, magic } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!

        egui::TopBottomPanel::top("Title").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                ui.heading("NGU Ratio Calculator");
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                
                ui.heading("Energy / Magic Cap");
    
                ui.horizontal(|ui| {
                    ui.label("Energy ");
                    ui.add(egui::Slider::new(energy, 0.0..=9999999999.9).text("value"))
                });
                ui.horizontal(|ui| {
                    ui.label("Magic   ");
                    ui.add(egui::Slider::new(magic, 0.0..=999999999.9).text("value"))
                });
    
                ui.separator();
                ui.heading("Power and Bars");
    
                ui.horizontal(|ui| {
                    ui.label("Energy ");
                    ui.label(format!("{}",*energy/37500.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Magic   ");
                    ui.label(format!("{}",*magic/10000.0));
                });
            })


        });

    }
}
