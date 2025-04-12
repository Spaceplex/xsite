use maud::{html, Markup};

pub async fn home() -> Markup {
    html! {
        p { "Home stuff over here :(" }
    }
}

pub async fn projects() -> Markup {
    html! {
        p { "It's in the works, maybe idk I promise" }
    }
}

pub async fn blogs() -> Markup {
    html! {
        p { "Blogs coming" }
    }
}

pub async fn socials() -> Markup {
    html! {
        div .socials {
            a href="https:/x.com/_spaceplex" {
                h1 {"Twitter"}
            }
            a href="https:/youtube.com/@spaceplex5079" {
                h1 {"Youtube"}
            }
        }
    }
}
