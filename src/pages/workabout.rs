use leptos::prelude::*;
/// Renders the "What I do for money" page.
#[component]
pub fn WorkAbout() -> impl IntoView {
    let (show_resume, set_show_resume) = signal(false);

    view! {
        <div class="text-zinc-900 dark:text-zinc-100 min-h-screen bg-zinc-100 dark:bg-zinc-900">
            <div class="mx-auto max-w-6xl w-full px-4">
                <div class="flex flex-col md:flex-row gap-8 md:gap-12 py-8">
                    // Left column — professional summary
                    <div class="flex-1 min-w-0 md:max-w-md">
                        <h1 class="text-3xl mb-4">
                            "What I do for money"
                        </h1>
                        <p class="text-xl font-light text-zinc-900 dark:text-zinc-100">
                            "I'm a computer scientist looking for career opportunities to enable my continued education and exploration of the many lovely avenues of knowledge and inquiry."
                        </p>
                    </div>

                    // Right column — the rest of the details
                    <div class="flex-1 min-w-0 md:max-w-xl">
                        // Education highlights
                        <div class="text-zinc-900 dark:text-zinc-100">
                            <h2 class="text-sm font-semibold uppercase tracking-wider text-zinc-500 dark:text-zinc-400 mb-3">
                                "Education"
                            </h2>
                            <ul class="space-y-4 text-xl font-light">
                                <li>
                                    <div class="text-base text-zinc-500 dark:text-zinc-400">
                                        "B.S. in Computer Science"
                                    </div>
                                    <div>
                                        "Rutgers University | "
                                        "2023 - 2026"
                                    </div>
                                    <p class="text-base mt-1">
                                        "Relevant coursework in systems programming, networking, and Artificial Intelligence."
                                    </p>
                                </li>
                                <li>
                                    <div class="text-base text-zinc-500 dark:text-zinc-400">
                                        "A.S. in Computer Science"
                                    </div>
                                    <div>
                                        "Brookdale Community College | "
                                        "2021 - 2023"
                                    </div>
                                    <p class="text-base mt-1">
                                        "Established a basis in systems programming, algorithms and programming languages."
                                    </p>
                                </li>
                            </ul>
                        </div>

                        // Connect — professional social links
                        <div class="text-zinc-900 dark:text-zinc-100 mt-10">
                            <h2 class="text-sm font-semibold uppercase tracking-wider text-zinc-500 dark:text-zinc-400 mb-3">
                                "Connect"
                            </h2>
                            <ul class="space-y-2 text-xl font-light">
                                <li>
                                    <a
                                        class="text-teal-600 dark:text-teal-200 inline-block hover:scale-105 hover:underline"
                                        target="_blank"
                                        href="https://www.linkedin.com/in/iris-martinez"
                                    >
                                        "LinkedIn"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="text-teal-600 dark:text-teal-200 inline-block hover:scale-105 hover:underline"
                                        target="_blank"
                                        href="https://github.com/irismartinez"
                                    >
                                        "GitHub"
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                // Resume — full-width section at the bottom
                <div class="text-zinc-900 dark:text-zinc-100 pb-12">
                    <h2 class="text-sm uppercase tracking-wider text-zinc-500 dark:text-zinc-400 mb-3">
                        "Resume"
                    </h2>
                    <button
                        type="button"
                        aria-expanded=move || show_resume.get().to_string()
                        on:click=move |_| set_show_resume.update(|v| *v = !*v)
                        class="inline-flex items-center gap-2 rounded bg-teal-600 text-white px-4 py-2 text-base font-medium origin-center transition-transform hover:scale-105 dark:bg-teal-500 dark:text-zinc-900"
                    >
                        <span>{move || if show_resume.get() { "▾" } else { "▸" }}</span>
                        {move || if show_resume.get() { "Hide resume" } else { "View resume" }}
                    </button>
                    {move || show_resume.get().then(|| view! {
                        <iframe
                            class="w-full h-[85vh] mt-4 rounded border border-zinc-300 dark:border-zinc-700 bg-white dark:invert dark:hue-rotate-180"
                            title="Iris Martinez — Résumé"
                            src="/resume.pdf#toolbar=0&navpanes=0"
                        />
                    })}
                </div>
            </div>
        </div>
    }
}
