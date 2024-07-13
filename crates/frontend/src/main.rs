use std::str::FromStr;

use gloo_net::http::Request;
use hirola::dom::app::router::Router;
use hirola::dom::app::App;
use hirola::dom::effects::prelude::*;
use hirola::dom::Dom;
use hirola::prelude::{Suspend, *};
use home::{queue_card, resolve_json};
use log::Level;
use shared::{Filter, GetJobsResult, JobState, Worker};
use strum::IntoEnumIterator;
use web_sys::EventSource;
mod home;

#[derive(Debug, Clone)]
pub struct State {
    event_source: EventSource,
    namespaces: MutableVec<String>,
}

impl State {
    async fn list_namespaces() -> Result<Vec<String>, gloo_net::Error> {
        let res = Request::get(&format!("{API_PATH}")).send().await?;
        res.json().await
    }

    async fn list_jobs(
        namespace: String,
        filter: Filter,
    ) -> Result<GetJobsResult<serde_json::Value>, gloo_net::Error> {
        let res = Request::get(&format!("{API_PATH}/{namespace}"))
            .query([
                ("page", filter.page.to_string()),
                ("status", filter.status.to_string()),
            ])
            .send()
            .await?;
        res.json().await
    }

    async fn list_workers(namespace: String) -> Result<Vec<Worker>, gloo_net::Error> {
        let res = Request::get(&format!("{API_PATH}/{namespace}/workers"))
            .send()
            .await?;
        res.json().await
    }
}

const API_PATH: &str = "/api/v1/backend";

fn namespace_page(app: &App<State>) -> Dom {
    html! {
        <>
            <NamespaceContent router=app.router().clone() />
        </>
    }
}

fn namespace_status_page(app: &App<State>) -> Dom {
    html! {
        <>
            <NamespaceStatusContent router=app.router().clone() />
        </>
    }
}

fn queues_page(app: &App<State>) -> Dom {
    let namespaces = &app.state().namespaces;
    html! {
        <section class="w-full px-4 pt-2">
            <h2 class="text-xl font-bold">"Queues"</h2>
            <p class="text-gray-500">"These are all the queues you have created on this apalis-board instance."</p>
            <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-3">
            {namespaces
                .signal_vec_cloned()
                .map_render(|ns| {
                    html! {
                        <a href=format!("/queue/{ns}")>
                            {queue_card(&ns, "0 jobs")}
                        </a>
                    }
                })
            }

            </div>
        </section>
    }
}

#[component]
fn SidebarInput() -> Dom {
    html! {
        <div class="mt-8">
            <input
                class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 mb-4"
                placeholder="Filter queues"
                type="text"
            />
        </div>
    }
}

impl EffectAttribute for Link {
    type Handler = XEffect;
    fn read_as_attr(&self) -> String {
        "link".to_owned()
    }
}

struct Link;

#[component]
fn SidebarItem(
    href: String,
    icon: &'static str,
    label: String,
    router: &'static Router<State>,
) -> Dom {
    html! {
        <a class="flex items-center p-2 space-x-2 text-white bg-[#253545] rounded-md" href=href x:link=router.link()>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-6 w-6"
            >
                <path d={icon}></path>
            </svg>
            <span>{label}</span>
        </a>
    }
}

#[component]
fn NamespaceContent(router: Router<State>) -> Dom {
    let namespace = router.current_params().get("namespace").unwrap().clone();
    html! {
        <div class="flex flex-col lg:flex-row w-full">
            <div class="flex-1 px-4">
                <QueueNav router={router} />
                <div class="space-y-1">
                {match State::list_workers(namespace).suspend().await {
                    Loading => html! { <div>"Loading..."</div> },
                    Ready(Ok(workers)) => {
                        html! {
                            <ul>
                                {for worker in workers {
                                    html! {
                                        <>
                                            <Card title={worker.worker_id.to_string()} status="●" />
                                        </>
                                    }
                                }}
                            </ul>
                        }
                    },
                    Ready(Err(err)) => html! { <div>"An error occurred: " {err.to_string()}</div> }
                } }
                </div>
            </div>
        </div>
    }
}

#[component]
fn QueueNav(router: Router<State>) -> Dom {
    let params = router.current_params();
    let namespace = params.get("namespace").unwrap().clone();
    // let status = params.get("status").cloned();

    html! {
        <div class="flex flex-col items-left mb-4 pt-2">
        <h2 class="text-xl font-bold">{format!("Queue: {namespace}")}</h2>
            <p class="text-gray-500">"Active · 606 jobs · 500 failed · 60 pending · 5 dead"</p>
                        <div class="flex space-x-4 mt-2">


                                <div class=" items-center px-1 text-sm font-medium text-center text-white border-r pr-4 border-gray-800">
                                <a x:link={router.link()} href={format!("/queue/{namespace}")} class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium h-10 py-2">
                                            "Workers"
                                        </a>
                                <span class="inline-flex items-center justify-center w-4 h-4 ms-2 text-xs font-semibold text-blue-800 bg-blue-200 rounded-full">
                                "2"
                                </span>
                                </div>



                                                        {for status in JobState::iter() {
                                html! {
                                    <>
                                        <NavItem router={router.clone()} label={status.to_string()} />
                                    </>
                                }
                            }}
                        </div>
                    </div>
    }
}

#[component]
fn NamespaceStatusContent(router: Router<State>) -> Dom {
    let namespace = router.current_params().get("namespace").unwrap().clone();
    let status = router
        .current_params()
        .get("status")
        .map(|s| JobState::from_str(s).unwrap());
    html! {
            <div class="flex flex-col lg:flex-row w-full">
                <div class="flex-1 px-4">
                    <QueueNav router={router} />
                    <div class="space-y-1">
                    {match State::list_jobs(namespace, Filter { page: 1, status:status.unwrap()}).suspend().await {
                        Loading => html! { <div>"Loading..."</div> },
                        Ready(Ok(res)) => {
                            html! {
                                <>
                                <ul class="flex flex-col space-y-4">
                                    {for task in res.jobs{
                                        html! {
                                            <li>
                                            <div class="rounded-sm border text-card-foreground focus:outline-none font-mono text-sm shadow-sm p-2 m-1"><div class="p-6 flex items-center space-x-4">
                                                {resolve_json(&task)}
                                                </div></div>
                                            </li>
                                        }
                                    }}
                                </ul>


    <div class="flex pb-4">
    <a href="#" class="flex items-center justify-center px-3 h-8 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-sm hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
      "Previous"
    </a>


    <a href="#" class="flex items-center justify-center px-3 h-8 ms-3 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-sm hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
      "Next"
    </a>
    </div>

                                </>
                            }
                        },
                        Ready(Err(err)) => html! { <div>"An error occurred: " {err.to_string()}</div> }
                    } }
                    </div>
                </div>
            </div>
        }
}

#[component]
fn NavItem<L: AsRef<str>>(label: L, router: Router<State>) -> Dom {
    let label = label.as_ref();
    let current_params = router.current_params();
    let namespace = current_params.get("namespace").unwrap();
    html! {
        <a x:link=router.link() href=format!("/queue/{namespace}/{label}") class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
            {label}
        </a>
    }
}

#[component]
fn Card<T: AsRef<str>, S: AsRef<str>>(title: T, status: S) -> Dom {
    html! {
        <div class="rounded-sm border text-card-foreground shadow-sm">
            <div class="p-6 flex items-center space-x-4">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-6 w-6 text-blue-600"
                >
                    <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>
                </svg>
                <div>
                    <h5 class="text-lg font-semibold">{title.as_ref()}</h5>
                    <div class="text-sm text-gray-500">
                        "Last seen less than a minute ago"
                    </div>
                </div>
                <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                    {status.as_ref()}
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    let es = EventSource::new(&format!("{API_PATH}/events")).unwrap();
    let api = State {
        event_source: es,
        namespaces: Default::default(),
    };
    dbg!(&api.event_source);
    let mut app = App::new(api);
    app.route("/", home::page);
    app.route("/queue/:namespace", namespace_page);
    app.route("/queue/:namespace/:status", namespace_status_page);
    app.route("/queues", queues_page);

    let parent_node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();

    app.mount_with(&parent_node, |app| {
        use crate::home::Header;
        let app = Box::leak(Box::new(app.clone()));
        let ns_poller = async {
            let values = State::list_namespaces().await.unwrap();
            app.state().namespaces.lock_mut().replace_cloned(values);
        };
        let wrapper = html! {
            <main use:future=ns_poller class="flex w-full">
                //<--!App goes here -->
            </main>
        };
        let inner = app.router().render(app, &wrapper);
        html! {
            <div class="flex w-full">
            <div class="container mx-auto w-full">
                <Header app={app} />
                {inner}
            </div>
            </div>
        }
    });
}
