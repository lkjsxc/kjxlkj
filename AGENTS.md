I’m thinking of deleting all source code and rebuilding it from scratch. Keep rewriting the documentation until you can swear to God that this is the best possible state. I allow bold and major changes.

The documentation must be structured to the absolute limit.

When I had you implement this last time, I instructed you to write down the points for improvement. Please use that as a reference. After that, completely delete the log directory.

From the TODO list, make it so everything is directly linked to the relevant documentation files. In other words, I want the project to end up fully compliant with the entire documentation if someone proceeds with implementation by following the TODOs and jumping to the linked docs as they work.

In the documentation, make sure the final file structure at completion is fully described.

For the web app on small screens, the menu button should be in the top-right, and selecting a note should close the menu.

In data/config.json, make it possible to configure everything except secrets.

Secrets should be in .env.

Please add a test that verifies whether a note is correctly added when the “Create New Note” button is pressed.

You can spend as much time as you want investigating—please investigate everything completely.

Finally, delete the source code and also reset things like the TODO list checkboxes back to their initial state.
