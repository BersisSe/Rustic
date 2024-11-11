use tera::{Tera, Context};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_dir: &str) -> Self {
        // Use glob pattern to include all .html files in the specified directory and subdirectories
        let tera = Tera::new(&format!("{}/*.html", template_dir))
            .expect("Error initializing Tera templates");

        TemplateEngine { tera }
    }

    pub fn render(&self, template_name: &str, data: &Context) -> Result<String, tera::Error> {
        self.tera.render(template_name, data)
    }
}
