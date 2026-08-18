use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlInputElement;
use leptos::{ev, prelude::*};

#[derive(Clone)]
struct HashValues {
    sha1: String,
    sha256: String,
    sha512: String,
}

impl HashValues {
    pub fn new() -> Self {
        Self {
            sha1: String::new(),
            sha256: String::new(),
            sha512: String::new(),
        }
    }

    pub async fn calculate_sha(file: web_sys::File) -> Result<Self, String> {
        let mut result_vec = Vec::new();
        let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
            .await
            .map_err(|_| "Failed to read file buffer".to_string())?;

        let window = web_sys::window().ok_or("No global window found")?;
        let crypto = window.crypto().map_err(|_| "Crypto not supported")?;
        let subtle = crypto.subtle();
        let array_buffer_object: &js_sys::Object = array_buffer.unchecked_ref();

        for algo in ["SHA-1", "SHA-256", "SHA-512"] {
            let sha_256_hash_promise = subtle
                .digest_with_str_and_buffer_source(algo, array_buffer_object)
                .map_err(|_| "Digest initiation failed")?;

            let hash_buffer = wasm_bindgen_futures::JsFuture::from(sha_256_hash_promise)
                .await
                .map_err(|_| "Hashing failed")?;

            let uint8_array = js_sys::Uint8Array::new(&hash_buffer);
            let mut hex_string = String::new();
            for byte in uint8_array.to_vec() {
                hex_string.push_str(&format!("{:02x}", byte));
            }
            result_vec.push(hex_string.clone());
        }
        println!("Calculated SHA!!!!"); 
        return Ok(Self {
            sha1: result_vec.get(0).unwrap().to_owned(),
            sha256: result_vec.get(1).unwrap().to_owned(),
            sha512: result_vec.get(2).unwrap().to_owned(),
        });
    }
}

#[component]
pub fn UartAsciiConverter() -> impl IntoView {
    let (ascii, set_ascii) = signal(String::new());

    let bytes = move || ascii.get().bytes().collect::<Vec<u8>>();
    view! {
            <section class="uart-tool">

                <div class="section-number">
                    "UART / ASCII"
                </div>

                <h2>
                    "ASCII Converter"
                </h2>

                <div class="converter">

                    <div class="converter-panel">
                    <div class="converter-panel-title">
                        "INPUT"
                    </div>
                        <textarea
                            class = "converter-input"
                            placeholder="Enter ASCII text..."
                            on:input=move |event| {
                                set_ascii.set(event_target_value(&event));
                            }
                        />
                    </div>

    <div class="byte-inspector">

        <table class="byte-table">

            <thead>
                <tr>
                    <th>"OFFSET"</th>
                    <th>"HEX"</th>
                    <th>"DEC"</th>
                    <th>"BIN"</th>
                    <th>"ASCII"</th>
                </tr>
            </thead>

            <tbody>
                {move || {
                    bytes()
                        .into_iter()
                        .enumerate()
                        .map(|(offset, byte)| {
                            let ascii_char = if byte.is_ascii_graphic() || byte == b' ' {
                                byte as char
                            } else {
                                '.'
                            };

                            view! {
                                <tr>
                                    <td class="byte-offset">
                                        {format!("0x{:02X}", offset)}
                                    </td>

                                    <td class="byte-hex">
                                        {format!("{:02X}", byte)}
                                    </td>

                                    <td class="byte-decimal">
                                        {byte.to_string()}
                                    </td>
                                    <td class="byte-hex">
                                        {format!("{:08b}",byte)}
                                    </td>
                                    <td class="byte-ascii">
                                        {ascii_char.to_string()}
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()
                }}
            </tbody>

        </table>

    </div>

                </div>

            </section>
        }
}

#[component]
pub fn FileHasher() -> impl IntoView {
    let (sha_hash, set_sha_hash) = signal(HashValues::new());

    let hash_action = Action::new_local(move |file: &web_sys::File| {
        let file = file.clone();
        async move { HashValues::calculate_sha(file).await }
    });

Effect::new(move |_| {
    if let Some(result) = hash_action.value().get() {
        match result {
            Ok(hash) => {
                set_sha_hash.set(hash);
            }

            Err(error) => {
                web_sys::console::error_1(
                    &error.into()
                );
            }
        }
    }
});

    let on_change = move |ev: ev::Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                hash_action.dispatch(file);
            }
        }
    };

    view! {
        <div>
            <div class="section-number">
                    "File Hasher"
                </div>

                <h2>
                    "File Hasher"
                </h2>

            <input
            id="file-upload"
            class="file-input"
            type="file"
            on:change=on_change
            />
                <label for="file-upload" class="btn btn-primary">
                "SELECT FILE"
                </label>

            <div>
                {move || if hash_action.pending().get() {
                    view! { <p><em>"Calculating hash..."</em></p> }.into_any()
                } else if !sha_hash.get().sha1.is_empty() {
                    let sha_value = sha_hash.get(); 
                    view! {
                        <h4>
                            <strong>"SHA-1 Hash: "</strong>
                            <code>
                                {sha_value.sha1}
                            </code>
                        </h4>
                        <h4>
                            <strong>"SHA-256 Hash: "</strong>
                            <code>
                                {sha_value.sha256}
                            </code>
                        </h4>
                        <h4>
                            <strong>"SHA-512 Hash: "</strong>
                            <code>
                                {sha_value.sha512}
                            </code>
                        </h4>
                    }.into_any()
                } else {
                    view! { <p >"Select a file to see its hash."</p> }.into_any()
                }}
            </div>
        </div>
    }
}
