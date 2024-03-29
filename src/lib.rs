use maud::{html, Markup, DOCTYPE};

pub struct Boilerplate {
    title: &'static str,
    links: Vec<Markup>,
    body: Option<Markup>,
}

impl Boilerplate {
    pub fn new(title: &'static str) -> Self {
        Boilerplate {
            title,
            links: Vec::new(),
            body: None,
        }
    }

    /// Adds arbitrary markup to the body
    ///
    /// Note this should *include* the body element itself e.g.
    /// ```
    /// boilerplate.body(html! body {
    ///     p {
    ///         your body goes here
    ///     }
    /// })
    /// ```
    pub fn body(mut self, body: Markup) -> Self {
        self.body = Some(body);
        self
    }

    /// Adds a css stylesheet link to the head
    ///
    /// ```html
    /// <link rel="stylesheet" type="text/css" href="(self.0)">
    /// ```
    pub fn link_css(mut self, href: &'static str) -> Self {
        self.links.push(html! {
            link rel="stylesheet" type="text/css" href=(href);
        });
        self
    }

    /// Adds an icon link to the head
    ///
    /// ```html
    /// <link rel="icon" type="text/x-icon" href="(self.0)">
    /// ```
    pub fn link_icon(mut self, href: &'static str) -> Self {
        self.links.push(html! {
            link rel="icon" type="text/x-icon" href=(href);
        });
        self
    }

    /// Adds arbitrary markup to the head alongisde other links, it's up to you to render the link
    ///
    /// Use this if the available link_* methods don't meet your requirements
    pub fn link(mut self, link: Markup) -> Self {
        self.links.push(link);
        self
    }
}

impl maud::Render for Boilerplate {
    fn render(&self) -> Markup {
        let mut buffer = String::new();
        self.render_to(&mut buffer);
        maud::PreEscaped(buffer)
    }

    fn render_to(&self, buffer: &mut String) {
        let page = html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http-equiv="X-UA-Compatible" content="ie=edge";
                    title {(self.title)}
                    @for link in &self.links {
                        (link)
                    }
                }
            }
            @if let Some(ref body) = self.body {
                (body)
            }
        };
        page.render_to(buffer)
    }
}

#[cfg(feature = "rocket")]
mod rocket_support {
    use maud::Render;
    use rocket::{response::Responder, Request};

    use crate::Boilerplate;

    impl<'r> Responder<'r, 'static> for Boilerplate {
        fn respond_to(self, req: &Request) -> rocket::response::Result<'static> {
            self.render().respond_to(req)
        }
    }
}
