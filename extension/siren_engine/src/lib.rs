use wasm_bindgen::prelude::*;
use tract_onnx::prelude::*;
use tokenizers::Tokenizer;
use std::str::FromStr;


// Embed the model and vocabulary file into the WASM binary at compile time.
const MODEL_BYTES: &[u8] = include_bytes!("../../../model/model_quantized.onnx");
const VOCAB_BYTES: &str = include_str!("../vocab.txt");

// Helper to log errors to the browser console for easier debugging.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Main function exposed to JavaScript. It takes a URL and returns a prediction string.
#[wasm_bindgen]
pub fn detect(url: &str) -> String {
    // We wrap the core logic in a separate function to handle potential errors gracefully.
    match run_inference(url) {
        Ok(result) => result,
        Err(e) => {
            // If anything goes wrong, log the detailed error and return a user-friendly message.
            log(&format!("Error during inference: {:?}", e));
            "Error processing URL".to_string()
        }
    }
}

// The internal function that contains the core inference logic.
fn run_inference(url: &str) -> Result<String, TractError> {
    // 1. Load the tokenizer from the pre-configured `tokenizer.json` file.
    let tokenizer_path = "../../../model/tokenizer.json";
    let mut tokenizer = Tokenizer::from_file(tokenizer_path)
        .map_err(|e| TractError::msg(format!("Failed to load tokenizer: {}", e)))?;

    let truncation = tokenizers::TruncationParams { max_length: 512, ..Default::default() };
    let padding = tokenizers::PaddingParams {
        strategy: tokenizers::PaddingStrategy::Fixed(512), // Pad all sequences to max_length
        ..Default::default()
    };
    let _ = tokenizer.with_truncation(Some(truncation));
    let _ = tokenizer.with_padding(Some(padding));

    // 3. Tokenize the input URL.
    let encoding = tokenizer.encode(url, true).map_err(|e| TractError::msg(format!("Encoding failed: {}", e)))?;
    let input_ids = encoding.get_ids();
    let attention_mask = encoding.get_attention_mask();

    // 4. Convert the tokenized output into Tract tensors.
    // The model expects i64 tensors with shape [batch_size, sequence_length].
    let input_ids_tensor: Tensor = tract_ndarray::Array2::from_shape_vec(
        (1, input_ids.len()),
        input_ids.iter().map(|&x| x as i64).collect(),
    )?.into();

    let attention_mask_tensor: Tensor = tract_ndarray::Array2::from_shape_vec(
        (1, attention_mask.len()),
        attention_mask.iter().map(|&x| x as i64).collect(),
    )?.into();

    // 5. Load the ONNX model and make it runnable.
    let model = tract_onnx::onnx()
        .model_for_read(&mut std::io::Cursor::new(MODEL_BYTES))?
        .into_optimized()?
        .into_runnable()?;

    // 6. Run inference by feeding the tensors to the model.
    let result = model.run(tvec!(input_ids_tensor.into(), attention_mask_tensor.into()))?;

    // 7. Process the output tensor to get the final prediction.
    // The output is logits; we find the index with the highest value (argmax).
    let output_tensor = result[0].to_array_view::<f32>()?;
    let prediction_index = output_tensor
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap_or(0); // Default to 0 if something goes wrong

    // 8. Map the prediction index to a human-readable label.
    let prediction = if prediction_index == 1 {
        "Phishing"
    } else {
        "Safe"
    };

    Ok(prediction.to_string())
}
