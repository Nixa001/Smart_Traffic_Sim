# 🚦 Smart_Traffic_Sim

## Overview
**Smart_Traffic_Sim** is a cutting-edge simulation project focused on developing a new traffic control strategy for autonomous vehicles (AVs) at intersections, eliminating the need for traditional traffic lights. This project aims to minimize traffic congestion and collisions through smart intersection management, leveraging the capabilities of AVs to create a safer and more efficient traffic flow.

## Features
- 🧠 **Smart Intersection Management:** Advanced algorithms to manage vehicle flow at intersections without traffic lights.
- 🚗 **Autonomous Vehicle Physics:** Realistic physics simulation for AVs, including velocity, distance, and time calculations.
- 🎮 **Dynamic Vehicle Generation:** Keyboard commands to generate vehicles with various routes, ensuring diverse traffic patterns.
- 🎞️ **Animation:** Smooth and realistic animation of AVs as they navigate through the intersection.
- 📊 **Statistics Tracking:** Comprehensive statistics to analyze intersection performance, including max/min velocity, time, and close calls.

## Goals
- 🚥 **Reduce Traffic Congestion:** Optimize vehicle flow to prevent traffic jams.
- 🔒 **Enhance Safety:** Minimize collisions and ensure safe distances between vehicles.
- 🚀 **Prepare for AV Future:** Develop traffic strategies tailored for the upcoming era of autonomous vehicles.

## Instructions
1. **Intersection Setup:** Focus on a cross intersection with distinct lanes for right turns, straight ahead, and left turns.
2. **Vehicle Control:** Implement physics for AVs, ensuring they follow the designated routes and maintain safe distances.
3. **Simulation Commands:** Use keyboard events to generate vehicles and control the simulation, with specific keys for different directions.
4. **Animation Requirements:** Animate vehicles realistically as they move and turn within the intersection.
5. **Statistics Window:** Display detailed statistics upon exiting the simulation.

## Getting Started

### Prerequisites
To run this project, you need to have Rust installed. Follow the instructions below to install Rust.

### Installing Rust
1. Open a terminal.
2. Run the following command to install Rust using `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3. Follow the on-screen instructions to complete the installation.
4. After installation, ensure the Rust toolchain is up to date:
```bash
rustup update
```
## Getting Started
Clone the repository and follow the instructions in the README to set up the simulation environment and start experimenting with smart intersection management for AVs.

```bash
git clone https://github.com/nixa001/Smart_Traffic_Sim.git
cd Smart_Traffic_Sim
cargo run
```
### Running the Simulation
To run the simulation, use the following commands:

#### Generate Vehicles:

- ↑ Arrow Up: Generate vehicles from south to north.
- ↓ Arrow Down: Generate vehicles from north to south.
- → Arrow Right: Generate vehicles from west to east.
- ← Arrow Left: Generate vehicles from east to west.
- R: Continually generate random vehicles (using the game loop).
- Esc: Finish the simulation and generate a window with all statistics.
- Exit Simulation:
