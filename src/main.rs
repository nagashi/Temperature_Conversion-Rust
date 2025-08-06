use std::{fmt, io, process, str::FromStr};
use colored::Colorize;

/*
MERMAID_DIAGRAM_START
graph TD
A["main()->"Start Program]; 
A -- Call run_app() --> B{"run_app()"};

    A -- Ok(()) --> C[Program finished normally.];
    A -- Err(AppExitStatus::Quit) --> D[User quit. No further message.];
    A -- Err(AppExitStatus::IoError) --> E[Print I/O error & Exit with status 1];

    subgraph " "
        F[Display ""Temperature Conversion"" header] --> G{Get From Unit};
        G -- Valid Input --> H{Determine Temp Value Prompt};
        G -- Invalid Input --> G;
        G -- "QUIT" typed --> Z{Return Err AppExitStatus::Quit};

        H --> I{Get Original Value};
        I -- Valid Input --> J[Perform Temperature Conversion];
        I -- Invalid Input --> I;
        I -- "QUIT" typed --> Z;

        J --> K[Format Conversion Output];
        K --> L[Print Converted Temperature];
        L --> M{"Return Ok(())"};
    end

    B -- run_app() calls --> F;

    %%% Style Definitions %%%
    classDef startEndNode fill:#f9f,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;
    classDef successNode fill:#9C6,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;
    classDef quitNode fill:#C96,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;
    classDef errorNode fill:#F66,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;
    classDef processNode fill:#ADD8E6,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;
    classDef decisionNode fill:#FFFF99,stroke:#333,stroke-width:2px,color:#000,font-weight:bold;


    %%% Applying Styles to Nodes %%%
    class A,Z,M startEndNode;
    class C successNode;
    class D quitNode;
    class E errorNode;
    class F,H,J,K,L processNode;
    class B,G,I decisionNode;
MERMAID_DIAGRAM_END
*/

// --- Custom Error Type for Application Flow ---
#[derive(Debug)]
enum AppExitStatus {
    Quit,
    IoError(io::Error),
}

impl From<io::Error> for AppExitStatus {
    fn from(err: io::Error) -> Self {
        AppExitStatus::IoError(err)
    }
}

// --- Data Definitions (No Changes) ---
#[derive(Debug, PartialEq, Clone, Copy)]
enum TemperatureUnit {
    Fahrenheit,
    Celcius,
}

impl FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "c" | "celcius" => Ok(TemperatureUnit::Celcius),
            "f" | "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureUnit::Fahrenheit => write!(f, "F"),
            TemperatureUnit::Celcius => write!(f, "C"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Temperature {
    value: f64,
    unit: TemperatureUnit,
}

impl Temperature {
    fn new(value: f64, unit: TemperatureUnit) -> Self {
        Temperature { value, unit }
    }

    fn to_celcius(&self) -> Temperature {
        let celsius_value = (self.value - 32.0) * (5.0 / 9.0);
        Temperature::new(celsius_value, TemperatureUnit::Celcius)
    }

    fn to_fahrenheit(&self) -> Temperature {
        let fahrenheit_value = (self.value * (9.0 / 5.0)) + 32.0;
        Temperature::new(fahrenheit_value, TemperatureUnit::Fahrenheit)
    }

    fn convert_to(&self, target_unit: TemperatureUnit) -> Temperature {
        match (self.unit, target_unit) {
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celcius) => self.to_celcius(),
            (TemperatureUnit::Celcius, TemperatureUnit::Fahrenheit) => self.to_fahrenheit(),
            _ => *self,
        }
    }
}

// --- Pure Formatting Functions (No Changes) ---
fn format_conversion_output(
    original_value: f64,
    original_unit: TemperatureUnit,
    converted_temp: Temperature,
) -> String {
    let converted_unit_char = converted_temp.unit.to_string();

    match original_unit {
        TemperatureUnit::Fahrenheit => {
            if converted_temp.value.fract() != 0.0 {
                format!(
                    "\n({:.1}°{} - 32) * (5/9) = {:.1}°{}",
                    original_value, original_unit, converted_temp.value, converted_unit_char
                )
            } else {
                format!(
                    "\n({:.0}°{} - 32) * (5/9) = {:.0}°{}",
                    original_value,
                    original_unit,
                    converted_temp.value as u64,
                    converted_unit_char
                )
            }
        }
        TemperatureUnit::Celcius => {
            if converted_temp.value.fract() != 0.0 {
                format!(
                    "\n({:.1}°{} * 9/5) + 32 = {:.1}°{}",
                    original_value, original_unit, converted_temp.value, converted_unit_char
                )
            } else {
                format!(
                    "\n({:.0}°{} * 9/5) + 32 = {:.0}°{}",
                    original_value,
                    original_unit,
                    converted_temp.value as u64,
                    converted_unit_char
                )
            }
        }
    }
}

// --- Input Handling Functions (No Changes) ---
fn read_sanitized_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

fn get_user_input<T>(
    prompt_msg: &str,
    error_msg: &str,
    quit_action: &impl Fn(&str),
) -> Result<T, AppExitStatus>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    loop { // This loop remains for *invalid input retries* within a single prompt
        quit_action(prompt_msg);
        let input_str = read_sanitized_line()?;

        if input_str == "quit" {
            println!("{}", "Exiting program.".yellow().bold());
            return Err(AppExitStatus::Quit);
        }

        match input_str.parse::<T>() {
            Ok(value) => return Ok(value),
            Err(_) => println!("{}", error_msg.red().bold()),
        }
    }
}

// --- Main Application Logic (Modified) ---

// This function now performs a single conversion cycle and returns its status.
fn run_app() -> Result<(), AppExitStatus> {
    let quit_prompt = |msg: &str| {
        println!("\nType \"{}\" to end the program or\n{}", "QUIT".yellow().bold(), msg)
    };

    let msg_conversion: &'static str = "Enter C to convert to Fahrenheit or F to convert to Celsius";
    let error_conversion: &'static str = "Invalid input. Please enter 'C' or 'F'.";

    println!("\n{}", "--- Temperature Conversion ---".cyan().bold());

    // Get conversion unit, handling potential quit/errors
    let from_unit: TemperatureUnit = get_user_input(
        msg_conversion,
        error_conversion,
        &quit_prompt,
    )?; // The `?` operator propagates AppExitStatus::Quit or AppExitStatus::IoError

    // Determine the prompt message and error message for temperature value
    let (prompt_temp_value, error_temp_value) = match from_unit {
        TemperatureUnit::Celcius => (
            "Enter a number to convert Celsius to Fahrenheit.",
            "Invalid temperature. Please enter a number."
        ),
        TemperatureUnit::Fahrenheit => (
            "Enter a number to convert Fahrenheit to Celsius.",
            "Invalid temperature. Please enter a number."
        ),
    };

    // Get temperature value, handling potential quit/errors
    let original_value: f64 = get_user_input(
        prompt_temp_value,
        error_temp_value,
        &quit_prompt,
    )?; // The `?` operator propagates AppExitStatus::Quit or AppExitStatus::IoError

    // Pure computation
    let original_temp = Temperature::new(original_value, from_unit);
    let converted_temp = original_temp.convert_to(match from_unit {
        TemperatureUnit::Celcius => TemperatureUnit::Fahrenheit,
        TemperatureUnit::Fahrenheit => TemperatureUnit::Celcius,
    });

    // Pure formatting, then side effect
    let output_string = format_conversion_output(original_value, from_unit, converted_temp);
    println!("{}", output_string.green().bold());

    // If we reached this point, it means input was successful and conversion was printed.
    // So, we return Ok(()) to signal normal completion to `main`.
    Ok(())
}

fn main() {
    // Call the main application logic.
    match run_app() {
        Ok(_) => {
            // This arm is reached after a successful conversion.
            println!("\nProgram finished normally.");
        }
        Err(AppExitStatus::Quit) => {
            // User quit, the "Exiting program." message was already printed by get_user_input.
            // No additional message here.
        }
        Err(AppExitStatus::IoError(e)) => {
            eprintln!("{}", format!("Program terminated due to I/O error: {}", e).red().bold());
            process::exit(1); // Exit with a non-zero status for errors.
        }
    }
}