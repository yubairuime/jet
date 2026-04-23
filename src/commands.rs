use crate::articles;
use crate::generate;
use crate::blog;
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
pub struct Cli {
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

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Command::Build { output_dir } => {
                if let Some(output_dir) = output_dir {
                    build_site(output_dir.to_string_lossy().into_owned());
                } else {
                    build_site("public/".to_string());
                }
            }
            Command::Serve => { serve(); },
            Command::Create { article_slug } => {
                let _ = create_article(article_slug.clone());
            }    
        }
    }
}

pub fn build_site(output_dir: generate::Path) {
    let articles_directory = "./articles".to_string();

    let blog = blog::Blog {
        config: blog::read_blog_config("jet.toml".to_string()),
        articles: articles::get_articles(&articles_directory)
    };

    let articles = articles::get_articles(&articles_directory);

    let _ = generate::create_homepage_html_file(articles, &output_dir, true);
    let articles = articles::get_articles(&articles_directory);

    for article in articles {
        if !article.draft {
            let mut output_dir_path = path::PathBuf::from(&output_dir);
            output_dir_path.push("posts/");

            match articles::create_article_html_file(
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

pub fn create_article(article_slug: String) -> io::Result<()> {
    let article_content = DEFAULT_ARTICLE_TEMPLATE
        .replace("{date}", chrono::Local::now().format("%Y-%m-%d").to_string().as_str())
        .replace("{slug}", article_slug.as_str());

    fs::write(format!("articles/{}.md", article_slug), article_content)?;
    println!("Create article: articles/{}.md", article_slug.clone());
    return Ok(());
}

pub fn serve() {
    println!("Web Server is available at http://localhost:3000/ (bind address 127.0.0.1) ");
    println!("Press Ctrl+C to stop");
    server::start_server();
}
