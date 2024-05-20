use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-web.css"/>
        
        // sets the document title
        <Title text="DSoft"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Link rel="icon" href="logo.svg" />
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {



    view! {
        <main class="container">
            <div class="pageSection">
                <p class="topright">
                    <a href="#top">
                    Domů
                    </a>
                    <a href="#sluzby">
                    Služby
                    </a>
                    <a href="#portfolio">
                    Porfólio
                    </a>
                    <a href="#aboutme">
                    O mně
                    </a>
                    <a href="#contact">
                    Kontakt
                    </a>
                </p>
                <img id="home" src="logo.svg" class="logo tauri" alt="Tauri logo"/>
            </div>
            <div class="pageSection">
                <div id="sluzby" class="h1Style">
                <h1>Služby</h1>
                </div>
                <div class="row-services">
                    <div>
                        <h2>"Mobilní aplikace"</h2>
                        <div class="containerPart">
                            <img src="mobileapp.svg" class="containerPartImage"></img>
                            <p class="divText">
                            "Vytvářím mobilní aplikace na míru v jazyku Dart a frameworku Flutter pro iOS a Android."
                            </p>
                        </div>
                    </div>
                    <div>
                        <h2>"Webové stránky"</h2>
                        <div class="containerPart">
                            <img src="web.svg" class="containerPartImage"></img>
                            <p class="divText">
                            "Webovky jakéhokoliv rázu. Od firemních webovek po blogy."
                            </p>
                        </div>
                    </div>
                    <div>
                        <h2>"Software"</h2>
                        <div class="containerPart">
                            <img src="software.svg" class="containerPartImage"></img>
                            <p class="divText">
                            "Software na míru, který je přesně přizpůsoben vašim potřebám."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="pageSection">
                <div id="portfolio" class="h1Style">
                    <h1>Porfólio</h1>
                </div>
                <div class="row">
                    <div class="rounded-image">
                        <img src="porfie.svg" />
                    </div>
                    <div class="row-image">
                        <p>"Jedná se o aplikaci a webovou stránku, která vám má pomoct se zaznamenáváním vašeho osobního, ale i
                        kariérního života v podobě příspěvků podobných příspěvkům na sociálních sítích."</p>
                    </div>
                </div>
                <div class="row"> 
                    <div class="rounded-image">
                        <img src="Sova_1.svg" id="sova" />
                    </div>
                    <div class="row-image">
                        <p>"V rámci digitalizace chytrých knih s elektronickými pery jsem byl součástí týmu pro výrobu aplikace pro estonské školky."</p>
                    </div>
                </div>
                <div class="row"> 
                    <div class="rounded-image">
                        <img src="SC.svg" id="SC" />
                    </div>
                    <div class="row-image">
                        <p>"Má maturitní práce, která je napodobeninou Instagramu/Twitteru a vytvořil jsem jí v rámci zakončení studia."</p>
                    </div>
                </div>
                <div class="row"> 
                    <div class="rounded-image">
                        <img src="myweb.svg" id="myweb" />
                    </div>
                    <div class="row-image">
                        <p>"Tato stránka jako taková je součástí mého portfólia. Je vytvořena pomocí HTML,CSS a hlavně backend Rust společně s frameworkem Leptos."</p>
                    </div>
                </div>
            </div>
            <div class="pageSection">
                <div id="aboutme" class="h1Style">
                    <h1>"O mně"</h1>
                </div>
                <div class="row-services">
                    <div class="rounded-image">
                        <img src="ja.jpg" id="ja" />
                    </div>
                    <div class="row-image">
                        <p>"Jsem (zatím) studentem IT oboru ve 4. ročníku na SŠTE Olomoucká. Zaměřuju se na tvorbu webových a mobilních aplikací v jazyku Dart a frameworku
                        Flutter. Poslední dobou se zaměřuji hlavně na backend a to v jazyku Rust, popřípadě frameworku Leptos pro frontend."</p>
                    </div>
                </div>
            </div>
            <div class="pageSection">
                <div id="contact" class="h1Style">
                    <h1>"Kontakt"</h1>
                </div>
                    <div class="formDiv">
                        <form>
                            <input type="text" id="sender" placeholder="Váš e-mail"></input>
                            <input type="text" id="subject" placeholder="Předmět"></input>
                            <textarea id="content" placeholder="Obsah zprávy"></textarea>
                            <input id="send" type="button" value="Odeslat"></input>
                        </form>
                    </div>
            </div>
        </main>
    }
    
}
