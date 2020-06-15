mod bounding_box;
mod light;

use bounding_box::BoundingBox;
use light::Light;

use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut lights: Vec<Light> = vec![];
    for line in contents.lines() {
        lights.push(line.parse()?);
    }

    let steps = find_correct_state(&mut lights);
    let output = render(&mut lights);

    println!("{}", output);
    println!("Message displayed in {} steps", steps);

    Ok(())
}

fn get_bounding_box(lights: &[Light]) -> BoundingBox {
    BoundingBox {
        min_x: lights.iter().map(|l| l.position.x).min().unwrap(),
        max_x: lights.iter().map(|l| l.position.x).max().unwrap(),
        min_y: lights.iter().map(|l| l.position.y).min().unwrap(),
        max_y: lights.iter().map(|l| l.position.y).max().unwrap(),
    }
}

fn step_forward(lights: &mut [Light]) {
    for light in lights {
        light.position.x += light.velocity.x;
        light.position.y += light.velocity.y;
    }
}

fn step_backward(lights: &mut [Light]) {
    for light in lights {
        light.position.x -= light.velocity.x;
        light.position.y -= light.velocity.y;
    }
}

fn find_correct_state(lights: &mut [Light]) -> u32 {
    let mut min_bounding_box = i64::MAX;
    let mut bounding_box;
    let mut steps = 0;

    loop {
        step_forward(lights);

        bounding_box = get_bounding_box(lights);
        if bounding_box.area() < min_bounding_box {
            min_bounding_box = bounding_box.area();
        } else {
            // BOUNDING BOX NOT GETTING SMALLER =>
            // previous state contained the message
            step_backward(lights);
            break;
        }

        steps += 1;
    }

    steps
}

fn render(lights: &mut [Light]) -> String {
    let bounding_box = get_bounding_box(lights);

    let width = bounding_box.width() + 1;
    let height = bounding_box.height() + 1;

    let mut buffer = vec![vec!['.'; width]; height];

    for light in lights {
        let x = (light.position.x - bounding_box.min_x) as usize;
        let y = (light.position.y - bounding_box.min_y) as usize;

        buffer[y][x] = '#';
    }

    buffer
        .into_iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
