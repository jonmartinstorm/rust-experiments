# A combined TCP and UNIX listener

* [x] Create a working unix listener
* [x] Create a working tcp listener
* [x] Combine the unix and tcp listener so the work at the same time
* [ ] Send values too and from the listeners.
    * [ ] Change a value in the server?
    * [ ] Run a function on the server?
    * [ ] Using channels!
    * [ ] Use a manager task?
* [ ] Implement websockets on the tcp side
    * [ ] Find a crate (tokio websockets or something)
        * tungstenite.rs
        * tokio-tungstenite = "0.13.0"
    * [ ] Get it working
* [ ] Restructure and put into modules where it makes sense
    * [ ] Refactor
    * [ ] Modules