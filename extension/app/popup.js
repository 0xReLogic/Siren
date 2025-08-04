// Import the 'detect' function from our WASM package.
import { detect } from '../siren_engine/pkg/siren_engine.js';

document.addEventListener('DOMContentLoaded', () => {
  console.log('Siren Debug: popup.js script loaded and DOM is ready.');

  const checkBtn = document.getElementById('check-button');
  const urlInput = document.getElementById('url-input');
  const resultDiv = document.getElementById('result');

  if (!checkBtn || !urlInput || !resultDiv) {
    console.error('Siren Debug: A required UI element is missing from popup.html.');
    return;
  }

  checkBtn.addEventListener('click', async () => {
    console.log('Siren Debug: Check URL button clicked.');
    const url = urlInput.value;
    if (!url) {
      resultDiv.textContent = 'Please enter a URL.';
      return;
    }

    try {
      resultDiv.textContent = 'Analyzing...';
      console.log(`Siren Debug: Calling WASM 'detect' function with URL: ${url}`);
      const prediction = await detect(url);
      console.log(`Siren Debug: WASM function returned: '${prediction}'`);
      resultDiv.textContent = `Result: ${prediction}`;
      resultDiv.className = prediction === 'Phishing' ? 'phishing' : 'safe';
    } catch (error) {
      const errorMessage = `Siren Debug: Error caught during detection: ${error}`;
      resultDiv.textContent = `Error: ${error}`;
      console.error(errorMessage, error);
    }
  });
});


