# original code

- https://github.com/flippingbitss/rust-tetris

<hr />

# 게임이 중간에 원인없이 그냥 꺼짐...(이제 되나? rand버젼 올리니 해결?? 250710)
- 일부 블락이 우측 넘어가면 패닉남 코드 고쳐야함
- sdl2 어렵다. ㅠㅠ
- 디버깅이 역시 어려워.. 더러운 C언어.. 랩퍼로 땡기니 .... 디버깅이 안되네 --

<hr />

# run

- debug디버깅 테스트 해보기

```bash
$ RUST_LOG=debug cargo r --release
```

<hr />

# rust-tetris
playing around with SDL2 graphics in rust lang, prototype a tetris game in rust with no libs except SDL opengl graphics


## gameplay
This is what the game looks like 

screenshot_1.png           |  screenshot_2.png
:-------------------------:|:-------------------------:
![Game Screenshot 1](/screenshot_1.png?raw=true "Game Screenshot 1") |  ![Game Screenshot 2](/screenshot_2.png?raw=true "Game Screenshot 2")


## controls
 Key | Action
 :-------------------------:|:-------------------------:
 Left or Right Arrow | Move Horizontal
 Down Arrow | Move Piece Down
 Upper Arrow | Rotate Piece
 Space | Drop piece
 
 
## installation
download the repo and use 
`cargo build --release` in root directory of the repo to build the executable

this game uses SDL2 for graphics so you might have to install SDL2 for your respective OS.
