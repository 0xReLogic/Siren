# RFC-001: Threat Model

- **Status**: Draft
- **Author**: Allen Elzayn, Cascade
- **Date**: 2025-08-04

## 1. Summary

This document outlines the threat model for the Siren project. It identifies potential security threats, attack vectors, and the mitigation strategies Siren will employ to detect phishing websites in real-time. The goal is to define the scope of protection and guide the development of the core detection engine.

## 2. Threat Actors & Motives

- **Threat Actors**: Financially motivated cybercriminals, organized groups, and script kiddies.
- **Motives**: Theft of sensitive information (credentials, financial data, personal identity), malware distribution, and social engineering.

## 3. Attack Vectors

Siren will primarily focus on threats delivered through web browsers. The main vectors are:

- **Malicious URLs**: 
  - Typosquatting (e.g., `go0gle.com`)
  - Subdomain abuse (e.g., `google.com.secure-login.net`)
  - URL shorteners obscuring the final destination.
  - Homograph attacks (using non-ASCII characters).

- **Deceptive Page Content (HTML)**:
  - Cloning legitimate websites (login forms, branding).
  - Using images instead of text to bypass text-based analysis.
  - Obfuscated JavaScript to hide malicious behavior.
  - Favicon spoofing.

- **Social Engineering Triggers**:
  - Urgency and fear (e.g., "Your account is suspended!").
  - Fake offers and prizes.

## 4. Proposed Security Controls (Siren's Mitigation)

- **URL Analysis**: A machine learning model will analyze URL features (length, special characters, keyword presence, domain age) to identify suspicious patterns.
- **HTML Content Analysis**: The core transformer model will analyze the DOM structure, text, and forms to classify the page's intent. It will look for known phishing kit patterns and inconsistencies.
- **Threat Intelligence Feeds**: Integrate with URLhaus and PhishTank to block known malicious URLs preemptively.
- **On-device Inference**: Perform classification locally using WASM + ONNX to ensure low latency (<100ms) and preserve user privacy.

## 5. Edge Cases & Evasion Techniques

Siren must be robust against adversaries who will actively try to evade detection. Key challenges include:

- **Cloaking**: Serving different content to the Siren detector than to the user.
- **Legitimate Services Abuse**: Using trusted services like Google Docs or Microsoft Forms to host phishing content.
- **Time-bombed Pages**: Pages that only activate their malicious payload after a certain time or user interaction.
- **Human-in-the-loop Phishing**: Phishing pages that require a live operator to interact with the victim.

## 6. Out of Scope (For MVP)

- **Malware Detection**: Siren is not an antivirus. It will not scan file downloads.
- **Email-level Phishing Detection**: Siren operates at the browser level, not within an email client.
- **Advanced Evasion (e.g., Vision-based)**: The initial model will focus on URL and HTML. Screenshot analysis is a stretch goal (Phase 8).
- **Zero-Day Vulnerability Protection**: Siren does not protect against browser or OS exploits.
