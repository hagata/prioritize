# prioritize


todo,
- `push n` take a task from todays `ls` and push it to tomorrow
    - creates a Day for tomorrow if there isn't one yet
- [x] `carryover -1 ` [defaults to -1] pull incomplete items from yesterday (or last input list) to todays list
    - should also handle creating today if today doesn't exist yet

- move the ui loop in the None match arm into it's on crate mod
- move the actions and their function definitIon into their own mod 

- store the tasks json file somewhere generic on the system
    - allow users to define where the todo/task json file lives ... like dropbox synced dir for example
