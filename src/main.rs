use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <main class="w-4/5 md:w-2/5 mx-auto">
                <h1 class="pt-14 text-5xl font-bold">"Aryan Beezadhur"</h1>

                <p class="my-8">"I am a self-taught programmer with experience using a wide range of technologies."</p>

                <p class="my-8">"My programming portfolio can be found on my GitHub below."</p>

                <p class="my-8">"This website was written in Rust."</p>

                <p class="my-8">
                    <a
                        href="mailto:aryanbeezadhur@gmail.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="bg-gradient-to-r from-neutral-500 to-neutral-500 hover:text-neutral-500 duration-150 bg-[length:100%_0.125rem] bg-[0_100%] bg-no-repeat"
                    >
                        "Email"
                    </a>

                    <span class="text-neutral-500">", "</span>

                    <a
                        href="https://github.com/cobrexus"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="bg-gradient-to-r from-neutral-500 to-neutral-500 hover:text-neutral-500 duration-150 bg-[length:100%_0.125rem] bg-[0_100%] bg-no-repeat"
                    >
                        "GitHub"
                    </a>

                    <span class="text-neutral-500">", "</span>

                    <a
                        href="https://www.linkedin.com/in/aryanbeezadhur/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="bg-gradient-to-r from-neutral-500 to-neutral-500 hover:text-neutral-500 duration-150 bg-[length:100%_0.125rem] bg-[0_100%] bg-no-repeat"
                    >
                        "LinkedIn"
                    </a>

                    <span class="text-neutral-500">", "</span>

                    <a
                        href="https://stackoverflow.com/users/12860895"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="bg-gradient-to-r from-neutral-500 to-neutral-500 hover:text-neutral-500 duration-150 bg-[length:100%_0.125rem] bg-[0_100%] bg-no-repeat"
                    >
                        "Stack Overflow"
                    </a>
                </p>
            </main>
        }
    })
}
