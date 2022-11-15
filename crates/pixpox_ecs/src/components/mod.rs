use log::info;

/*
 * Base traits that all components must implement
 */
pub trait Label {
    fn label(&mut self) -> &'static str;
}

pub trait Run {
    fn run(&mut self);
}

pub struct BaseComponent {
    label: &'static str,
}

impl Label for BaseComponent {
    fn label(&mut self) -> &'static str {
        {
            self.label
        }
    }
}

impl Run for BaseComponent {
    fn run(&mut self) {
        info!("Kur kapan");
    }
}