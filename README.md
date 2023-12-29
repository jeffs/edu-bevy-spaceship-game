# Controls

* hjkl to steer the ship
* wasd to pan the camera
* qe to zoom
* shift to do a barrel roll
* cmd+q to quit

Shoot asteroids to destroy them.  If the ship hits an asteroid, it despawns.
Then you're left to watch asteroids float through empty space, and to wonder
what it was all for.  You might as well quit (cmd+q) and start over.  Or go
outside and get some exercise, rather than sitting at your computer all day.

# Running

To run natively:
```sh
cargo run
```

To run in the browser:
```sh
cd game-main
cargo run --target=wasm32-unknown-unknown
```

# Links

* Bevy/WASM
  - <https://bevy-cheatbook.github.io/platforms/wasm.html>
  - <https://bevy-cheatbook.github.io/platforms/wasm/webpage.html>
* Hosting
  - <https://dashboard.render.com/>
* Assets
  - from Ultimate Space Kit by Quaternius on Poly Pizza:
    <https://poly.pizza/bundle/Ultimate-Space-Kit-YWh743lqGX>
