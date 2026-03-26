# Health-chain-stellar

HealthDonor Protocol is an open-source platform built on Stellar Soroban smart contracts that enables transparent health donations, secure fund escrow, and immutable healthcare supply chain tracking.

The protocol is designed for blood donations, medical supplies, vaccines, and healthcare funding, ensuring that every donation is:

Traceable

Auditable

Released only when real-world conditions are met

🎯 Problem Statement

Healthcare donation systems today suffer from:

❌ Lack of transparency

❌ Centralized control and trust issues

❌ Poor donor visibility into impact

❌ Limited auditability of supply chains

Donors often cannot verify:

Where funds go

Whether supplies reach recipients

If medical standards are followed

✅ Solution

HealthDonor Protocol leverages Stellar + Soroban smart contracts to provide:

🔐 On-chain escrow for medical donations

📊 Donor impact tracking

🧾 Immutable healthcare supply chain events

🏥 Verified healthcare actor registry

🕒 Time-locked fund releases

🔒 Privacy-preserving donor identifiers

All critical actions are enforced by smart contracts, reducing fraud and manual intervention.

## Getting Started

### For Contributors

```bash
# Clone repository
git clone https://github.com/Christopherdominic/Health-chain-stellar.git
cd Health-chain-stellar

# Setup pre-commit hooks and dependencies
./scripts/setup-hooks.sh

# Start local development environment
docker-compose up -d

# Start backend
cd backend
npm run start:dev
```

See [PRE_COMMIT_HOOKS.md](./PRE_COMMIT_HOOKS.md) for hook configuration and [DOCKER_SETUP.md](./DOCKER_SETUP.md) for detailed setup instructions.
