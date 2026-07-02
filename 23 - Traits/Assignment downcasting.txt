// //usecase of downcasting (assignment)
// use std::any::{self, Any};
// trait UIComponent: Any {
//     fn render(&self);
// }

// struct Button;
// struct Slider;

// impl UIComponent for Button {
//     fn render(&self) {
//         println!("Rendering Button");
//     }
// }

// impl UIComponent for Slider {
//     fn render(&self) {
//         println!("Rendering Slider");
//     }
// }

// fn render_component(component: &dyn Any) {
//     if let Some(button) = component.downcast_ref::<Button>() {
//         println!("Rendering Button specifically.");
//         button.render();
//     } else if let Some(slider) = component.downcast_ref::<Slider>() {
//         println!("Rendering Slider specifically.");
//         slider.render();
//     } else {
//         println!("Unknown component");
//         //component.render();
//     }
// }

// fn main() {
//     let b = Button;
//     let c = Slider;
//     let b = &b as &dyn Any;
//     let c = &c as &dyn Any;

//     let ui_components = vec![b, c];

//     for components in ui_components {
//         render_component(components);
//     }
// }
