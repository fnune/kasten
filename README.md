# Kasten

A journaling tool with support for linking entries, which can be used to create a Zettelkasten.

## Features

- [x] Store notes in a Sqlite3 database with the following columns:
  - [x] `id`: integer
  - [x] `title`: text
  - [x] `content`: text, to be parsed as Markdown
  - [x] `created_at`: a date-time
  - [ ] `updated_at`: a date-time
- [x] Enter the notes using your `$EDITOR`.
- [ ] References notes inside the content of a note using Markdown inline-style link syntax, where the URL is the ID of the referenced note. For example: `[comment](000001)`.
- [ ] Use autocompletion to reference notes in LSP-compatible editors.
  - Autocompletion triggers when the user has the cursor between the two square brackets: `[▁]`.
  - In the autocompletion list, the user can choose using the title of the target note. It autocompletes to `[title](id)` of the referenced note.
  - The user can change the title section to their own comment and the reference will still be valid. That means once we store the note, the only part that matters is the `id` of the referenced note.
- [ ] Query for notes filtering by their `created_at`, `updated_at`, `title` or `content` values.
- [ ] Query for notes by reference. To do this, instead of parsing all the notes for each query, we keep a `references` table in the database:
  - `id`: integer
  - `source_note_id`: integer
  - `target_note_id`: integer
  - `created_at`: a date-time
- [ ] Edit or delete notes:
  - This will remove all references whose `source_note_id` is this note's `id` from the database and create new ones from the edited text after saving.
  - It will also update the `updated_at` field on the note.
  - Saving an empty note will delete it, together with its references.
  - It is not possible to delete a note if other notes reference it.
- [ ] Export as Markdown using `stdout`, in a way that can be combined with the filters for notes mentioned above.
