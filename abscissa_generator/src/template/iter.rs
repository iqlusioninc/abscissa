//! Template iterator

use super::Template;

/// Iterator over all registered template files
pub struct Iter<'a> {
    /// Template collection
    // TODO(tarcieri): less hacky implementation
    templates: std::vec::IntoIter<Template<'a>>,
}

impl<'a> Iter<'a> {
    pub(super) fn new(templates: Vec<Template<'a>>) -> Self {
        Iter {
            templates: templates.into_iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Template<'a>;

    fn next(&mut self) -> Option<Template<'a>> {
        self.templates.next()
    }
}
