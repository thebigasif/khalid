/// What `cargo build` says when `admin/dist/` is empty, one line per
/// warning: what happened, why it matters, and what to do. `build.rs`
/// prints it and `lib.rs` tests it, so both read this one file.
const BUILD_WARNING: [&str; 3] = [
    "This Khalid has no admin, because admin/dist/ was empty when it was built.",
    "The blog and its APIs work, but /admin/ only says how to add the admin.",
    "To add it, run npm ci and npm run build in the admin folder, then build Khalid again.",
];
