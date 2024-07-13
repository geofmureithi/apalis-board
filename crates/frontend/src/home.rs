use hirola::dom::XEffect;
use hirola::{
    dom::{app::App, Dom},
    prelude::*,
};
use serde::Serialize;

use crate::Link;

use crate::State;

pub fn page(app: &App<State>) -> Dom {
    html! {
        <>
            <Main app={app.clone()} />
        </>
    }
}

pub fn resolve_json<V: Serialize>(val: V) -> String {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    \n");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    val.serialize(&mut ser).unwrap();
    String::from_utf8(buf).unwrap()
}

#[component]
fn Main(app: App<State>) -> Dom {
    html! {
        <div class="p-4 space-y-8 w-full">
            <section>
                <h2 class="text-xl font-bold">"Stats"</h2>
                <p class="text-gray-500">"Your system instance stats"</p>
                <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-4">
                    {stats_card("Max memory", "0G", None, None)}
                    {stats_card("Used memory", "0M", Some("0.01%"), Some("text-green-500"))}
                    {stats_card("Connections", "0", None, None)}
                    {stats_card("Queues", "0", None, None)}
                </div>
            </section>
            <section>
                <h2 class="text-xl font-bold">"Overview"</h2>
                <p class="text-gray-500">"An overview of execution stats"</p>
                <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-4">
                    {overview_card("Total jobs in queues", "0")}
                    {overview_card("Jobs per minute", "0")}
                    {overview_card("Jobs past hour", "0")}
                    {overview_card("Failed jobs past 7 days", "0")}
                </div>
            </section>
            <section>
                <h2 class="text-xl font-bold">"Queues"</h2>
                <p class="text-gray-500">"These are all the queues you have created on your apalis instance."</p>
                <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-3">
                    {app.state().namespaces
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
        </div>
    }
}

fn stats_card(
    title: &str,
    value: &str,
    extra_info: Option<&str>,
    extra_class: Option<&str>,
) -> Dom {
    html! {
        <div class="rounded-sm border text-card-foreground shadow-sm bg-gray-900" data-v0-t="card">
            <div class="flex-col space-y-1.5 p-6 flex justify-between">
                <h3 class="whitespace-nowrap text-2xl font-semibold leading-none tracking-tight">{title}</h3>
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
                    class="w-4 h-4 text-gray-500"
                >
                    <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
                </svg>
            </div>
            <div class="p-6">
                <div class="text-2xl font-bold">{value}</div>
                {if let Some(info) = extra_info {
                    html! { <p class=extra_class.unwrap_or("text-gray-500")>{info}</p> }
                } else {
                    html! { <></> }
                }}
            </div>
        </div>
    }
}

fn overview_card(title: &str, value: &str) -> Dom {
    html! {
        <div class="rounded-sm border text-card-foreground shadow-sm bg-gray-900" data-v0-t="card">
            <div class="flex flex-col space-y-1.5 p-6">
                <h3 class="whitespace-nowrap text-2xl font-semibold leading-none tracking-tight">{title}</h3>
            </div>
            <div class="p-6">
                <div class="text-2xl font-bold">{value}</div>
            </div>
        </div>
    }
}

pub fn queue_card(title: &str, info: &str) -> Dom {
    html! {
        <div class="rounded-sm border text-card-foreground shadow-sm bg-gray-900" data-v0-t="card">
            <div class="flex-col space-y-1.5 p-6 flex justify-between">
                <h3 class="whitespace-nowrap text-2xl font-semibold leading-none tracking-tight">{title}</h3>
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
                    class="w-4 h-4 text-gray-500"
                >
                    <path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"></path>
                    <circle cx="12" cy="13" r="3"></circle>
                </svg>
            </div>
            <div class="p-6">
                <div class="text-sm">{info}</div>
                <div class="mt-2">
                    // <div class="w-full h-8">
                    //     <div
                    //         class="flex aspect-video justify-center text-xs [&amp;_.recharts-cartesian-axis-tick_text]:fill-muted-foreground [&amp;_.recharts-cartesian-grid_line]:stroke-border/50 [&amp;_.recharts-curve.recharts-tooltip-cursor]:stroke-border [&amp;_.recharts-dot[stroke='#fff']]:stroke-transparent [&amp;_.recharts-layer]:outline-none [&amp;_.recharts-polar-grid_[stroke='#ccc']]:stroke-border [&amp;_.recharts-radial-bar-background-sector]:fill-muted [&amp;_.recharts-rectangle.recharts-tooltip-cursor]:fill-muted [&amp;_.recharts-reference-line-line]:stroke-border [&amp;_.recharts-sector[stroke='#fff']]:stroke-transparent [&amp;_.recharts-sector]:outline-none [&amp;_.recharts-surface]:outline-none min-h-[300px]"
                    //         data-chart="chart-r0"
                    //     >
                    //         // Add your recharts or other chart library code here
                    //     </div>
                    // </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Logo() -> Dom {
    html! {
       <img
            src=r#"data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%20512%20512%22%20fill%3D%22white%22%3E%3Cpolygon%20points%3D%22141.598%2C307.12%200%2C448.707%2042.972%2C448.707%20174.577%2C317.114%22%3E%3C%2Fpolygon%3E%3Cpath%20d%3D%22M511.324%2C156.078c-1.335-3.15-4.427-5.197-7.848-5.197H459.55c-4.709%2C0-8.524%2C3.816-8.524%2C8.524%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20l12.519%2C41.258c1.655%2C1.602%2C3.793%2C2.399%2C5.927%2C2.399c2.229%2C0%2C4.454-0.868%2C6.126-2.596l34.006-35.133%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20C511.981%2C162.873%2C512.659%2C159.229%2C511.324%2C156.078z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M321.452%2C365.844c-91.686%2C0-129.88-64.005-128.392-110.162%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c-0.011-0.011%2C192.355-192.389%2C192.355-192.389c37.778%2C20.889%2C67.236%2C55.007%2C82.09%2C96.115c4.069%2C11.229%2C7.035%2C22.98%2C8.785%2C35.13%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c1.227%2C8.456%2C1.864%2C17.093%2C1.864%2C25.878c0%2C2.75-0.057%2C5.501-0.193%2C8.217C477.961%2C228.633%2C425.246%2C365.844%2C321.452%2C365.844z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M409.805%2C228.633h68.157c-4.285%2C95.285-82.897%2C171.216-179.24%2C171.216%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c-56.542%2C0-106.969-26.163-139.848-67.032c-6.478-8.024-12.252-16.616-17.275-25.697l51.45-51.45%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c14.775%2C44.21%2C56.508%2C76.078%2C105.673%2C76.078C357.457%2C331.749%2C405.577%2C286.288%2C409.805%2C228.633z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M393.325%2C197.174c-20.824%2C0-37.766-16.942-37.766-37.766c0-20.831%2C16.942-37.778%2C37.766-37.778%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c20.831%2C0%2C37.778%2C16.947%2C37.778%2C37.778C431.103%2C180.232%2C414.156%2C197.174%2C393.325%2C197.174z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M393.325%2C144.36c8.308%2C0%2C15.047%2C6.74%2C15.047%2C15.047s-6.74%2C15.036-15.047%2C15.036%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20s-15.036-6.728-15.036-15.036S385.017%2C144.36%2C393.325%2C144.36z%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E"#
        />
    }
}

#[component]
pub fn Header(app: &'static App<State>) -> Dom {
    html! {
        <header class="flex items-center justify-between p-4 border-b border-gray-800">
            <a class="flex items-center space-x-2" x:link={app.router().link()} href="/">
                <div class="flex items-center justify-center w-12 h-12">
                    <Logo />
                </div>
                <h1 class="text-xl font-bold">"Apalis Board"<span class="text-xs font-small">" · 0.3.0"</span></h1>
            </a>
            <nav class="flex space-x-4">
                <a x:link={app.router().link()} class="text-white" href="/" rel="ugc">"Overview"</a>
                <a x:link={app.router().link()} class="text-gray-500" href="/queues" rel="ugc">"Queues"</a>
                <a x:link={app.router().link()} class="text-gray-500" href="/settings" rel="ugc">"Settings"</a>
                <a class="text-gray-500" href="https://github.com/geofmureithi/apalis-board" rel="ugc">"Github"</a>
            </nav>
        </header>
    }
}
