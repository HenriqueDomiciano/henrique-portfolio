use std::num::ParseIntError;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlInputElement;
use leptos::{ev, prelude::*};


#[derive(Clone)]
struct ByteInfo {
    offset: usize,
    hex: String,
    decimal: String,
    binary: String,
    ascii: char,
}
fn convert_ascii(input: &str) -> Vec<ByteInfo> {
    input
        .bytes()
        .enumerate()
        .map(|(offset, byte)| ByteInfo {
            offset,
            hex: format!("{:02X}", byte),
            decimal: byte.to_string(),
            binary: format!("{:08b}", byte),
            ascii: if byte.is_ascii_graphic() || byte == b' ' {
                byte as char
            } else {
                '.'
            },
        })
        .collect()
}

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

    let bytes = move || convert_ascii(&ascii.get());

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
                        class="converter-input"
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
                                    .map(|byte| {
                                        view! {
                                            <tr>
                                                <td class="byte-offset">
                                                    {format!("0x{:02X}", byte.offset)}
                                                </td>

                                                <td class="byte-hex">
                                                    {byte.hex}
                                                </td>

                                                <td class="byte-decimal">
                                                    {byte.decimal}
                                                </td>

                                                <td class="byte-binary">
                                                    {byte.binary}
                                                </td>

                                                <td class="byte-ascii">
                                                    {byte.ascii.to_string()}
                                                </td>
                                            </tr>
                                        }
                                        .into_any()
                                    })
                                    .collect::<Vec<_>>()
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

#[derive(Clone)]
pub enum ShowModes
{
    Decimal,
    Binary,
    Hexadecimal
}


fn parse_input(value:String, mode:ShowModes) -> Result<u64,ParseIntError>
{
    match mode {
        ShowModes::Decimal => u64::from_str_radix(&value, 10) ,
        ShowModes::Binary =>  u64::from_str_radix(&value, 2) ,
        ShowModes::Hexadecimal => u64::from_str_radix(&value, 16) ,
    }
}


#[component]
fn BitResult(value:Signal<u64>) -> impl IntoView
{
    view! { 
        <div class="packet-display">
            <div>
                {move || format!("{:b}", value.get())}
            </div>

            <div>
                {move || format!("{}", value.get())}
            </div>

            <div>
                {move || format!("{:0X}", value.get())}
            </div>
        </div>
    }    
}


#[component]
pub fn BitManipulator() -> impl IntoView{
    let (bit_value_1, set_bit_value_1) = signal(0u64);
    let (bit_value_2, set_bit_value_2) = signal(0u64); 
    let (option_read, option_write) = signal(ShowModes::Decimal);
    let and_result = Signal::derive(move || {
        bit_value_1.get() & bit_value_2.get()
    });
    
    let or_result = Signal::derive(move || {
        bit_value_1.get() | bit_value_2.get()
    });
    
    let xor_result = Signal::derive(move || {
        bit_value_1.get() ^ bit_value_2.get()
    });
    
    let nand_result = Signal::derive(move || {
        !(bit_value_1.get() & bit_value_2.get())
    });
    
    let nor_result = Signal::derive(move || {
        !(bit_value_1.get() | bit_value_2.get())
    });
view! {
    <section class="uart-converter">

        <div class="section-number">
            "BIT MANIPULATION"
        </div>

        <h2>
            "Bitwise Calculator"
        </h2>

        <p class="uart-description">
            "Perform 64-bit bitwise operations and inspect the results in multiple formats."
        </p>


        <div>

            <div>
                <div class="converter-panel-title">
                    "INPUT REGISTERS"
                </div>
            </div>

            <div class="lab-body">

                <div class="lab-grid">

                    <div class="converter-binary-panel-title">

                        <label for="bit-value-1">
                            "VALUE A"
                        </label>

                        <input
                            id="bit-value-1"
                            class="converter-binary-input"
                            type="text"
                            placeholder="Enter value..."
                            on:input=move |ev| {
                                match parse_input(event_target_value(&ev),option_read.get())
                                {
                                    Ok(a) => set_bit_value_1.set(a),
                                    Err(_) => println!("ERROR PARSING"),
                                };
                            }
                        />

                    </div>


                    <div class="converter-binary-panel-title">

                        <label for="bit-value-2">
                            "VALUE B"
                        </label>

                        <input
                            id="bit-value-2"
                            class="converter-binary-input"
                            type="text"
                            placeholder="Enter value..."
                            on:input=move |ev| {
                                match parse_input(event_target_value(&ev),option_read.get())
                                {
                                    Ok(a) => set_bit_value_2.set(a),
                                    Err(_) => {},
                                };
                            }
                        />

                    </div>

                </div>


                <div
                    class="converter-panel-title"
                    style="margin-top: 20px ;text-align: center ;"
                >

                    <label for="display-mode">
                        "INPUT FORMAT"
                    </label>

                    <select
                        id="display-mode"
                        class="converter-binary-input"
                        style="text-align: center "
                        on:change=move |event| {

                            let value = event_target_value(&event);

                            let current_display_option =
                                match value.as_str() {

                                    "decimal" =>
                                        ShowModes::Decimal,

                                    "binary" =>
                                        ShowModes::Binary,

                                    "hexadecimal" =>
                                        ShowModes::Hexadecimal,

                                    _ =>
                                        ShowModes::Decimal,
                                };

                            option_write.set(current_display_option);
                        }
                    >

                        <option value="decimal">
                            "DECIMAL"
                        </option>

                        <option value="binary">
                            "BINARY"
                        </option>

                        <option value="hexadecimal">
                            "HEXADECIMAL"
                        </option>

                    </select>

                </div>

            </div>

        </div>

        <div
            class=""
            style="margin-top: 30px;"
        >

            <div class="">

                <h4>
                    "BITWISE OPERATIONS"
                </h4>

                <p>
                    "U64"
                </p>

            </div>


            <div class="converter-panel">

                <table class="byte-table">

                    <thead>
                        <tr>
                            <th>"OPERATION"</th>
                            <th>"RESULT"</th>
                        </tr>
                    </thead>

                    <tbody>

                        <tr>
                            <td class="byte-hex">
                                "AND"
                            </td>

                            <td>
                                <BitResult
                                    value=and_result
                                />
                            </td>
                        </tr>


                        <tr>
                            <td class="byte-hex">
                                "OR"
                            </td>

                            <td>
                                <BitResult
                                    value=or_result
                                />
                            </td>
                        </tr>


                        <tr>
                            <td class="byte-hex">
                                "XOR"
                            </td>

                            <td>
                                <BitResult
                                    value=xor_result
                                />
                            </td>
                        </tr>


                        <tr>
                            <td class="byte-hex">
                                "NAND"
                            </td>

                            <td>
                                <BitResult
                                    value=nand_result
                                />
                            </td>
                        </tr>


                        <tr>
                            <td class="byte-hex">
                                "NOR"
                            </td>

                            <td>
                                <BitResult
                                    value=nor_result
                                />
                            </td>
                        </tr>

                    </tbody>

                </table>

            </div>

        </div>

    </section>
}


}



