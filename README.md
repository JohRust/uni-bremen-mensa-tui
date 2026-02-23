# Uni-Bremen-Mensa-TUI
Decide what you want to eat on campus in Bremen - with your terminal!

# Installation
Clone or download the repository.
```sh
git clone git@github.com:JohRust/uni-bremen-mensa-tui.git
cd uni-bremen-mensa-tui
```
 Build the binary
```sh
cargo build
```
Then do one of these three options:
 - Option 1 (recommended): run `cargo install --path .` to install `uni-bremen-mensa-tui` to the default install folder (Usually `~/.cargo/bin`)
 - Option 2: Run `cargo build` and manually put the binary `target/debug/uni-bremen-mensa-tui` somewhere on your PATH, e.g. `/usr/bin`

To make access easier, create an alias in your `~/.bashrc` (or `~/.zshrc`, etc.) and add the price category (`-p 0` for student, `-p 1` employee, `-p 2` external)
```sh
alias essen="uni-bremen-mensa-tui -p 2"
```
After that you can run "essen" (you can of course give the alias whatever name you prefer)
```
> essen
Today is a good day to get fat on campus!

=== Mensa ===
Ausgabe 1
   Kabeljau-Tails Rote Bete-Panade, Kartoffelsalat, Aioli - 6.05
Ausgabe 2
   Veganes Chili, Orangen-Reis, Avocado-Dip - 5.00 🌱
Ausgabe 3
   Käsespätzle, Röstzwiebeln, Salatmix, Chefkoch-Dressing - 6.20 🥛🥚
Saucen/Dips
   Sauce Bernaise - 1.40 🥛🥚
   "All Arrabiata" - 0.70 🌱
KombinierBar
   Rote Beete Puffer - 2.80 🌱
   Hähnchenschenkel - 3.65
   Grillkäse - 2.80 🥛🥚
   Kürbisgemüse - 1.55 🌱
   Criss Cuts - 2.05 🌱
   Spätzle - 1.55 🥛🥚
   Orangen-Reis - 1.55 🌱
   Bunte Möhren - 1.55 🌱
   Brokkoli - 1.55 🌱
Dessert
   Panna Cotta, Schlagcreme, Mango - 2.25 🌱
   Quarkspeise, Erdbeeren - 1.70 🥛🥚
   Mandelpudding, Sahnetupfer, Himbeeren - 2.25 🥛🥚
   Creme Tiramisu, Topping Rhabarber, Sahnetupfer - 2.55
   Zitronenpudding, Topping Karamell - 2.25 🌱
   Melone - 2.55 🌱
   Mousse au Chocolat, Sahnetupfer - 2.55

=== GW2 ===
Pizza
   Türkische Pizza, Veganer Tzatziki - 5.50 🌱
   Türkische Pizza, Tzatziki - 5.50 🥛🥚
Snacks
   Veganer Rollo nach Tex Mex-Art - 4.90 🌱
Bowl
   One Pot: Hähnchen mit Reisnudeln - 6.75

```
Add `-d <#days>` to see the menu for in `#days` days.

# Current Features
Currently it only lists the food in the Central Mensa and GW2. I might add other locations later. Feel free to extend it yourself and send me a pull request!

# One more thing
Please be mindful when using this application and the requests it sends to the KQL database. Sending many invalid requests or accidentally flooding the server with requests by putting them in a loop could annoy the maintainers of the database and make them restrict public access.
