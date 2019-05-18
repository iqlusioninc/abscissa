//! Template iterator

use super::Template;
use hashbrown::hash_map;

type HandlebarsIter<'a> = hash_map::Iter<'a, String, handlebars::Template>;

/// Iterator over all registered template files
pub struct Iter<'a>(HandlebarsIter<'a>);

impl<'a> Iter<'a> {
    pub(super) fn new(templates: HandlebarsIter<'a>) -> Self {
        Iter(templates)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Template<'a>;

    fn next(&mut self) -> Option<Template<'a>> {
        self.0
            .next()
            .map(|(path, template)| Template::new(path.as_ref(), template))
    }
}
