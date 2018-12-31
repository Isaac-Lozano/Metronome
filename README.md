# Metronome
A 1024-frame metronome for RNG manipulation in Sonic Adventure 2.

## Installation
The latest release is available
[here](https://github.com/Isaac-Lozano/Metronome/releases).

## Theory
When entering a level, RNG is seeded to a consistent state every time. This
means that missions such as Meteor Herd Mission 2 have consistent RNG and thus
consistent meteor spawns regardless of the frame you enter the stage. When
generating a set, however, the game will do a loop calling the RNG function
`frame_count % 1024` times.

Two things to note from this:
- If you enter twice on the same frame count, you will get the same set.
- If you enter twice with frame count being a multiple of 1024 apart, you will
get the same set.

The initial idea, then, would be to have a timer which ticks every 1024 frames
and use that to get a consistent set every time and then shift the timer so
that you land on the IL set every time. However, during loads, the frame
counter doesn't increment, so they must be accounted for.

Now it's time for the math.

First, we want to find out what our load delay is. We can do this by entering
the stage twice using a standard non-changing 1024-frame metronome. Entering
the stage for the first time can be represented by the equation:

![Hi there.](https://latex.codecogs.com/gif.latex?s&plus;d_e%3De_1%20%5Cmod%201024)

Where `s` is the frame you press A to enter the level, `d_e` is the amount of
load delay on entering, and `e_1` is the frame you enter the level on.

The second entry can be represented with:

![How are you?](https://latex.codecogs.com/png.latex?%5Cbegin%7Balign%7D%20%28s%20&plus;%201024k%29%20&plus;%20d_e%20&plus;%20d_l%20&plus;%20d_e%20%26%5Cequiv%20e_2%20%5Cmod%201024%20%5Cnonumber%20%5C%5C%20s%20&plus;%202d_e%20&plus;%20d_l%20%26%5Cequiv%20e_2%20%5Cmod%201024%20%5Cnonumber%20%5Cend%7Balign%7D)

Where `d_l` is the amount of load delay on leaving the level and `e_2` is the
frame you enter the level on your second entry. By combining these two
equations, you can figure out what your load time is.

![I'm glad you're reading this.](https://latex.codecogs.com/png.latex?d_e%20&plus;%20d_l%20%5Cequiv%20e_2%20-%20e_1%20%5Cmod%201024)

Now for the practical application. By rearranging the previous equation, we can
find out our `e_2` frame based on our `e_1` frame and our load times.

![I enjoy math. It's fun.](https://latex.codecogs.com/png.latex?e_1%20&plus;%20d_l%20&plus;%20d_e%20%5Cequiv%20e_2%20%5Cmod%201024)

Now we can add a timer delay `d_t` such that we reach our target frame `T`. (I
also combine `d_l` and `d_e` into a single term `d` for simplicity.)

![FILLER TEXT](https://latex.codecogs.com/png.latex?e_2%20&plus;%20d_t%20%26%5Cequiv%20T%20%5Cmod%201024%20%5C%5C%20e_1%20&plus;%20d%20&plus;%20d_t%20%26%5Cequiv%20T%20%5Cmod%201024)

Now we can figure out our `d_t` and thus how long we need to wait by
rearranging the last equation.

![u ef ssabk aavf](https://latex.codecogs.com/png.latex?d_t%20%5Cequiv%20T%20-%20%28e_1%20&plus;%20d%29%20%5Cmod%201024)
