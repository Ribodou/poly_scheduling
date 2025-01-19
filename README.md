# Poly scheduler

**You thought dating was hard? It's actually $NP$-hard!**

> *Smarts can help this situation untangle...*\
> *So professors, teach me the math of love triangles!*

Poly scheduler is a program that lets you plan periodic dates between members of a group. It's time to ditch your google calendars.

## What does the program optimize?
Assumptions:
- Each relation involves two persons.
- Each person can only have one date per day.
- Each relation has a weight.

The weight of a relationship indicates how much each person in said relationship misses each other when they don't date. 

The weighted wait is defined by (`time between two consecutive dates` \* `weight of relationship`). The program will minimize the largest weighted wait.

## Install
Installing `poly_scheduling` will install [`clingo`](https://github.com/potassco/clingo), so the first build might take some time.
```bash
git clone https://github.com/Ribodou/poly_scheduling
cd poly_scheduling
cargo build --release
```
## Run
```bash
cargo run --release
```

## Filling your own data
Each person should choose a unique short name. Then, replace the names in `data.yaml`. For each person, write the short unique name, followed by ":" and the full name. Multiple individuals can share the same full name, but the short name should be unique.

Then, fill in your relationships: the two short names, then the weight of the relationship.


## License

This program is as open as relationships should be: everyone is welcome to (git) commit, but please respect boundaries (e.g. mind the AGPL license).