use crate::components::button::Button;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, InputEvent, SubmitEvent};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Todo {
    pub task: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoResponse {
    pub id: i64,
    pub task: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub data: Vec<TodoResponse>,
}

#[function_component]
pub fn Home() -> Html {
    let todos: UseStateHandle<Vec<TodoResponse>> = use_state(|| vec![]);
    let loading = use_state(|| false);
    let task_input = use_state(|| String::new());

    let fetch_todos = {
        let todos = todos.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let todos = todos.clone();
            let loading = loading.clone();

            // let response =Request::get()

            spawn_local(async move {
                loading.set(true);
                match Request::get("http://127.0.0.1:8000/todo").send().await {
                    Ok(response) => {
                        if let Ok(res) = response.json::<ApiResponse>().await {
                            console::log_1(
                                &format!("Fetched {} todos from API", res.data.len()).into(),
                            );
                            todos.set(res.data);
                        }
                        loading.set(true);
                    }
                    Err(err) => {
                        console::log_1(
                            &format!("There's an error fetching todos {:?}", err).into(),
                        );
                        todos.set(vec![]);
                        loading.set(true);
                    }
                }
            });
        })
    };

    {
        let fetch_todos = fetch_todos.clone();

        use_effect_with((), move |_| {
            fetch_todos.emit(());
        });
    }

    let handle_input_change = {
        let task_input = task_input.clone();

        Callback::from(move |e: InputEvent| {
            let input_val: HtmlInputElement = e.target_unchecked_into();
            console::log_1(&format!("input value =  {:?}", input_val.value()).into());
            task_input.set(input_val.value());
        })
    };

    let handle_add_todo = {
        let task_input = task_input.clone();
        let fetch_todos = fetch_todos.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input_val = (*task_input).clone().trim().to_string();

            if !input_val.is_empty() {
                let task_input = task_input.clone();
                let fetch_todos = fetch_todos.clone();

                spawn_local(async move {
                    let new_todo = Todo { task: input_val };
                    match Request::post("http://127.0.0.1:8000/todo")
                        .header("Content-Type", "application/json")
                        .json(&new_todo)
                    {
                        Ok(req) => {
                            if let Ok(res) = req.send().await {
                                task_input.set(String::new());

                                fetch_todos.emit(());
                            }
                        }
                        Err(err) => {
                            console::log_1(
                                &format!("There's an error adding todo {:?}", err).into(),
                            );
                        }
                    }
                });
            }
        })
    };

    html!(
        <main class="flex flex-col w-full h-screen justify-center items-center gap-10">
            <h1>{"Todo Application"}</h1>

            <div>
                <form class="flex flex-col items-center justify-center gap-10" onsubmit={handle_add_todo}>
                    <input type="text" placeholder="Enter a task to do..."
                    oninput={handle_input_change}
                    value={(*task_input).clone()}
                    />
                    <Button>{"Add Task"}</Button>
                </form>
            </div>

            if todos.is_empty() {
                // let loading = loading.clone();
                    if *loading  {
                        <div>{"Tasks Loading"}</div>
                    } else {
                        <div>
                        <p>{"No tasks found. Please add the task above"}</p>
                    </div>
                    }

            } else {
                {
                    for todos.iter().map(|todo| {
                        let task =todo.clone();
                        html!(
                            <div>
                            <p>{task.task}</p>
                            </div>
                        )
                    })
                }
            }
        </main>
    )
}

// let added_todo = false;
// useEffect =>
// useEffect(() => {
// console.log()
// fetch the todos
// },[added_todo, task_checange])

// use_effect()
// use_effect_with((dskdsa))
