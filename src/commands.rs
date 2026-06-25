use crate::article;
use crate::blog::Blog;
use crate::generate;
use crate::rss;
use crate::server;
use crate::helper;
use chrono;
use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

const DEFAULT_ARTICLE_TEMPLATE: &str = "---\n\
                                        title: \"\"\n\
                                        date: \"{date}\"\n\
                                        slug: \"{slug}\"\n\
                                        draft: true\n\
                                        description: \"\"\n\
                                        ---";

#[derive(Parser)]
#[command(version)]
pub struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Build {
        #[arg[short, long]]
        output_dir: Option<PathBuf>
    },
    Serve,
    Create {
        article_slug: String,
    }
}

impl CLI {
    pub fn run(&self) {
        match &self.command {
            Command::Build { output_dir } => {
                if let Some(output_dir) = output_dir {
                    self.build_site(output_dir.to_string_lossy().into_owned());
                } else {
                    self.build_site("public/".to_string());
                }
            }
            Command::Serve => self.serve(),
            Command::Create { article_slug } => {
                let _ = self.create_article(article_slug.clone());
            }    
        }
    }

    fn build_site(&self, output_dir: generate::Path) {
        let config_path = "jet.toml".to_string();
        let articles_dir = "./articles".to_string();

        let blog = Blog::new(config_path, &articles_dir);
        let articles = article::get_articles(&articles_dir);

        let _ = generate::create_homepage_html_file(articles, &output_dir, true);
        let articles = article::get_articles(&articles_dir);

        for article in articles {
            if !article.draft {
                let mut output_dir_path = path::PathBuf::from(&output_dir);
                output_dir_path.push("posts/");

                match article::create_article_html_file(
                    &article,
                    "templates/article.html".to_string(),
                    output_dir_path.to_str().unwrap().to_string(),
                ) {
                    Ok(_ok) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                };
            }
        }

        helper::copy_assets_to_output_dir("assets/", &output_dir);
        rss::create_rss_xml(&blog, output_dir);

        println!("Site was generated successfully.");
    }

    fn create_article(&self, slug: String) -> io::Result<()> {
        let article_content = DEFAULT_ARTICLE_TEMPLATE
            .replace("{date}", chrono::Local::now().format("%Y-%m-%d").to_string().as_str())
            .replace("{slug}", slug.as_str());

        fs::write(format!("articles/{}.md", slug), article_content)?;
        println!("Create article: articles/{slug}.md");
        Ok(())
    }

    fn serve(&self) {
        println!("Web Server is available at http://localhost:3000/ (bind address 127.0.0.1) ");
        println!("Press Ctrl+C to stop");
        server::start_server();
        
    }
}
