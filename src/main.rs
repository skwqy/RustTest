use std::env;

#[path = "lessons/01_variables.rs"]
mod lesson01_variables;
#[path = "lessons/02_data_types.rs"]
mod lesson02_data_types;
#[path = "lessons/03_functions.rs"]
mod lesson03_functions;
#[path = "lessons/04_control_flow.rs"]
mod lesson04_control_flow;
#[path = "lessons/05_ownership.rs"]
mod lesson05_ownership;
#[path = "lessons/06_structs.rs"]
mod lesson06_structs;
#[path = "lessons/07_enums_and_match.rs"]
mod lesson07_enums_and_match;
#[path = "lessons/08_collections.rs"]
mod lesson08_collections;
#[path = "lessons/09_crates_and_modules.rs"]
mod lesson09_crates_and_modules;
#[path = "lessons/10_error_handling.rs"]
mod lesson10_error_handling;
#[path = "lessons/11_generics.rs"]
mod lesson11_generics;
#[path = "lessons/12_traits.rs"]
mod lesson12_traits;
#[path = "lessons/13_lifetimes.rs"]
mod lesson13_lifetimes;

struct Lesson {
    id: &'static str,
    title: &'static str,
    run: fn(),
}

fn lesson_list() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "01",
            title: "变量",
            run: lesson01_variables::run,
        },
        Lesson {
            id: "02",
            title: "数据类型",
            run: lesson02_data_types::run,
        },
        Lesson {
            id: "03",
            title: "函数",
            run: lesson03_functions::run,
        },
        Lesson {
            id: "04",
            title: "流程控制",
            run: lesson04_control_flow::run,
        },
        Lesson {
            id: "05",
            title: "所有权",
            run: lesson05_ownership::run,
        },
        Lesson {
            id: "06",
            title: "结构体",
            run: lesson06_structs::run,
        },
        Lesson {
            id: "07",
            title: "枚举和模式匹配",
            run: lesson07_enums_and_match::run,
        },
        Lesson {
            id: "08",
            title: "常见集合与操作",
            run: lesson08_collections::run,
        },
        Lesson {
            id: "09",
            title: "包和模块",
            run: lesson09_crates_and_modules::run,
        },
        Lesson {
            id: "10",
            title: "错误处理",
            run: lesson10_error_handling::run,
        },
        Lesson {
            id: "11",
            title: "泛型",
            run: lesson11_generics::run,
        },
        Lesson {
            id: "12",
            title: "Trait",
            run: lesson12_traits::run,
        },
        Lesson {
            id: "13",
            title: "生命周期",
            run: lesson13_lifetimes::run,
        },
    ]
}

fn print_usage(lessons: &[Lesson]) {
    println!("Rust 学习示例入口");
    println!("用法:");
    println!("  cargo run -- <序号>");
    println!("  cargo run -- all");
    println!();
    println!("可选序号:");
    for lesson in lessons {
        println!("  {} - {}", lesson.id, lesson.title);
    }
}

fn run_lesson(lesson: &Lesson) {
    println!();
    println!("========== {} {} ==========", lesson.id, lesson.title);
    (lesson.run)();
}

fn main() {
    let lessons = lesson_list();
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("all") => {
            for lesson in &lessons {
                run_lesson(lesson);
            }
        }
        Some(id) => {
            if let Some(lesson) = lessons.iter().find(|lesson| lesson.id == id) {
                run_lesson(lesson);
            } else {
                println!("未找到序号: {}", id);
                print_usage(&lessons);
            }
        }
        None => {
            print_usage(&lessons);
        }
    }
}