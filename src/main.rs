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
            /*
            script src="/assets/htmx.min.js" { }
            script src="/assets/preload.js" { }
            */
        }
    }
}

pub fn header() -> Markup {
    html! {
        header {
            nav preload hx-boost="true" {
                img src=(LOGO) alt="Lost to time";
                ul.grid-5 {
                    li { a href="#creators" class="button" { "Creators" } }
                    li { a href="#art" class="button" { "Art" } }
                    li { a href="#tech" class="button" { "Tech" } }
                    li { a href="#roadmap" class="button" { "Roadmap" } }
                    //li { a href="#story" class="button" { "Story" } }
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
        #hero .banner .caution-border {
            .grid-2 {
                .vid-wrap {
                    iframe src="https://www.youtube.com/embed/IN2wRIxQKzE?controls=0&autoplay=1&mute=1&loop=1&playlist=IN2wRIxQKzE" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen {}
                }
                div {
                    h1 { "Lost to Time" }
                    p { "In Lost to Time you play as a scientist discovering your own time-rewinding powers. Use the powers to solve puzzles, save your daughter, and escape the facility you are held in." }
                    p {
                        a href="https://davidcode.itch.io/losttotime" target="_blank" { "Try the demo!" }
                        "Or scroll on..."
                    }
                }
            }
        }
    }
}

fn creators() -> Markup {
    html! {
        #creators .banner {
            h1 { "CREATORS" }
            .creators-grid-top {
                div { }
                .card .caution-border {
                    h1 { "David Giraldo" }
                    img src="https://avatars.githubusercontent.com/u/147272329";
                    ul { 
                        li { "LEAN development" }
                        li { "Diegetic UI" }
                        li { "Targeted rewind" }
                        li { "Git flow" }
                        li { "Level design" }
                        li { "." } //lazy fix for aligment issue (pad all cards to equal number of
                                   //points)
                    }
                    a href="https://github.com/DavidGiraldoCode" target="_blank" { "GitHub" }
                }
.card .caution-border {
                    h1 { "Samuel Söderberg" }
                    img src="https://avatars.githubusercontent.com/u/22292096";
                    ul {
                        li { "Game concept" }
                        li { "Sound design" }
                        li { "Object selection logic" }
                        li { "Visual effects" }
                        li { "Trailer & logo" }
                        li { "Level concepts" }
                    }
                    a href="https://github.com/sasoder" target="_blank" { "GitHub" }
                }
            div { }
            }
            .creators-grid-bot {
                .card .caution-border {
                    h1 { "NooN" }
                    img src="https://avatars.githubusercontent.com/u/14049705";
                    ul {
                        li { "Software architecture" }
                        li { "Time-keeping system" }
                        li { "Interaction system" }
                        li { "UI/HUD system" }
                        li { "🦭" }
                        li { "." }
                    }
                    a href="https://github.com/noon-io" target="_blank" { "GitHub" }
                }
                .card .caution-border {
                    h1 { "Håvard Alstadheim" }
                    img src="https://avatars.githubusercontent.com/u/56519858";
                    ul {
                        li { "Grabbing system" }
                        li { "Website" }
                        li { "Hatch system" }
                        li { "Hatch animation" }
                        li { "." }
                        li { "." }
                    }
                    a href="https://github.com/haval0" target="_blank" { "GitHub" }
                }
                .card .caution-border {
                    h1 { "ErzaDuNord" }
                    img src="https://avatars.githubusercontent.com/u/102242407";
                    ul {
                        li { "SFX system" }
                        li { "Conveyor belts" }
                        li { "Object (de)spawning" }
                        li { "Playtesting & QA" }
                        li { "." }
                        li { "." }
                    }
                    a href="https://github.com/ErzaDuNord" target="_blank" { "GitHub" }
                }
            }
        }
    }
}

fn art() -> Markup {
    html! {
        #art .banner .caution-border {
            .grid-2 {
                .card {
                    h1 { "Art" }
                    p { "Lost to Time has a very thoroughly executed PSX-like art style." }
                    p { "We use low-poly models and low-resolution textures to evoke the sense of 90s/00s games we are inspired by, such as Half Life or Portal." }
                    p { "We are also exploring various shaders with effects such as dithering that may help create a sense of playing on a classic CRT monitor." }
                }
                #art-showcase {
                    div {
                    img src="assets/scr1.png";
                    img src="assets/prop1.png";
                    img src="assets/scr2.png";
                    img src="assets/scr3.png";
                    img src="assets/prop2.png";
                    }
                    div {
                    img src="assets/scr4.png";
                    img src="assets/prop3.png";
                    img src="assets/scr5.png";
                    img src="assets/scr6.png";
                    }
                    div {
                    img src="assets/prop4.png";
                    img src="assets/prop5.png";
                    img src="assets/prop6.png";
                    img src="assets/scr7.png";
                    }
                }
            }
        }
    }
}

fn tech() -> Markup {
    html! {
        #tech .banner .caution-border {
            h1 { "Tech" }
            h2 { "Tools" }
            p { "The game is created in Unity and programmed in C#. We use several advanced features and extentions to Unity such as ProBuilder and URP." }
            h2 { "Systems" }
            p { "The game itself is built on several sophisticated systems. Handling the storing of time and rewinding objects through time and space requires careful consideration. The grabbing system also turned out to demand more nuance and thought than expected, in order to fit our specific requirements." }
        }
    }
}

fn roadmap() -> Markup {
    html! {
        #roadmap .banner .caution-border {
            h1 { "Roadmap" }
            p { "The game is planned to launch in early access on Steam. The current Demo represents part of the first chapter of the game. Future chapters will introduce more of the story, and new gameplay mechanics, making for new and exciting puzzle scenarios." }
        }
    }
}

fn note() -> Markup {
    html! {
        #note .banner {
            strong { "NOTE: Lost to Time was created as a student project at KTH and is no longer being worked on." }
        }
    }
}

fn content() -> Markup {
    html! {
        (hero())
        (note())
        (creators())
        (art())
        (tech())
        (roadmap())
    }
}

fn main() {
    let _ = fs::write("index.html", index().into_string());
}
