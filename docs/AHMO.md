**Version:** 2.0.0 (Novel Hybrid Design)

**Date:** April 5, 2025

**Status:** Final Implementation Logic Specification

## 1. Introduction

### 1.1. Goal

This document specifies the core logic for the **AHMO-NG (Adaptive Hierarchical Multi-Optimizer - Next Generation)**. The objective is to determine and facilitate the **best achievable execution strategy** for swapping a given quantity Q of input token X for the maximum possible quantity of output token Y across the Solana DEX ecosystem, explicitly balancing theoretical optimality with ultra-low latency execution and strict adherence to Solana’s single-transaction constraints.

### 1.2. Scope

The algorithm MUST handle the diverse range of Solana DEX protocols detailed in the source document (including Constant Product AMMs, Order Books, Concentrated Liquidity AMMs, StableSwap variants, Dynamic/Oracle-Priced AMMs, Batch Auctions, Hybrids), accounting for their unique mathematical models and fee structures. It MUST support optimal trade splitting across multiple venues and multi-hop paths (typically limited to 2-3 hops maximum due to practical constraints).

### 1.3. Core Challenges Addressed

Ultra-low latency execution (sub-millisecond to low-millisecond targets for core logic), state synchronization with real-time market changes, **strict adherence to Solana single-transaction constraints** (compute, size, accounts, CPI depth), MEV mitigation considerations, compliance verification, and precise numerical computation using fixed-point arithmetic.

### 1.4. Approach

**AHMO-NG employs a novel multi-phase, hybrid optimization approach within a hierarchical structure.** It strategically combines **provably optimal solutions for well-defined, computationally tractable sub-problems** (e.g., specific types of splits like AMM-only or OB walks) with an **enhanced marginal rate allocation heuristic** for general, complex scenarios. This may be augmented by **time-constrained local search refinement** and includes **solution quality assessment**.

### 1.5. Execution Guarantee

**ALL execution derived from this algorithm MUST occur within a SINGLE Solana transaction** to guarantee atomicity. This ensures either complete success or complete failure with no partial execution states.

### 1.6. Important Trade-off & Guarantees

Given the latency constraints and the non-convex, high-dimensional, dynamic nature of the full optimization problem, AHMO-NG **does not guarantee finding the mathematically provable global optimum for every complex scenario.** However, it **leverages provably optimal methods for specific, identifiable sub-problems** where feasible within latency budgets, providing mathematical guarantees for those parts. For the overall problem, it uses advanced heuristics optimized for speed and constraint compliance. The algorithm aims to provide **quality indicators** reflecting confidence in the solution’s proximity to theoretical optima where possible. This hybrid design represents a deliberate strategy to achieve the best *practical* and *executable* outcome.

## 2. Design Philosophy

### 2.1. Hierarchy for Latency

Separate distinct computational stages (Scouting, Allocation & Refinement, Execution) with aggressive, escalating time budgets.

### 2.2. Hybrid Optimization Engine

Employ a multi-stage allocation process designed to maximize output quality under strict constraints:
- **Prioritize Provable Optimality:** Identify and solve specific, well-defined sub-problems (e.g., homogeneous AMM splits, order book walks) using mathematically guaranteed optimal methods via dedicated solvers where latency permits.
- **Enhanced Heuristics:** Utilize an advanced marginal rate allocation heuristic (potentially incorporating look-ahead concepts or price impact curve data) for complex, mixed scenarios where provable methods are infeasible or too slow.
- **Refinement (Time-Permitting):** Apply rapid, strictly time-budgeted local search techniques post-allocation to escape potential local optima found by the heuristic phase.

### 2.3. Strict Transaction Constraint Awareness

Proactively integrate and continuously validate against Solana’s single-transaction compute, account (direct & ALT), size, and CPI depth limits **at every step** of the allocation and refinement process, ensuring the final generated route is always executable.

### 2.4. Real-time Adaptivity

Utilize near real-time state data from the cache and perform critical final market condition checks before execution.

### 2.5. Abstraction for Heterogeneity

Define strict, well-defined interfaces (DEX Protocol Interface) to encapsulate diverse protocol logic, enabling extensibility.

### 2.6. Execution Integrity

Leverage single-transaction atomicity; implement configurable MEV mitigation strategies.

### 2.7. User Protection

Rigorously enforce user-defined slippage tolerance limits.

### 2.8. Compliance Integration

Embed verification of token and protocol compliance status.

### 2.9. Continuous Algorithmic Improvement

Implement systematic feedback loops through post-execution analysis, comparing actual results against theoretical optima to continuously refine the algorithm’s performance.

## 3. Core Components & Data Structures

### 3.1. DEX State Cache

- **Requirement:** A low-latency data store MUST hold the latest validated state for all monitored DEX markets, updated continuously via Solana state monitoring (e.g., Geyser).
- **Required Data Per Venue/Pool:** Must be sufficient for all DEX Interface functions. Includes: Protocol ID/Type, Venue Address, Token Pair, Token Decimals, Reserves/Invariant (k), Full Tick/Liquidity Data (CLMM), Order Book Snapshot, Fee Structures, Protocol Parameters (A, D, sqrt(P)), Required Account Keys, Last Update Timestamp.
- **Protocol-Specific Data:** For batch auction DEXs, MUST also track auction cycle timing data (e.g., current batch end time estimate).
- **State Freshness:** Each cache entry MUST have a timestamp and validity indicator. Entries older than configurable thresholds MUST be flagged for verification or refresh.
- **Volatility Metrics:** Track and store price and liquidity volatility metrics per venue to inform solution confidence estimates and state update prioritization.

### 3.2. DEX Protocol Interface (Implementation Required Per Protocol)

- **Requirement:** A well-defined, testable module for each supported DEX protocol MUST implement this interface accurately, efficiently, and using required fixed-point precision (Section 5).
- **Interface Methods:**
    - **Core Methods (All Implementations):**
        - CalculateOutput(input_token, output_token, input_amount, venue_state) -> output_amount: Calculates net output after fees.
        - GetCurrentMarginalRate(input_token, output_token, venue_state, current_path_input) -> marginal_rate: Calculates instantaneous d(NetOutput)/d(Input). May incorporate logic considering price impact curve data for better decay estimation.
        - GetCapacityAtCurrentRate(input_token, output_token, venue_state, current_path_input) -> capacity: Estimates input capacity before rate degrades significantly, possibly informed by price impact data.
        - EstimateComputeCost(input_token, output_token, input_amount) -> compute_units: Provides empirically derived compute estimate.
        - GetAccountKeys(input_token, output_token, venue_state) -> AccountKeyInfo[]: Returns complete list of required accounts with metadata.
        - GetProtocolExecutionRequirements() -> execution_requirements: Returns metadata on protocol-specific needs.
        - CalculateMinimumOutput(input_token, output_token, input_amount, slippage_bps, venue_state) -> minimum_output_amount: Calculates minimum acceptable output based on slippage.
        - HandleTokenSpecificRequirements(token) -> token_requirements: Returns special token handling needs.
        - CheckTokenCompliance(token) -> compliance_status: Verifies compliance.
        - GetRateLookahead(input_token, output_token, venue_state, current_path_input, lookahead_delta) -> future_rate: Estimates marginal rate after applying lookahead_delta additional input.
        - IdentifyOptimizableSubproblem(venue_state) -> OptimizableSubproblemType: Identifies if the venue/state configuration represents a sub-problem potentially solvable with guaranteed optimality within acceptable latency (e.g., CPMM_SPLIT_CANDIDATE, ORDER_BOOK_WALK, STABLESWAP_DIRECT, NONE).
        - GetPriceImpactCurve(input_token, output_token, venue_state, max_input) -> PriceImpactData: Returns data representing the price impact curve (e.g., sampled points, function parameters) used to inform enhanced heuristic decisions.
    - **Mathematical Optimality Methods (Protocol-Specific):**
        - SupportsOptimalSolver() -> bool: Indicates whether this protocol implementation supports provably optimal solutions.
        - GetOptimalAllocation(input_token, output_token, input_amount, venue_state, venue_set) -> allocation_map: For protocols supporting optimal solutions, calculates provably optimal input distribution across venues of this type.
        - CalculateErrorBounds(input_token, output_token, input_amount, venue_state) -> error_bounds: Provides mathematical bounds on calculation error.
        - GetSolutionConfidence(input_token, output_token, input_amount, venue_state, state_age) -> confidence_metric: Returns a quantitative assessment of solution quality.

### 3.3. Path State Tracker

- **Requirement:** Tracks simulated input/output per hop for active multi-hop paths during Level 2. E.g., Map<PathID, Map<HopIndex, {InputAmount, OutputAmount}>>.
- **Hop Data:** For each hop, must store input token, output token, venue, applied input amount, calculated output amount, accumulated fees.
- **Path Information:** Store aggregated path statistics including total expected output, end-to-end equivalent exchange rate, compound slippage, total fees paid.
- **Path Interdependencies:** Track shared hops or venues across different paths to identify potential allocation interdependencies.

### 3.4. Enhanced Priority Queue (Max Heap)

- **Requirement:** Stores potential allocation steps prioritized by multiple criteria.
- **Priority Criteria:** Primary - E2E_MR (End-to-End Marginal Rate), Secondary - Solution confidence metric.
- **Entry Data:** {capacity_at_rate, path_details, estimated_path_compute, set_of_path_accounts, lookahead_rate}.
- **Implementation Optimizations:** Lock-free concurrent implementation for parallel processing, optimized for frequent updates and resorting.

### 3.5. Transaction Constraint Tracker

- **Requirement:** Continuously tracks the cumulative transaction constraints during allocation to ensure single-transaction viability.
- **State:** Maintains {total_compute: u32, unique_accounts: Set, account_owners: Set, mutable_accounts: Set, immutable_accounts: Set, instruction_count: u16, estimated_tx_size: u32, account_lookup_tables_used: Set, total_cpi_depth: u8, token_program_cpi_count: u8} and validates against Solana limits.
- **ALT Management:** Maintains reference to available Account Lookup Tables and their contained accounts, either provided via configuration or discovered through network queries. Tracks which accounts can be referenced via ALTs versus direct inclusion.
- **Verification Methods:** Includes functions to verify if adding a new path would violate any transaction constraints, with detailed reporting on specific constraints violated.
- **Constraint Modeling:** Uses empirically validated models to predict actual on-chain resource consumption with high accuracy, continuously refined through post-execution feedback.

### 3.6. Network Condition Monitor

- **Requirement:** Tracks current Solana network conditions to optimize transaction parameters.
- **Data Collection:** Monitors recent block production times, confirmation delays, average priority fees, and congestion indicators through validator RPC endpoints or specialized services.
- **Update Frequency:** Refreshes data at configurable intervals, with more frequent updates during high volatility or congestion periods.
- **Metrics:** Maintains rolling statistics including median and 95th percentile confirmation times by priority fee tier, success rates, and congestion classification (Low/Medium/High/Extreme).
- **MEV Analysis:** Tracks patterns indicative of MEV activity to inform protection strategies.

### 3.7. Compliance Registry

- **Requirement:** Maintains up-to-date compliance information for tokens and protocols.
- **Token Data:** Stores token compliance status, risk classification, and any restrictions.
- **Protocol Data:** Tracks protocol compliance status and any specific requirements or restrictions.
- **Verification API:** Provides fast lookup methods for compliance checks during path scouting and allocation.

### 3.8. Optimal Subproblem Solver

- **Requirement:** A dedicated module containing highly efficient algorithms for solving specific, mathematically well-defined sub-problems identified by IdentifyOptimizableSubproblem.
- **Examples:** Convex optimization solver (e.g., interior point or Lagrangian-based) for direct CPMM splits, greedy algorithm for optimal order book walk, direct calculation for StableSwap curves.
- **Constraint:** Each solver MUST execute within a strict, very low latency budget (e.g., sub-millisecond) to be viable within Level 2.
- **Solver Types:**
    - **AMM Convex Optimizer:** For optimal splits across constant product AMMs.
    - **OrderBook Optimal Router:** For finding provably optimal routes through order books.
    - **StableSwap Optimizer:** For mathematically optimal routing through StableSwap pools.
    - **Mixed Protocol Approximator:** For bounded approximation solutions in mixed protocol scenarios.
- **Solver Selection:** Dynamically selects appropriate solver based on DEX composition, token pairs, and available computation time.
- **Performance Monitoring:** Tracks solver performance to inform future selection decisions.

### 3.8.1 AMM Convex Optimizer Algorithm

The AMM Convex Optimizer produces provably optimal allocations across constant product AMM pools:

```
AMM_Convex_Optimizer(pools, input_amount_Q):
    // Initialize allocation and remaining pools
    allocation = new Map<PoolID, Amount>()
    remaining_pools = filter(pools, pool => pool.hasLiquidity())

    // Calculate initial marginal rates for each pool
    pool_marginal_rates = []
    for each pool in remaining_pools:
        mr = (pool.reserves_x * pool.reserves_y * (1-pool.fee)) / (pool.reserves_x)²
        pool_marginal_rates.append({pool: pool, mr: mr})

    // Binary search for the optimal Lagrange multiplier
    lower_bound = 0
    upper_bound = max(pool_marginal_rates).mr
    λ = (lower_bound + upper_bound) / 2

    MAX_ITERATIONS = 20
    TOLERANCE = 1e-10

    for i = 0 to MAX_ITERATIONS:
        // Calculate tentative allocations using current λ
        total_allocated = 0
        for each pool_data in pool_marginal_rates:
            pool = pool_data.pool
            // Apply the KKT solution formula
            q_i = (1/(1-pool.fee)) * (sqrt((pool.reserves_x * pool.reserves_y * (1-pool.fee))/λ) - pool.reserves_x)

            // Ensure non-negative allocation
            q_i = max(0, q_i)

            // Track allocation
            allocation[pool.id] = q_i
            total_allocated += q_i

        // Check if we've converged
        if abs(total_allocated - input_amount_Q) < TOLERANCE:
            break

        // Adjust λ based on allocation result
        if total_allocated > input_amount_Q:
            lower_bound = λ
        else:
            upper_bound = λ

        λ = (lower_bound + upper_bound) / 2

    // Final normalization to ensure exact sum
    scale_factor = input_amount_Q / sum(allocation.values())
    for pool_id in allocation.keys():
        allocation[pool_id] *= scale_factor

    return allocation
```

### 3.8.2 OrderBook Optimal Router Algorithm

The OrderBook Optimal Router implements a provably optimal strategy for order book DEXs:

```
OrderBook_Optimal_Router(order_books, input_amount_Q):
    // Collect all orders from all books
    all_orders = []
    for each book in order_books:
        all_orders.extend(book.orders)

    // Sort by price (best price first)
    all_orders.sort(key=lambda order: order.price)

    // Walk the order book in price order
    allocation = new Map<OrderBookID, Map<OrderID, Amount>>()
    remaining_amount = input_amount_Q

    for each order in all_orders:
        if remaining_amount <= 0:
            break

        // Determine how much can be filled from this order
        fillable = min(order.volume, remaining_amount)

        // Record the allocation
        if order.book_id not in allocation:
            allocation[order.book_id] = new Map<OrderID, Amount>()
        allocation[order.book_id][order.id] = fillable

        // Update remaining amount
        remaining_amount -= fillable

    return allocation
```

### 3.8.3 StableSwap Optimizer Algorithm

The StableSwap Optimizer calculates optimal allocations for StableSwap pools:

```
StableSwap_Optimizer(pools, input_amount_Q):
    // Initialize
    allocation = new Map<PoolID, Amount>()

    // Newton-Raphson parameters
    MAX_ITERATIONS = 30
    TOLERANCE = 1e-10

    // For each pool, calculate optimal split based on state and amplification
    remaining_amount = input_amount_Q
    pool_data = []

    for each pool in pools:
        // Calculate pool properties
        A = pool.amplification
        x = pool.balance_x
        y = pool.balance_y

        // Estimate D invariant using Newton-Raphson
        D = estimate_invariant_D(A, x, y)

        // Calculate marginal rate at current state
        mr = calculate_stableswap_marginal_rate(A, x, y, D)

        pool_data.append({
            pool: pool,
            A: A,
            x: x,
            y: y,
            D: D,
            mr: mr
        })

    // Sort pools by marginal rate (highest first)
    pool_data.sort(key=lambda data: data.mr, reverse=True)

    // Iterative allocation to highest marginal rate first
    for each data in pool_data:
        if remaining_amount <= 0:
            break

        // Calculate maximum amount that can be allocated to this pool
        // while maintaining reasonable slippage
        capacity = estimate_stableswap_capacity(data.A, data.x, data.y, data.D)

        // Allocate
        amount = min(capacity, remaining_amount)
        allocation[data.pool.id] = amount
        remaining_amount -= amount

    // If we still have remaining amount, distribute proportionally
    if remaining_amount > 0:
        // Calculate weighted distribution based on liquidity depth
        weights = []
        for each data in pool_data:
            depth = estimate_liquidity_depth(data.A, data.x, data.y, data.D)
            weights.append(depth)

        // Normalize weights
        total_weight = sum(weights)
        normalized_weights = [w / total_weight for w in weights]

        // Distribute remaining amount
        for i = 0 to pool_data.length - 1:
            additional = remaining_amount * normalized_weights[i]
            pool_id = pool_data[i].pool.id
            allocation[pool_id] += additional

    return allocation
```

### 3.8.4 Mixed Protocol Approximator Algorithm

The Mixed Protocol Approximator implements a bounded approximation for heterogeneous DEX scenarios:

```
Mixed_Protocol_Approximator(paths, input_amount_Q, epsilon):
    // Discretization step size
    delta = (epsilon * input_amount_Q) / paths.length

    // Build dynamic programming table
    // dp[i][j] = maximum output when using first i paths with j*delta input
    n = paths.length
    m = floor(input_amount_Q / delta) + 1

    dp = new Array(n+1, m+1)
    decisions = new Array(n+1, m+1)

    // Base case: no paths means no output
    for j = 0 to m:
        dp[0][j] = 0

    // Fill the DP table
    for i = 1 to n:
        for j = 0 to m:
            // Default: don't use this path
            dp[i][j] = dp[i-1][j]
            decisions[i][j] = 0

            // Try different allocation amounts to this path
            for k = 1 to j:
                input_to_path = k * delta
                output = calculate_path_output(paths[i-1], input_to_path)

                if dp[i-1][j-k] + output > dp[i][j]:
                    dp[i][j] = dp[i-1][j-k] + output
                    decisions[i][j] = k

    // Reconstruct solution
    allocation = new Map<PathID, Amount>()
    i = n
    j = m

    while i > 0:
        k = decisions[i][j]
        if k > 0:
            allocation[paths[i-1].id] = k * delta
            j -= k
        i -= 1

    return allocation
```

### 3.9. Local Search Engine

- **Requirement:** Performs targeted local search around candidate solutions to escape local optima.
- **Search Methods:**
    - **Perturbation Analysis:** Tests small shifts in allocation to identify potential improvements.
    - **Alternative Path Substitution:** Evaluates path replacements to identify potentially better combinations.
    - **Boundary Condition Analysis:** Examines solutions at constraint boundaries to ensure optimality.
- **Termination Criteria:** Configurable time budget and improvement thresholds.

### 3.9.1 Pairwise Shift Local Search Algorithm

The primary local search algorithm uses efficient pairwise shifts to improve solutions:

```
Pairwise_Shift_Local_Search(initial_allocation, paths, constraint_tracker, max_iterations, epsilon_amount, time_budget):
    // Initialize
    current_allocation = copy(initial_allocation)
    current_output = calculate_total_output(current_allocation, paths)
    start_time = current_time()
    iterations = 0
    improvement_found = true

    // Main loop - continue until no more improvements or limits reached
    while improvement_found and iterations < max_iterations and (current_time() - start_time) < time_budget:
        improvement_found = false

        // Get pairs of paths with non-zero and zero allocations
        active_paths = [p for p in paths if current_allocation[p.id] >= epsilon_amount]

        // Try shifting between all pairs of active paths
        for i = 0 to active_paths.length - 1:
            for j = i+1 to active_paths.length - 1:
                path_i = active_paths[i]
                path_j = active_paths[j]

                // Skip if first path doesn't have enough allocation
                if current_allocation[path_i.id] < epsilon_amount:
                    continue

                // Try shifting epsilon from path_i to path_j
                test_allocation = copy(current_allocation)
                test_allocation[path_i.id] -= epsilon_amount
                test_allocation[path_j.id] += epsilon_amount

                // Calculate new output
                test_output = calculate_total_output(test_allocation, paths)

                // If improvement found and constraints satisfied
                if test_output > current_output:
                    // Check constraints
                    if constraint_tracker.canAccommodate(test_allocation):
                        current_allocation = test_allocation
                        current_output = test_output
                        improvement_found = true
                        break  // Found an improvement, restart with new allocation

            if improvement_found:
                break

        iterations += 1

    return current_allocation
```

### 3.9.2 Path Substitution Local Search Algorithm

This algorithm explores substituting entire paths to find improvements:

```
Path_Substitution_Local_Search(current_allocation, active_paths, candidate_paths, constraint_tracker, time_budget):
    // Initialize
    best_allocation = copy(current_allocation)
    best_output = calculate_total_output(best_allocation, active_paths)
    start_time = current_time()

    // Filter candidate paths not already in active allocation
    inactive_candidates = [p for p in candidate_paths if p.id not in current_allocation.keys() or current_allocation[p.id] == 0]

    // For each path in the current allocation
    for active_path in active_paths:
        if (current_time() - start_time) >= time_budget:
            break

        active_amount = current_allocation[active_path.id]
        if active_amount == 0:
            continue

        // Try substituting with each inactive candidate
        for candidate_path in inactive_candidates:
            // Create test allocation by moving amount to candidate
            test_allocation = copy(current_allocation)
            test_allocation[active_path.id] = 0
            test_allocation[candidate_path.id] = active_amount

            // Calculate new output
            test_output = calculate_total_output(test_allocation, active_paths + [candidate_path])

            // If improvement found and constraints satisfied
            if test_output > best_output:
                if constraint_tracker.canAccommodate(test_allocation):
                    best_allocation = test_allocation
                    best_output = test_output

    return best_allocation
```

### 3.9.3 Boundary Analysis Algorithm

This algorithm examines solutions at constraint boundaries:

```
Boundary_Analysis_Local_Search(current_allocation, active_paths, constraint_tracker, time_budget):
    // Initialize
    best_allocation = copy(current_allocation)
    best_output = calculate_total_output(best_allocation, active_paths)
    start_time = current_time()

    // Identify paths near constraint boundaries
    boundary_paths = []
    for path in active_paths:
        // Check if this path's allocation could be increased
        test_allocation = copy(current_allocation)
        test_allocation[path.id] += epsilon_amount

        // If increasing allocation would violate constraints
        if not constraint_tracker.canAccommodate(test_allocation):
            boundary_paths.append(path)

    // For each path near a boundary
    for boundary_path in boundary_paths:
        if (current_time() - start_time) >= time_budget:
            break

        // For each other active path
        for other_path in active_paths:
            if boundary_path.id == other_path.id:
                continue

            // Try redistributing small amounts near the boundary
            test_amounts = [0.5*epsilon_amount, 0.25*epsilon_amount, 0.1*epsilon_amount]

            for test_amount in test_amounts:
                // Skip if other path doesn't have enough allocation
                if current_allocation[other_path.id] < test_amount:
                    continue

                // Create test allocation by redistributing
                test_allocation = copy(current_allocation)
                test_allocation[other_path.id] -= test_amount
                test_allocation[boundary_path.id] += test_amount

                // Check if this violates constraints
                if not constraint_tracker.canAccommodate(test_allocation):
                    continue

                // Calculate new output
                test_output = calculate_total_output(test_allocation, active_paths)

                // If improvement found
                if test_output > best_output:
                    best_allocation = test_allocation
                    best_output = test_output

    return best_allocation
```

### 3.10. Quality Assessor

- **Requirement:** Module executed at the end of Level 2 to generate a quality_score (e.g., a numerical value or categorical rating).
- **Calculation Basis:** Considers factors like the proportion of the trade allocated via Optimal Subproblem Solver vs. heuristic methods, comparison against theoretical bounds (if computed), success and impact of the local search refinement phase, convergence metrics from the heuristic phase.
- **Metrics:**
    - **Optimality Gap:** Estimated distance from theoretical optimum.
    - **Confidence Score:** Probability that solution is within specified percentage of global optimum.
    - **Sensitivity Analysis:** How solution quality varies with small input changes.
    - **Constraint Utilization:** How close the solution is to transaction constraint limits.
- **Integration:** Provides feedback to users and drives continuous algorithm improvement.

### 3.10.1 Solution Quality Assessment Algorithm

The comprehensive quality assessment algorithm:

```
Solution_Quality_Assessment(
    allocation,
    paths,
    input_amount_Q,
    allocation_method_stats,
    constraint_tracker,
    theoretical_bounds=None
):
    // 1. Calculate proportion of trade allocated using optimal methods
    optimal_amount = allocation_method_stats.amount_allocated_by_optimal_solvers
    ratio_optimal = optimal_amount / input_amount_Q

    // 2. Measure refinement impact
    output_before_refinement = allocation_method_stats.output_before_refinement
    output_after_refinement = calculate_total_output(allocation, paths)
    refinement_gain = 0
    if output_before_refinement > 0:
        refinement_gain = (output_after_refinement / output_before_refinement) - 1

    // 3. Calculate marginal rate variance for active paths
    active_paths = [p for p in paths if allocation[p.id] > 0]
    marginal_rates = []

    for path in active_paths:
        // Get current MR for this path with its allocation
        mr = calculate_path_marginal_rate(path, allocation[path.id])
        marginal_rates.append(mr)

    mr_variance = calculate_variance(marginal_rates)
    mr_uniformity = 1.0 / (1.0 + mr_variance) // Normalized to [0,1]

    // 4. Calculate optimality gap if theoretical bounds available
    gap_estimate = 0
    if theoretical_bounds is not None and theoretical_bounds.upper_bound > 0:
        gap_estimate = 1 - (output_after_refinement / theoretical_bounds.upper_bound)

    // 5. Calculate constraint utilization metrics
    compute_utilization = constraint_tracker.total_compute / constraint_tracker.MAX_COMPUTE
    account_utilization = constraint_tracker.unique_accounts.size() / constraint_tracker.MAX_ACCOUNTS
    size_utilization = constraint_tracker.estimated_tx_size / constraint_tracker.MAX_TX_SIZE
    cpi_utilization = constraint_tracker.total_cpi_depth / constraint_tracker.MAX_CPI_DEPTH

    constraint_utilization = max(
        compute_utilization,
        account_utilization,
        size_utilization,
        cpi_utilization
    )

    // 8. Categorize the quality score
    quality_category = "Unknown"
    if quality_score >= 0.90:
        quality_category = "Excellent" // Very high confidence in solution quality
    else if quality_score >= 0.75:
        quality_category = "Good"      // Good confidence in solution quality
    else if quality_score >= 0.50:
        quality_category = "Adequate"  // Moderate confidence in solution quality
    else:
        quality_category = "Heuristic" // Solution based primarily on heuristics

    // 9. Calculate sensitivity metrics
    sensitivity_metrics = calculate_sensitivity(allocation, paths, input_amount_Q)

    // 10. Return comprehensive assessment
    return {
        quality_score: quality_score,
        quality_category: quality_category,
        optimality_gap: gap_estimate,
        confidence: map_score_to_confidence(quality_score),
        sensitivity: sensitivity_metrics,
        constraint_utilization: constraint_utilization,
        details: {
            ratio_optimal: ratio_optimal,
            mr_uniformity: mr_uniformity,
            refinement_gain: refinement_gain,
            constraint_breakdown: {
                compute: compute_utilization,
                accounts: account_utilization,
                size: size_utilization,
                cpi_depth: cpi_utilization
            }
        }
    }
```

### 3.10.2 Sensitivity Analysis Algorithm

The sensitivity analysis calculates how solution quality varies with input changes:

```
calculate_sensitivity(allocation, paths, input_amount_Q):
    DELTA_PERCENT = 0.02  // 2% perturbation

    // Calculate base output
    base_output = calculate_total_output(allocation, paths)

    // Calculate output with slightly increased input
    delta_amount = input_amount_Q * DELTA_PERCENT
    increased_output = estimate_output_for_increased_input(allocation, paths, delta_amount)

    // Calculate sensitivity metric - how much output changes relative to input change
    // Lower values indicate more stable solutions
    if increased_output > base_output:
        sensitivity = abs((increased_output - base_output) / base_output) / DELTA_PERCENT
    else:
        sensitivity = 1.0  // Maximum sensitivity if output decreases with increased input

    // Normalized stability score (lower sensitivity is better)
    stability_score = 1.0 / (1.0 + sensitivity)

    return {
        sensitivity: sensitivity,
        stability_score: stability_score
    }
```

## 4. Algorithm Logical Steps

### 4.1. Level 0: Continuous State Management (Background Process)

- **Logic:** Monitor Solana accounts, parse data, validate, update DEX State Cache. Ensure thread-safe access and cache invalidation. For batch auction DEXs, track cycle timing/state.
- **Frequency:** Update frequency MUST be configurable per protocol type, with recommended defaults:
    - Order Books: Multiple updates per second (100-1000ms)
    - Concentrated Liquidity: 500-2000ms
    - Standard AMMs: 1000-5000ms
    - StableSwaps: 2000-5000ms
- **Validation:** Apply sanity checks to incoming state data (e.g., reserve changes within expected ranges, prices not deviating dramatically). Flag suspicious state for verification or exclusion.
- **Optimization:** Prioritize updates for high-volume pairs and frequently used venues. Implement adaptive update frequency based on market volatility and trading activity.
- **Volatility Tracking:** Calculate and store volatility metrics to inform solution confidence calculations and state update priorities.

### 4.2. Level 1: Candidate Path Scouting & Subproblem ID (Target: < 1ms, Parallelized)

- **Input:** source_token, target_token, input_amount_Q, user_slippage_bps, transaction_constraints, compliance_requirements.
- **Logic (Highly Parallelized):**
    1. **Path Generation:** Identify potential 1-hop and multi-hop paths (up to MAX_PATH_HOPS - typically 2-3) via DEX State Cache. Track estimated CPI requirements meticulously (including token programs).
    2. **Problem Classification:** Analyze the set of potential paths to classify the trading problem:
        - **Homogeneous AMM:** If all viable paths are through identical AMM types.
        - **Order Book Only:** If all viable paths are through order books.
        - **Mixed Protocol Simple:** If viable paths include a mix of protocols but with clear dominance.
        - **Mixed Protocol Complex:** If viable paths include a complex mix of protocols with similar performance.
    3. **Subproblem Identification:** For each potential path segment/venue, invoke IdentifyOptimizableSubproblem. Tag paths/venues that are candidates for guaranteed optimization in Level 2.
    4. **Basic Filtering:** Apply heuristic filters (liquidity thresholds, fee estimates, initial slippage checks) using DEX Interface estimates.
    5. **Transaction Viability Pre-Check:** Estimate resources (EstimateComputeCost, GetAccountKeys) for each potential path. Immediately discard paths that would exceed transaction_constraints even if taken alone.
    6. **Compliance/Token Checks:** Verify token/venue compliance via Compliance Registry. Check and flag special token handling needs via HandleTokenSpecificRequirements. Filter non-compliant paths.
    7. **Protocol-Specific Checks:** Incorporate checks for batch auctions, etc.
    8. **Theoretical Bound Context (Optional):** Calculate simple theoretical price bounds (e.g., best initial spot rate) for later quality assessment context.
    9. **Token Program CPI Tracking:** Account for token program CPIs required for approvals and transfers.
    10. **Slippage Check:** Apply user slippage check by calling CalculateMinimumOutput for end-to-end paths.
    11. **Fallback Path:** If path count < MIN_PATHS, CONSIDER adding default fallback path if viable.
- **Parallel Processing:** Elements of path generation, basic filtering, and compliance verification SHOULD be executed in parallel where possible.
- **Output:**
    - Filtered set P of candidate concrete paths (up to MAX_CANDIDATE_PATHS), each individually viable within a single transaction.
    - Problem classification determining which optimization approach to use in Level 2.
    - Terminate if P is empty.

### 4.3. Level 2: Hybrid Optimization with Constraint Awareness (Target: < 9ms)

- **Input:**
    - Path set P: Filtered candidate paths from Level 1, tagged with OptimizableSubproblemType.
    - problem_classification: Overall classification from Level 1 (e.g., Homogeneous AMM, Mixed Complex).
    - input_amount_Q: The total input quantity to be allocated in this Level.
    - user_slippage_bps: User’s maximum slippage tolerance.
    - transaction_constraints: Solana limits (compute, accounts, size, CPI).
- **Output:**
    - final_allocation: Dictionary {path_id: allocated_input_amount}.
    - quality_score: Metrics indicating solution confidence/methodology.
    - Returns empty/error if no viable allocation found.
- **Core Principle:** **Every** allocation decision or refinement step MUST be validated against the Transaction Constraint Tracker *before* being committed to ensure the final solution remains within Solana’s single-transaction limits.

**Logic:**

1. **Initialization (Target: << 1ms):**
    - remaining_Q = input_amount_Q
    - final_allocation = {} (Map<PathID, Amount>)
    - path_states = {} (Map<PathID, CurrentSimulatedState>) - Initialize based on initial state from DEX State Cache.
    - Initialize tx_constraint_tracker (Component 3.5): Reset counters, analyze available ALTs based on config/discovery.
    - level_2_start_time = now()
    - time_budget_L2 = 9ms (Configurable)
2. **Phase 2.1: Guaranteed Allocation Pass (Target: < 1-2ms):**
    - **Goal:** Identify if a significant portion (ideally all) of remaining_Q can be allocated via a *single*, provably optimal sub-method quickly. Focus on high-impact, fast-solving scenarios.
    - **Identify Candidates:** Scan P and use problem_classification to find dominant, optimizable sub-problems:
        - **Scenario A (Pure OB Walk):** If *all* significant paths in P are direct Order Book paths.
        - **Scenario B (Pure Homogeneous AMM Split):** If *all* significant paths are direct, identical-type AMMs (e.g., only CPMMs, only specific StableSwap type) flagged as optimizable.
    - **Attempt Optimal Solution:**
        - If Scenario A:
            - Call Optimal Subproblem Solver.solve_order_book_walk(ob_paths, remaining_Q). Returns sub_allocation, sub_output.
        - If Scenario B:
            - Call Optimal Subproblem Solver.solve_amm_convex_split(amm_paths, remaining_Q). Returns sub_allocation, sub_output.
    - **Constraint Check & Application:**
        - If a solver returned a valid sub_allocation for the *entire* remaining_Q:
            - Estimate resources required for sub_allocation using precise interface methods.
            - **Verify** if tx_constraint_tracker.can_add(sub_allocation_resources) is **TRUE**.
            - If **VALID** and sub_output is positive:
                - final_allocation = sub_allocation
                - remaining_Q = 0
                - Update tx_constraint_tracker fully.
                - Log allocation method as “Optimal Solver”.
                - **Proceed directly to Phase 2.4 (Quality Assessment).**
            - If **INVALID** (exceeds constraints) or solver failed/returned poor result: Continue to Phase 2.2.
        - *(Self-Correction: Avoid trying partial optimal allocations here for simplicity and speed; let Phase 2.2 handle the complex mixing if the whole problem isn’t cleanly solvable optimally).*
    - **If no single optimal method applied to the full amount, proceed to Phase 2.2.**
3. **Phase 2.2: Enhanced Marginal Allocation (Core Loop - Target: < 6-7ms, bulk of budget):**
    - **Goal:** Allocate the remaining_Q across all available paths using an enhanced marginal rate heuristic, continuously respecting constraints.
    - **Initialize Priority Queue (PQ - Component 3.4):**
        - For each path p in P (use current state from path_states if modified):
            - If p has remaining capacity:
                - current_MR = DEX_Interface[p.type].GetCurrentMarginalRate(…)
                - capacity = DEX_Interface[p.type].GetCapacityAtCurrentRate(…)
                - lookahead_delta = min(capacity, remaining_Q * 0.05) // Small lookahead amount (configurable %)
                - lookahead_MR = DEX_Interface[p.type].GetRateLookahead(…, lookahead_delta)
                - priority_score = calculate_priority(current_MR, lookahead_MR) // Function prioritizes high current MR but penalizes sharp drops indicated by lookahead_MR. E.g., 0.7 * current_MR + 0.3 * lookahead_MR.
                - Estimate resources (step_compute, step_accounts) for allocating a small epsilon_amount via path p.
                - If priority_score > 0, add {priority_score, capacity, p, step_compute, step_accounts} to PQ.
    - Allocation Loop (Iterative, Constraint-Checked):** While remaining_Q > epsilon_amount AND PQ is not empty:1. **Select Path:** Extract highest priority_score entry p* from PQ. If p*.priority_score <= 0 (considering fees), break.1. **Constraint Pre-Check (Fast Estimate):** If tx_constraint_tracker.would_violate(p*.step_accounts, p*.step_compute), discard p* from PQ, continue.1. **Determine Step Size (delta_alloc):**Use p*.capacity, remaining_Q.*Enhancement:* Use DEX_Interface[p*.type].GetPriceImpactCurve(…) data to potentially select a delta_alloc smaller than capacity if price impact accelerates rapidly. Aim for a step size where the *average* rate over the step remains reasonably close to current_MR. Define delta_alloc = determine_dynamic_step(p*, remaining_Q, price_impact_data). Ensure delta_alloc >= epsilon_amount.
        
        ---
        
        ---
        
        ---
        
        ---
        
        ---
        
        ---
        
        ```
          1. **Simulate & Get Precise Cost:**
          - output_delta = DEX_Interface[p*.type].CalculateOutput(…, delta_alloc)
          - Calculate *precise* resources (precise_compute, precise_accounts, precise_size_delta, precise_cpi_delta) needed for *this specific* delta_alloc on path p*.
        ```
        
        - 
            1. **Constraint Verification & Commit (CRITICAL):**
            - **Verify:** tx_constraint_tracker.can_add(precise_compute, precise_accounts, precise_size_delta, precise_cpi_delta)
            - If **VALID**:
                - Commit: final_allocation[p*] += delta_alloc, remaining_Q -= delta_alloc.
                - Update path_states[p*] based on simulation.
                - **Permanently Update tx_constraint_tracker**.
                - Log allocation step (path, amount, method=Heuristic).
            - If **INVALID**: Discard p* permanently from PQ. Continue to next iteration.
        - 
            1. **Recalculate & Re-insert (If Committed & Path Has Capacity):**
            - Update current_MR, capacity, lookahead_MR, priority_score, resource estimates for p*.
            - Re-insert p* into PQ.
        - 
            1. **Handle Interdependencies (Optional/Advanced):** If implemented, update metrics for dependent paths in PQ.
    - **Loop Termination:** Check elapsed time against L2 budget periodically; break if approaching limit.
4. **Phase 2.3: Local Search Refinement (Target: < 1ms, Optional & Time-Budgeted):**
    - **Goal:** Make small adjustments to the final_allocation to potentially improve output, if time permits.
    - **Check Time Budget:** if now() > level_2_start_time + (time_budget_L2 - budget_phase_2_3) then skip. (budget_phase_2_3 is configurable, e.g., 1ms).
    - **Algorithm (Example: Iterative Pairwise Shift):**
        - iterations = 0, max_iterations = Config.local_search_iterations
        - delta_shift = epsilon_amount
        - While iterations < max_iterations and time_remains_in_budget:
            - Select a pair (p_i, p_j) from final_allocation (e.g., randomly, or based on similar final simulated marginal rates). Ensure final_allocation[p_i] >= delta_shift.
            - Simulate shifting delta_shift from p_i to p_j. Calculate net_output_change.
            - If net_output_change > 0:
                - Calculate net_resource_change for the shift.
                - **Constraint Check:** if tx_constraint_tracker.can_accomodate_net_change(net_resource_change):
                    - Commit shift: Update final_allocation, path_states. Update tx_constraint_tracker. Log refinement.
            - iterations += 1
            - Check time budget again.
5. **Phase 2.4: Quality Assessment (Target: << 1ms):**
    - **Goal:** Calculate the quality_score.
    - **Algorithm:**
        - Calculate ratio_optimal = amount_allocated_by_phase_2_1 / input_amount_Q.
        - Calculate refinement_gain = (output_after_2_3 / output_after_2_2) - 1 (if 2.3 ran).
        - Estimate final_MR_variance across paths with non-zero allocation.
        - (If bounds available) gap_estimate = 1 - final_total_output / theoretical_upper_bound.
        - Combine into quality_score using a defined formula (e.g., weighted average, or categorical mapping). score = calculate_score(ratio_optimal, refinement_gain, final_MR_variance, gap_estimate).
        - Calculate other defined metrics (Confidence, Sensitivity, Constraint Utilization) using Quality Assessor logic.

### 4.4. Level 3: Execution & Finalization (Target: Microseconds + Network Latency)

- **Input:** final_allocation, quality_score, user_slippage_bps, execution_options.
- **Logic:**
    1. **Protocol Requirements:** For each path/venue in final_allocation, call GetProtocolExecutionRequirements.
    2. **Batch Timing:** If batch auction DEXs are involved, calculate optimal submission timing.
    3. **ALT Integration:** If ALTs were identified, include them in transaction setup.
    4. **Instruction Planning:** Determine correct instruction ordering considering:
        - Protocol-specific requirements (e.g., cranking before trading)
        - Data dependencies (outputs feeding into inputs)
        - Account ownership and borrowing rules
        - CPI depth limitations (ensure deepest CPI chains don’t exceed Solana limits)
        - Account mutability conflicts and sequencing
        - Token program approval and transfer instruction placement
    5. **Transaction Construction:** Generate precise Solana instructions with appropriate account references, slippage protection, and approvals.
    6. **Transaction Validation:** Perform final validation against transaction limits.
    7. **Pre-Execution State Capture:** Log current token balances for post-execution analysis.
    8. **Market Condition Check:** Query GetCurrentMarginalRate for critical paths using latest state. If divergence > FINAL_CHECK_DIVERGENCE_THRESHOLD, MUST abort and return “Requote needed” error.
    9. **Network Condition Adaptation:** Query Network Condition Monitor for optimal transaction parameters.
    10. **Batch Auction Strategy:** For batch auction DEXs, implement timing optimization.
    11. **MEV Protection Implementation:** Apply specific configured mitigation strategies.
    12. **Priority Fee Calculation:** Calculate appropriate priority fee based on network conditions.
    13. **Transaction Submission Strategy:** Submit the single transaction with appropriate parameters.
    14. **Confirmation Monitoring:** Track transaction status with timeouts and retry logic.
    15. **Result Verification:** Parse transaction results to extract execution details.
    16. **Post-Execution Validation:** Verify final token balances against expectations.
    17. **Enhanced Post-Execution Validation & Feedback Loop:**
        - Verify final balances against expected. Log discrepancies.
        - **Critical:** Compare the actual executed final_output_amount against results from an **offline, slower, more exhaustive optimization algorithm** run using the pre-execution captured state. This comparison provides the ground truth for assessing AHMO-NG’s real-world effectiveness.
        - Feed analysis results (heuristic performance, sub-solver accuracy, refinement impact, constraint estimation errors) back into the algorithm’s tuning parameters and development backlog.
    18. **Performance Metrics Capture:** Record execution performance metrics including:
        - Path selection quality (achieved vs estimated output)
        - Timing data (path finding, allocation, execution, confirmation)
        - Fee effectiveness (output gained per priority fee unit)
        - State freshness impact (correlation between state age and price divergence)
        - Include quality_score, timing breakdowns, comparison to offline optimum if calculated
    19. **Solution Quality Feedback:** Compare actual results with predicted quality metrics to refine future predictions.
- **Output:** Transaction signature, execution status, performance metrics, solution quality assessment, and final output amount or error code.

### 4.5. Level 4: Post-Execution Analysis (Background Process)

- **Input:** Historical execution data, predicted vs. actual output amounts, solution quality metrics.
- **Logic:**
    1. **Theoretical Optimum Calculation:** Using more exhaustive methods and complete market data, calculate the theoretical optimal route that was possible at execution time.
    2. **Gap Analysis:** Quantify the difference between AHMO-NG’s solution and the theoretical optimum.
    3. **Pattern Recognition:** Identify common patterns in suboptimal solutions to guide algorithm refinement.
    4. **Constraint Model Validation:** Compare predicted vs. actual resource consumption to refine constraint models.
    5. **Parameter Optimization:** Automatically tune algorithm parameters based on historical performance.
    6. **DEX Interface Validation:** Identify potential inaccuracies in protocol interface implementations.
- **Output:** Algorithm refinement recommendations, parameter updates, and performance analytics.

## 5. Numerical Precision & Fixed-Point Arithmetic

### 5.1. Precision Requirements

- All monetary/token quantities MUST use u128 or equivalent high-precision fixed-point library.
- Intermediate calculations MUST preserve precision.
- Rounding MUST be done consistently (recommend rounding down for outputs to ensure slippage checks are conservative).
- Error propagation MUST be explicitly calculated and tracked for multi-hop routes.

### 5.2. Token Decimal Handling

- Implementations MUST handle tokens with different decimal places correctly, especially in multi-hop routes.
- Special care is required for tokens with very low decimals (e.g., 4 or fewer) or unusually high decimals (e.g., 18+).
- When combining tokens with different decimal places, scale all values to a common precision before performing operations.
- Decimal conversion operations MUST be carefully ordered to minimize precision loss.

### 5.3. Numerical Edge Case Handling

- **Low Decimal Tokens:** Implement special handling for tokens with few decimals to prevent excessive quantization errors.
- **Extreme Pool Ratios:** Use specialized techniques when dealing with pools having extreme token ratios.
- **Catastrophic Cancellation:** Implement algebraic reformulations to avoid subtracting nearly equal large numbers.
- **Precision Limits:** Monitor calculations for approach to precision limits and implement safeguards.
- **Numerical Stability:** Implement stability checks throughout calculation pipelines.
- **Alternative Formulations:** Maintain algebraically equivalent alternative calculation paths for numerically challenging scenarios.

### 5.4. Testing Requirements

- Implement comprehensive test suites with edge case coverage including extreme values, ratios, and precision challenges.
- Compare results with alternative calculation methods to verify precision and accuracy.
- Implement Monte Carlo testing with random inputs to identify potential instability regions.
- Validate numerical precision against on-chain execution results to ensure consistency.

## 6. Configuration & Tuning

### 6.1. Algorithm Parameters

- **Level 1 Parameters:** MAX_PATH_HOPS, MAX_CANDIDATE_PATHS, MIN_PATHS, filtering thresholds.
- **Level 2 Parameters:** epsilon_amount, lookahead_depth, local_search_iterations, allocation step size, parameters controlling Phase 2.1 (subproblem solver enabling/triggers), Phase 2.3 (refinement enabling, time budget, search parameters), Phase 2.4 (quality score calculation weights/logic).
- **Level 3 Parameters:** FINAL_CHECK_DIVERGENCE_THRESHOLD, transaction retry settings.
- **Level 4 Parameters:** analysis_depth, parameter_tuning_frequency, learning_rate.

### 6.2. Transaction Constraints

- **Solana Limits:** MAX_ALLOWED_COMPUTE (default: 1.4M units), MAX_ALLOWED_ACCOUNTS (default: 64 without ALTs), MAX_TRANSACTION_SIZE (default: 1232 bytes), MAX_INSTRUCTIONS (configurable based on complexity), MAX_CPI_DEPTH (fixed at 4 levels).
- **Safety Margins:** Configurable buffer percentages for each constraint.
- **Execution Parameters:** Preflight check options, commitment level requirements, timeout durations.
- **Constraint Models:** Configurable parameters for resource estimation models.

### 6.3. Account Lookup Table Management

- **ALT Sources:** Configuration for ALT discovery and selection.
- **ALT Selection:** Strategy for prioritizing which ALTs to use.
- **ALT Fallback:** Behavior when preferred ALTs are unavailable.

### 6.4. Network Adaptation

- **Priority Fee Model:** Parameters for dynamic priority fee calculation.
- **Congestion Response:** Configuration for behavior during different network congestion levels.
- **Timing Strategy:** Parameters for transaction submission timing.
- **MEV Mitigation:** Configuration for MEV protection strategies.

### 6.5. Protocol-Specific Settings

- **Protocol Weights:** Optional biasing factors for protocol selection.
- **Special Protocol Handling:** Configuration for protocol-specific behaviors.
- **Protocol Versioning:** Version compatibility settings.
- **Solver Selection:** Parameters governing when to use specialized solvers.

### 6.6. User Protection

- **Default Slippage:** System default slippage tolerance in basis points.
- **Slippage Limits:** Minimum and maximum allowed slippage settings.
- **Advanced Slippage:** Optional path-specific or token-specific slippage configurations.
- **Quality Thresholds:** Minimum acceptable solution quality for execution.

### 6.7. Token Handling

- **Special Token Registry:** Configuration for known tokens with non-standard behavior.
- **Fee Tokens:** Settings for tokens with transfer fees.
- **Rebasing Tokens:** Configuration for handling rebasing tokens.
- **Token Compliance:** Settings for token compliance checking.

### 6.8. Performance Tuning

- **Parallel Execution:** Thread pool sizes and work distribution parameters.
- **Cache Configuration:** Memory allocation and invalidation policies.
- **Optimization Method Selection:** Thresholds for choosing between optimization approaches.
- **Time Budgeting:** Allocation of computational time across algorithm phases.

## 7. Error Handling & Robustness

### 7.1. Error Categories & Responses

- **State Errors:** Missing, stale, or invalid DEX state data
    - *Response:* Retry with alternative data sources, fall back to more conservative estimates, or exclude affected venues.
- **Calculation Errors:** Numerical issues, precision errors, overflow/underflow
    - *Response:* Log details, apply defensive bounds, fall back to alternative calculation methods.
- **Constraint Violations:** Transaction size, account count, or compute exceeding limits
    - *Response:* Attempt simplification, reduce path complexity, or clearly report inability to execute.
- **Market Condition Changes:** Price divergence exceeding thresholds
    - *Response:* Abort with clear “requote needed” indication, never execute at significantly worse prices.
- **Network Errors:** Submission failures, timeouts, dropped transactions
    - *Response:* Apply configurable retry strategy with exponential backoff, track and report final status.
- **Execution Errors:** On-chain failures, reverts, compute exhaustion
    - *Response:* Capture detailed diagnostic information, analyze failure reasons, improve constraint models.
- **Optimization Errors:** Solver failures, convergence issues, or timeout in specialized optimizers
    - *Response:* Fall back to heuristic methods, log solver issues for analysis, ensure robustness of Quality Assessor.

### 7.2. Monitoring & Alerting

- **System Health:** Track cache freshness, calculation performance, execution success rates.
- **Market Conditions:** Monitor for unusual volatility, liquidity drops, or protocol pauses.
- **Network Status:** Track confirmation times, fee requirements, congestion indicators.
- **Error Patterns:** Aggregate error statistics to identify systemic issues.
- **Solution Quality:** Monitor optimality gaps and confidence metrics over time.

### 7.3. Circuit Breakers

- **Rate Limiting:** Restrict execution volume during extreme conditions.
- **Error Thresholds:** Pause specific functionality when error rates exceed thresholds.
- **Venue Exclusion:** Temporarily remove venues with suspicious behavior or high failure rates.
- **Gradual Recovery:** Implement controlled reintroduction after circuit breaker activation.
- **Solver Fallbacks:** Automatically revert to more reliable solvers when specialized ones fail.

### 7.4. Validation Strategy

- **Pre-Execution:** Verify account existence, token balances, and transaction simulation results.
- **Post-Execution:** Confirm token receipts, validate against expected outputs, track slippage accuracy.
- **Continuous Validation:** Regularly validate calculation accuracy against actual execution results.
- **Discrepancy Analysis:** Track patterns in estimation vs. actual performance to refine models.
- **Solution Quality Verification:** Validate solution quality metrics against actual outcomes.

## 8. Assumptions & Limitations

### 8.1. Algorithm Design

AHMO-NG is a **hybrid, heuristic optimization algorithm**. It incorporates provably optimal solvers for specific, tractable sub-problems where feasible within strict latency and transaction constraints. For the general, complex, non-convex problem, it relies on advanced, constraint-aware heuristics. Global optimality for all scenarios is **not guaranteed** due to computational complexity and real-time dynamics. The optional local search provides refinement, not necessarily global optimality. The quality_score offers an indicator of confidence based on the solution methodology, not a strict mathematical bound unless specific bounded approximation techniques are researched and implemented.

### 8.2. Implementation Dependencies

- Correctness relies critically on accurate DEX Protocol Interface implementations.
- Prediction quality depends on state cache freshness and accuracy.
- Performance depends on efficient implementation of core data structures and parallel execution.
- Optimal solver effectiveness depends on accurate mathematical modeling of protocol behavior.

### 8.3. Solana Network Constraints

- **Single Transaction Requirement:** Limits complexity and scale of routes:
    - Maximum 2-3 hops in practice due to CPI depth limits
    - Upper bound on number of venues that can be included
    - Potential limitations on maximum input size for certain paths
- **CPI Depth Limit:** Solana’s 4-level CPI depth constraint fundamentally limits route complexity.
- **Account Reference Limit:** Even with ALTs, there are practical limits to account references.
- **Compute Budget:** Complex calculations may hit compute limits despite optimization.
- **Transaction Size:** Limits the total amount of data that can be included in a single transaction.

### 8.4. Market Conditions

- State latency risk exists between calculation and execution, mitigated but not eliminated by final checks.
- Extreme volatility may increase requote rates and reduce execution success.
- MEV remains a risk, especially for large trades, despite enhanced mitigation measures.
- Mathematical guarantees may weaken in highly volatile market conditions.

### 8.5. Token Considerations

- Non-standard tokens (transfer fees, rebasing) may have reduced routing effectiveness.
- Tokens with unusual decimal places may require special handling.
- Compliance requirements may restrict available routes for certain tokens.
- Specialized token behavior may not be fully captured in mathematical models.

### 8.6. Protocol Specifics

- Batch auction DEXs introduce timing dependencies that affect execution and optimization.
- Some protocols may have idiosyncratic behaviors not fully captured in the abstraction.
- Protocol upgrades may require interface adaptations over time.
- Novel DEX mechanisms may not fit cleanly into existing optimization frameworks.

## 9. Practical Implementation Notes

## 10. Mathematical Foundation

### 10.1. Optimization Problem Formulation

The DEX aggregation problem can be formalized as a constrained non-linear optimization problem. Let:

- X be the input token
- Y be the output token
- Q be the total input quantity
- P = {p₁, p₂, …, pₙ} be the set of all viable paths between X and Y
- qᵢ be the portion of Q allocated to path pᵢ
- Oᵢ(qᵢ) be the output quantity of Y received when sending qᵢ of X through path pᵢ
- Cⱼ(P, q) be the consumption of resource j (compute units, accounts, etc.) when executing paths with allocations

The formal optimization problem is:

max∑ᵢ₌₁ⁿ Oᵢ(qᵢ)

Subject to constraints:
∑ᵢ₌₁ⁿ qᵢ = Q
Cⱼ(P, q) ≤ Bⱼ ∀j ∈ {1, 2, …, m}
qᵢ ≥ 0 ∀i ∈ {1, 2, …, n}

Where:
- Bⱼ represents the Solana-imposed limit on resource j
- m is the number of distinct constraint types (compute, accounts, size, CPI depth, etc.)

The key challenges that make this problem non-standard:

1. **Non-Linear Output Functions**: Each Oᵢ(qᵢ) is a non-linear function of the input amount qᵢ, due to price impact mechanics of DEXs.
2. **Non-Separable Constraints**: The resource consumption Cⱼ(P, q) is not always a simple sum of individual path consumptions, due to account deduplication and instruction merging.
3. **Heterogeneous Path Types**: Different DEX protocols have fundamentally different mathematical models governing their output functions Oᵢ(qᵢ).
4. **Dynamic Environment**: The state used to calculate Oᵢ(qᵢ) is continuously changing, requiring real-time adaptation.

### 10.2. Convex Optimization for AMM Splits

For the specific sub-problem of allocating input across multiple constant product AMM pools with the same token pair, we can prove the problem is convex and derive the optimal solution analytically.

For a constant product AMM with reserves (Rₓ, Rᵧ) and fee rate f, the output function for input q is:

O(q) = Rᵧ - (Rₓ · Rᵧ)/(Rₓ + q · (1-f))

The marginal rate (derivative) is:

dO/dq = (Rₓ · Rᵧ · (1-f))/((Rₓ + q · (1-f))²)

For n pools with the same pair, Karush-Kuhn-Tucker (KKT) conditions state that at optimality, the marginal rates must be equal across all pools with non-zero allocation:

dO₁/dq₁ = dO₂/dq₂ = … = dOₙ/dqₙ = λ

Where λ is the Lagrange multiplier associated with the constraint ∑qᵢ = Q.

This yields the system of equations:

(Rₓ,ᵢ · Rᵧ,ᵢ · (1-fᵢ))/((Rₓ,ᵢ + qᵢ · (1-fᵢ))²) = λ ∀i with qᵢ > 0

∑ᵢ₌₁ⁿ qᵢ = Q

Solving this system:

qᵢ = (1/(1-fᵢ)) · (√((Rₓ,ᵢ · Rᵧ,ᵢ · (1-fᵢ))/λ) - Rₓ,ᵢ)

Where λ is determined to satisfy the total quantity constraint.

This can be solved efficiently using optimization techniques such as binary search for λ or the Newton-Raphson method, with guaranteed convergence due to convexity.

### 10.3. Order Book Optimal Routing

For order book DEXs, we can prove that the optimal allocation strategy is to consume liquidity in strict price order, regardless of which order book it comes from.

**Theorem**: Given multiple order books for the same token pair, the optimal strategy for maximizing output is to execute against the best available price across all order books, then the second-best, and so on until the input quantity is exhausted.

**Proof**:

Let orders across all books be represented as pairs (pⱼ, vⱼ) where:
- pⱼ is the price of order j
- vⱼ is the maximum volume executable at this price (in terms of input token)

Sort all orders by price: p₁ ≤ p₂ ≤ … ≤ pₘ

Assume by contradiction that an optimal solution exists where order k is used before order j, where pₖ > pⱼ.

The output from using volume v from order j is Oⱼ = v · (1/pⱼ)
The output from using volume v from order k is Oₖ = v · (1/pₖ)

Since pₖ > pⱼ, we have 1/pₖ < 1/pⱼ, which means Oₖ < Oⱼ for the same input amount.

Therefore, by swapping the allocation to prioritize order j before order k, we increase the total output, contradicting the assumption of optimality.

This proves that consuming liquidity in strict price order is optimal. The algorithm implementation is a simple greedy approach that walks the combined order books in price order.

### 10.4. Bounded Approximation for Mixed Protocols

For heterogeneous DEX scenarios, we can establish theoretical bounds on the quality of solutions produced by AHMO-NG.

### 10.4.1. Error Bounds for AMM Approximations

For constant product AMMs, when using discrete allocation steps of size δ, the maximum error in output can be bounded by:

Error_max = ∑ᵢ₌₁ⁿ (max_q∈[qᵢ,qᵢ+δ] (d²Oᵢ/dq²) · (δ²/2))

Where d²Oᵢ/dq² is the second derivative of the output function, representing the rate of change of price impact.

For constant product AMMs, this second derivative is:

d²Oᵢ/dq² = -2 · Rₓ,ᵢ · Rᵧ,ᵢ · (1-fᵢ)²/(Rₓ,ᵢ + q · (1-fᵢ))³

Since this is always negative (confirming the convexity of the problem), and its absolute value decreases as q increases, the maximum value occurs at q=0:

max_q≥0 |d²Oᵢ/dq²| = 2 · Rₓ,ᵢ · Rᵧ,ᵢ · (1-fᵢ)²/Rₓ,ᵢ³

This allows us to establish a worst-case bound on the approximation error for a given allocation step size.

### 10.4.2. ε-Approximation for Mixed Protocol Routing

For the general case with mixed protocols, we can apply an ε-approximation scheme:

1. Discretize the allocation space into steps of size δ = (ε · Q)/n
2. For each path i, compute outputs for allocations in {0, δ, 2δ, …, Q}
3. Use dynamic programming to find the allocation that maximizes output while satisfying constraints

This approach guarantees a solution within (1-ε) of the theoretical optimum, with computational complexity O(n · (n/ε)).

While this is too computationally expensive for real-time execution in most cases, it provides a theoretical framework for bounding the quality of AHMO-NG’s solutions during post-execution analysis.

### 10.5. Enhanced Marginal Rate Analysis

The core heuristic of AHMO-NG is based on an enhanced marginal rate analysis that improves upon simple greedy allocation.

### 10.5.1. Basic Marginal Rate Allocation

The marginal rate for path i with current allocation qᵢ is defined as:

MRᵢ(qᵢ) = dOᵢ/dq(qᵢ)

In a simple greedy approach, we allocate to the path with the highest marginal rate at each step.

### 10.5.2. Enhanced Approach with Lookahead

AHMO-NG employs a lookahead mechanism that estimates future marginal rates after an allocation step:

FMRᵢ(qᵢ, Δ) = (Oᵢ(qᵢ + Δ) - Oᵢ(qᵢ))/Δ

This allows the algorithm to anticipate rate degradation and make better allocation decisions by considering:

Scoreᵢ(qᵢ) = α · MRᵢ(qᵢ) + (1-α) · FMRᵢ(qᵢ, Δ)

Where α ∈ [0,1] is a configurable weight parameter.

### 10.5.3. Path Interdependency Analysis

For multi-hop paths with shared segments, the marginal rates become interdependent. If paths i and j share a hop through venue v, allocating to path i affects the marginal rate of path j.

AHMO-NG addresses this by maintaining a dependency graph G = (V, E) where:
- Vertices V represent paths
- Edges E connect paths that share venues

When allocating to path i, the algorithm updates marginal rates for all paths j where (i,j) ∈ E.

### 10.5.4. Dynamic Step Size Determination

The algorithm dynamically adjusts allocation step sizes based on price impact curve analysis:

```
function determine_dynamic_step(path, remaining_Q, price_impact_data):
    // Get current marginal rate
    current_MR = path.getCurrentMarginalRate()

    // Calculate maximum step where average rate remains close to current
    // For constant product AMMs, this can be derived analytically
    if path.type == "CPMM":
        // Extract reserves and fee
        Rx = path.reserves_x
        Ry = path.reserves_y
        fee = path.fee

        // Calculate step where average rate is X% of current rate (configurable)
        target_ratio = 0.9  // Target 90% of current rate
        step = (Rx * (1/sqrt(target_ratio) - 1)) / (1-fee)

        // Cap the step
        step = min(step, remaining_Q, path.capacity)
        return max(step, epsilon_amount)

    // For other protocols, use price impact curve data
    else:
        // Find largest step where average slope >= target_percent * current_MR
        target_percent = 0.9
        target_minimum_rate = current_MR * target_percent

        // Binary search for step size that maintains target rate
        low = epsilon_amount
        high = min(remaining_Q, path.capacity)

        while high - low > epsilon_amount:
            mid = (low + high) / 2
            avg_rate = (price_impact_data.getOutput(mid) - price_impact_data.getOutput(0)) / mid

            if avg_rate >= target_minimum_rate:
                low = mid
            else:
                high = mid

        return low
```

### 10.5.5. Price Impact Curve-Informed Allocation

The enhanced heuristic uses price impact curve data to make more informed allocation decisions:

```
function calculate_priority_score(current_MR, lookahead_MR, curve_data):
    // Base prioritization from current and lookahead rates
    base_score = ALPHA * current_MR + (1-ALPHA) * lookahead_MR

    // Apply curve shape analysis
    curve_acceleration = curve_data.getAcceleration()
    decay_factor = 1.0

    // Penalize paths with rapidly accelerating price impact
    if curve_acceleration > ACCELERATION_THRESHOLD:
        decay_factor = exp(-BETA * curve_acceleration)

    return base_score * decay_factor
```

### 10.6. Solution Quality Metrics

AHMO-NG quantifies solution quality using several mathematically rigorous metrics:

### 10.6.1. Optimality Ratio

For solutions where a theoretical bound can be established:

R_opt = O_AHMO-NG/O_theoretical

Where O_theoretical is determined through offline, exhaustive computation or mathematical bounds.

### 10.6.2. Confidence Intervals

For statistical confidence in solution quality:

CI_95% = [R_opt - 1.96 · σ/√n, R_opt + 1.96 · σ/√n]

Where σ is the standard deviation of optimality ratios observed in similar historical scenarios, and n is the sample size.

### 10.6.3. Sensitivity Analysis

The stability of a solution is measured by its sensitivity to small input perturbations:

S_Δ = |O(Q + Δ) - O(Q) - MR(Q) · Δ|/O(Q)

A smaller value indicates a more stable solution that is likely closer to the global optimum.

### 10.6.4. Constraint Utilization

The efficiency of constraint utilization is measured by:

U_j = C_j(P, q)/B_j

Where U_j close to 1 indicates efficient use of available resources without violation.

## 11. Appendices

### 11.1. Protocol-Specific Mathematical Models

### 11.1.1. Constant Product AMM (CPMM)

The core formula for a CPMM with reserves (Rₓ, Rᵧ) and fee rate f is:

Rₓ · Rᵧ = k (constant)

For input amount qₓ of token X, the output amount qᵧ of token Y is:

qᵧ = Rᵧ - k/(Rₓ + qₓ · (1-f))

The marginal rate function is:

MR(qₓ) = dqᵧ/dqₓ = k · (1-f)/(Rₓ + qₓ · (1-f))²

The capacity function, representing the input amount needed to cause a specific percentage slippage s, is:

Cap(s) = (Rₓ · √(1+s) - Rₓ)/(1-f)

### 11.1.2. Concentrated Liquidity (CLMM)

For a CLMM with current price P and liquidity L distributed across ticks, the effective reserves at price P are:

Rₓ = L · (1/√P_a - 1/√P)
Rᵧ = L · (√P - √P_a)

Where P_a is the lower bound of the current active range.

The output for input qₓ requires solving for the new price P’ such that:

qₓ = L · (1/√P_a - 1/√P’) - Rₓ

This yields:

P’ = (L/√P_a - Rₓ - qₓ)⁻²

The output is then:

qᵧ = L · (√P’ - √P_a) - Rᵧ

If P’ crosses tick boundaries, separate calculations must be performed for each tick range with its associated liquidity.

The marginal rate function is complex due to tick crossings, but can be approximated as:

MR(qₓ) = {
L · (1-f)/(L/√P_a - Rₓ - qₓ)³ if no tick crossing
Piecewise function if tick crossing
}

### 11.1.3. StableSwap

The StableSwap invariant for a two-token pool with balances (x, y) and amplification parameter A is:

A · (x + y) + x · y = A · D + D³/(4 · x · y)

Where D is approximately the total token value.

Solving for the output requires iterative methods (Newton-Raphson):

1. Calculate current invariant D
2. For input qₓ, set x’ = x + qₓ
3. Solve for y’ that maintains invariant D
4. Output qᵧ = y - y’

The marginal rate is calculated numerically using finite differences due to the complexity of the analytical derivative.

### 11.1.4. Order Book

For an order book with bid orders {(p₁, v₁), (p₂, v₂), …} sorted by price (p₁ > p₂ > …), the output function for input q is:

qᵧ(qₓ) = ∑ᵢ₌₁ᵐ vᵢ · (1/pᵢ)

Where m is the smallest value such that ∑ᵢ₌₁ᵐ vᵢ ≥ qₓ.

The marginal rate at any point is:

MR(qₓ) = 1/pⱼ

Where j is the current order being filled.

### 11.2. Transaction Constraint Models

### 11.2.1. Compute Unit Estimation

Empirical modeling of compute units follows the formula:

C_compute = ∑ᵢ₌₁ⁿ (Base_i + ∑ⱼ₌₁ᵐᵢ Hop_i,j)

Where:
- Base_i is the base compute cost for path i
- Hop_i,j is the compute cost for hop j in path i

For each protocol type, Hop_i,j is modeled as:

Hop_i,j = a_type + b_type · q_i,j + c_type · features_i,j

Where:
- a_type, b_type, c_type are protocol-specific coefficients
- q_i,j is the input amount for this hop
- features_i,j represents protocol-specific complexity factors

### 11.2.2. Account Reference Optimization

The account deduplication algorithm minimizes the total account references by solving:

min_(A_ref, A_ALT) |A_ref| + ∑_t∈ALTs y_t

Subject to:
∀a ∈ A_req: a ∈ A_ref ∪ (∪_t∈ALTs A_ALT,t · y_t)
|A_ref| ≤ MAX_ACCOUNTS
y_t ∈ {0, 1}

Where:
- A_req is the set of all required accounts
- A_ref is the set of directly referenced accounts
- A_ALT,t is the set of accounts in ALT t
- y_t indicates whether ALT t is used

This is solved using a greedy algorithm that prioritizes frequently referenced accounts for ALT inclusion.

### 11.2.3. CPI Depth Management

CPI depth requirements are modeled as a directed graph where:
- Nodes represent instructions
- Edges represent CPI relationships
- Edge weights represent CPI levels

The maximum CPI depth is calculated as:

D_CPI = max_p∈Paths Length(p) - 1

Where Paths is the set of all instruction paths in the transaction graph.

The algorithm ensures D_CPI ≤ MAX_CPI_DEPTH (4 for Solana).

### 11.2.4. Transaction Size Calculation

Transaction size is estimated using:

Size_tx = Header + ∑ᵢ₌₁ⁿ Instr_i + ∑ⱼ₌₁ᵐ Acct_j + ∑ₖ₌₁ˡ Sig_k

Where:
- Header is the fixed transaction header size (1+8+32+64 bytes)
- Instr_i is the size of instruction i (1+1+compact-u16+data)
- Acct_j is the account metadata size (1+32+1 bytes)
- Sig_k is the signature size (64 bytes)

### 11.3. Numerical Precision Analysis

### 11.3.1. Error Propagation in Multi-hop Routes

For a path with h hops, the error propagation follows:

ε_total = ∑ᵢ₌₁ʰ εᵢ · ∏ⱼ₌ᵢ₊₁ʰ ∂q_j/∂q_j-1

Where:
- εᵢ is the error in hop i
- ∂q_j/∂q_j-1 is the sensitivity of hop j to its input

For conservative error bounds, we use:

ε_total ≤ ∑ᵢ₌₁ʰ εᵢ · ∏ⱼ₌ᵢ₊₁ʰ max(|∂q_j/∂q_j-1|)

### 11.3.2. Catastrophic Cancellation Mitigation

For operations susceptible to catastrophic cancellation, we implement alternative formulations:

**Example: CPMM Output Calculation**

Instead of:
q_y = R_y - k/(R_x + q_x · (1-f))

Use:
q_y = (R_y · q_x · (1-f))/(R_x + q_x · (1-f))

**Example: Price Impact Calculation**

Instead of:
I = 1 - P_spot/P_effective

Use:
I = (P_effective - P_spot)/P_effective

### 11.3.3. Fixed-Point Arithmetic for U128

For U128 fixed-point operations, we use a standard format with 64 bits for the integer part and 64 bits for the fractional part:

**Multiplication:**

function multiply(a, b):
// Perform 256-bit multiplication
hi, lo = full_multiply_u128(a, b)

```
// Extract middle 128 bits (shift right by 64)
return (hi << 64) | (lo >> 64)
```

**Division:**

function divide(a, b):
// Scale a up by 2^64
a_scaled = a << 64

```
// Perform division
return a_scaled / b
```

### 11.3.4. Token Decimal Normalization

For tokens with different decimal places d_X and d_Y, scale amounts using:

q_Y^normalized = q_Y · 10^(d_max - d_Y)

Where d_max = max(d_X, d_Y) is the maximum decimal places of any token in the system.

For multi-hop paths, maintain intermediate values at the highest precision:

Scale_i,j = 10^(d_j - d_i)

Apply scaling at the appropriate points:

q_j = truncate(output_i/Scale_i,j)

### 11.5.2. Configuration Parameters

**Primary Tuning Parameters:**

| Parameter | Default | Range | Description |
| --- | --- | --- | --- |
| MAX_PATH_HOPS | 3 | 1-4 | Maximum path length |
| MAX_CANDIDATE_PATHS | 50 | 10-100 | Maximum paths in Level 1 |
| ALLOCATION_EPSILON | 0.0001 | 0.00001-0.001 | Minimum allocation step |
| LOCAL_SEARCH_ITERATIONS | 20 | 0-100 | Number of refinement steps |
| COMPUTE_SAFETY_MARGIN | 0.05 | 0.01-0.2 | Safety buffer on compute limit |
| FINAL_CHECK_THRESHOLD | 0.01 | 0.001-0.05 | Maximum price divergence |
| PARALLEL_WORKERS | 4 | 1-16 | Thread pool size |
