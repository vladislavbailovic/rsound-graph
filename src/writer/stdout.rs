use std::io::{self, Result, Write};

use super::Writer;
use crate::{Graph, ImageRenderer};

#[derive(Default)]
pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Writer for StdoutWriter {
    fn write<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        let mut stdout = io::stdout();
        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            let _ = stdout.write(header)?;
        }
        let buffer = graph.draw(renderer);
        let _ = stdout.write(&buffer)?;
        if let Some(footer) = footer {
            let _ = stdout.write(footer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{svg::Renderer, Block, Roll};

    #[test]
    fn stdout_graph() {
        let graph = Roll::new(&[Block(4.0, 1.0)]);
        let renderer = Renderer::new(&graph.size());
        let w = StdoutWriter::new();

        if let Err(e) = w.write(renderer, graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "Out rendered");
        }
    }
}
