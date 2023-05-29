use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,  <div class="flex flex-col justify-center items-center min-h-screen">
        <span>
            "Hello, world!"
        </span>
    </div> }
}
