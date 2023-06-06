use leptos::*;
use leptos_router::*;

mod pages;
use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::notfound::NotFound;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <main>
                <nav class="w-full fixed h-14 flex items-center py-2 px-3 font-semibold">
                    "LeptosRS"
                </nav>
                <div class="app-root">
                    <Routes>
                        <Route path="/" view=|cx| view! { cx, <Home/> }/>
                        <Route path="/about" view=|cx| view! { cx, <About/> }/>
                        <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App /> }
    })
}
