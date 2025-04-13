use std::io;
use std::error::Error;
use std::fs::File;
use csv::Writer;

struct Lift {
    name: String,

    weeks: [[f32; 3]; 4],
}

impl Lift {
    fn new(name: &str, max: f32) -> Self {
        Self {
            name: name.to_string(),
            weeks: [
                calculate_weight(max, 1),
                calculate_weight(max, 2),
                calculate_weight(max, 3),
                calculate_weight(max, 4),
            ],
        }
    }

    fn weeks2string(&self) -> [String; 4] {
        std::array::from_fn(|index| {
            self.weeks[index]
                .iter()
                .map(|weight| weight.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        })
    }
}

fn get_maxes() -> (f32, f32, f32, f32) {
    let mut input = String::new();

    println!("Enter your max for the press:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let press_max: f32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter your max for the squat:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let squat_max: f32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter your max for the bench:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let bench_max: f32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter your max for the deadlift:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let deadlift_max: f32 = input.trim().parse().expect("Please enter a valid number");

    (press_max, squat_max, bench_max, deadlift_max)
}

fn round_to_pattern(value: f32) -> f32 {
    let base = (value / 10.0).floor() * 10.0; // Get the base multiple of 10
    let remainder = value % 10.0; // Get the remainder

    // Determine the closest valid number in the pattern (0, 3, 5, 8)
    let adjustment = if remainder < 1.5 {
        0.0
    } else if remainder < 4.0 {
        3.0
    } else if remainder < 6.5 {
        5.0
    } else if remainder < 9.0 {
        8.0
    } else {
        10.0
    };

    let rounded = base + adjustment;

    // Ensure the value is at least 20
    if rounded < 20.0 {
        20.0
    } else {
        rounded
    }
}

fn calculate_weight(max: f32, week: i8) -> [f32; 3] {
    let weights = if week == 1 {
        [max * 0.65, max * 0.75, max * 0.85]
    } else if week == 2 {
        [max * 0.70, max * 0.80, max * 0.90]
    } else if week == 3 {
        [max * 0.75, max * 0.85, max * 0.95]
    } else if week == 4 {
        [max * 0.40, max * 0.50, max * 0.60]
    } else {
        println!("Invalid week number. Please enter a number between 1 and 4.");
        return [0.0, 0.0, 0.0];
    };

    [
        round_to_pattern(weights[0]),
        round_to_pattern(weights[1]),
        round_to_pattern(weights[2]),
    ]
}

fn generate_file(lifts: [Lift; 4]) -> Result<(), Box<dyn Error>> {
    // Create a new CSV writer
    let file = File::create("output.csv")?;
    let mut wtr = Writer::from_writer(file);

    // Write headers
    wtr.write_record(&["Lift", "Week 1", "Week 2", "Week 3", "Week 4"])?;

    // Write rows
    for lift in lifts {
        let weights_by_week = lift.weeks2string();
        wtr.write_record(&[
            lift.name, 
            weights_by_week[0].clone(), 
            weights_by_week[1].clone(), 
            weights_by_week[2].clone(), 
            weights_by_week[3].clone()
        ])?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    println!("CSV file written successfully!");
    Ok(())
}

fn main() {
    // Calculate 531 for press, squat, bench and deadlift using BBB

    // Get maxes for each lift
    let (press, squat, bench, deadlift) = get_maxes();

    println!("Your maxes are:");
    println!("Press: {}", press);
    println!("Squat: {}", squat);
    println!("Bench: {}", bench);
    println!("Deadlift: {}", deadlift);

    let press_lift = Lift::new("Press", press);
    let squat_lift = Lift::new("Squat", squat);
    let bench_lift = Lift::new("Bench", bench);
    let deadlift_lift = Lift::new("Deadlift", deadlift);
    
    if let Err(e) = generate_file([press_lift, squat_lift, bench_lift, deadlift_lift]) {
        eprintln!("Error generating CSV file: {}", e);
    }
}
