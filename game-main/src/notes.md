# Notes on zymarku Bevy Basics (YouTube)

* Entity: ID for set of components
* Resource: global to world
* Bundle: set of components
  - can be nested
  
* Bevy has built-in support for GLTF assets
  - GLTF is a popular, but not de facto standard, 3D model file format
  - GLB is the binary form of GLTF
  - Each GLTF can define multiple scenes; load `foo.gltf#Scene0`
* A 3D model includes:
  - Transform -- position/rotation/scale of entity
  - Mesh -- vertex and index data for rendering the model
  - Material -- how model interacts with light; how it looks like when rendered
  - Other components for animation, skinning, etc.

Physics engines:
* [Rapier](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/)
* [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd)
