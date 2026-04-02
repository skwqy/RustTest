// 定义一个 trait，用于绘制图形,相当与Java中的接口
pub trait Draw {
    fn draw(&self);
}

// 定义一个 Screen 结构体，用于管理图形组件
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 实现 Screen 结构体的 run 方法，用于绘制图形
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
        println!("Drawing a button with width {}, height {}, and label {}", self.width, self.height, self.label);
    }
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing a select box with width {}, height {}, and options {:?}", self.width, self.height, self.options);
    }
}



fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}