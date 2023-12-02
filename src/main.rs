use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="text-4xl font-bold">{"Hello, World!"}</h1>
            <button class="bg-neutral-600 hover:bg-neutral-800 text-white font-bold py-2 px-4 rounded" onclick={onclick}>
                {"Clicked: "} {*counter}
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
