### Resources

- Accessing google sheets...
    - Rust google-sheets crate: <https://crates.io/crates/google-sheets4>
    - Google Sheets API documentation: <https://developers.google.com/sheets/api/quickstart/quickstarts-overview>
    - Creating Google Service Account: <https://www.analyticsvidhya.com/blog/2020/07/read-and-update-google-spreadsheets-with-python/>

- Facetted searching...
    - <https://eikes.github.io/facetedsearch/>

- Rocket examples...
    - <https://github.com/SergioBenitez/Rocket/tree/master/examples>

---

### Next

- Read more of Rocket intro
    - At: <https://rocket.rs/v0.5-rc/guide/overview/#async-routes>

- Figure out logging

- Figure out where static assets, ie favicon, go.

- Call a separate function -- which would be the trigger to process the spreadsheet.
    - The processing would be tracked elsewhere.

---

### Questions

- When to use, say, `use rocket::tokio::time::Instant;` vs `use std::time::Instant;`? The first version of [this code](https://github.com/birkin/keeper_code/blob/a1d2ff485b2c08baa86f70c29e381f84e23c1082/gsheet_processor/src/main.rs#L30) used a separate `use std::time::Instant;`, and all seemed to work fine.

    - This [tokio time-documentation](https://docs.rs/tokio/1.7.1/tokio/time/index.html#examples) shows duration once using `std::time`, and once using `tokio::time`. Why? I can imagine a thought being "synchronous code will generally be faster than asynchronous code (for a given function), so only use asynchronous code when needed". On the other hand, I've heard that over time, libraries will evolve to be able to be used synchronously or asynchronously. Hmm... my working thought-plan is to use tokio modules since they're stable and won't accidentally block. Plus, that may force me to always be thinking in async-await terms.

---
