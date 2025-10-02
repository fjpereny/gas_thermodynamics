use colored::Colorize;
use aga8::composition::Composition;
use aga8::detail::Detail;
use std::io;

struct ProgramState {
    gas: String,
    gas_state: Detail,
    gas_comp: Composition,
    unit_text: UnitText,
    units : Units,
    inlet_state: Detail,
    discharge_state: Detail,
    show_inlet_state: bool,
    show_discharge_state: bool,
}

struct Units {
    pressure: UnitPressure,
    temp: UnitTemp,
    internal_energy: UnitInternalEnergy,
}

#[derive(Clone, Copy)]
enum UnitPressure {
    kPa,
    PSI,
    Bar,
}

#[derive(Clone, Copy)]
enum UnitTemp {
    C,
    K,
    F,
    R,
}

#[derive(Clone, Copy)]
enum UnitInternalEnergy {
    J_mol,
    kJ_kg,
    BTU_lbm,
}

fn main() {

    let gas = String::from("Air");
    let initial_pressure= 100.0;
    let initial_temperature = 273.15;

    let unit_text = UnitText {
        pressure: "kPa",
        temperature: "K",
        internal_energy: "J/mol",
    };

    let units = Units {
        pressure: UnitPressure::kPa,
        temp: UnitTemp::K,
        internal_energy: UnitInternalEnergy::J_mol,
    };
    
    let gas_state: Detail = Detail::new();
    let gas_comp = get_gas_comp(GasComp::Air);
    let mut program_state = ProgramState {
        gas: gas,
        gas_state: gas_state,
        gas_comp: gas_comp,
        unit_text: unit_text,
        units: units,
        inlet_state: Detail::default(),
        discharge_state: Detail::default(),
        show_inlet_state: false,
        show_discharge_state: false,
    };

    program_state.gas_state.set_composition(&program_state.gas_comp).unwrap();
    program_state.gas_state.p = initial_pressure;
    program_state.gas_state.t = initial_temperature;
    calculate_state(&mut program_state);



    println!();
    println!("{}", "Thermodynamic Properties Calculator".blue().bold());
    println!("{}", "Frank Pereny - 2025".blue().italic());
    println!("{}", "-----------------------------------".blue());
    print_gas_state(&mut program_state);
}

fn quit() {
    std::process::exit(0);
}

fn calculate_state(program_state: &mut ProgramState) {
    let density = program_state.gas_state.density();
    match density {
        Ok(()) => (),
        Err(_err) => println!("{}", "** Error calculating density.  Pressure or temperature out of bounds! **".red().bold().italic()),
    }
    program_state.gas_state.properties();
}


fn print_main_menu(program_state: &mut ProgramState) {
    println!();
    println!("{}", "Main Menu".blue());
    println!("{}", "---------".blue());
    println!("{}", "g - Select Gas Composition".green());
    println!("{}", "p - Set Pressure".yellow());
    println!("{}", "t - Set Temperature".red());
    println!("---------");
    println!("{}", "1 - Set as inlet condition".cyan());
    println!("{}", "2 - Set as discharge condition".cyan());
    println!("{}", "u - Change Units");
    println!("---------");
    println!("q - Quit Program");
    println!();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");
    let input = input.trim();
    
    if input== "q" {
        quit();
    }
    match input {
        "g" => set_gas_comp(program_state),
        "p" => set_pressure(program_state),
        "t" => set_temperature(program_state),
        "u" => change_units(program_state),
        "1" => set_inlet(program_state),
        "2" => set_discharge(program_state),
        "q" => quit(),
        _ => {
            println!("{}", "**Invalid selection!**".bold().red());
            print_main_menu(program_state);
        },
    }
}

fn set_inlet(program_state: &mut ProgramState) {
    program_state.show_inlet_state = true;
    program_state.inlet_state.p = program_state.gas_state.p;
    program_state.inlet_state.t = program_state.gas_state.t;
    program_state.inlet_state.set_composition(&program_state.gas_comp).unwrap();
    print_gas_state(program_state);
}

fn set_discharge(program_state: &mut ProgramState) {
    program_state.show_discharge_state = true;
    program_state.discharge_state.p = program_state.gas_state.p;
    program_state.discharge_state.t = program_state.gas_state.t;
    program_state.discharge_state.set_composition(&program_state.gas_comp).unwrap();
    print_gas_state(program_state);
}

fn set_gas_comp(program_state: &mut ProgramState) {
    println!();
    println!("Select Gas:");
    println!("1 - Air");
    println!("2 - Argon");
    println!("3 - Nitrogen");
    println!("4 - Oxygen");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let mut new_gas_comp = Composition::default();
    match choice {
        "1" => {
            program_state.gas = "Air".to_string();
            new_gas_comp = get_gas_comp(GasComp::Air);
        },
        "2" => {
            program_state.gas = "Argon".to_string();
            new_gas_comp = get_gas_comp(GasComp::Argon);
        },
        "3" => {
            program_state.gas = "Nitrogen".to_string();
            new_gas_comp = get_gas_comp(GasComp::Nitrogen);
        },
        "4" => {
            program_state.gas = "Oxygen".to_string();
            new_gas_comp = get_gas_comp(GasComp::Oxygen);
        },
        _ => set_gas_comp(program_state),
    }
    program_state.gas_state.set_composition(&new_gas_comp).unwrap();
    
    program_state.show_inlet_state = false;
    program_state.show_discharge_state = false;
    calculate_state(program_state);
    print_gas_state(program_state);

}


fn set_pressure(program_state: &mut ProgramState) {
    println!();
    println!("Enter pressure ({}):", program_state.unit_text.pressure);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().parse::<f64>();
    let p = match input {
        Ok(num) => num,
        Err(_) => {
            set_pressure(program_state);
            0.0
        }
    };
    match program_state.units.pressure {
        UnitPressure::kPa => program_state.gas_state.p = p,
        UnitPressure::Bar => program_state.gas_state.p = p / 0.01,
        UnitPressure::PSI => program_state.gas_state.p = p / 0.145038,
    }
    calculate_state(program_state);
    print_gas_state(program_state);
}


fn set_temperature(program_state: &mut ProgramState) {
    println!();
    println!("Enter temperature ({}):", program_state.unit_text.temperature);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().parse::<f64>();
    let t = match input {
        Ok(num) => num,
        Err(_) => {
            set_temperature(program_state);
            0.0
        }
    };

    match program_state.units.temp {
        UnitTemp::K => program_state.gas_state.t = t,
        UnitTemp::C => program_state.gas_state.t = t + 273.15,
        UnitTemp::F => program_state.gas_state.t = (t - 32.0) * 5.0 / 9.0 + 273.15,
        UnitTemp::R => program_state.gas_state.t = t * 5.0 / 9.0,
    }

    calculate_state(program_state);
    print_gas_state(program_state);
}

fn get_pressure(pressure: f64, unit: UnitPressure) -> f64 {
    match unit {
        UnitPressure::kPa => pressure,
        UnitPressure::Bar => pressure * 0.01,
        UnitPressure::PSI => pressure * 0.145038,
    }
}

fn get_temperature(temperature: f64, unit: UnitTemp) -> f64 {
    match unit {
        UnitTemp::K => temperature,
        UnitTemp::C => temperature - 273.15,
        UnitTemp::F => (temperature - 273.15) * 9.0 / 5.0 + 32.0,
        UnitTemp::R => temperature * 9.0 / 5.0,
    }
}

fn get_internal_energy(program_state: &mut ProgramState) -> f64 {
    let internal_energy = program_state.gas_state.u;
    match program_state.units.internal_energy {
        UnitInternalEnergy::J_mol => internal_energy,
        UnitInternalEnergy::kJ_kg => internal_energy / program_state.gas_state.mm,
        UnitInternalEnergy::BTU_lbm => internal_energy / program_state.gas_state.mm * 0.429923,
    }
}

fn print_gas_state(program_state: &mut ProgramState) {
    println!();
    if program_state.show_inlet_state || program_state.show_discharge_state {
        println!("{:<32} {:21} {:23} {:10}", "Gas: ", program_state.gas, "Inlet", "Discharge");
        println!("{:<30} {:10.4} {:10} {:10.4} {:10} {:10.4} {:10}", 
            "Absolute Pressure: ", get_pressure(program_state.gas_state.p, program_state.units.pressure), program_state.unit_text.pressure,
            get_pressure(program_state.inlet_state.p, program_state.units.pressure), program_state.unit_text.pressure,
            get_pressure(program_state.discharge_state.p, program_state.units.pressure), program_state.unit_text.pressure);
        println!("{:<30} {:10.4} {:10} {:10.4} {:10} {:10.4} {:10}",
            "Absolute Temperature: ", get_temperature(program_state.gas_state.t, program_state.units.temp), program_state.unit_text.temperature,
            get_temperature(program_state.inlet_state.t, program_state.units.temp), program_state.unit_text.temperature,
            get_temperature(program_state.discharge_state.t, program_state.units.temp), program_state.unit_text.temperature);
        println!("{:<30} {:10.4} {:10}", "Density: ", program_state.gas_state.d, "mol/l");
        println!("{:<30} {:10.4} {:10}", "Molar Mass ", program_state.gas_state.mm, "g/mol");
        println!("{:<30} {:10.4} {:10}", "Internal Energy u: ", get_internal_energy(program_state), program_state.unit_text.internal_energy);
        println!("{:<30} {:10.4} {:10}", "Enthalpy: ", program_state.gas_state.h, "J/mol");
        println!("{:<30} {:10.4} {:10}", "Entropy: ", program_state.gas_state.s, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cp: ", program_state.gas_state.cp, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cv: ", program_state.gas_state.cv, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cp/Cv: ", program_state.gas_state.cp / program_state.gas_state.cv, "[]");
        println!("{:<30} {:10.4} {:10}", "Compressibility Z: ", program_state.gas_state.z, "[]");
        println!("{:<30} {:10.4} {:10}", "Isentropic Exponent k: ", program_state.gas_state.kappa, "[]");
        println!("{:<30} {:10.4} {:10}", "Speed of Sound w: ", program_state.gas_state.w, "m/s");
        println!("{:<30} {:10.4} {:10}", "Gibbs Energy: ", program_state.gas_state.g, "J/mol");
        println!("{:<30} {:10.4} {:10}", "Joule-Thompson Coefficient: ", program_state.gas_state.jt, format!("{}/kPa", program_state.unit_text.temperature));
        println!();
    } else {
        println!("{}", "Current State".italic().bold());
        println!("{:<32} {:20}", "Gas: ", program_state.gas);
        println!("{:<30} {:10.4} {:10}", "Absolute Pressure: ", get_pressure(program_state.gas_state.p, program_state.units.pressure), program_state.unit_text.pressure);
        println!("{:<30} {:10.4} {:10}", "Absolute Temperature: ", get_temperature(program_state.gas_state.t, program_state.units.temp), program_state.unit_text.temperature);
        println!("{:<30} {:10.4} {:10}", "Density: ", program_state.gas_state.d, "mol/l");
        println!("{:<30} {:10.4} {:10}", "Molar Mass ", program_state.gas_state.mm, "g/mol");
        println!("{:<30} {:10.4} {:10}", "Internal Energy u: ", get_internal_energy(program_state), program_state.unit_text.internal_energy);
        println!("{:<30} {:10.4} {:10}", "Enthalpy: ", program_state.gas_state.h, "J/mol");
        println!("{:<30} {:10.4} {:10}", "Entropy: ", program_state.gas_state.s, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cp: ", program_state.gas_state.cp, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cv: ", program_state.gas_state.cv, format!("J/(mol-{})", program_state.unit_text.temperature));
        println!("{:<30} {:10.4} {:10}", "Cp/Cv: ", program_state.gas_state.cp / program_state.gas_state.cv, "[]");
        println!("{:<30} {:10.4} {:10}", "Compressibility Z: ", program_state.gas_state.z, "[]");
        println!("{:<30} {:10.4} {:10}", "Isentropic Exponent k: ", program_state.gas_state.kappa, "[]");
        println!("{:<30} {:10.4} {:10}", "Speed of Sound w: ", program_state.gas_state.w, "m/s");
        println!("{:<30} {:10.4} {:10}", "Gibbs Energy: ", program_state.gas_state.g, "J/mol");
        println!("{:<30} {:10.4} {:10}", "Joule-Thompson Coefficient: ", program_state.gas_state.jt, format!("{}/kPa", program_state.unit_text.temperature));
        println!();
    }

    if program_state.show_inlet_state && program_state.show_discharge_state {
        let pr = program_state.discharge_state.p / program_state.inlet_state.p;
        let tr = program_state.discharge_state.t / program_state.inlet_state.t;
        let td = program_state.discharge_state.t - program_state.inlet_state.t;
        println!("{:<30} {:10.4} {:10}", "Pressure Ratio: ", pr, "[]");
        println!("{:<30} {:10.4} {:10}", "Temperature Ratio: ", tr, "[]");
        println!("{:<30} {:10.4} {:10}", "Temperature Rise: ", td, program_state.unit_text.temperature);        
    }

    print_main_menu(program_state);
}

enum GasComp {
    Air,
    Argon,
    Nitrogen,
    Oxygen,
}

fn get_gas_comp(gas_comp: GasComp) -> Composition{

    match gas_comp {
        GasComp::Air => Composition {
            nitrogen: 0.78,
            oxygen: 0.21,
            argon: 0.01,
            ..Default::default()
        },

        GasComp::Argon => Composition {
            argon: 1.0,
            ..Default::default()
        },

        GasComp::Nitrogen => Composition {
            nitrogen: 1.0,
            ..Default::default()
        },

        GasComp::Oxygen => Composition {
            oxygen: 1.0,
            ..Default::default()
        },
    }

}

struct UnitText {
    pressure: &'static str,
    temperature: &'static str,
    internal_energy: &'static str,
}


fn change_units(program_state: &mut ProgramState) {
    println!();
    println!("Select Unit:");
    println!("1 - Pressure ({})", program_state.unit_text.pressure);
    println!("2 - Temperature ({})", program_state.unit_text.temperature);
    println!("3 - Internal Energy ({})", program_state.unit_text.internal_energy);

    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => change_unit_pressure(program_state),
        "2" => change_unit_temperature(program_state),
        "3" => change_unit_internal_energy(program_state),
        _ => change_units(program_state),
    }
}

fn change_unit_pressure(program_state: &mut ProgramState) {
    println!("Select Pressure Unit:");
    println!("1 - kPa");
    println!("2 - Bar");
    println!("3 - PSI");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    match choice {
        "1" => {
            program_state.unit_text.pressure = "kPa";
            program_state.units.pressure = UnitPressure::kPa;
        },
        "2" => {
            program_state.unit_text.pressure = "Bar";
            program_state.units.pressure = UnitPressure::Bar;
        },
        "3" => {
            program_state.unit_text.pressure = "PSI";
            program_state.units.pressure = UnitPressure::PSI;
        },
        _ => change_unit_temperature(program_state),
    }
    print_gas_state(program_state);
}

fn change_unit_temperature(program_state: &mut ProgramState) {
    
    println!("Select Temperature Unit:");
    println!("1 - Celcius C");
    println!("2 - Kelvin K");
    println!("3 - Fahrenheit F");
    println!("4 - Rankine R");    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    match choice {
        "1" => {
            program_state.unit_text.temperature = "C";
            program_state.units.temp = UnitTemp::C;
        },
        "2" => {
            program_state.unit_text.temperature = "K";
            program_state.units.temp = UnitTemp::K;
        },
        "3" => {
            program_state.unit_text.temperature = "F";
            program_state.units.temp = UnitTemp::F;
        },
        "4" => {
            program_state.unit_text.temperature = "R";
            program_state.units.temp = UnitTemp::R;
        },
        _ => change_unit_temperature(program_state),
    }
    print_gas_state(program_state);
}

fn change_unit_internal_energy(program_state: &mut ProgramState) {
    println!("Select Internal Energy Unit:");
    println!("1 - J/mol");
    println!("2 - kJ/kg");
    println!("3 - BTU/lbm");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    match choice {
        "1" => {
            program_state.unit_text.internal_energy = "J/mol";
            program_state.units.internal_energy = UnitInternalEnergy::J_mol;
        },
        "2" => {
            program_state.unit_text.internal_energy = "kJ/kg";
            program_state.units.internal_energy = UnitInternalEnergy::kJ_kg;
        },
        "3" => {
            program_state.unit_text.internal_energy = "BTU/lbm";
            program_state.units.internal_energy = UnitInternalEnergy::BTU_lbm;
        },
        _ => change_unit_internal_energy(program_state),
    }
    print_gas_state(program_state);
}