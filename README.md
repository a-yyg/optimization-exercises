# Optimization Exercises
Contains examples of optimization problems and their solutions.
- pso: Particle Swarm Optimization

Uses [nix](https://nixos.org/) to run them.
To run the pso example with <n> particles and <iter> iterations, for instance, run:
```bash
nix run "github:a-yyg/optimization-exercises?dir=pso" -- <n> <iter>
```
You can also use pinned commits:
```bash
$ nix run "github:a-yyg/optimization-exercises/8029765dc27ace06d40cc0d149786a6ff4e5ac7d?dir=pso" -- 3 10
Particle Swarm Optimization Demo
Function to optimize: y = (x - 1)^2
Initialized 3 particles:
Best value of x: 1.0197495589114842
Best value of y: 0.00039004507719818583
```
