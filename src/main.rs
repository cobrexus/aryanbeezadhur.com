use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <main>
                <h1>"Aryan Beezadhur"</h1>

                <p>"I am a self-taught programmer based in London."</p>

                <p>"My portfolio of programming projects is on my GitHub."</p>

                <p>"This website was written in Rust."</p>

                <p>
                    <a href="mailto:aryanbeezadhur@gmail.com" target="_blank" rel="noopener noreferrer">"Email"</a>

                    " • "

                    <a href="https://github.com/cobrexus" target="_blank" rel="noopener noreferrer">"GitHub"</a>

                    " • "

                    <a href="https://www.linkedin.com/in/aryanbeezadhur/" target="_blank" rel="noopener noreferrer">"LinkedIn"</a>

                    " • "

                    <a href="https://stackoverflow.com/users/12860895" target="_blank" rel="noopener noreferrer">"Stack Overflow"</a>
                </p>
            </main>
        }
    })
}
