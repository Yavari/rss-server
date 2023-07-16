use scraper::ElementRef;

pub trait Extensions {
    fn get_string(self) -> String;
    fn get_url(self) -> Option<String>;
}

impl Extensions for ElementRef<'_> {
    fn get_string(self) -> String {
        self.text().collect::<Vec<_>>().join("")
    }

    fn get_url(self) -> Option<String> {
        self.value().attr("href").map(ToString::to_string)
    }
}
