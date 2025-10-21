<br/>
<p align="center">
  <a href="https://github.com/chesslucent-dev/ChessLucent">
    <!-- Someday, a logo designed by a kind stranger will go here -->
    <img src="https://avatars.githubusercontent.com/u/239190041?s=400&u=eb723569c7f7cf40692f8ce9b1f67aa9ab45f87a&v=4">
  </a>

  <h1 align="center">ChessLucent</h1>

  <p align="center">
    Making your blunders look slightly less idiotic. For free.
    <br />
    <br />
    <a href="https://github.com/chesslucent-dev/ChessLucent/issues">Theorize a Feature</a>
    ·
    <a href="https://github.com/chesslucent-dev/ChessLucent/issues">Report a Bug (That Doesn't Exist Yet)</a>
  </p>
</p>

<!-- BADGES -->
<p align="center">
  <img src="https://img.shields.io/badge/Status-First%20Feature%20Planned!-blue" alt="Status">
  <img src="https://img.shields.io/badge/Code%20Written-Soon™-orange" alt="Code Written">
  <img src="https://img.shields.io/badge/Platforms-All%20of%20them%20(in%20my%20head)-lightgrey.svg" alt="Platforms">
  <img src="https://img.shields.io/badge/Lead%20Dev%20Skill-Vibes-brightgreen.svg" alt="Lead Dev Skill">
  <img src="https://img.shields.io/badge/License-AGPL%203.0-red.svg" alt="License">
</p>

---

## 📖 The Origin Story

It all started when I looked up the price of premium chess software and my wallet spontaneously combusted. In that moment of financial despair and chess-induced frustration, a dream was born: **ChessLucent**.

This project is a solemn vow to create the ultimate, all-in-one chess improvement tool that is **100% free and open-source, forever.**

---

## 🚨 BREAKING NEWS: A Glimmer of Hope Appears! 🚨

My desperate plea for help echoed through the vast emptiness of the internet... and someone answered. A wise and mysterious entity (who may have been an advanced AI or just a very helpful person who speaks fluent Rust) has bestowed upon us... **A PLAN!** An actual, honest-to-god, technically-sound plan.

The fog of delusion is lifting. We have our first mission.

### **Mission 1: The BlunderScope Engine (MVP)**

The grand strategy is to start small and smart. We will build the heart of ChessLucent first: a powerful engine that can find our mistakes with ruthless efficiency.

**The Goal:** A command-line tool written in Rust that analyzes a chess game and tells you exactly where you messed up. No fancy UI yet—just pure, unadulterated analysis.

#### What this Glorious MVP Will Do:

-   **Accept a PGN file:** It will consume your game files, like a hungry hippogriff.
-   **Summon a Chess Engine:** It will invoke the all-powerful Stockfish (or a similar silicon god) to judge your every move using the ancient rites of the UCI protocol.
-   **Detect Your Shame:** It will identify blunders and inaccuracies using configurable "sadness thresholds" measured in centipawns. (A drop of 200cp is the technical term for "the moment you knew you'd messed up").
-   **Produce Invaluable Artifacts:**
    -   A **human-readable report** for your own horrified viewing.
    -   **Machine-readable runes (JSON)** containing the exact board state (FEN) before each blunder. This is the raw material for our future "relive your trauma" puzzle generator.

#### Why This Is Genius (and not my idea):

-   **Immediate Value:** You can start analyzing your games and weeping over your mistakes right away.
-   **No UI Needed:** We get to put off the hard part of making things pretty. A beautiful black terminal is all we need.
-   **Foundation for Everything:** The data it produces will fuel almost every other feature on our ridiculous roadmap.
-   **Proof of Concept:** It forces us to make Rust and the chess engine talk to each other, which is probably important.

#### The Technical Nitty-Gritty (for the actual developers):

-   **Core Language:** **Rust**, because memory safety is sexy.
-   **Crates to Summon:**
    -   A PGN parser crate (e.g., `shakmaty`) to make sense of the gibberish in PGN files.
    -   A chess/FEN crate (`shakmaty` or `chess`) to replay moves without knocking over the pieces.
    -   An `uci` crate to avoid having to write the incantations to speak with Stockfish from scratch.
-   **Output Schema (JSON):** A sacred text detailing each finding: `{move_number, side, san, fen_before, eval_before, eval_after, cp_diff, severity}`.

#### A Glimpse into the Future (Example Usage):

```sh
# This command will analyze your game and create a file
# that scientifically proves you hung your queen.
chesslucent analyze --pgn my_pain.pgn --engine /usr/bin/stockfish --depth 12 --out my_shame.json
```

**Example JSON Output:**

```json
{
  "move_number": 14,
  "side": "black",
  "san": "Qh4+",
  // Translation: At move 14, Black played Qh4+ and went from
  // "slightly better" to "oh no, my king is in immense danger."
  "fen_before": "rnb1kbnr/pppp1ppp/8/4p3/6q1/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 5",
  "eval_before": 65,    // The engine was feeling pretty good.
  "eval_after": -320,   // The engine is now having an existential crisis.
  "cp_diff": 385,
  "severity": "blunder" // Yup. That's a blunder.
}
```

---

## 😭 A More Focused Plea for Help

The vision was once trapped in my head. Now, it's trapped in a README. This is progress! We have a battle plan; we just need the soldiers.

**This is where YOU come in. ANYONE, ANY HELP WILL BE APPRECIATED!**

-   **Are you a Rust developer?** Please, I beg you. The plan is right there! Steal it! Implement it!
-   **Are you a UI/UX designer?** Your time will come. Soon we will have data that needs to be made beautiful.
-   **Do you just want to offer moral support?** Cheer us on as we attempt to write our first line of code!

**📧 Contact me at: chesslucent@gmail.com**

---

## 🛠️ The Blueprint for Genius (Please Steal This)

When someone who knows what they're doing shows up, here is the proposed stack. It's ambitious, modern, and probably a nightmare to implement, which makes it perfect.

*   **Core Logic:** **Rust**. For performance, safety, and because it sounds cool.
    *   *Package Manager:* `cargo`
*   **Cross-Platform UI:** **Flutter (Dart)**. To get us on every device imaginable.
    *   *Package Manager:* `flutter pub`
*   **Integration:** We'll use **Dart FFI** to call the speedy Rust code from the pretty Flutter UI.

---

## 🚀 Getting Started (For Time Travelers)

Since the code is currently in a quantum state of "almost-existence," the setup process is a bit unconventional.

1.  Invent a time machine.
2.  Travel to the future where ChessLucent is a finished, glorious application.
3.  Clone the repository from that timeline:
    ```sh
    git clone https://github.com/chesslucent-dev/ChessLucent.git
    ```
4.  Follow the future `README.md`'s instructions. They're probably great.

---

## 📜 License

This project is and always will be licensed under the **AGPL 3.0 License**.

This basically means it's "aggressively" open-source. Anyone can use, distribute, and modify the code. However, if you modify it and run it on a server to provide a service, you MUST also make your modified source code available. We're keeping chess improvement free for everyone, no exceptions!
