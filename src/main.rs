#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;


#[derive(Clone)]
struct Body{
    mass : f32,
    radius : f32,

    y_pos : f32,
    y_vel : f32,
    
    x_pos : f32,
    x_vel : f32,

    rgb : Color32, 
}
impl Default for Body {
    fn default() -> Self {
        return Self {
            mass: 100.0,
            radius : 100.0,

            y_pos : 500.0,
            y_vel : 0.0,

            x_pos : 500.0,
            x_vel : 0.0,
            rgb : Color32::from_rgb(0, 0, 0)
        }
    } 
}
impl Body {

    fn new(m:f32, r:f32, Y_p:f32, Y_v:f32, X_p:f32, X_v:f32, rgb_:Color32) -> Self {
        return Self {
            mass: m,
            radius : r,

            y_pos : Y_p,
            y_vel : Y_v,

            x_pos : X_p,
            x_vel : X_v,
            rgb : rgb_
        }
    }
}

struct Variables {
    time_per_frame: f32,
    time_elapsed: f32, 
    
    g: f32,
    min_distance : f32,
    run: bool,

    can_exit: bool,
    is_exiting: bool,

    body_to_edit: usize,

    //When simulation starts clone Vec for return point after simulation ends
    bodies: Vec<Body>,
    bodies_cpy: Vec<Body>,
}
impl Default for Variables {
    fn default() -> Self {
        return Self {
            time_per_frame : 1.0,
            time_elapsed : 0.0,

            g: 5.0,
            min_distance: 50.0,  
            run : false,

            can_exit : false,
            is_exiting : false,

            body_to_edit: 0,

            bodies: vec![],
            bodies_cpy: vec![],
        }
    }
}
impl Variables {
    fn Simulation(&mut self) {

                //Run simulation
                for i in 0..self.bodies.len(){
                    for j in 0..self.bodies.len(){
                        if i == j {continue;}
                        
                        //Distances for each axis
                        let x_distance = self.bodies[i].x_pos - self.bodies[j].x_pos;
                        let y_distance = self.bodies[i].y_pos - self.bodies[j].y_pos;
                    
                        //total Distance^2
                        let mut r2 = (x_distance * x_distance) + (y_distance * y_distance);
                        
                        //Magnitude
                        let magnitude = r2.sqrt();

                        //Set r^2 to minimum distance if less than that distance
                        if r2 < self.min_distance * self.min_distance {
                            r2 = self.min_distance * self.min_distance;
                        }


                        //Update x and y velocity (assume 1ms passes for testing)
                        self.bodies[i].x_vel -= ((self.bodies[j].mass * self.g)/(r2)) * (x_distance/magnitude) * self.time_per_frame;
                        self.bodies[i].y_vel -= ((self.bodies[j].mass * self.g)/(r2)) * (y_distance/magnitude) * self.time_per_frame;

                    }
                }

                //Update position (assume 1ms passes for testing)
                for i in 0..self.bodies.len(){
                    self.bodies[i].x_pos += self.bodies[i].x_vel * self.time_per_frame;
                    self.bodies[i].y_pos += self.bodies[i].y_vel * self.time_per_frame;

                }
            
    }
}
impl eframe::App for Variables {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        return self.can_exit;
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){

        //Update every frame
        ctx.request_repaint(); 
    
        //Run main window
        egui::CentralPanel::default().show(ctx, |ui| {

            if self.run == false {
                
                if self.bodies.len() != 0 {

                    //Change body settings
                    ui.horizontal(|ui| {
                        ui.label("Mass:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].mass).speed(1));
                        if self.bodies[self.body_to_edit].mass < 1.0 {self.bodies[self.body_to_edit].mass = 1.0;}

                        ui.label("Radius:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].radius).speed(1));
                        if self.bodies[self.body_to_edit].radius < 1.0 {self.bodies[self.body_to_edit].radius = 1.0;}
                    });
                    ui.horizontal(|ui| {
                        ui.label("X Position:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].x_pos).speed(1));
                        if self.bodies[self.body_to_edit].x_pos < 0.0 {self.bodies[self.body_to_edit].x_pos = 0.0;}

                        ui.label("Y Position:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].y_pos).speed(1));
                        if self.bodies[self.body_to_edit].y_pos < 0.0 {self.bodies[self.body_to_edit].y_pos = 0.0;}
                    });
                    ui.horizontal(|ui| {
                        ui.label("X Velocity:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].x_vel).speed(1));

                        ui.label("Y Velocity:");
                        ui.add(egui::DragValue::new(&mut self.bodies[self.body_to_edit].y_vel).speed(1));
                    });

                    ui.label("Colour:");
                    ui.color_edit_button_srgba(&mut self.bodies[self.body_to_edit].rgb);
                }
                ui.horizontal(|ui| {
                    //Push new body onto vector
                    if ui.button("New").clicked() {
                        
                        self.bodies.push(Body::default());
                        self.body_to_edit = self.bodies.len() - 1;
                    }
                    if ui.button("Copy").clicked() {
                        
                        self.bodies.push(self.bodies[self.body_to_edit].clone());
                        self.body_to_edit = self.bodies.len() - 1;
                    }
                    if self.bodies.len() != 0 && ui.button("Delete").clicked() {
                        
                        self.bodies.remove(self.body_to_edit);

                        if self.bodies.len() == 0 {self.body_to_edit = 0;}
                        else {self.body_to_edit = self.bodies.len() - 1;}
                    }
                    if self.bodies.len() != 0 && ui.button("Reset").clicked() {
                        
                        self.bodies.clear();
                        self.body_to_edit = 0;
                    }
                    if self.bodies.len() > 1 && ui.button("Run").clicked() {

                        self.time_elapsed = 0.0;
                        self.bodies_cpy = self.bodies.clone();
                        self.run = true;

                    }
                    ui.label("G:");
                    ui.add(egui::DragValue::new(&mut self.g).speed(0.01));
                    ui.label("Min distance:");
                    ui.add(egui::DragValue::new(&mut self.min_distance).speed(0.01));
                    if self.min_distance < 0.0 {self.min_distance = 0.0;}
                    ui.label("Time per frame (s):");
                    ui.add(egui::DragValue::new(&mut self.time_per_frame).speed(0.00001));
                    if self.min_distance < 0.0 {self.time_per_frame = 0.00001;}
                });
            }
            else{
               
                //Update time elapsed since simulation started
                self.time_elapsed += self.time_per_frame;
                
                //Display time since run was activated
                ui.heading(self.time_elapsed.to_string() + " s");

                //Run simulation iteration
                self.Simulation();

                if ui.button("end").clicked() {
                    self.bodies = self.bodies_cpy.clone();
                    self.run = false;
                }
            }

            //Set up new draw area
            let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), ui.available_height()), Sense::hover());

            //Draw area interaction 
            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            if self.run == false {

                //Move bodies and edit body_to_edit 
                for i in 0..self.bodies.len(){
                    if self.run == false {

                        let mut Pos_temp = Pos2::new(self.bodies[i].x_pos, self.bodies[i].y_pos);

                        let size_pos = Vec2::splat(1.5 * self.bodies[i].radius);
                        let point_rect_pos = Rect::from_center_size(Pos_temp, size_pos);
                        let point_id_pos = response.id.with(i);

                        //If body is clicked on change body to edit
                        let point_response_click = ui.interact(point_rect_pos, point_id_pos, Sense::click());
                        if point_response_click.clicked() {self.body_to_edit = i;}

                        //If body is dragged move its position
                        let point_response_pos = ui.interact(point_rect_pos, point_id_pos, Sense::drag());
                        if point_response_pos.dragged() {self.body_to_edit = i;}

                        //Update position if dragged
                        Pos_temp += point_response_pos.drag_delta();
                        Pos_temp = to_screen.from().clamp(Pos_temp);
    
                        self.bodies[i].x_pos = Pos_temp.x;
                        self.bodies[i].y_pos = Pos_temp.y;
                    }
                }
            }
            
            //Render all bodies
            for i in 0..self.bodies.len(){
                painter.add(Shape::circle_filled(Pos2::new(self.bodies[i].x_pos,  self.bodies[i].y_pos), self.bodies[i].radius, self.bodies[i].rgb));
            }
        });


        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.can_exit = true;
                            frame.quit();
                        }
                        if ui.button("No").clicked() {
                            self.is_exiting = false;
                        }
                    });
                });
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    
    //Run main application window
    eframe::run_native(
        "N_body Simulation", 
        options, 
        Box::new(|_cc| Box::new(Variables::default()))
    );

}
