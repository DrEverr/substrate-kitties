# Steps

After each step, add tests to test current changes.

## Storage Basic

### 1. Storage value

- [ ] Create [storage value](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageValue.html)
for kitties count (u32)

### 2. Kitty counter

- [ ] Add logic to increment kitties count in `mint()` call

### 3. Safety

- [ ] Make `mint()` call math safe with `checked_*` API
- [ ] Introduce a new error for overloding kitties counter

### 4. Value Query

- [ ] Update kitty counter - add `QueryKind`
- [ ] Adjust code for above changes

### 5. Storage Maps

- [ ] Create [storage map](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html)
with key `[u8;32]` and value `()` (for now)

### 6. Kitties map

- [ ] Add logic to insert kitty to map in `mint()` call
(you will need to add second parameter `dna`)
- [ ] Create and pass the `dna` (`[u8;32]`) to `mint()` call

### 7. Duplicate kitty check

- [ ] `ensure` the kitty map doesn't contain a `dna` key
- [ ] Introduce new error for already existing kitty

## Storing Objects

### 1. Kitty struct

- [ ] Create a struct for kitty with fields `dna`and `owner`
- [ ] Make this struct generic over `T`
- [ ] Create default kitty in tests

### 2. Storing struct

- [ ] Add required traits to kitty struct to allow it to be stored in Kitties map
- [ ] Skip type params (hint: `#[scale_info(skip_type_params(T))]`)
- [ ] Update Kittes map to store our kitties struct
- [ ] Update `mint()` to create and store our kitty

### 3. Generate unique DNA

- [ ] Create a function generating sudo-random dna
  - [ ] construct a payload with `parent hash`, `block number`,
  `extrinsic index`, `amout of already minted kitties`
  - [ ] use `BlakeTwo256` to calculate `hash` of this payload
  - [ ] return this `hash` as `[u8; 32]`
- [ ] Update previously generated `dna` with our new function

### 4. Track owned kitties

- [ ] Create a storage map with key `account id` and value `Vec<dna>`
- [ ] Update `mint()` to insert owned kitty to new map

### 5. Bounded vector

- [ ] Update storage map value of owned kitties to
[BoundedVec](https://docs.rs/frame-support/38.0.0/frame_support/struct.BoundedVec.html)
- [ ] Update `mint()` according to new value
- [ ] Add error for overflowing bounded vec

## Marketplace
