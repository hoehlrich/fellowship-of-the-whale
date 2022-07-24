use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::about_us::AboutUs;
use pages::calendar::Calendar;
use pages::contact::Contact;
use pages::documents::Documents;
use pages::donate::Donate;
use pages::home::Home;
use pages::page_not_found::PageNotFound;
use pages::posts::Posts;
use pages::press::Press;
use pages::sponsors::Sponsors;
use pages::teams::Teams;

use yew::html::Scope;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about_us")]
    AboutUs,
    #[at("/teams")]
    Teams,
    #[at("/posts")]
    Posts,
    #[at("/sponsors")]
    Sponsors,
    #[at("/documents")]
    Documents,
    #[at("/press")]
    Press,
    #[at("/calendar")]
    Calendar,
    #[at("/contact")]
    Contact,
    #[at("/donate")]
    Donate,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

#[derive(PartialEq)]
pub struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                   <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                     <div class="columns">
                        <div class="column">
                            <div class="content has-text-centered">
                                { "Copyright © 2022 FRC Team 4242: The Fellowship of the Whale" }
                            </div>
                        </div>
                        <div class="column">
                            <div class="content has-text-centered">
                                { "Powered by " }
                                <a href="https://yew.rs">{ "Yew" }</a>
                                { " using " }
                                <a href="https://bulma.io">{ "Bulma" }</a>
                            </div>
                        </div>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <>
                <section class="has-background-primary">
                    <div class="container">
                        <section class="hero is-primary">
                            <div class="hero-body ml-0">
                                <p class="title">
                                    { "FRC Team 4242: The Fellowship of the Whale" }
                                </p>
                                <p class="subtitle">
                                    { "Definately a real subtitle" }
                                </p>
                            </div>
                        </section>
                    </div>
                </section>
                <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
                    <div class="container">
                        <div class="navbar-brand">
                            <Link<Route> classes={"navbar-item"} to={Route::Home}>
                                <img src="../static/img/fotw_logo.png"/>
                            </Link<Route>>
                            <button class={classes!("navbar-burger", "burger", active_class)}
                                aria-label="menu" aria-expanded="false"
                                onclick={link.callback(|_| Msg::ToggleNavbar)}
                            >
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                            </button>
                        </div>
                        <div class={classes!("navbar-menu", active_class)}>
                            <div class="navbar-start">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                                    { "Home" }
                                </Link<Route>>
                                <div class="navbar-item has-dropdown is-hoverable">
                                    <div class="navbar-link">
                                        { "About Us" }
                                    </div>
                                    <div class="navbar-dropdown">
                                        <Link<Route> classes={classes!("navbar-item")} to={Route::AboutUs}>
                                            { "About Us" }
                                        </Link<Route>>
                                        <Link<Route> classes={classes!("navbar-item")} to={Route::Teams}>
                                            { "Teams" }
                                        </Link<Route>>
                                    </div>
                                </div>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                                    { "Posts" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Sponsors}>
                                    { "Sponsors" }
                                </Link<Route>>
                                <div class="navbar-item has-dropdown is-hoverable">
                                    <div class="navbar-link">
                                        { "Resources" }
                                    </div>
                                    <div class="navbar-dropdown">
                                        <Link<Route> classes={classes!("navbar-item")} to={Route::Documents}>
                                            { "Documents" }
                                        </Link<Route>>
                                        <Link<Route> classes={classes!("navbar-item")} to={Route::Press}>
                                            { "Press" }
                                        </Link<Route>>
                                    </div>
                                </div>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Calendar}>
                                    { "Calendar" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                                    { "Contact" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Donate}>
                                { "Donate" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </nav>
            </>
        }
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        Route::AboutUs => {
            html! { <AboutUs /> }
        }
        Route::Teams => {
            html! { <Teams /> }
        }
        Route::Posts => {
            html! { <Posts /> }
        }
        Route::Sponsors => {
            html! { <Sponsors /> }
        }
        Route::Documents => {
            html! { <Documents /> }
        }
        Route::Press => {
            html! { <Press /> }
        }
        Route::Calendar => {
            html! { <Calendar /> }
        }
        Route::Contact => {
            html! { <Contact /> }
        }
        Route::Donate => {
            html! { <Donate /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
