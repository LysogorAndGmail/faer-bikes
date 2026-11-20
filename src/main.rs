use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn ProgressBar(
	#[prop(default = 100)]
        _max: u16,
        #[prop(into)]
        progress: Signal<i32>
) -> impl IntoView {
	view! {
		<progress 
			max="50"
			value=progress
                />
	}
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| *set_count.write() += 3
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count) />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
