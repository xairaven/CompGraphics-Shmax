# Computer Graphics Labs

A collection of high-performance computer graphics laboratory works implemented in Rust. 
This project demonstrates various graphics algorithms, geometric transformations, and rendering techniques using an interactive GUI.

> **Note:** This project was developed as part of a mentorship program.
> 
> **Mentee:** @miamlet
> 
> **Mentor:** @xairaven

## 🚀 Features

* **Interactive Visualization:** Real-time rendering using `egui` and `eframe`.
* **Modular Architecture:** Powered by a custom `geometry` engine shared across all labs.
* **Comprehensive Math Library:**
    * **2D & 3D Primitives:** Points, Vectors, Lines, Circles.
    * **Transformations:** Affine, Euclidean (Rotation, Offset), and Projective.
    * **Curves & Fractals:** Ferguson curves, Zigzag fractals.
    * **Projections:** Two-point perspective and 3D-to-2D conversion.
* **Animations:** Support for dynamic figures like rotating cones, walking simulations, and morphing contours.

## 📂 Project Structure

The project is organized as a Rust Workspace:

* **`geometry/`**: The core library containing all mathematical logic, primitives, and rendering algorithms.
* **`Lab1` - `Lab7`**: Individual applications focusing on specific graphics topics.

| Module | Description |
| :--- | :--- |
| `geometry::transformations` | Implements Affine, Euclidean, and Projective matrices. |
| `geometry::figures` | Contains complex shapes like Epicycloids, 3D Grids, and Surfaces. |
| `geometry::smooth` | Algorithms for curve smoothing (e.g., Ferguson). |
| `geometry::projections` | Logic for projecting 3D objects onto a 2D viewport. |

## 🛠️ Getting Started

### Prerequisites

* **Rust & Cargo:** Ensure you have the latest stable version of Rust installed.

### Installation

1.  Clone the repository:
    ```bash
    git clone [https://github.com/xairaven/compgraphics-shmax.git](https://github.com/xairaven/compgraphics-shmax.git)
    cd compgraphics-shmax
    ```

2.  Build the workspace:
    ```bash
    cargo build --release
    ```

### Running the Labs

You can run any specific lab using the package name defined in the workspace.

### ⚙️ Configuration

Each lab checks for a configuration file (typically `config.toml` or similar, handled by `src/config.rs`) to load themes and initial settings. If a custom config is not provided, defaults are used.
