# rust-prediction

A rewriting of the [LINFO1104 projet of year 2022-2023](https://github.com/beMang/LINFO1104_Projet) but in rust, a much faster and mordern language than oz.
The projet is overall really faster and can analyse more text for a better prediction.

## How to lanch
* Simply install rust with cargo (see documentation of rust)
* Launch `cargo run`
* The programme should launch

## Implementatoins
* BTreeMap for the data structure for the prediction [(a interesting article on this std rust structure)](https://cglab.ca/~abeinges/blah/rust-btree-case/)
* egui and eframe for the gui
* Serde for serializing and deserializing the configuration structure (being implemented)
