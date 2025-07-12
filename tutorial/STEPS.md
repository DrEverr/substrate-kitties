# Steps

After each step, add tests to test current changes.

## Storage Basic

### 1. Storage value

[] Create [storage value](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageValue.html)
for kitties count (u32)

### 2. Kitty counter

[] Add logic to increment kitties count in `mint()` call

### 3. Safety

[] Make `mint()` call math safe with `checked_*` API
[] Introduce a new error for overloding kitties counter

### 4. Value Query

[] Update kitty counter - add `QueryKind`
[] Adjust code for above changes

### 5. Storage Maps

[] Create [storage map](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html)
with key `[u8;32]` and value `()` (for now)

### 6. Kitties map

[] Add logic to insert kitty to map in `mint()` call (you will need to add second parameter `dna`)
[] Create and pass the `dna` (`[u8;32]`) to `mint()` call

### 7. Duplicate kitty check

[] `ensure` the kitty map doesn't contain a `dna` key
[] Introduce new error for already existing kitty

## Storing Objects

## Marketplace
