use clap::{command, Arg};

fn main() {
    println!("This is advent of code 2023!");

    let matches = command!()
        .arg(Arg::new("day").short('d').long("day"))
        .arg(Arg::new("task").short('t').long("task"))
        .get_matches();

    let registered = vec![
        vec![
            advent_of_code::days::day_1::task_1::run,
            advent_of_code::days::day_1::task_2::run,
        ],
        vec![
            advent_of_code::days::day_2::task_1::run,
            advent_of_code::days::day_2::task_2::run,
        ],
        vec![
            advent_of_code::days::day_3::task_1::run,
            advent_of_code::days::day_3::task_2::run,
        ],
        vec![
            advent_of_code::days::day_4::task_1::run,
            advent_of_code::days::day_4::task_2::run,
        ],
        vec![
            advent_of_code::days::day_5::task_1::run,
            advent_of_code::days::day_5::task_2::run,
        ],
        vec![
            advent_of_code::days::day_6::task_1::run,
            advent_of_code::days::day_6::task_2::run,
        ],
        vec![
            advent_of_code::days::day_7::task_1::run,
            advent_of_code::days::day_7::task_2::run,
        ],
    ];

    let day = matches.get_one::<String>("day");
    let task = matches.get_one::<String>("task");

    match (day, task) {
        (Some(day), Some(task)) => {
            let day = day.parse::<usize>().unwrap();
            let task = task.parse::<usize>().unwrap();
            let res = registered[day - 1][task - 1]("input.txt");
            println!("Day: {}, Task: {}...", day, task);
            println!("Result: {}", res);
        }

        (Some(day), None) => {
            let day = day.parse::<usize>().unwrap();
            let last_task = registered[day - 1].len() - 1;
            let res = registered[day - 1][last_task]("input.txt");
            println!("Day: {}, Task: {}...", day, last_task + 1);
            println!("Result: {}", res);
        }

        _ => {
            let last_day = registered.len() - 1;
            let last_task = registered[last_day].len() - 1;
            let res = registered[last_day][last_task](&format!("input.txt"));
            println!("Day: {}, Task: {}...", last_day + 1, last_task + 1);
            println!("Result: {}", res);
        }
    }
}
