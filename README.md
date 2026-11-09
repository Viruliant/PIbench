# PIbench

Test various Pi digit generating libraries for speed.

## Building

```bash
nix build
```

The result will be in the `result` symlink:
- `./result/bin/PIbench` - The compiled binary

## Development Workflow

1. Clone the repository:
   ```bash
   git clone https://github.com/Viruliant/PIbench
   cd ./PIbench
   ```

2. Make your changes to the code

3. Build and test:

```bash
./Lock2LocalPkgs.sh # optionally update locks to match your system
./upd8cargoV2.sh    # update cargo
nix build
nix run
```

## Features

- Computes Pi using the Chudnovsky series algorithm
- Uses the `dashu` library for arbitrary precision arithmetic
- Command line argument to specify number of decimal digits (default: 100)
- Display output in any base from 2 to 37:
  - Bases 2-10: standard digits (0-9)
  - Bases 11-36: digits + letters (A-Z)
  - Base 37+: values beyond Z shown as [XX] notation
- Shows execution time for the calculation
- Hex dump of internal representation with `--hex-dump` flag

## Usage with Arguments

```bash
# Default (100 digits, base 10)
./result/bin/PIbench

# Specify custom number of digits
./result/bin/PIbench 50
./result/bin/PIbench 200
./result/bin/PIbench 1000

# Change output base (-b flag)
./result/bin/PIbench -b 16 50      # Hexadecimal (base 16)
./result/bin/PIbench -b 2 100       # Binary (base 2)
./result/bin/PIbench -b 37 50       # Base 37 with [XX] notation
./result/bin/PIbench -b16 50        # Compact format: -b16

# Show hex dump of internal representation (--hex-dump flag)
./result/bin/PIbench --hex-dump 50
./result/bin/PIbench -b 16 --hex-dump 30
```

- No arguments: uses default of 100 decimal digits in base 10
- With digit argument: calculates Pi to the specified number of decimal digits
- `-b <base>` or `-b<base>`: changes output base (2-37)
- `--hex-dump`: shows internal representation as hex dump
- Invalid input will show an error message and usage instructions

## Example Output

### Default (base 10)
```bash
[owner@nixos:~/sync/PIbench]$ ./Lock2LocalPkgs.sh
./upd8cargoV2.sh
nix build
Updating flake to nixpkgs revision: cd4ea8d57de16f4810bc5f14207bac45fbf19f67
warning: Git tree '/home/owner/sync/PIbench' is dirty
warning: updating lock file "/home/owner/sync/PIbench/flake.lock":
• Added input 'flake-parts':
    'github:hercules-ci/flake-parts/ea2630b7470978622c7689a1ff75d2b389175399?narHash=sha256-V1h85lUaJBQoHFrB2vpXZOE0Ju1MYSnIpsehoVZYlDo%3D' (2026-11-01)
• Added input 'flake-parts/nixpkgs-lib':
    'github:nix-community/nixpkgs.lib/7851ba61fd862b277cb6a9cef37394bc18bb94e4?narHash=sha256-ThkfL3D/FhCiZFc9%2B3BtxwuMmpuK%2B/bGtXCSo1%2ByVew%3D' (2026-11-01)
• Added input 'nixpkgs':
    'github:NixOS/nixpkgs/cd4ea8d57de16f4810bc5f14207bac45fbf19f67?narHash=sha256-vhe2jyvRCgufWPqDuDq1S3t3ypE%2Bka46IFutV02xKso%3D' (2026-10-20)
    Updating crates.io index
     Locking 16 packages to latest compatible versions
updated cargoHash = sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
updated cargoHash = sha256-QPlBDn4O6hSDehj72e2CD4LhGJBP/9ezQlvn0JMv7rc=
done.
warning: Git tree '/home/owner/sync/PIbench' is dirty

[owner@nixos:~/sync/PIbench]$ ./result/bin/PIbench 50
./result/bin/PIbench -b 2 50       # Binary (base 2)
Pi × 10^50 (binary splitting):
314159265358979323846264338327950288419716939937510

Binary splitting: 0.000516s

Factorial version:
314159265358979323846264338327950288419716939937510
Factorial version: 0.000038s

Both implementations agree.
355/113 approximation (base 10)
314159292035398230088495575221238938053097345132743
Correct leading digits: 7
First mismatch at digit 8: expected '6', got '9'
Pi × 10^50 (binary splitting):
314159265358979323846264338327950288419716939937510

Binary splitting: 0.000017s

Factorial version:
110101101111010011011100101011001101101111100110100000000000010011010111011001100110101001001101000000011101111110100011001100100001111000110011111001000111011011100110
Factorial version: 0.000033s

Both implementations agree.
355/113 approximation (base 2)
1100100100001111110110111100000010010000111111011011
Correct leading digits: 3
First mismatch at digit 4: expected '1', got '0'
```