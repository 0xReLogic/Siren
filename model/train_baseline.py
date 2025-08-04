import pandas as pd
import torch
from torch.utils.data import Dataset, DataLoader
from transformers import DistilBertTokenizer, DistilBertForSequenceClassification, AdamW

# 1. Configuration
MODEL_NAME = 'distilbert-base-uncased'
DATA_PATH = 'model/data/dummy_data.csv'
NUM_EPOCHS = 3
BATCH_SIZE = 2
LEARNING_RATE = 5e-5

# 2. Custom Dataset Class
class PhishingDataset(Dataset):
    def __init__(self, texts, labels, tokenizer, max_len=128):
        self.texts = texts
        self.labels = labels
        self.tokenizer = tokenizer
        self.max_len = max_len

    def __len__(self):
        return len(self.texts)

    def __getitem__(self, item):
        text = str(self.texts[item])
        label = self.labels[item]

        encoding = self.tokenizer.encode_plus(
            text,
            add_special_tokens=True,
            max_length=self.max_len,
            return_token_type_ids=False,
            padding='max_length',
            return_attention_mask=True,
            return_tensors='pt',
            truncation=True
        )

        return {
            'text': text,
            'input_ids': encoding['input_ids'].flatten(),
            'attention_mask': encoding['attention_mask'].flatten(),
            'labels': torch.tensor(label, dtype=torch.long)
        }

# 3. Main Training Function
def train_baseline_model():
    """Main function to load data, initialize model, and run training."""
    print("--- Starting Baseline Model Training ---")

    # Load tokenizer and model
    tokenizer = DistilBertTokenizer.from_pretrained(MODEL_NAME)
    model = DistilBertForSequenceClassification.from_pretrained(MODEL_NAME, num_labels=2)

    # Load data
    df = pd.read_csv(DATA_PATH)
    print(f"Loaded {len(df)} records from {DATA_PATH}")

    # Create dataset and dataloader
    dataset = PhishingDataset(
        texts=df.text.to_numpy(),
        labels=df.label.to_numpy(),
        tokenizer=tokenizer
    )
    dataloader = DataLoader(dataset, batch_size=BATCH_SIZE)

    # Setup optimizer
    optimizer = AdamW(model.parameters(), lr=LEARNING_RATE)
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    model.to(device)

    print(f"Using device: {device}")
    print("--- Setup complete. Starting training loop (placeholder) ---")

    # 4. Training Loop (Placeholder)
    # In a real scenario, this loop would be fully implemented.
    model.train()
    for epoch in range(NUM_EPOCHS):
        print(f"\nEpoch {epoch + 1}/{NUM_EPOCHS}")
        for batch in dataloader:
            # Move batch to device
            input_ids = batch["input_ids"].to(device)
            attention_mask = batch["attention_mask"].to(device)
            labels = batch["labels"].to(device)

            # Forward pass
            outputs = model(
                input_ids=input_ids,
                attention_mask=attention_mask,
                labels=labels
            )
            loss = outputs.loss

            # Backward pass (placeholder - not running optimizer yet)
            # loss.backward()
            # optimizer.step()
            # optimizer.zero_grad()

            print(f"  - Batch processed. Loss: {loss.item():.4f}")

    print("\n--- Baseline model training script finished (placeholder) ---")

if __name__ == '__main__':
    train_baseline_model()
