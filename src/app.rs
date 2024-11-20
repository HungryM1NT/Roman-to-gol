use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use regex::Regex;

use super::program::roman_to_arabic;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fullstack.css"/>
        <link data-trunk rel="copy-dir" href="/assets"/>

        // sets the document title
        <Title text="Number converter"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let re = Regex::new(r"^GO+L$").unwrap();

    let (input_value, set_input_value) = create_signal("".to_string());
    let (output_value, set_output_value) = create_signal(Ok(0));
    let on_input = move |ev| {
        set_input_value(event_target_value(&ev));
        set_output_value(roman_to_arabic(&input_value.get().to_uppercase()));
    };
    let output_element = move || {
        match re.captures(&input_value.get().to_uppercase()){
            Some(cap) => {
                let index = cap[0].chars().filter(|c| *c == 'O').count();
                let w = (200 * index).to_string();

                // view!{<p><img width={w} height="300" src="data :image/jpeg;base64,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"/></p>}
                view!{<p><img width={w} height="300" src="assets/gol_bear.jpg"/></p>}
            },
            None => {
                match output_value.get() {
                    Ok(value) => {
                        view!{<p>{format!("Arabic number: {value}")}</p>}
                    },
                    Err(err) => {
                        view!{<p>{format!("Error: {err}")}</p>}
                    }
                }
            }
        }
            // view!{<p><img width="1200" height="300" src="data:image/jpeg;base64,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"/></p>}
        // } else {
        //     match output_value.get() {
        //         Ok(value) => {
        //             view!{<p>{format!("Arabic number: {value}")}</p>}
        //         },
        //         Err(err) => {
        //             view!{<p>{format!("Error: {err}")}</p>}
        //         }
        //     }
        // }
    };
    
    view! {
        <h1>"Roman to arabic converter"</h1>
        <p>"Enter roman number: "
        <input type="text" size="20" on:input=on_input prop:value=input_value/>
        </p>
        {output_element}
    }
        

}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
