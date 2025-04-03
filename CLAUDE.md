# ALGO Project

## Project Overview
ALGO is a project focused on developing an advanced trading algorithm for Solana DEXs. The algorithm, called AHMO-NG (Adaptive Hierarchical Multi-Optimizer - Next Generation), aims to determine the best achievable execution strategy for swapping tokens across the Solana DEX ecosystem while balancing theoretical optimality with practical execution constraints.

## Documentation Structure
- **AHMO.md**: Main specification for the core algorithm logic (version 2.0.0)
- **mathematical_optimization.md**: Details mathematical approaches for solving DEX routing problems
- **problem.md**: Describes the market landscape and vision for a permissionless community-owned DEX aggregator
- **trading_algorithms.md**: Technical overview of Solana DEX protocols and their specific algorithms

## Project Goals
- Create an ultra-low latency execution system (sub-millisecond to low-millisecond targets)
- Optimize trade routing across multiple DEX venues and protocols
- Balance theoretical optimality with practical execution constraints
- Ensure all execution occurs within a SINGLE Solana transaction for atomicity
- Provide quality indicators reflecting solution confidence

## Key Components (see AHMO.md sections 3.1-3.10)
- **DEX State Cache**: Low-latency data store for the latest validated DEX market states
- **DEX Protocol Interface**: Well-defined modules for each supported DEX protocol
- **Path State Tracker**: Tracks simulated input/output per hop during allocation
- **Enhanced Priority Queue**: Stores potential allocation steps prioritized by multiple criteria
- **Transaction Constraint Tracker**: Continuously validates against Solana transaction limits
- **Optimal Subproblem Solver**: Dedicated modules for solving specific mathematical optimization problems
- **Local Search Engine**: Performs targeted local search to escape local optima
- **Quality Assessor**: Evaluates confidence in the solution's proximity to theoretical optimum

## Algorithm Execution Flow (see AHMO.md section 4)
1. **Level 0**: Continuous State Management (background process)
2. **Level 1**: Candidate Path Scouting & Subproblem ID (<1ms)
3. **Level 2**: Hybrid Optimization with Constraint Awareness (<9ms)
4. **Level 3**: Execution & Finalization (microseconds + network latency)
5. **Level 4**: Post-Execution Analysis (background process)

## Supported DEX Protocols (see trading_algorithms.md)
- **Constant Product AMMs**: Raydium, Orca standard pools, GooseFX
- **Order Books**: OpenBook-v2, Phoenix
- **Concentrated Liquidity AMMs**: Orca Whirlpools, Invariant, Cykura, Raydium CLMM 
- **StableSwap variants**: Saber
- **Dynamic/Oracle-Priced AMMs**: Meteora, Lifinity
- **Batch Auctions**: Phoenix
- **Hybrid models**: Aldrin

## Mathematical Approaches (see mathematical_optimization.md)
- **Convex Optimization**: For optimal splits across constant product AMMs
- **Modified Graph Algorithms**: Extensions for non-linear edge weights
- **Approximation Algorithms**: ε-approximation schemes with provable bounds
- **Advanced Data Structures**: Specialized priority queues for route exploration

## Solana Constraints (see AHMO.md section 8.3)
- Single transaction requirement for atomicity
- Compute budget limitations (default: 200,000 units, max: ~1,400,000 units)
- Transaction size constraints (limited to 1232 bytes)
- Maximum 4-level CPI depth constraint
- Account reference limit (64 accounts without lookup tables)

## Common Commands
- TBD: Will be populated with build, test, and run commands as they're established

## For Detailed Information
- For algorithm logic details: See AHMO.md
- For mathematical foundations: See mathematical_optimization.md
- For DEX protocol-specific information: See trading_algorithms.md
- For market vision and problem description: See problem.md