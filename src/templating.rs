use tera::{Context, Tera};
use crate::config::Config;
use crate::meta::MetaDataCollection;


pub struct TemplateEngine {
    tera: Tera,
    metadata: MetaDataCollection
}

impl TemplateEngine {
    pub fn new(template_dir: &str, metadata_file: &str) -> Self {
        let config = Config::load("rustic.config.json")
            .expect("Could not load the config file(Maybe The File is missing)");
        let tera = Tera::new(&format!("{}/{}/**/*.html", template_dir, config.template_path))
            .expect("Error initializing Tera templates");
        let metadata = MetaDataCollection::load(metadata_file)
            .expect("Failed to load metadata(Maybe The File is missing)");

        TemplateEngine { tera,  metadata}
    }

    pub fn render(&self, page_key: &str ,config: &Config, template_name: &str, content: &str) -> Result<String, tera::Error> {
        let mut context = Context::new();

    // Attempt to fetch metadata for the page
    if let Some(page_meta) = self.metadata.pages.get(page_key) {
        // Inject metadata if found
        context.insert("title", &page_meta.title);
        context.insert("description", &page_meta.description);
        context.insert("keywords", &page_meta.keywords);
        context.insert("canonical", &page_meta.canonical);
        context.insert("config", &config);
        context.insert("content", &content);
    } else {
        // Provide a clear error message if metadata is missing
        return Err(tera::Error::msg(format!(
            "Metadata for page '{}' is missing in the meta.json file.",
            page_key
        )));
    }

    // Render the template
    self.tera.render(template_name, &context)
}
}
