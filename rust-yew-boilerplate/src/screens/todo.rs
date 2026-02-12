use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent, SubmitEvent, console};
use yew::{Callback, Html, TargetCast, function_component, html, use_effect_with, use_state};

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpdateTodoType {
    pub task: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ApiSingleResponse {
    pub message: String,
    pub data: TodoResponse,
}

#[function_component(TodoScreen)]
pub fn todo() -> Html {
    let todos = use_state(|| vec![]);
    let input_value = use_state(|| String::new());
    let edit_todo_id = use_state(|| Option::<i64>::None);
    let edit_todo_text = use_state(|| String::new());
    let loading = use_state(|| false);

    let fetch_todos = {
        let todos = todos.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            console::log_1(&"fetch_todos called".into());

            let todos = todos.clone();
            let loading = loading.clone();

            loading.set(true);
            spawn_local(async move {
                console::log_1(&"Fetching todos from API...".into());
                match Request::get("http://127.0.0.1:8000/todo").send().await {
                    Ok(response) => {
                        if let Ok(api_response) = response.json::<ApiResponse>().await {
                            console::log_1(
                                &format!("Fetched {} todos from API", api_response.data.len())
                                    .into(),
                            );
                            todos.set(api_response.data);
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("Failed to fetch todos: {:?}", e).into());
                        todos.set(vec![]);
                    }
                }
                loading.set(false);
            });
        })
    };

    {
        let fetch_todos = fetch_todos.clone();
        use_effect_with((), move |_| {
            fetch_todos.emit(());
            || ()
        });
    }

    let handle_add_todo = {
        let input_value = input_value.clone();
        let fetch_todos = fetch_todos.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let task_value = (*input_value).clone().trim().to_string();

            if !task_value.is_empty() {
                let input_value = input_value.clone();
                let fetch_todos = fetch_todos.clone();
                let loading = loading.clone();

                loading.set(true);
                spawn_local(async move {
                    let new_todo = Todo { task: task_value };

                    match Request::post("http://127.0.0.1:8000/todo")
                        .header("Content-Type", "application/json")
                        .json(&new_todo)
                    {
                        Ok(req) => {
                            if let Ok(response) = req.send().await {
                                if response.status() == 201 {
                                    input_value.set(String::new());
                                    fetch_todos.emit(());
                                } else {
                                    loading.set(false);
                                }
                            } else {
                                loading.set(false);
                            }
                        }
                        Err(_) => {
                            loading.set(false);
                        }
                    }
                });
            }
        })
    };

    let handle_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let handle_delete_todo = {
        let fetch_todos = fetch_todos.clone();
        let loading = loading.clone();

        Callback::from(move |todo_id: i64| {
            let fetch_todos = fetch_todos.clone();
            let loading = loading.clone();

            loading.set(true);
            spawn_local(async move {
                match Request::delete(&format!("http://127.0.0.1:8000/todo/{}", todo_id))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            fetch_todos.emit(());
                        } else {
                            loading.set(false);
                        }
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
        })
    };

    let handle_toggle_status = {
        let fetch_todos = fetch_todos.clone();
        let loading = loading.clone();
        let todos = todos.clone();

        Callback::from(move |todo_id: i64| {
            console::log_1(&format!("Toggle status clicked for todo ID: {}", todo_id).into());

            let fetch_todos = fetch_todos.clone();
            let loading = loading.clone();
            let todos = todos.clone();

            loading.set(true);

            spawn_local(async move {
                console::log_1(&"Fetching current todo status...".into());
                match Request::get(&format!("http://127.0.0.1:8000/todo/{}", todo_id))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(current_todo) = response.json::<ApiSingleResponse>().await {
                            let new_status = match current_todo.data.status {
                                Status::PENDING => "DONE".to_string(),
                                Status::DONE => "PENDING".to_string(),
                            };

                            console::log_1(
                                &format!(
                                    "Current status: {:?}, updating to: {}",
                                    current_todo.data.status, new_status
                                )
                                .into(),
                            );

                            let update_data = UpdateTodoType {
                                task: None,
                                status: Some(new_status),
                            };

                            console::log_1(&"Sending PUT request...".into());
                            match Request::put(&format!("http://127.0.0.1:8000/todo/{}", todo_id))
                                .header("Content-Type", "application/json")
                                .json(&update_data)
                            {
                                Ok(req) => match req.send().await {
                                    Ok(response) => {
                                        console::log_1(
                                            &format!("PUT response status: {}", response.status())
                                                .into(),
                                        );
                                        if response.status() == 200 {
                                            console::log_1(
                                                &"Status update successful, fetching todos..."
                                                    .into(),
                                            );
                                            fetch_todos.emit(());
                                        } else {
                                            console::log_1(
                                                &"PUT request failed with non-200 status".into(),
                                            );
                                            loading.set(false);
                                        }
                                    }
                                    Err(_) => {
                                        console::log_1(
                                            &"Failed to get response from PUT request".into(),
                                        );
                                        loading.set(false);
                                    }
                                },
                                Err(e) => {
                                    console::log_1(&format!("PUT request error: {:?}", e).into());
                                    loading.set(false);
                                }
                            }
                        } else {
                            console::log_1(&"Failed to parse current todo response".into());
                            loading.set(false);
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("Failed to fetch current todo: {:?}", e).into());
                        loading.set(false);
                    }
                }
            });
        })
    };

    let handle_edit_click = {
        let edit_todo_id = edit_todo_id.clone();
        let edit_todo_text = edit_todo_text.clone();

        Callback::from(move |(id, text): (i64, String)| {
            edit_todo_id.set(Some(id));
            edit_todo_text.set(text);
        })
    };

    let handle_edit_input_change = {
        let edit_todo_text = edit_todo_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_todo_text.set(input.value());
        })
    };

    let handle_save_edit = {
        let edit_todo_id = edit_todo_id.clone();
        let edit_todo_text = edit_todo_text.clone();
        let fetch_todos = fetch_todos.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            if let Some(todo_id) = *edit_todo_id {
                let new_text = (*edit_todo_text).clone().trim().to_string();

                if !new_text.is_empty() {
                    let edit_todo_id = edit_todo_id.clone();
                    let fetch_todos = fetch_todos.clone();
                    let loading = loading.clone();

                    let update_data = UpdateTodoType {
                        task: Some(new_text),
                        status: None,
                    };

                    loading.set(true);
                    spawn_local(async move {
                        match Request::put(&format!("http://127.0.0.1:8000/todo/{}", todo_id))
                            .header("Content-Type", "application/json")
                            .json(&update_data)
                        {
                            Ok(req) => {
                                if let Ok(response) = req.send().await {
                                    if response.status() == 200 {
                                        edit_todo_id.set(None);
                                        fetch_todos.emit(());
                                    } else {
                                        loading.set(false);
                                    }
                                } else {
                                    loading.set(false);
                                }
                            }
                            Err(_) => {
                                loading.set(false);
                            }
                        }
                    });
                }
            }
        })
    };

    let handle_cancel_edit = {
        let edit_todo_id = edit_todo_id.clone();
        Callback::from(move |_| {
            edit_todo_id.set(None);
        })
    };

    html! {
        <div class="min-h-screen py-8 bg-gray-900">
            <div class="max-w-4xl px-4 mx-auto">
                <div class="mb-8 text-center">
                    <h1 class="mb-2 text-4xl font-bold text-white">
                        {"Todo Application"}
                    </h1>
                </div>

                <div class="p-6 mb-6 bg-gray-800 rounded-lg shadow-lg">
                    <form onsubmit={handle_add_todo} class="flex gap-4">
                        <input
                            type="text"
                            placeholder="What needs to be done?"
                            value={(*input_value).clone()}
                            oninput={handle_input_change}
                            disabled={*loading}
                            class="flex-1 px-4 py-3 text-white placeholder-gray-400 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                        <button
                            type="submit"
                            disabled={*loading || input_value.trim().is_empty()}
                            class="px-6 py-3 font-semibold text-white bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            if *loading {
                                {"Adding..."}
                            } else {
                                {"Add Todo"}
                            }
                        </button>
                    </form>
                </div>

                if todos.is_empty() && !*loading {
                    <div class="py-12 text-center">
                        <div class="mb-4 text-4xl">{"🎯"}</div>
                        <h3 class="mb-2 text-xl font-semibold text-white">{"No todos yet!"}</h3>
                        <p class="text-gray-400">{"Add your first todo above to get started"}</p>
                    </div>
                } else {
                    <div class="space-y-3">
                        { for todos.iter().map(|todo| {
                            let is_editing = *edit_todo_id == Some(todo.id);
                            let is_completed = todo.status == Status::DONE;

                            let delete_click = {
                                let handle_delete = handle_delete_todo.clone();
                                let todo_id = todo.id;
                                Callback::from(move |_| handle_delete.emit(todo_id))
                            };

                            let toggle_click = {
                                let handle_toggle = handle_toggle_status.clone();
                                let todo_id = todo.id;
                                Callback::from(move |_| handle_toggle.emit(todo_id))
                            };

                            let edit_click = {
                                let handle_edit = handle_edit_click.clone();
                                let todo_id = todo.id;
                                let task = todo.task.clone();
                                Callback::from(move |_| handle_edit.emit((todo_id, task.clone())))
                            };

                            html! {
                                <div class={format!("bg-gray-800 rounded-lg p-4 shadow-md transition-all duration-200 hover:bg-gray-750 {}", if is_completed { "opacity-75" } else { "" })}>
                                    <div class="flex items-center gap-3">
                                        <button
                                            onclick={toggle_click}
                                            disabled={*loading}
                                            class={format!("w-5 h-5 rounded-full border-2 flex items-center justify-center text-xs {}",
                                                if is_completed {
                                                    "bg-green-500 border-green-500 text-white"
                                                } else {
                                                    "border-gray-400 hover:border-green-400"
                                                }
                                            )}
                                        >
                                            if is_completed {
                                                {"✓"}
                                            }
                                        </button>

                                        <div class="flex-1">
                                            if is_editing {
                                                <div class="flex gap-2">
                                                    <input
                                                        type="text"
                                                        value={(*edit_todo_text).clone()}
                                                        oninput={handle_edit_input_change.clone()}
                                                        class="flex-1 px-3 py-2 text-white bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                    />
                                                    <button
                                                        onclick={handle_save_edit.clone()}
                                                        class="px-3 py-2 text-sm text-white bg-green-600 rounded hover:bg-green-700"
                                                    >
                                                        {"Save"}
                                                    </button>
                                                    <button
                                                        onclick={handle_cancel_edit.clone()}
                                                        class="px-3 py-2 text-sm text-white bg-gray-600 rounded hover:bg-gray-700"
                                                    >
                                                        {"Cancel"}
                                                    </button>
                                                </div>
                                            } else {
                                                <div class="flex items-center justify-between">
                                                    <span class={format!("text-white {}", if is_completed { "line-through opacity-75" } else { "" })}>
                                                        {&todo.task}
                                                    </span>
                                                    <div class="flex items-center gap-2">
                                                        <span class={format!("px-2 py-1 rounded text-xs {}",
                                                            if is_completed {
                                                                "bg-green-900 text-green-300"
                                                            } else {
                                                                "bg-yellow-900 text-yellow-300"
                                                            }
                                                        )}>
                                                            {if is_completed { "Completed" } else { "Pending" }}
                                                        </span>
                                                    </div>
                                                </div>
                                            }
                                        </div>

                                        if !is_editing {
                                            <div class="flex gap-1">
                                                <button
                                                    onclick={edit_click}
                                                    disabled={*loading}
                                                    class="p-2 text-blue-400 rounded hover:text-blue-300 hover:bg-blue-900 disabled:opacity-50"
                                                    title="Edit todo"
                                                >
                                                    {"✏️"}
                                                </button>
                                                <button
                                                    onclick={delete_click}
                                                    disabled={*loading}
                                                    class="p-2 text-red-400 rounded hover:text-red-300 hover:bg-red-900 disabled:opacity-50"
                                                    title="Delete todo"
                                                >
                                                    {"🗑️"}
                                                </button>
                                            </div>
                                        }
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                }

                if !todos.is_empty() {
                    <div class="p-6 mt-6 bg-gray-800 rounded-lg shadow-md">
                        <div class="grid grid-cols-3 gap-4 text-center">
                            <div class="p-4 bg-gray-700 rounded-lg">
                                <div class="text-2xl font-bold text-blue-400">{todos.len()}</div>
                                <div class="text-sm text-gray-400">{"Total"}</div>
                            </div>
                            <div class="p-4 bg-gray-700 rounded-lg">
                                <div class="text-2xl font-bold text-green-400">{todos.iter().filter(|t| t.status == Status::DONE).count()}</div>
                                <div class="text-sm text-gray-400">{"Completed"}</div>
                            </div>
                            <div class="p-4 bg-gray-700 rounded-lg">
                                <div class="text-2xl font-bold text-yellow-400">{todos.iter().filter(|t| t.status == Status::PENDING).count()}</div>
                                <div class="text-sm text-gray-400">{"Pending"}</div>
                            </div>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}

// let added_todo = false;
// useEffect =>
// useEffect(() => {
// console.log()
// fetch the todos
// },[added_todo, task_checange])

// use_effect()
// use_effect_with((dskdsa))
