# Rhabarberbar

> [!WARNING]
> Something is currently wrong with this tool, the saves it generates can sometimes crash when browsing songs. Still investigating.

A command line tool for modifying the set of custom (downloaded/studio) songs inside a [Jam with the Band](https://en.wikipedia.org/wiki/Jam_with_the_Band) save file. Written in Rust.

## How do I install this?

Make sure you have [Rust](https://www.rust-lang.org/) installed.

```
git clone https://github.com/soxfox42/rhabarberbar.git
cd rhabarberbar
cargo install --path rhabarberbar-cli
```

## How do I use it?

Just run `rhabarberbar edit yoursave.sav`, then press <kbd>Enter</kbd> once you've added or removed songs in the window that appears.

## Why is it called "rhabarberbar"?

That's the name of a German tongue twister turned brief viral song. The full name is "Barbaras Rhabarberbar", or in English: "Barbara's Rhubarb Bar". So basically, it's a silly play on the JwtB mascot's name -- "Barbara the Bat".

## Does it support Daigasso Band Brothers DX?

Maybe? I haven't checked this myself, but it probably works alright. It will only be able to access 150 of the 200 available song slots in that version, though. The character encoding when unpacking files also won't match up, but non-ASCII characters are filtered right now anyway.

## Why did you write this now, 14 years after JwtB released?

I felt like playing some custom songs, but found that the existing tools for modifying saves were impossible or near-impossible to obtain, and only ran on Windows.

As for why I want to *play* the game now... there's no good reason.