use std::fs;
use maud::{html, Markup, DOCTYPE};

const LOGO: &str = "assets/logo.png";

pub fn head() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en";
        head {
            title { "Lost to Time" }
            link rel="stylesheet" href="style.css";
            link rel="icon" href=(LOGO);
            script src="/assets/htmx.min.js" { }
            script src="/assets/preload.js" { }
        }
    }
}

pub fn header() -> Markup {
    html! {
        header {
            nav preload hx-boost="true" {
                img src=(LOGO) alt="Lost to time";
                ul.grid-5 {
                    li { a href="#art" class="button" { "Art" } }
                    li { a href="#creators" class="button" { "Creators" } }
                    li { a href="#tech" class="button" { "Tech" } }
                    li { a href="#roadmap" class="button" { "Roadmap" } }
                    li { a href="#story" class="button" { "Story" } }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            p { "Have a question? Reach us via our " a href="#creators" { "GitHub accounts" } }
        }
    }
}

pub fn base(content: Markup) -> Markup {
    html! {
        (head())
        body hx-ext="preload" {
            (header())
            main id="content" {
                (content)
            }
            (footer())
        }
    }
}

fn index() -> Markup {
    base(content())
}

fn hero() -> Markup {
    html! {
        #hero .banner {
            .grid-2 {
                .vid-wrap {
                    iframe src="https://www.youtube.com/embed/F79U9fpLYzs?controls=0&autoplay=1&mute=1&loop=1" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen {}
                }
                div {
                    h1 { "Lost to Time" }
                    p { "In Lost to Time you play as a scientist discovering your own time-rewinding powers. Use the powers to solve puzzles, save your daughter, and escape the facility you are held in." }
                    p {
                        a href="https://davidcode.itch.io/losttotime" target="_blank" { "Try the demo!" }
                        "Or scroll to read more..."
                    }
                }
            }
        }
    }
}

fn creators() -> Markup {
    html! {
        #creators .banner {
            h1 { "Creators" }
            p { "These descriptions should be way longer and more accurate! I just put some random stuff for now!" }
            .grid-5 {
                .card {
                    h1 { "David Giraldo" }
                    img src="https://avatars.githubusercontent.com/u/147272329";
                    p { "Level Designer" }
                    a href="https://github.com/DavidGiraldoCode" target="_blank" { "GitHub" }
                }
                .card {
                    h1 { "Samuel Söderberg" }
                    img src="https://avatars.githubusercontent.com/u/22292096";
                    p { "Visual Effects Programmer" }
                    a href="https://github.com/sasoder" target="_blank" { "GitHub" }
                }
                .card {
                    h1 { "NooN" }
                    img src="https://avatars.githubusercontent.com/u/14049705";
                    p { "Lead Programmer" }
                    a href="https://github.com/noon-io" target="_blank" { "GitHub" }
                }
                .card {
                    h1 { "Håvard Alstadheim" }
                    img src="https://avatars.githubusercontent.com/u/56519858";
                    p { "Lead Winner & Most Swaggy" }
                    a href="https://github.com/haval0" target="_blank" { "GitHub" }
                }
                .card {
                    h1 { "ErzaDuNord" }
                    img src="https://avatars.githubusercontent.com/u/102242407";
                    p { "Godlike Programmer" }
                    a href="https://github.com/ErzaDuNord" target="_blank" { "GitHub" }
                }
            }
        }
    }
}

fn content() -> Markup {
    html! {
        (hero())
        (creators())
    }
}

fn main() {
    let _ = fs::write("index.html", index().into_string());
}
