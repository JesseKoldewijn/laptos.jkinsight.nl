use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,  <div class="flex flex-col justify-center items-center min-h-screen gap-3">
        <span class="font-bold">
            "Home!"
        </span>
        <a href="/about" class="main-btn">
            "About"
        </a>
    </div> }
}
