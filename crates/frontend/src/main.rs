use hirola::dom::effects::prelude::*;
use hirola::dom::Dom;
use hirola::prelude::*;

fn app() -> Dom {
    html! {
        <>
            <div class="flex flex-1">
                <div class="flex flex-col w-64 bg-[#1F2A37] p-4" data-id="2">
                    <div class="flex items-center space-x-2 text-white" data-id="3">
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
                            class="h-8 w-8"
                            data-id="4"
                        >
                            <path d="M16 7h.01"></path>
                            <path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20"></path>
                            <path d="m20 7 2 .5-2 .5"></path>
                            <path d="M10 18v3"></path>
                            <path d="M14 17.75V21"></path>
                            <path d="M7 18a6 6 0 0 0 3.84-10.61"></path>
                        </svg>
                        <span class="font-bold text-xl" data-id="5">
                            "Apalis"
                        </span>
                    </div>
                    <div class="mt-8" data-id="6">
                        <input
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 mb-4"
                            placeholder="Filter queues"
                            data-id="7"
                            type="text"
                        />
                        <div class="flex flex-col space-y-1" data-id="8">
                            <a
                                class="flex items-center p-2 space-x-2 text-white bg-[#253545] rounded-md"
                                data-id="9"
                                href="#"
                            >
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
                                    data-id="10"
                                >
                                    <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
                                    <path d="M3 5V19A9 3 0 0 0 21 19V5"></path>
                                    <path d="M3 12A9 3 0 0 0 21 12"></path>
                                </svg>
                                <span data-id="11">"sqlite::Notification"</span>
                            </a>
                            <a
                                class="flex items-center p-2 space-x-2 text-white"
                                data-id="12"
                                href="#"
                            >
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
                                    data-id="13"
                                >
                                    <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
                                    <path d="M3 5V19A9 3 0 0 0 21 19V5"></path>
                                    <path d="M3 12A9 3 0 0 0 21 12"></path>
                                </svg>
                                <span data-id="14">"apalis::Email"</span>
                            </a>
                            <a
                                class="flex items-center p-2 space-x-2 text-white"
                                data-id="15"
                                href="#"
                            >
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
                                    data-id="16"
                                >
                                    <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
                                    <path d="M3 5V19A9 3 0 0 0 21 19V5"></path>
                                    <path d="M3 12A9 3 0 0 0 21 12"></path>
                                </svg>
                                <span data-id="17">"postgres::Document"</span>
                            </a>
                        </div>
                    </div>
                    <div
                        class="mt-auto flex items-center justify-between text-xs text-gray-400"
                        data-id="18"
                    >
                        <span data-id="19">"0.3.0"</span>
                        <span data-id="20">"© 2023"</span>
                    </div>
                </div>

                <div class="flex h-screen bg-gray-100 flex-col lg:flex-row w-full">
                    <button
                        class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 w-10 lg:hidden"
                        type="button"
                        aria-haspopup="dialog"
                        aria-expanded="false"
                        aria-controls="radix-:r3:"
                        data-state="closed"
                    >
                        <svg
                            //www.w3.org/2000/svg"
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
                            <line x1="4" x2="20" y1="12" y2="12"></line>
                            <line x1="4" x2="20" y1="6" y2="6"></line>
                            <line x1="4" x2="20" y1="18" y2="18"></line>
                        </svg>
                        <span class="sr-only">"Toggle navigation menu"</span>
                    </button>
                    <div class="flex-1 p-8">
                        <div class="flex items-center justify-between mb-4">
                            <div class="flex space-x-4">
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Home"
                                </button>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Running"
                                </button>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Pending"
                                </button>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Done"
                                </button>
                                <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80">
                                    "1000"
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Failed"
                                </button>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Scheduled"
                                </button>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    "Killed"
                                </button>
                            </div>
                            <svg
                                //www.w3.org/2000/svg"
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="h-6 w-6 text-gray-400"
                            >
                                <circle cx="12" cy="12" r="1"></circle>
                                <circle cx="12" cy="5" r="1"></circle>
                                <circle cx="12" cy="19" r="1"></circle>
                            </svg>
                        </div>
                        <div class="space-y-4">
                            <div
                                class="rounded-lg border text-card-foreground shadow-sm bg-white"
                                data-v0-t="card"
                            >
                                <div class="p-6 flex items-center space-x-4">
                                    <svg
                                        //www.w3.org/2000/svg"
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
                                        <h5 class="text-lg font-semibold">"Worker #21544644"</h5>
                                        <div class="text-sm text-gray-500">
                                            "Last seen less than a minute ago"
                                        </div>
                                    </div>
                                    <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                                        "●"
                                    </div>
                                </div>
                            </div>
                            <div
                                class="rounded-lg border text-card-foreground shadow-sm bg-white"
                                data-v0-t="card"
                            >
                                <div class="p-6 flex items-center space-x-4">
                                    <svg
                                        //www.w3.org/2000/svg"
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
                                        <h5 class="text-lg font-semibold">"Worker #26b7f335"</h5>
                                        <div class="text-sm text-gray-500">
                                            "Last seen less than a minute ago"
                                        </div>
                                    </div>
                                    <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                                        "●"
                                    </div>
                                </div>
                            </div>
                            <div
                                class="rounded-lg border text-card-foreground shadow-sm bg-white"
                                data-v0-t="card"
                            >
                                <div class="p-6 flex items-center space-x-4">
                                    <svg
                                        //www.w3.org/2000/svg"
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
                                        <h5 class="text-lg font-semibold">"Worker #84a4f4dbb"</h5>
                                        <div class="text-sm text-gray-500">
                                            "Last seen less than a minute ago"
                                        </div>
                                    </div>
                                    <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                                        "●"
                                    </div>
                                </div>
                            </div>
                            <div
                                class="rounded-lg border text-card-foreground shadow-sm bg-white"
                                data-v0-t="card"
                            >
                                <div class="p-6 flex items-center space-x-4">
                                    <svg
                                        //www.w3.org/2000/svg"
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
                                        <h5 class="text-lg font-semibold">"Worker #ba8efb36"</h5>
                                        <div class="text-sm text-gray-500">
                                            "Last seen less than a minute ago"
                                        </div>
                                    </div>
                                    <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                                        "●"
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    hirola::dom::mount(app()).unwrap();
}
