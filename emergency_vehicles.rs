pub trait EmergencyVehicle {
    fn get_id(&self) -> String;
    fn get_position(&self) -> (i32, i32);
    fn get_equipment_level(&self) -> i32;
    fn get_availability(&self) -> bool;
    fn serve_emergency(&mut self, position: (i32, i32));
    fn get_speed(&self) -> f32;
    fn time_to_serve(&self, emerg_position: (i32, i32)) -> f32;
}

pub struct Emergency {
    level: i32,
    position: (i32, i32),
}

impl Emergency {
    fn get_level(&self) -> i32 {
        self.level
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}

pub struct Fleet {
    vehicles: Vec<Box<dyn EmergencyVehicle>>,
}

impl Fleet {
    fn add_vehicle(&mut self, vehicle: Box<dyn EmergencyVehicle>) {
        self.vehicles.push(vehicle);
    }
    
    fn serve_emergency(&mut self, emergency: Emergency) {
        let emergency_position = emergency.get_position();

        let mut best_vehicle_index = self.vehicles.len();
        let mut min_time = std::f32::INFINITY;

        for v in 0.. self.vehicles.len() {
            if self.vehicles[v as usize].get_availability(){
                if self.vehicles[v as usize].get_equipment_level() >= emergency.get_level(){
                    let time = self.vehicles[v as usize].time_to_serve(emergency_position);
    
                    if time < min_time {
                        min_time = time;
                        best_vehicle_index = v;
                    }
                }   
            }      
        }

        if best_vehicle_index < self.vehicles.len(){
            println!("Selected vehicle: id = {}. Time to serve the emergency = {} minutes", self.vehicles[best_vehicle_index as usize].get_id(), min_time);
            self.vehicles[best_vehicle_index as usize].serve_emergency(emergency_position)
        }
        else{
            println!("No available vehicles in the fleet to serve the emergency.");
        }
    }

    fn display_vehicles(&self) {
        println!("--- Vehicles ---");

        for vehicle in &self.vehicles {
            println!("id: {}, position: ({}, {}), equipment level: {}, availability: {}", vehicle.get_id(), vehicle.get_position().0, vehicle.get_position().1, vehicle.get_equipment_level(), vehicle.get_availability());
        }

        println!("-----------------")
    }

    fn id_already_present(&self, id: &String) -> bool{
        for vehicle in &self.vehicles {
            if vehicle.get_id() == id.to_string(){
                return true;
            }
        }
        return false;
    }

}

pub struct Ambulance {
    id: String,
    position: (i32, i32),
    equipment_level: i32,
    availability: bool, 
    speed: f32
}

impl EmergencyVehicle for Ambulance {
    fn get_id(&self) -> String {
        String::from(self.id.to_string())
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_equipment_level(&self) -> i32 {
        self.equipment_level
    }

    fn get_availability(&self) -> bool {
        self.availability
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn serve_emergency(&mut self, position: (i32, i32)) {
        self.position = position;
        self.availability = false;
    }

    fn time_to_serve(&self, emerg_position: (i32, i32)) -> f32 {
    
        let distance_x = (emerg_position.0 - self.position.0).abs(); 
        let distance_y = (emerg_position.1 - self.position.1).abs(); 

        let distance = distance_x + distance_y; // Manhattan distance

        let time = (distance as f32 / self.speed as f32) * 60.0; // time to serve the emergency

        time 
    }
}

pub struct Helicopter {
    id: String,
    position: (i32, i32),
    equipment_level: i32,
    availability: bool,
    speed: f32,
}

impl EmergencyVehicle for Helicopter {
    fn get_id(&self) -> String {
        String::from(self.id.to_string())
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_equipment_level(&self) -> i32 {
        self.equipment_level
    }

    fn get_availability(&self) -> bool {
        self.availability
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn serve_emergency(&mut self, position: (i32, i32)) {
        self.position = position;
        self.availability = false;
    }

    fn time_to_serve(&self, emerg_position: (i32, i32)) -> f32 {
    
        let distance_x = (emerg_position.0 - self.position.0) as f32;
        let distance_y = (emerg_position.1 - self.position.1) as f32;

        let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt(); // Straight-line distance

        let time = (distance / self.speed as f32) * 60.0 + 5.0; // time to serve the emergency

        time
    }
    
}

fn add_new_vehicle(fleet: &mut Fleet){
    use std::io::*;

    let mut vehicle_type : i32= -1;

    while vehicle_type != 0 && vehicle_type != 1{
        println!("Insert the type of the vehicle (0 = Ambulance, 1 = Helicopter):");
        vehicle_type = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    }
    
    let mut already_present = true;

    let mut id = String::from("");

    while already_present{
        println!("Insert the id of the vehicle: ");
        id = stdin().lock().lines().next().unwrap().unwrap();
        if !fleet.id_already_present(&id){
            already_present = false;
        }
        else{
            println!("Id already present!");
        }
    }
    
    println!("Insert the x coordinate of the vehicle: ");
    let x : i32 = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    println!("Insert the y coordinate of the vehicle: ");
    let y : i32 = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    let mut equip_lvl : i32 = -1;

    while equip_lvl != 0 && equip_lvl != 1 && equip_lvl != 2 && equip_lvl != 3{
        println!("Insert the equipment level of the vehicle (0, 1, 2 or 3): ");
        equip_lvl = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    }

    if vehicle_type == 0 {
        fleet.add_vehicle(Box::new(Ambulance{id: id, position: (x, y), equipment_level: equip_lvl, availability: true, speed: 100.0}))
    }
    else{
        fleet.add_vehicle(Box::new(Helicopter{id: id, position: (x, y), equipment_level: equip_lvl, availability: true, speed: 250.0}))
    }

    fleet.display_vehicles();
}

fn serve_new_emergency(fleet: &mut Fleet){
    use std::io::*;

    let mut emergency_lvl : i32= -1;

    while emergency_lvl != 0 && emergency_lvl != 1 && emergency_lvl != 2 && emergency_lvl != 3{
        println!("Insert the emergency level (0, 1, 2 or 3): ");
        emergency_lvl = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    }

    println!("Insert the x coordinate of the emergency: ");
    let x : i32 = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    println!("Insert the y coordinate of the emergency: ");
    let y : i32 = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    fleet.serve_emergency(Emergency{level: emergency_lvl, position: (x, y)});
}

fn main(){
    let mut fleet = Fleet{vehicles: vec![]};
    let mut exit = false;
    
    use std::io::*;
    while !exit {
        println!("What do you want to do?\n1. Show vehicles\n2. Add new vehicle\n3. Serve an emergency\n-1. Exit");
        let user_choice : i32 = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    
        match user_choice {
            1 => fleet.display_vehicles(),
            2 => add_new_vehicle(&mut fleet),
            3 => serve_new_emergency(&mut fleet),
            -1 => exit = true,
            _ => println!("Choose one of the available operations!"),
        };
    }
    
}