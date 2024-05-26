use {
  crate::{
    consts,
    log::Err,
    mycology::generate::{CatInfo, GenInfo, SpecInfo},
    types::{Categories, Genera, Species, YamlChunks},
  },
  std::fs,
};

#[derive(Copy, Clone)]
enum Layer {
  Category,
  Genus,
  Species,
}

impl Layer {
  fn condition(&self, s: &str) -> bool {
    use Layer::*;
    match self {
      Category => !s.starts_with("  ") && s.ends_with(':'),
      // uhh, always false?
      Genus => s.starts_with("  ") && !s.starts_with("   ") && s.ends_with(':'),
      Species => s.starts_with("    ") && s.ends_with(':'),
    }
  }
}

pub enum Parse {
  All,
  JustCats,
}

trait Construct {
  fn struct_category(self, parse_all: Parse) -> Categories;
  fn struct_genus(self) -> Genera;
  fn struct_species(self) -> Species;
}

impl Construct for YamlChunks {
  fn struct_category(self, parse_all: Parse) -> Categories {
    self
      .into_iter()
      .map(|lines| {
        let label = lines.first().sanitise();
        let title = lines
          .iter()
          .find(|l| l.trim().starts_with("title:"))
          .sanitise();

        let genera = match parse_all {
          Parse::JustCats => vec![],
          Parse::All => split_by(lines, Layer::Genus).struct_genus(),
        };

        CatInfo {
          label,
          title,
          genera,
        }
      })
      .collect()
  }

  fn struct_genus(self) -> Genera {
    self
      .into_iter()
      .map(|lines| {
        let title = lines.first().sanitise();
        let species = split_by(lines, Layer::Species).struct_species();
        GenInfo { title, species }
      })
      .collect()
  }

  fn struct_species(self) -> Species {
    self
      .into_iter()
      .map(|lines| {
        let mut species = lines.iter();
        let title = species.next().sanitise();
        let name = species.next().sanitise();
        let blurb = species.map(|s| (Some(s)).sanitise()).collect();
        SpecInfo { title, name, blurb }
      })
      .collect()
  }
}

trait Sanitise {
  fn sanitise(self) -> String;
}

impl Sanitise for Option<&String> {
  fn sanitise(self) -> String {
    self
      .unwrap_or(&String::new())
      .trim()
      .trim_start_matches("blurb: ")
      .trim_start_matches("common_name: ")
      .trim_start_matches("title: ")
      .replace(':', "")
  }
}

pub fn yaml(parse_all: Parse) -> Categories {
  match fs::read_to_string(consts::YAML_FILE) {
    Ok(v) => {
      let yaml = v.split('\n').map(str::to_string).collect();
      let categories = split_by(yaml, Layer::Category);
      categories.struct_category(parse_all)
    }
    Err(e) => {
      format!("yaml munching error. :( - {} {}", e, consts::YAML_FILE).log_err();
      vec![]
    }
  }
}

fn split_by(lines: Vec<String>, layer: Layer) -> YamlChunks {
  let divisions: Vec<usize> = lines
    .iter()
    .enumerate()
    .filter(|(_, s)| layer.condition(s))
    .map(|(i, _)| i)
    .collect();
  let m_divs = &divisions;

  divisions
    .iter()
    .enumerate()
    .map(|(i, n)| {
      lines[match m_divs.get(i + 1) {
        Some(v) => *n..*v,
        None => *n..lines.len(),
      }]
      .to_vec()
    })
    .collect()
}
