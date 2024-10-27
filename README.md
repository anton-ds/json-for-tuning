A project in RUST for educational purposes. It prepares a JSONL file for uploading to OpenAI for Vision fine-tuning.

In main.rs, you need to specify:

- `DIRECTORY`, where the data is stored (n.txt for the image description and n.png for the image itself, where n is any number).
- `OUTPUT_FILE` - path to the final file.
- `PICTURE_URL_TEMPLATE` - URL of the file stored on an external server, where {name} is the n from the filename.
- `SYSTEM_MESSAGE` and `USER_MESSAGE` - general system and user instructions for a single training item.