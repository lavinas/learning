use oop::{Button, Screen, Screen2, SelectBox, Draw};

fn main() {
    main1();
    main2();
}

fn main1() {
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
            }) as Box<dyn Draw>,
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }) as Box<dyn Draw>,
        ],
    };

    screen.run();
}

fn main2() {
    let screen = Screen2 {
        components: vec![
            Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            },
        ]
    };
    screen.run();

}
