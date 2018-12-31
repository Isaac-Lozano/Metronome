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

## How to use
There are three phases to using this RNG manip.
1. Finding Load Delay.
2. Tuning the timer.
3. RUN MODE.

You only need to find your Load Delay once per stage (It does change slightly
between stages, it seems), but once you have your Load Delay value, you can
skip step 1.

### Finding Load Delay
To find load delay:
1. Start up the program and set Load Delay and Target Frame to 0.
2. Enter the level on a tick.
3. Note which set you got and reference a 1024 list to find out what frame you
landed on. (1024 list sold separately)
4. Exit the level.
5. Re-enter the level on another tick.
6. Find out what frame you landed on again and note it down.
7. Your load delay is `(second_entry_frame - first_entry_frame) % 1024`. (You
can type the equation in google to calculate it. It should be a number in
the range 0 to 1023)

### Everything else
Steps two and three technically are the same, but are differentiated only for
statistics purposes. The steps are:
1. Start up the program and set your Load Delay and Target Frame.
2. Enter the level on a tick.
3. Attempt your IL movement.
4. Note which set you got and reference a 1024 list to find out what frame you
landed on. (1024 list sold separately)
5. Press Enter in the metronome and type in what frame you got.
6. Exit the level and go to step 2.

The first time you enter the stage is your "tuning set". This set is ignored by
the printed-out statistics as it's basically random.

## Stats
This program prints out stats as you play. The meanings are as follows.
- n = number of attempts
- mean = your average distance from the target frame
- stdd = your standard deviation from the target frame
- mean_10 = your average distance from the target frame (last 10 attempts)
- stdd_10 = your standard deviation from the target frame (last 10 attempts)

By keeping track of your mean, you can use it to fine-tune your Load Delay. Say
your mean after 20 attempts is "-5", that means you should reduce your Load
Delay by 5 frames. Note that you shouldn't do this every time your mean isn't
0, but if it consistently is away from 0 after many attempts.
