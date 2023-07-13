use scraper::{Html, Selector};

pub fn test_allocated_before_match() {
    let con = Con::Bar;
    let foo = Foo {
        document: Html::parse_fragment(&"<h1>hello World Foo!</h1>"),
    };

    let bar = Bar {
        document: Html::parse_fragment(&"<h1>hello World Bar!</h1>"),
    };

    let result = match con {
        Con::Foo => foo.html(),
        Con::Bar => bar.html(),
    };

    let document = Html::parse_fragment(&result);
    let html = document.select(&Selector::parse("*").unwrap()).next().unwrap().inner_html();
    println!("{}", html);
}

pub fn test_allocated_in_match() {
    let con = Con::Bar;
    let result = match con {
        Con::Foo => Foo {
            document: Html::parse_fragment(&"<h1>hello World Foo!</h1>"),
        }.html(),
        Con::Bar => Bar {
            document: Html::parse_fragment(&"<h1>hello World Bar!</h1>"),
        }.html(),
    };

    let document = Html::parse_fragment(&result);
    let html = document.select(&Selector::parse("*").unwrap()).next().unwrap().inner_html();
    println!("{}", html);
}

enum Con {
    Foo,
    Bar,
}
pub struct Foo {
    document: Html,
}

pub struct Bar {
    document: Html,
}

impl Foo {
    fn html(&self) -> String {
        self.document.html()
    }

}

impl Bar {
    fn html(&self) -> String {
        self.document.html()
    }
}
