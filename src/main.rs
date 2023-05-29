use leptos::*;
use leptos_router::*;

mod pages;
use crate::pages::home::Home;
use crate::pages::notfound::NotFound;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
        <main>
            <Routes>
                <Route path="/" view=|cx| view! { cx, <Home/> }/>
                <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App /> }
    })
}
