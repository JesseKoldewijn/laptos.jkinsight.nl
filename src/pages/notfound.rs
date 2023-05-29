use leptos::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,  <div class="flex flex-col justify-center items-center min-h-screen">
        <div>
            <h1>
                "404"
            </h1>
            "|"
            <span>
                "Page not found"
            </span>
        </div>
    </div>  }
}
