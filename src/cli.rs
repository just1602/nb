use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(New),
    Move(Move),
    Tags,
    Tag(Tag),
    Link(Link),
    Search(Search),
}

#[derive(Parser)]
#[command(name = "new", alias = "n", about = "Create a new note")]
pub struct New {
    pub title: String,
    // TODO: allow to pass tags to the new command
}

#[derive(Parser)]
#[command(name = "move", aliases = ["rn", "mv"], visible_alias = "rename", about = "Move or rename a note")]
pub struct Move {
    src: PathBuf,
    dest: PathBuf,
}

#[derive(Parser)]
#[command(name = "tag", about = "Tag notes")]
pub struct Tag {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Parser)]
#[command(name = "link", about = "Link notes together")]
pub struct Link {
    pub note1: PathBuf,
    pub note2: PathBuf,
}

#[derive(Parser)]
#[command(name = "search", about = "Search notes based user input query")]
pub struct Search {
    pub query: Vec<String>,
}
