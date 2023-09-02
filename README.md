# Advent of Code 2022 to learn rust
This repo is just my code as I work through [Advent of Code](https://adventofcode.com/) in order to learn rust.

I was urged not to do this yet, to instead go back and walk through [rustlings](https://github.com/rust-lang/rustlings) and to read [the book](https://www.rust-lang.org/learn). I certainly might do that, I too often confuse `rustc` for `cargo build` and make trivial errors. Yet, I am largely up and running, and [the Primeagen](https://github.com/ThePrimeagen) recently suggested advent of code is a great way to begin to learn a new (ternary) language in [How to Learn A New Programming Language](https://youtu.be/E8cM12jRH7k). Further in the rust discord several people over the past week suggested _Advent of Code_ as a good way to learn the language. None of this means that working through _rustlings_ and taking time to read _the book_ is not worht my time, but I would like to see how far I will go before wanting to come back and find the kernels of knowledge I wish I knew earlier. :)

### Takeaways 
done!

I need to go back and look at rustlings now. Towards the end I really began to understand when to use rc's, but I need to focus more on how. Too often I prefer to use GPT to solve a borrow issue still, though less since Rcs became a part of my repertoire. I also found that, after submitting and scanning the reddit, others had found a much faster solution, sometimes considerably so. I often fall back on deconstructing a problem into a standard form instead of working with the raw data. 

Looking forward to contributing more to good-lp, and seeing where this goes :)

## Highlights

Days 15 and 22 were brutal. For day 15 part 2 I basically gave up solving it without any research, while day 22 part 2 I decided to work _really hard_ and do it myself. In the later case, I even found what turned out to be a unique way to fold the cube, but I still had a subtle bug with rotations (which were recalculated on the spot), and I ended up using a more typical approach instead fo fixing my unique approach. A few others were surprisingly difficult too, at least 7 of the days were more than one day to solve for me. But generally, even using this to learn a new language, everything was fairly facile. 

Some projects were firsts for me. 

### Day 1

Day 1 was not my first project in rust. For example, there were some lightweight server admin processes I wrote last year. This year I was working on learning more rust in leet code until I switched to Advent of Code. But this is the first project that wasn't born of necessity, and also wasn't just testing things locally.

### Day 2

tiny celebration here: got to check out [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs)

### Day 5

tiny celebration: first time using [regex](https://github.com/rust-lang/regex)

### Day 6

Got to check out the awesome [Shuttle](https://www.shuttle.rs/) and publish my solver on an api! :)

### Day 7

Got to play with [petgraph](https://github.com/petgraph/petgraph). This was awesome because there were a lot of problems on leetcode that gave me some trouble because implementing graphs is not elementary in rust.

### Day 8

Since Shuttle showed me the awesomeness of [axum](https://github.com/tokio-rs/axum) (adapted for shuttle), I decided to dive in with it.

### Day 9

Publishing to google [cloud function](https://cloud.google.com/run/docs/quickstarts/build-and-deploy/deploy-service-other-languages) using [hyper](https://github.com/hyperium/hyper) service.

### Day 12

Big day for me, exploring game programming with [bevy](https://bevyengine.org/). Also got to set up [features](https://doc.rust-lang.org/cargo/reference/features.html) for the first time. Did it as an isomorphic visualization and also allowed the user to set start and end points.
![screenshot](assets/day_12.png)

### Day 14

I made a not-terrible visualization in [pancurses](https://github.com/ihalila/pancurses).

![screenshot](assets/day_14.png)

Although this task does not really need it, I built it multi-threaded, which turned into a bit of a challenge because of the underlying curses library (which has no public repository).

### Day 15

I used day 15 to try out [dioxus](https://dioxuslabs.com/). 

I have to say it was a joy and a pain at the same time. React in rust, in the browser even, sounds great, but the reality is rsx! macro leaves you with runtime borrow errors and it is a tricky thing to work through while still learning rust. The amazing debugger errors, that sometimes aren't really so amazing in the first place but more often than not are, become far more often opaque in this environment. And I totally gave up on making it a mobile app, as it did not spin up the emulator after starting a mobile project.

Despite the negatives, and the fact that javascript speed improvements keep pushing the bar, in a way that probably obviates the virtual dom approach of dioxus in the long term, its amazingly javascript-y and delivers in full on its promise of giving you a rusty experience in html composition.

Part 2 was unsolvable in reasonable time without reading at least enough in searches to find that people were talking about lines and intersections.

![screenshot](assets/day_15.png)

### Day 18

tiny celebration here: got to check out [ndarray](https://github.com/ndarray/ndarray)

### Day 19

tiny celebration here: got to check out [good_lp](https://github.com/rust-or/good_lp)

### Day 21

this was a bit of a mixed bag. I quickly reached completion in pt 1, and a day later I had located the symbolic math computer algerba system (CAS) [savage](https://github.com/p-e-w/savage). Savage could not solve for the variable though, equations of the form `2*a=10`, etc, require that you supply the value for "a". So I went to python's sympy just to make sure I wasn't asking too much from a CAS. This worked from my equation string just fine, I assumed. I did not submit the result, I still wanted to try to solve in rust. 

I also found [rusymbols](https://github.com/simensgreen/rusymbols), but it needed the expression to be built operation-wise. So, I found the parser lib [chumsky](https://github.com/zesterer/chumsky): Chumsky's [error messages can get huge fast](https://github.com/zesterer/chumsky/issues/485), but this was not too difficult to convert to an AST of (custom enum of) Tokens. I then convert the tokens to rusymbol's Expressions, and.. I ran into errors. Some were mine, but among them, rusymbols itself appears to suffer from low precision.  I wrote Display implementations of my Tokens and Vec of Token, so I could recovert it to a String, and try to pass the factored equation to savage again. But savage produces a stack overflow parsing such a long and complicated expression.  

tl-dr, eventually I went back to the equation and solved it in a more mundane way with [evalexpr](https://docs.rs/evalexpr/latest/evalexpr/) and regex.
