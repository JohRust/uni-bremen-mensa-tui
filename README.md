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
Then put the binary `target/debug/uni-bremen-mensa-tui` somewhere on your PATH. You can simply copy it to `/usr/bin` or just create an alias in your `~/.bashrc` (or `~/.zshrc`, etc.) like this:
```sh
alias essen=<PATH_TO_REPOSITORY>/target/debug/uni-bremen-mensa-tui
```
After that you can run "essen" (you can of course give the alias whatever name you prefer)
```
> essen
Today is a good day to get fat on campus:
Uni Mensa
KombinierBar: Kürbisgemüse: 1.55
KombinierBar: Hausgemachte Bio-Pasta: 2.05
KombinierBar: Twister: 2.05
KombinierBar: Bratkartoffeln: 2.05
KombinierBar: Mandelrosenkohl: 1.55
KombinierBar: Sommergemüse: 1.55
KombinierBar: Geflügelbälle "Chili Cheese": 3.65
KombinierBar: 7 Gemüsenuggets: 4.90
KombinierBar: Panierter Hirtenkäse: 2.80
Ausgabe 1: Mini-Knödel, Schmorzwiebeln, Rustica Carrots, Salat, Ceasars-Dressing: 5.75
Ausgabe 2: Hühnersuppe, Fadennudeln: 5.10
Ausgabe 3: Cannelloni "Verdi", Mediterranes Gemüse, Tomatensauce, Rucola: 6.20
PastaWerk: Hausgemachte Bio-Pasta, Veganes Hack mit Tex-Mex-Gemüse, Käsesauce: 5.90
PastaWerk: Hausgemachte Bio-Pasta, Veganes Hack mit Tex-Mex-Gemüse, Petersilien-Limetten-Pesto: 5.90
KombinierBar: Gefüllte Paprikaschote, Gemüsepfanne "Mexiko", Reis: 2.80
```


# Current Features
Currently it only lists the food in the Central Mensa. I might add other locations later. Feel free to extend it yourself and send me a pull request!

# One more thing
Please be mindful when using this application and the requests it sends to the KQL database. Sending many invalid requests or accidentally flooding the server with requests by putting them in a loop could annoy the developers of the database server and make them restrict public access.