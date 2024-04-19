use {
    crate::{
        consts, html,
        mycology::parse,
        server::response::Response,
        types::{Categories, Content, Result},
    },
    std::io,
};

pub struct CatInfo {
    pub title: String,
    pub label: String,
    pub genera: Vec<GenInfo>,
}

pub struct GenInfo {
    pub title: String,
    pub species: Vec<SpecInfo>,
}

pub struct SpecInfo {
    pub title: String,
    pub name: String,
    pub blurb: String,
}

trait FilterData {
    fn contains(&self, requested_category: &str) -> bool;
    fn filter_data(self, requested_category: &str) -> CatInfo;
}

impl FilterData for Categories {
    fn contains(&self, requested_category: &str) -> bool {
        self.iter()
            .map(|cat| &cat.label)
            .any(|label| label == requested_category)
    }
    fn filter_data(self, requested_category: &str) -> CatInfo {
        self.into_iter()
            .find(|cat| cat.label == requested_category)
            .unwrap()
    }
}

trait FillTemplate {
    fn fill_menu(&self, categories: Categories, html_frag: &str) -> Content;
    fn fill_myc(&self, data: CatInfo) -> Result<Content>;
}

impl FillTemplate for String {
    fn fill_menu(&self, categories: Categories, html_frag: &str) -> Content {
        self.replace("{MENU}", &html::menu(&categories, html_frag))
            .as_bytes()
            .to_vec()
    }
    fn fill_myc(&self, data: CatInfo) -> Result<Content> {
        Ok(self
            .replace("{TITLE}", &data.title)
            .replace("{DATA}", &data.htmlify()?)
            .as_bytes()
            .to_vec())
    }
}

pub fn get(path: &str) -> Result<Response> {
    let requested_category = path.replace('/', "");
    let mime_type = "text/html";
    let categories = parse::yaml(parse::Parse::JustCats);
    match (
        categories.contains(&requested_category),
        requested_category.is_empty(),
    ) {
        (false, true) => Ok(Response {
            status: consts::status::HTTP_200,
            mime_type,
            content: html::from_file(consts::PATH.menu)?
                .fill_menu(categories, &html::from_file(consts::PATH.frag_menu)?),
        }),
        (true, false) => Ok(Response {
            status: consts::status::HTTP_200,
            mime_type,
            content: html::from_file(consts::PATH.shroompage)?
                .fill_myc(parse::yaml(parse::Parse::All).filter_data(&requested_category))?,
        }),
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "File Not Found",
        ))),
    }
}
