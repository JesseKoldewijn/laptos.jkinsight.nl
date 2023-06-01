use leptos::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx, <div class="flex flex-col justify-center items-center min-h-screen gap-3">
        <span class="font-bold">
            "About!"
        </span>
        <a href="/" class="main-btn">
            "Home"
        </a>
    </div> }
}
