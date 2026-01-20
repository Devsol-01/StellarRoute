# StellarRoute Task Plan

**Current Phase:** M1 - Phase 1.2: SDEX Indexer Development  
**Status:** 🔄 In Progress  
**Started:** Phase 1.2 implementation

---

## Goal

Build Phase 1.1 foundation for StellarRoute: Set up development environment, project structure, and tooling for SDEX orderbook indexing project.

---

## Phases

### Phase 1.1: Environment & Project Setup
**Status:** ✅ Complete (except manual Rust/Soroban installation)  
**Started:** Initial

**Tasks:**
- [ ] Set up Rust development environment (manual installation - see docs/development/SETUP.md)
- [ ] Install Soroban CLI (instructions in docs/development/SETUP.md)
- [x] Initialize project structure with workspace layout
- [x] Configure CI/CD pipelines (GitHub Actions)
- [x] Set up local development environment (Docker Compose for Postgres)
- [x] Create project documentation structure

**Deliverables:**
- Working Rust development environment
- Soroban CLI installed and configured
- Project workspace structure initialized
- CI/CD pipeline configured
- Local Postgres database via Docker Compose
- Documentation structure in place

---

## Decisions Made

- TBD (will update as we make decisions)

---

## Errors Encountered

| Error | Attempt | Resolution |
|-------|---------|------------|
| SSL connection error during Rust installation | 1 | Documented manual installation process in docs/development/SETUP.md |
| Homebrew tap `stellar/soroban/soroban` not found | 1 | Updated SETUP.md with alternative installation methods (cargo install, installer script, manual binary) |

---

## Files Created/Modified

### Planning Files
- `task_plan.md` - This file (initial creation)
- `findings.md` - Research notes (initial creation)
- `progress.md` - Progress log (initial creation)

### Project Structure
- `Cargo.toml` - Workspace configuration
- `crates/indexer/Cargo.toml` - Indexer crate
- `crates/api/Cargo.toml` - API server crate
- `crates/routing/Cargo.toml` - Routing engine crate
- `crates/contracts/Cargo.toml` - Smart contracts crate
- `crates/sdk-rust/Cargo.toml` - Rust SDK crate

### Source Code
- `crates/indexer/src/lib.rs` - Indexer main module
- `crates/indexer/src/error.rs` - Indexer error types
- `crates/indexer/src/sdex.rs` - SDEX indexing module
- `crates/indexer/src/soroban.rs` - Soroban indexing module
- `crates/api/src/lib.rs` - API main module
- `crates/api/src/error.rs` - API error types
- `crates/api/src/handlers.rs` - API handlers
- `crates/api/src/server.rs` - API server setup
- `crates/routing/src/lib.rs` - Routing engine main module
- `crates/routing/src/error.rs` - Routing error types
- `crates/routing/src/pathfinder.rs` - Pathfinding algorithms
- `crates/contracts/src/lib.rs` - Smart contracts
- `crates/sdk-rust/src/lib.rs` - Rust SDK main module
- `crates/sdk-rust/src/client.rs` - SDK client
- `crates/sdk-rust/src/error.rs` - SDK error types
- `crates/sdk-rust/src/types.rs` - SDK types

### Configuration & Infrastructure
- `docker-compose.yml` - Docker services (Postgres, Redis)
- `.github/workflows/ci.yml` - CI/CD pipeline
- `.gitignore` - Git ignore rules
- `scripts/setup.sh` - Setup script

### Documentation
- `docs/README.md` - Documentation index
- `docs/development/SETUP.md` - Development setup guide
- `docs/architecture/.gitkeep` - Architecture docs placeholder
- `docs/api/.gitkeep` - API docs placeholder
- `docs/contracts/.gitkeep` - Contract docs placeholder
- `docs/deployment/.gitkeep` - Deployment docs placeholder

### Phase 1.2: Indexer Implementation
- `crates/indexer/migrations/0001_init.sql` - Database schema
- `crates/indexer/src/config/mod.rs` - Configuration management
- `crates/indexer/src/models/asset.rs` - Asset model
- `crates/indexer/src/models/horizon.rs` - Horizon API response types
- `crates/indexer/src/models/offer.rs` - Offer model
- `crates/indexer/src/models/mod.rs` - Models module
- `crates/indexer/src/horizon/mod.rs` - Horizon module
- `crates/indexer/src/horizon/client.rs` - Horizon API client
- `crates/indexer/src/db/mod.rs` - Database module
- `crates/indexer/src/db/connection.rs` - Database connection and migrations
- `crates/indexer/src/db/migrations.rs` - Migration utilities
- `crates/indexer/src/sdex.rs` - SDEX indexer service (updated with full implementation)
- `crates/indexer/src/bin/stellarroute-indexer.rs` - Main indexer binary

---

### Phase 1.2: SDEX Indexer Development
**Status:** 🔄 In Progress  
**Started:** Phase 1.2 implementation

**Tasks:**
- [x] Research Stellar Horizon API endpoints for orderbook data (confirmed `/offers` endpoint)
- [x] Design database schema for orderbook storage (Postgres)
  - Offers table (price, amount, timestamp, asset pairs)
  - Asset metadata table
- [x] Implement Horizon API client integration (using reqwest directly)
- [x] Build orderbook indexer service foundation
  - Horizon client with `/offers` endpoint
  - Database connection and migration system
  - Asset and offer models
  - Basic indexing loop
- [ ] Add error handling and retry logic (basic error handling done, retry logic pending)
- [ ] Implement data validation and sanitization (basic validation done)
- [ ] Add orderbook snapshot endpoint support (pending - endpoint needs verification)
- [ ] Add streaming/real-time updates (pending - polling implemented first)

**Deliverables:**
- ✅ Database schema and migrations
- ✅ Horizon API client
- ✅ Asset and Offer models
- ✅ Indexer service with polling loop
- ✅ Main binary executable
- ⏳ Orderbook snapshot support (pending endpoint verification)
- ⏳ Real-time streaming (pending)

---

## Next Steps

1. Test the indexer binary with local Postgres
2. Verify Horizon API connectivity
3. Add retry logic and better error handling
4. Research and implement orderbook snapshot endpoint
5. Add streaming support for real-time updates
6. Begin Phase 1.3: Database Layer optimizations
