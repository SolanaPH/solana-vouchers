
<div align="center">
	<img width="20%" src="../icons/large.png" alt="Vouchers logo">
	<h3>Vouchers</h3>
	<p>Show them your love by giving them Solana</p>
	<hr />
</div>

### Table of Contents

1. Introduction

2. Vouchers Program

3. Vouchers Client

4. Security Considerations

# Introduction

Vouchers is a **fullstack Solana DApp** that aims to allow users to give / reward people Solana, without the immediate need for a wallet.

# Vouchers Program

### Constraints

**Expiration** - all vouchers not redeemed before expiration date may no longer be claimed. All expired vouchers also give voucher creator to revoke funds and withdraw lamports from expired voucher.

This addresses the following issues:
1. **Stolen**, **discarded**, **lost** vouchers can be recovered via the creator's dashboard if needed.

2. **Limits the capability** of the voucher creator to abuse its control over the voucher funds by ensuring that total contral can only be done beyon an agreed expiration date.

### Functions
```rust
pub fn create_batch(); // Create one or more vouchers

pub fn redeem_one(); // Redeem ONE voucher

pub fn revoke_batch(); // Revoke funds from one or more vouchers
```

### fn create_batch()
```rust
// Create one or more vouchers at the same price
pub fn create_batch() {
	require(balance is enough)
}
```

### fn redeem_one()
```rust
// Redeem a ONE voucher
pub fn redeem_one() {
	require(not expired)
}
```

### fn revoke_batch()
```rust
// Revoke access and funding for one or more vouchers at the same time
pub fn revoke_batch() {
	require(all vouchers to be revoked must be expired)
}
```

# Vouchers Client

## Creator
### Creation UI
### Revoke UI
### Tracking / Monitoring UI

## Holder
### Redeem UI
