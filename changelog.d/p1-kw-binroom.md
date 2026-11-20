Changed
The Linux binary is about 1.5 MB smaller: it stores the addresses the system adjusts at start in a packed form. It still loads at a random address, and nothing it does changes. Building Khalid from source on Linux now needs glibc 2.36 or newer; the release downloads run everywhere they did.
