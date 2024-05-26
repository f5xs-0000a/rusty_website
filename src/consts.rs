pub struct ReqFields {
  pub ip: &'static str,
  pub referer: &'static str,
  pub user_agent: &'static str,
}

pub struct Paths {
  pub root: &'static str,
  pub nf404: &'static str,
  pub pd403: &'static str,
  pub meta: &'static str,
  pub menu: &'static str,
  pub shroompage: &'static str,

  pub frag_category: &'static str,
  pub frag_genus: &'static str,
  pub frag_species: &'static str,
  pub frag_menu: &'static str,
}

pub const YAML_FILE: &str = "/var/www/html/data/shroom_info.yaml";

pub const IMAGE_DIR: &str = "/var/www/html/mycology/Smallimages";

pub const LOG_FILE: &str = "/tmp//rusty_website.log";

pub mod domains {
  pub const NO_DOMAIN: &str = "localhost:7878";
  pub const MYCOLOGY: &str = "mycology.localhost:7878";
}

pub const FIELDS: ReqFields = ReqFields {
  ip: "X-Forwarded-For: ",
  referer: "Referer: ",
  user_agent: "User-Agent: ",
};

pub mod status {
  pub const HTTP_200: &str = "HTTP/1.1 200 OK";
  pub const HTTP_404: &str = "HTTP/1.1 404 NOT FOUND";
  pub const HTTP_403: &str = "HTTP/1.1 403 FORBIDDEN";
}

pub const PATH: Paths = Paths {
  root: "/var/www/html",
  nf404: "/var/www/html/data/404.html",
  pd403: "/var/www/html/data/403.html",
  meta: "/var/www/html/data/meta.html",
  menu: "/var/www/html/data/menu.html",
  shroompage: "/var/www/html/data/page.html",

  frag_category: "/var/www/html/data/fragments/category.html",
  frag_genus: "/var/www/html/data/fragments/genus.html",
  frag_species: "/var/www/html/data/fragments/species.html",
  frag_menu: "/var/www/html/data/fragments/menu.html",
};

pub const MIMETYPES: [(&str, &str); 6] = [
  ("jpg", "image/jpeg"),
  ("png", "image/png"),
  ("html", "text/html"),
  ("txt", "text/plain"),
  ("css", "text/css"),
  ("xml", "application/xml"),
];
