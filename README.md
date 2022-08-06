# guessing-game
An implementation of the guessing game tutorial as described on the Rust website - https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html. 

The game is summarized as a lottery game, where a randomly selected winning number exists between 1-100 (integer)

The user is repeatedly prompted to select a number (integer) between 1-100: 

```
Winning number = 83

Guess a number between 1-100. 3 out of 3 tries remaining.
```

- if the winning number is selected, winning message is displayed 

```console
Winning number = 49

Guess a number between 1-100. 3 out of 3 tries remaining.
49
You guessed: 49
Winning number = 49
You win!
```

- if the winning number is not selected, re-prompt  
```console
Guess a number between 1-100. 3 out of 3 tries remaining.
1
You guessed: 1
Too small!

Guess a number between 1-100. 2 out of 3 tries remaining.
2
You guessed: 2
Too small!
```

until maximum number of tries is exhausted, then inform of loss
```console
Guess a number between 1-100. 1 out of 3 tries remaining.
3
You guessed: 3
Too small!
You get nothing! You lose! Good day sir!
```

![](https://media.giphy.com/media/10h8CdMQUWoZ8Y/giphy.gif)

# Bonus Feature(s)
- Maximum 3 tries allowed 
- Exclude invalid entries (out of range, non-numbers, etc.) from being counted as attempts
