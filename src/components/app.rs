use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="main" href="/assets/main.css"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Title text="Timelocker"/>

        <div class="container">
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                </Routes>
            </main>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Timelocker"</h1>
        </div>
    }
}
