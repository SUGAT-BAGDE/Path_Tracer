# InsploRay (Path Tracer)

InsploRay is a CPU based path tracing renderer. It is currently in an early stage of development. 

_InsploRay: Inspire(inspiration) + Explore(Exploration) + Ray Tracing_

The primary goal of writing a path tracer was to get a head start before getting into low level systems programming, which now grew into a project of its own. It’s being designed with modularity in mind.

## 🖥️ Demo

![image](https://private-user-images.githubusercontent.com/79888221/479675877-1c3e619e-d447-460c-8f36-a5c4c889cb09.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NTU2MzEwMTAsIm5iZiI6MTc1NTYzMDcxMCwicGF0aCI6Ii83OTg4ODIyMS80Nzk2NzU4NzctMWMzZTYxOWUtZDQ0Ny00NjBjLThmMzYtYTVjNGM4ODljYjA5LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTA4MTklMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwODE5VDE5MTE1MFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTllZGIxNGM1YTc0MzgxMGM2ZjEyNDdmMzM5YTY1ZmQ4ODVlNTE1ZDhjZDdmOTRlMTliNmExOWZmYTIwODY5MGMmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.ODAF9zOcaYGIeOgWQUmry1TS6KN97DgbPejEvyiLCz4)

## 🧩 Current Features:
- Ray sphere intersection _(Only)_
- Lambertian Diffuse _(Only)_
- EXR skybox support _(for HDR environment lighting and background)_
- Multithreaded
- Simulate a PinHole Camera
- Very Basic material system 
    - Albedo
    - Roughness _(planned, not yet implemented)_
    - Metalic _(planned, not yet implemented)_
    - Emissive Color
    - Emissive Strength
- Basic Tone Mapping
- More under way✨...

🖼️ Frontend (Experimental not main focus of the project)
Though viewport and interactivity is not the primary goal. However, the frontend currently supports the following features with caveats talked later:

- Interactive viewport (`WASDQE` for movement, `right-click + mouse` for look-around)
- Adjustable pinhole camera parameters
- Simple scene editor (currently supports spheres and materials only)

⚠️ _Limitations and caveats apply — see below._

## ⚙️ Installation and Usage

There is currently no packaged binary for Insploray. For reasons read more! But you can experience the frontend I talked about previously by following steps:

#### Step 1: Clone the repository
```bash
  git clone https://github.com/libsugat/InsploRay.git
  cd InsploRay
```
#### Step 2: Compile/Build and run the Code
```bash
  cargo run # for debug build
```
or
```bash
  cargo run --release # for release/optimized build
```

## 🧰 Project Setup and Development
The structure is a `Cargo Workspace`. Currently containing two main components/crates.
- `InsploRay` (core renderer) with folder name `core-engine`
- `the interactive frontend` which is a basic window and imgui UI with folder name `frontend`
- _(Planned)_ FFI-safe interface for integration
- _(Planned)_ A headless Cli for offline rendering

**🧱 Modularity and Architecture**

InsploRay is being built with **real modularity** in mind — not just internal code separation, but composable and swappable components that can be replaced or extended.

> ✅ **Currently modular**:  
> The camera system — define your own camera models by implementing a `Camera` trait and plugging them in.

> 🎯 **Goal**:  
> Make all core components — integrators, samplers, materials, light sources, scene loaders — modular via Rust traits and FFI-safe boundaries.

InsploRay is designed as a **library-first** project, with the intent to:
- Be embedded in other Rust projects as a crate
- Expose safe FFI for C/C++ or other languages
- Serve as a backend for Blender plugins, wasm, native android, or other platforms via FFI or bindings.

## 🧭 How to Contribute

- **Open an issue first** if you want to work on something — bug, feature, or idea. (Just make sure there are no duplicate issues.)
- Then, make the changes and **open a pull request** (PR).
- Keep PRs small and focused if possible — it makes things easier to understand.    
- Code formatting using `clippy` is appreciated.

>⚠️ **Few small constraints**:  
> Please keep performance and readability in mind.  
> Avoid excessive type casting — e.g., use `usize` only for indexing; otherwise prefer types like `u32`, etc.
> Maintain clean build (no build warnings)

That’s it for now! No strict rules — I’m here to learn too, and happy to figure things out with you as we go. 💬

Feel free to ask questions, suggest changes, or just explore the code!

## 🐛 Known issue
- [ ] The drag for material selector does not behave well with small values (Frontend)
- [ ] Imgui does not remember the window layout, it's a know issue with imgui-rs crate (Frontend)

## 🔜 My Side Plans
Order unknown because I am gonna join BTech soon within couple of days....
- [ ] Ray Triangle Intersection
- [ ] Specular BRDF
- [ ] Metallic BRDF
- [ ] Better Scene Representation in memory
- [ ] Loading Scene (`.glb`/`.gltf`/`.obj`)
- [ ] Save Image (`EXR` and/or `PNG`)
- [ ] MIS (Multiple Importance Sampling) in Primary (or currently only) Integrator

## License

Licensed under [AGPLv3](./LICENSE).  
For closed-source, commercial, SaaS, or academic use without attribution, please contact via Github Issues

## 👤 Author
This project was started by me ([@libsugat](https://www.github.com/libsugat))
— who knew **nothing** about rendering or graphics programming when it began!


